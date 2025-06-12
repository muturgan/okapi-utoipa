use std::sync::Arc;

use axum::extract::{Json, Path, State};
use okapi_operation::{JsonSchema, *};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::error::ApiError;

pub(crate) type AppStateInner = Arc<RwLock<Vec<User>>>;
pub(crate) type AppState = State<AppStateInner>;

#[derive(Deserialize, JsonSchema)]
pub struct UserData {
	first_name: String,
	age: u8,
}

#[derive(Clone, Serialize, JsonSchema)]
pub struct User {
	id: usize,
	first_name: String,
	age: u8,
}

impl User {
	pub fn create_store() -> AppStateInner {
		RwLock::new(Vec::new()).into()
	}
}

#[derive(Serialize, JsonSchema)]
pub struct Success {
	success: bool,
}

#[openapi(summary = "Get users list", tags = "users")]
pub(crate) async fn get_users_list(State(state): AppState) -> Result<Json<Vec<User>>, ApiError> {
	let store = state.read().await;
	Ok(store.clone().into())
}

#[openapi(summary = "Get user by id", tags = "users")]
pub(crate) async fn get_user_by_id(
	State(state): AppState,
	Path(id): Path<usize>,
) -> Json<Option<User>> {
	let store = state.read().await;
	store.get(id).cloned().into()
}

#[openapi(summary = "Create a new user", tags = "users")]
pub(crate) async fn create_user(
	State(state): AppState,
	Json(UserData { first_name, age }): Json<UserData>,
) -> Json<Success> {
	let mut store = state.write().await;
	let id = store.len();
	store.push(User {
		id,
		first_name,
		age,
	});
	Success { success: true }.into()
}

#[openapi(summary = "Update user", tags = "users")]
pub(crate) async fn update_user(
	State(state): AppState,
	Path(id): Path<usize>,
	Json(UserData { first_name, age }): Json<UserData>,
) -> Json<Success> {
	let mut store = state.write().await;
	if let Some(user) = store.get_mut(id) {
		user.first_name = first_name;
		user.age = age;
		return Success { success: true }.into();
	};
	Success { success: false }.into()
}

#[openapi(summary = "Delete user", tags = "users")]
pub(crate) async fn delete_user(State(state): AppState, Path(id): Path<usize>) -> Json<Success> {
	let mut store = state.write().await;
	if store.len() < id {
		return Success { success: false }.into();
	}
	store.remove(id);
	Success { success: true }.into()
}
