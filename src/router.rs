use std::error::Error;

use axum::Router as AxumRouter;
use okapi_operation::{
	axum_integration::{Router as OkapiRouter, get},
	oh,
	okapi::openapi3::OpenApi as OkapiSpec,
};
use utoipa::openapi::OpenApi as UtoipaSpec;
use utoipa_swagger_ui::SwaggerUi;

use crate::users::{self as U, AppStateInner};

pub fn create_router(state: AppStateInner) -> Result<AxumRouter, Box<dyn Error>> {
	let router = OkapiRouter::new()
		.nest(
			"/api",
			OkapiRouter::new().nest(
				"/v1",
				OkapiRouter::new()
					.route(
						"/users",
						get(oh!(U::get_users_list)).post(oh!(U::create_user)),
					)
					.route(
						"/users/{id}",
						get(oh!(U::get_user_by_id))
							.put(oh!(U::update_user))
							.delete(oh!(U::delete_user)),
					),
			),
		)
		.with_state(state);

	let mut spec = router
		.generate_openapi_builder()
		.title(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.build()?;
	spec.openapi = "3.1.0".into(); // utoipa requirement

	let spec = convert_spec(spec)?;

	let router = router.axum_router();

	let router = router.merge(SwaggerUi::new("/swagger").url("/swagger.json", spec));

	Ok(router)
}

fn convert_spec(okapi_spec: OkapiSpec) -> Result<UtoipaSpec, serde_json::Error> {
	let spec = serde_json::to_string(&okapi_spec)?;
	serde_json::from_str::<UtoipaSpec>(&spec)
}
