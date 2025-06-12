use std::{
	error::Error,
	fmt::{self, Display, Formatter},
};

use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response},
};
use okapi_operation::{
	JsonSchema, ToResponses,
	okapi::openapi3::{RefOr, Response as OpenApiResponse, Responses},
	*,
};
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
pub struct ApiError {
	error: String,
}

impl Error for ApiError {}

impl Display for ApiError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "ApiError: {}", self.error)
	}
}

impl IntoResponse for ApiError {
	fn into_response(self) -> Response {
		(StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
	}
}

impl ToResponses for ApiError {
	fn generate(_components: &mut Components) -> Result<Responses, anyhow::Error> {
		Ok(Responses {
			default: Some(RefOr::Object(OpenApiResponse {
				description: "ApiError".into(),
				..Default::default()
			})),
			..Default::default()
		})
	}
}
