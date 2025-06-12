use std::error::Error;

// cargo run --package okapi-utoipa --bin generate --features generate
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	#[cfg(not(feature = "generate"))]
	panic!(r#"A "generate" feature should be enabled"#);

	#[cfg(feature = "generate")]
	{
		use axum_test::TestServer;
		use okapi_utoipa::{router::create_router, users::User};

		let store = User::create_store();
		let router = create_router(store)?;
		let server = TestServer::new(router)?;
		let res = server.get("/openapi").await;
		let schema = res.text();
		let schema = schema.replace(r#""openapi":"3.0.0""#, r#""openapi":"3.1.0""#); // utoipa requirement
		std::fs::write("openapi.json", schema)?;
		println!("OpenApi schema successfully generated");
		return Ok(());
	}
}
