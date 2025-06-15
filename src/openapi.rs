use okapi_operation::okapi::openapi3::OpenApi as OkapiSpec;
use utoipa::openapi::OpenApi as UtoipaSpec;

pub(crate) fn convert_spec(okapi_spec: OkapiSpec) -> Result<UtoipaSpec, serde_json::Error> {
	let spec = serde_json::to_string(&okapi_spec)?;
	serde_json::from_str::<UtoipaSpec>(&spec)
}
