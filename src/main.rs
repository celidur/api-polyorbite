mod api_doc;
use api_doc::ApiDoc;
use tracing::Span;
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use tokio::time::timeout;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use api_polyorbite::{common::{Config, Ldap}, route::{create_router, AppState}};
use axum::http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        Method, Request,
    };
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = Config::init();

    let ldap_result = timeout(Duration::from_secs(5), Ldap::new(config.clone())).await;

    let mut ldap = match ldap_result {
        Ok(Ok(ldap)) => {
            tracing::debug!("🚀 Connected to the LDAP server successfully");
            ldap
        }
        Ok(Err(e)) => {
            tracing::debug!("🔥 Failed to connect to the LDAP server: {:?}", e);
            std::process::exit(1);
        }
        Err(_) => {
            tracing::debug!("🔥 Timeout while connecting to the LDAP server");
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::PATCH,
            Method::POST,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE, ORIGIN, ACCEPT, AUTHORIZATION]);
    let state = AppState::new(ldap, config);

    let app = create_router(state.clone())
        .merge(SwaggerUi::new("/doc").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            TraceLayer::new_for_http().on_request(|request: &Request<_>, _span: &Span| {
                println!("{:?} {}", request.method(), request.uri());
            }),
        )
        .layer(cors)
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4242")
        .await
        .unwrap();
    tracing::debug!("🚀 Server started successfully on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}