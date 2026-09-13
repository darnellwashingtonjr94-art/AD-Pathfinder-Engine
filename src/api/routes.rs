use axum::{routing::get, routing::post, Json, Router};
use serde_json::{json, Value};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/analyze", post(analyze_graph))
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "healthy" }))
}

async fn analyze_graph(Json(payload): Json<Value>) -> Json<Value> {
    Json(json!({ "status": "analyzed", "received": payload }))
}
