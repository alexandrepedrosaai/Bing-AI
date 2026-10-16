use axum::{
    routing::post,
    Json,
    Router
};

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct SearchRequest {
    query: String
}

#[derive(Serialize)]
struct SearchResponse {
    results: Vec<String>
}

async fn search(
    Json(req): Json<SearchRequest>
) -> Json<SearchResponse> {

    Json(SearchResponse {

        results: vec![
            format!("Result for {}", req.query)
        ]
    })
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/search", post(search));

    let listener =
        tokio::net::TcpListener::bind(
            "0.0.0.0:8080"
        )
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
