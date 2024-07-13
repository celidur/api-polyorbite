use utoipa::{openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}, Modify, OpenApi};

use api_polyorbite::route::{route,auth};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
        modifiers(&SecurityAddon),
        paths(
            auth::sign_in,
            route::get_user
        ),
        components(
            schemas(
                api_polyorbite::route::auth::AuthBody,
                api_polyorbite::route::auth::SignInData,
                api_polyorbite::route::route::UserResponse
            )
        ),
        tags(
            (name = "Polyorbite", description = "Polyorbite API")
        )
    )]
pub struct ApiDoc;