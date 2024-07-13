use axum::{
    extract::State, middleware, response::IntoResponse, routing::{get, post}, Extension, Json, Router
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::user::User;

use super::{auth, AppState};

pub fn create_router(state: AppState) ->  Router<AppState> {
    Router::new()
        .nest("/api/protected", protected().layer(middleware::from_fn_with_state(state, auth::authorize)))
        .nest("/api/auth", auth())
}

fn auth() -> Router<AppState> {
    Router::new()
    .route("/login", post(auth::sign_in))
}

fn protected() ->  Router<AppState> {
    Router::new()
    .route("/user", get(get_user))
    .route("/user/modify", post(modify_user))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    username: String,
    email: String,
}

#[utoipa::path(
    get,
    path = "/api/protected/user",
    responses(
        (status = 200, description = "Success", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 440, description = "Token has expired")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(UserResponse {
        username: user.uid.clone(),
        email: user.mail.clone(),
    })
}

#[utoipa::path(
    post,
    path = "/api/protected/user/modify",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Unauthorized"),
        (status = 440, description = "Token has expired")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn modify_user(State(data): State<AppState>, Extension(user): Extension<User>) -> impl IntoResponse {
    let res = data.ldap.lock().await.users.modify_user(&user.uid, crate::common::user::ModifyUser::new().password("test".to_string())).await.unwrap();
    println!("Modify user result: {}", res);
    "User modified".to_string()
}