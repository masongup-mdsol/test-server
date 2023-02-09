use axum::{
    error_handling::HandleErrorLayer, extract::{Extension}, http::StatusCode, routing::get,
    BoxError, Router
};
use mauth_client::{MAuthValidationError, tower::MAuthValidationLayer};
use tower::ServiceBuilder;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mauth_layer = MAuthValidationLayer::from_default_file().unwrap();
    let mauth_error_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_mauth_error))
        .layer(mauth_layer);

    let app = Router::new()
        .route("/", get(mauth_hello_world))
        .layer(mauth_error_layer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_mauth_error(err: BoxError) -> (StatusCode, String) {
    match err.downcast_ref::<MAuthValidationError>() {
        Some(mauth_err) => (StatusCode::UNAUTHORIZED, format!("{}", mauth_err)),
        None => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
    }
}

async fn mauth_hello_world(Extension(app_uuid): Extension<Uuid>) -> String {
    format!("Hello World, app UUID is {}", app_uuid).to_string()
}