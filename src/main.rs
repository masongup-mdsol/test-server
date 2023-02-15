use axum::{
    error_handling::HandleErrorLayer, extract::Extension, http::StatusCode, routing::get, BoxError,
    Router,
};
use mauth_client::{tower::MAuthValidationLayer, MAuthValidationError, ValidatedRequestDetails};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let mauth_layer = MAuthValidationLayer::from_default_file().unwrap();
    let mauth_error_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_mauth_error))
        .layer(mauth_layer);

    let app = Router::new()
        .route("/", get(mauth_hello_world).post(mauth_post))
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

async fn mauth_hello_world(Extension(rd): Extension<ValidatedRequestDetails>) -> String {
    format!("Hello World, app UUID is {}", rd.app_uuid).to_string()
}

async fn mauth_post(Extension(rd): Extension<ValidatedRequestDetails>, body: String) -> String {
    format!(
        "Got a signed post request from app UUID {} with body '{}'",
        rd.app_uuid, body
    )
    .to_string()
}
