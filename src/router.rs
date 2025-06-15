use std::error::Error;

use axum::Router as AxumRouter;
use okapi_operation::{
	axum_integration::{Router, get},
	oh,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
	openapi::convert_spec,
	users::{self as U, AppStateInner},
};

pub fn create_router(state: AppStateInner) -> Result<AxumRouter, Box<dyn Error>> {
	let router = Router::new()
		.nest(
			"/api",
			Router::new().nest(
				"/v1",
				Router::new()
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
