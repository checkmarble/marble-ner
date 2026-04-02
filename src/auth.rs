use axum::{
  Json, RequestPartsExt,
  extract::{FromRef, FromRequestParts, State},
  http::{StatusCode, request::Parts},
};
use axum_extra::{
  TypedHeader,
  headers::{Authorization, authorization::Bearer},
};
use serde::Serialize;

use crate::AppState;

#[non_exhaustive]
pub(crate) struct Auth;

#[derive(Serialize)]
pub struct Error {
  error: String,
}

impl Error {
  pub fn new(message: &str) -> Self {
    Self { error: message.to_string() }
  }
}

impl<S> FromRequestParts<S> for Auth
where
  AppState: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = (StatusCode, Json<Error>);

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let State(state) = parts.extract_with_state::<State<AppState>, S>(state).await.unwrap();

    let header = parts
      .extract::<TypedHeader<Authorization<Bearer>>>()
      .await
      .map_err(|_| (StatusCode::UNAUTHORIZED, Json(Error::new("missing api key"))))?;

    if header.token() != state.api_key {
      return Err((StatusCode::UNAUTHORIZED, Json(Error::new("invalid api key"))));
    }

    Ok(Auth)
  }
}
