use std::{num::NonZeroU16, sync::Arc};

use axum::{
    body::Body, extract::{Json, Request, State}, http::{self, Response, StatusCode}, middleware::Next, response::IntoResponse
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{OpenApi, ToSchema};

use crate::common::Config;

use super::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        sign_in,
    ),
    components(schemas(
        SignInData,
        AuthBody
    ))
)]
pub(super) struct authApi;


#[derive(Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub iat: usize,
    pub username: String,
}

pub struct AuthError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

pub fn encode_jwt(username: String, config: Config) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::minutes(config.jwt_maxage.into());
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Cliams { iat, exp, username };
    let secret = config.jwt_secret;

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String, config: Config) -> Result<TokenData<Cliams>, StatusCode> {

    let result: Result<TokenData<Cliams>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn authorize(State(data): State<AppState>, mut req: Request, next: Next) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN
        })?,
        None => return Err(AuthError {
            message: "Please add the JWT token to the header".to_string(),
            status_code: StatusCode::FORBIDDEN
        }),
    };

    let mut header = auth_header.split_whitespace();

    let (bearer, token) = (header.next(), header.next());
    if bearer != Some("Bearer") || token.is_none() {
        return Err(AuthError {
            message: "Invalid token".to_string(),
            status_code: StatusCode::FORBIDDEN
        });
    }

    let token_data = match decode_jwt(token.unwrap().to_string(), data.env) {
        Ok(data) => data,
        Err(_) => return Err(AuthError {
            message: "Unable to decode token".to_string(),
            status_code: StatusCode::UNAUTHORIZED
        }),
    };

    if token_data.claims.exp < Utc::now().timestamp() as usize {
        return Err(AuthError {
            message: "Token has expired".to_string(),
            status_code: StatusCode::from_u16(440).unwrap()
        });
    }

    let current_user = match data.ldap.lock().await.users.user(&token_data.claims.username).await {
        Some(user) => user,
        None => return Err(AuthError {
            message: "You are not an authorized user".to_string(),
            status_code: StatusCode::UNAUTHORIZED
        }),
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}


#[derive(Deserialize, ToSchema)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = SignInData,
    responses(
        (status = 401, description = "Unauthorized"),
        (status = 200, description = "Success", body = AuthBody)
    )
)]
pub async fn sign_in(
    State(data): State<AppState>,
    Json(user_data): Json<SignInData>
) -> Result<Json<AuthBody>, StatusCode> {
    let user = match data.ldap.lock().await.users.user(&user_data.username).await {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !user.verify_password(&user_data.password)
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.uid, data.env)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth = AuthBody {
        access_token: token,
        token_type: "Bearer".to_string(),
    };

    Ok(Json(auth))
}