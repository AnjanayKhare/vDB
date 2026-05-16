mod models;
pub mod routes;

use std::collections::HashMap;
use std::fmt::format;
use axum::{
    routing::get,
    Json,
    Router,
};
use serde_json::json;
use models::vector_manager::{
    VecRequest,
    MakeCollectionsRequest
};

use models::utils::Logger;
use tokio;

const PORT: &str = "127.0.0.1:3000";
const LOGGER: Logger = Logger::new("MAIN");

async fn handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "Hello, World!"
    }))
}

async fn new_vector(Json(body): Json<VecRequest> ) -> Json<serde_json::Value> {
    LOGGER.info(&format!("body: {:?}, {:#?}", body.data, body.collection));
    Json(
        json!({
            "data": body.data,
            "collection": body.collection
        })
    )
}

// async fn post_make_collections(Json(body) : Json<MakeCollectionsRequest> ) -> Json<serde_json::Value> {
//
// }

#[tokio::main]
async fn main() {
    // let all_collection = HashMap::new();
    let router: Router<> = Router::new()
        .route("/", get(handler))
        .route("/collections", get(new_vector));
    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();
    axum::serve(listener, router).await.expect("TODO: panic message");
}