mod types;
mod handlers;

use axum::{
    response::IntoResponse, routing::{delete, get, post}, Json, Router, Server
};
use std::net::SocketAddr;
use tracing_subscriber;
use handlers::{health, account, get_account};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
.route("/health", get(health))
        .route("/accounts", post(account))
        .route("/accounts/:id", get(get_account));

    let domain = SocketAddr::from(([127,0,0,1] , 3000));

    tracing::info!("Server started on https://{}" , domain);

    axum::Server::bind(&domain).
    serve(app.into_make_service())
    .await
    .unwrap();
}