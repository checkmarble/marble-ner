use std::{env, sync::OnceLock};

use anyhow::Context;
use gliner::{
  model::{GLiNER, input::text::TextInput, params::Parameters, pipeline::span::SpanMode},
  text::span::Span,
};
use orp::params::RuntimeParameters;

#[cfg(feature = "gpu")]
use ort::execution_providers::CUDAExecutionProvider;

static MODEL: OnceLock<GLiNER<SpanMode>> = OnceLock::new();
static LABELS: OnceLock<Vec<&'static str>> = OnceLock::new();

pub fn model() -> &'static GLiNER<SpanMode> {
  MODEL.get_or_init(|| {
    tracing::info!("preloading model...");

    let cpus = std::thread::available_parallelism().ok().map(|cpus| cpus.get()).unwrap_or(4);
    let threads = env::var("THREADS").ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(cpus);
    let params = RuntimeParameters::default().with_threads(threads);

    #[cfg(feature = "gpu")]
    let params = params.with_execution_providers([CUDAExecutionProvider::default().build()]);

    let model = GLiNER::<SpanMode>::new(
      Parameters::default(),
      params,
      &format!("{}/tokenizer.json", env::var("MODEL_PATH").unwrap_or_else(|_| "./model".into())),
      &format!("{}/onnx/model.onnx", env::var("MODEL_PATH").unwrap_or_else(|_| "./model".into())),
    )
    .unwrap();

    tracing::info!("preloaded model");

    model
  })
}

pub fn labels() -> &'static Vec<&'static str> {
  LABELS.get_or_init(|| {
    env::var("GLINER_LABELS")
      .unwrap_or_else(|_| "Person,Organization".into())
      .split(",")
      .map(|label| Box::leak(label.to_string().into_boxed_str()) as &'static str)
      .collect::<Vec<_>>()
  })
}

pub fn infer(text: &str) -> anyhow::Result<Vec<Vec<Span>>> {
  let input = TextInput::from_str(&[text], labels()).map_err(|e| anyhow::anyhow!(e)).context("could not build input")?;
  let output = model().inference(input).map_err(|e| anyhow::anyhow!(e)).context("could not run inference")?;

  Ok(output.spans)
}
