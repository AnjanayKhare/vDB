mod models;
pub mod routes;

use axum::routing::post;
use axum::{
    routing::get,
    Json,
    Router,
};
use models::vector_manager::VecRequest;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::vector_manager;
use crate::models::vector_manager::{Collection, MakeCollectionsRequest};
use models::utils::Logger;
use tokio;

const PORT: &str = "127.0.0.1:3000";
const LOGGER: Logger = Logger::new("MAIN");

async fn handler(axum::extract::State(all_collection): axum::extract::State<Arc<Mutex<HashMap<String, Collection>>>>,
                 Json(body): Json<serde_json::Value>
) -> Json<serde_json::Value> {
    let collection = all_collection.lock().unwrap();

    Json(json!({
        "status": "ok",
        "message": "Hello, World!",
        "collection": collection.len(),
        "body": body
    }))
}

async fn make_collection(
    axum::extract::State(all_collection): axum::extract::State<Arc<Mutex<HashMap<String, Collection>>>>,
    Json(body): Json<MakeCollectionsRequest> ) -> Json<serde_json::Value> {
    let mut collection = all_collection.lock().unwrap();
    let dimension = body.dimension;
    let name = body.name;

    if collection.contains_key(&name) {
        LOGGER.warn(
            &format!("The collection named `{}` already exists", name)
        );
        return Json(json!({
            "status": "error",
            "message": &format!("The collection named `{}` already exists", name),
        }))
    }

    collection.insert(name.clone(), Collection::new(&name, dimension));
    LOGGER.info(&format!("Collection `{}` created", name));

    Json(
        json!(
            {
                "status": "ok",
                "message": format!("The collection `{}` created", name),
            }
        )
    )
}


async fn add_embedding_to_collections(
    axum::extract::State(all_collection): axum::extract::State<Arc<Mutex<HashMap<String, Collection>>>>,
    Json(body): Json<VecRequest> ) -> Json<serde_json::Value> {
    LOGGER.info(&format!("body: {:?}, {:#?}", body.data, body.collection));
    let mut collections = all_collection.lock().unwrap();
    let collection_name = &body.collection;
    let embedding = body.data;
    if let Some(collection) = collections.get_mut(collection_name) {
        if embedding.len() != collection.get_dimension() {
            LOGGER.debug(&format!("Collection `{}` is of different dimensions", collection_name));
            return Json(json!({
                "status": "error",
                "message": &format!("The collection `{}` is of different dimensions, expected dimensions {}, provided dimensions {}", collection_name, collection.get_dimension(), embedding.len()),
            }))
        }
        
        collection.add_embedding(vector_manager::EmbeddingVector::new(embedding));
        LOGGER.info(&format!("Collection `{}` added", collection_name));
        LOGGER.info(&format!("Current collection length: {}", collection.len()));
        return Json(json!(
            {
                "status": "ok",
                "message": "Successfully added vector."
            }
        ))
    }
    LOGGER.warn(
        &format!("Collection `{}` not found", collection_name)
    );

    Json(json!({
        "status": "error",
        "message": format!("Collection `{}` not found", collection_name),
    }))
}

async fn get_all_collection(
    axum::extract::State(all_collection): axum::extract::State<Arc<Mutex<HashMap<String, Collection>>>>,
) -> Json<serde_json::Value> {
    let collection = all_collection.lock().unwrap();
    let mut list = Vec::new();
    for (name, collection) in collection.iter() {
        list.push(json!({
            "name": name,
            "dimensions" : collection.get_dimension(),
        }))
    }

    Json(json!(list))
}

#[tokio::main]
async fn main() {
    let all_collection = Arc::new(Mutex::new(HashMap::<String, Collection>::new()));
    let router: Router<> = Router::new()
        .route("/", get(handler))
        .route("/collections", get(add_embedding_to_collections))
        .route("/make_collections", post(make_collection))
        .route("/collections/new", post(add_embedding_to_collections))
        .route("/all_collections", get(get_all_collection))
        .with_state(all_collection.clone());
    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();
    axum::serve(listener, router).await.expect("TODO: panic message");
}