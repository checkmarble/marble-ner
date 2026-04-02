use std::{io::Write, net::SocketAddr, time::Instant};

use axum::{
  RequestPartsExt,
  body::{Body, HttpBody},
  extract::{ConnectInfo, Request},
  http::StatusCode,
  middleware::Next,
  response::Response,
};
use jiff::Timestamp;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(writer: impl Write + Send + 'static) -> WorkerGuard {
  let (appender, guard) = tracing_appender::non_blocking(writer);

  let formatter = json_subscriber::layer()
    .with_writer(appender)
    .flatten_event(true)
    .flatten_span_list_on_top_level(true)
    .with_current_span(false)
    .with_span_list(false);

  let filter = EnvFilter::builder().try_from_env().or_else(|_| EnvFilter::try_new("info")).unwrap().and_then(formatter);

  tracing_subscriber::registry().with(filter).init();

  guard
}

pub async fn api_logger(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
  let time = Timestamp::now().strftime("%Y-%m-%dT%H:%M:%S%z").to_string();
  let method = request.method().clone();
  let uri = request.uri().clone();

  let (mut parts, body) = request.into_parts();
  let ip = if let Ok(ConnectInfo(addr)) = parts.extract::<ConnectInfo<SocketAddr>>().await {
    addr.ip().to_string()
  } else {
    "-".to_string()
  };

  let then = Instant::now();
  let response = next.run(Request::from_parts(parts, body)).await;

  tracing::info!(
    time = time,
    remote = ip,
    method = %method,
    path = uri.path(),
    status = response.status().as_u16(),
    latency = then.elapsed().as_millis(),
    size = response.size_hint().exact().unwrap_or(0),
    "{} {}",
    method,
    uri,
  );

  Ok(response)
}
