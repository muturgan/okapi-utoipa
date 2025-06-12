use ::std::fs;
use utoipa::openapi::OpenApi;

pub(crate) fn get_schema() -> OpenApi {
	let str =
		fs::read_to_string("openapi.json").expect("OpenAPI schema reading from file system failed");

	serde_json::from_str::<OpenApi>(&str)
		.unwrap_or_else(|err| panic!("OpenAPI schema parsing error: {err}"))
}
