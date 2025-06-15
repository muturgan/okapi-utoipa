use std::error::Error;

use okapi_utoipa::{router::create_router, users::User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let store = User::create_store();
	let router = create_router(store)?;
	let listener = tokio::net::TcpListener::bind(("0.0.0.0", 5678)).await?;
	println!(
		"Server started successfully. Let's open a swagger documentation at http://127.0.0.1:5678/swagger"
	);
	axum::serve(listener, router).await?;
	Ok(())
}
