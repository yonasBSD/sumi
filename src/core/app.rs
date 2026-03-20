use chrono::{DateTime, Local};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use owo_colors::OwoColorize;

use crate::log::event::{LogEvent, LogSource};
use crate::process::runner;
use crate::web::server::Log;

pub struct App;

impl App {
    pub async fn run(
        command: Vec<String>,
        log_tx: broadcast::Sender<Log>,
        history: Arc<Mutex<Vec<Log>>>,
    ) -> anyhow::Result<()> {
        let (tx, mut rx) = mpsc::channel::<LogEvent>(100);

        tokio::spawn(async move {
            let _ = runner::run(command, tx).await;
        });

        while let Some(event) = rx.recv().await {
            let time: DateTime<Local> = event.timestamp.into();
            let formatted_time = time.format("%H:%M:%S").to_string();

            let log_message = event.message.clone();

            match event.source {
                LogSource::Stdout => {
                    println!(
                        "{} | {} | {}",
                        formatted_time.dimmed(),
                        "stdout".green(),
                        &log_message
                    );
                }
                LogSource::Stderr => {
                    eprintln!(
                        "{} | {} | {}",
                        formatted_time.dimmed(),
                        "stderr".red(),
                        &log_message
                    );
                }
            }
            
            let log = Log {
                message: log_message,
                source: match event.source {
                    LogSource::Stdout => "stdout".to_string(),
                    LogSource::Stderr => "stderr".to_string(),
                },
            };

            // Store in history and broadcast under the same lock to avoid race conditions with new subscriptions
            {
                let mut history = history.lock().await;
                history.push(log.clone());
                let _ = log_tx.send(log);
            }
        }

        Ok(())
    }
}
