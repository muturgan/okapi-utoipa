use std::error::Error;

use axum::Router as AxumRouter;
use okapi_operation::{
	axum_integration::{Router, get},
	oh,
};
#[cfg(not(feature = "generate"))]
use utoipa_swagger_ui::SwaggerUi;

#[cfg(not(feature = "generate"))]
use crate::openapi::get_schema;
use crate::users::{self as U, AppStateInner};

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
		.with_state(state)
		.finish_openapi(
			"/openapi",
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_VERSION"),
		)?;

	#[cfg(not(feature = "generate"))]
	let router = router.merge(SwaggerUi::new("/swagger").url("/swagger.json", get_schema()));

	Ok(router)
}
