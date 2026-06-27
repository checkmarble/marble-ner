mod auth;
mod inference;
mod trace;

use std::env;

use axum::{Json, Router, http::StatusCode, middleware, routing::*};
use serde::{Deserialize, Serialize};

use crate::{auth::Auth, inference::infer};

#[derive(Clone)]
pub struct AppState {
  api_key: String,
}

#[tokio::main]
async fn main() {
  let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);

  let _guard = trace::init_tracing(std::io::stdout());
  let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await.unwrap();

  let _label = inference::labels();
  let _preload = inference::model();

  let state = AppState {
    api_key: env::var("NER_API_KEY").expect("NER_API_KEY is required"),
  };

  let app = Router::new()
    .route("/", get(health))
    .route("/-/health", get(health))
    .route("/detect", post(handler))
    .layer(middleware::from_fn_with_state(state.clone(), trace::api_logger))
    .with_state(state);

  tracing::info!("listening on {}", listener.local_addr().unwrap());

  let _ = axum::serve(listener, app).with_graceful_shutdown(shutdown()).await;
}

async fn health() {}

#[derive(Deserialize)]
struct Input {
  text: String,
}

#[derive(Serialize)]
struct Output {
  kind: String,
  text: String,
}

async fn handler(_: Auth, Json(input): Json<Input>) -> (StatusCode, Json<Vec<Output>>) {
  match infer(&input.text) {
    Ok(output) => {
      if output.is_empty() {
        tracing::error!("inference returned no result");

        return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]));
      }

      let matches = output[0]
        .iter()
        .map(|span| Output {
          kind: span.class().to_string(),
          text: span.text().to_string(),
        })
        .collect::<Vec<_>>();

      (StatusCode::OK, Json(matches))
    }

    Err(err) => {
      tracing::error!(error = %err, "could not extract names");

      (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
    }
  }
}

async fn shutdown() {
  use tokio::signal::{self, unix::SignalKind};

  let ctrl_c = async { signal::ctrl_c().await.expect("failed to install sigint handler") };

  let terminate = async {
    signal::unix::signal(SignalKind::terminate()).expect("failed to install sigterm handler").recv().await;
  };

  tokio::select! {
      () = ctrl_c => tracing::info!("received ^C, initiating shutdown"),
      () = terminate => tracing::info!("received terminate signal, initiating shutdown")
  }
}
