use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use super::server::Log;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<Log>,
    pub history: Arc<Mutex<Vec<Log>>>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    let mut rx;
    let history_snapshot;

    // send history and subscribe under the same lock
    {
        let history = state.history.lock().await;
        history_snapshot = history.clone();
        rx = state.tx.subscribe();
    }

    // Send history snapshot
    for log in history_snapshot {
        let json_log = serde_json::to_string(&log).unwrap();
        if sender
            .send(Message::Text(json_log))
            .await
            .is_err()
        {
            return;
        }
    }

    // spawn a task to send messages to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(log) = rx.recv().await {
            let json_log = serde_json::to_string(&log).unwrap();
            if sender
                .send(Message::Text(json_log))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // spawn a task to receive messages from the client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Close(_) = msg {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
