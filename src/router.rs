use std::error::Error;

use axum::Router as AxumRouter;
use okapi_operation::{
	axum_integration::{Router as OkapiRouter, get},
	oh,
};
use utoipa::openapi::OpenApi as UtoipaSpec;
use utoipa_swagger_ui::SwaggerUi;

use crate::users::{self as U, AppStateInner};

pub fn create_router(state: AppStateInner) -> Result<AxumRouter, Box<dyn Error>> {
	let okapi_router = OkapiRouter::new()
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

	let spec = generate_spec(&okapi_router)?;

	let axum_router = okapi_router.axum_router();

	let axum_router = axum_router.merge(SwaggerUi::new("/swagger").url("/swagger.json", spec));

	Ok(axum_router)
}

fn generate_spec(router: &OkapiRouter) -> Result<UtoipaSpec, Box<dyn Error>> {
	let mut okapi_spec = router
		.generate_openapi_builder()
		.title(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.build()?;
	okapi_spec.openapi = "3.1.0".into(); // utoipa requirement

	let spec = serde_json::to_string(&okapi_spec)?;

	serde_json::from_str::<UtoipaSpec>(&spec).map_err(Into::into)
}
