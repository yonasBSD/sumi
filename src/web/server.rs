use axum::{
    routing::get,
    response::Html,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::web::ws::{ws_handler, AppState};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Log {
    pub message: String,
    pub source: String,
}

pub async fn start(tx: broadcast::Sender<Log>, history: Arc<Mutex<Vec<Log>>>) {
    let app_state = AppState {
        tx,
        history,
    };

    let app = app(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app(app_state: AppState) -> Router {
    // Incluimos el HTML directamente en el binario
    let index_html = include_str!("../../static/index.html");

    Router::new()
        .route("/", get(move || async move { Html(index_html) }))
        .route("/ws", get(ws_handler))
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}
