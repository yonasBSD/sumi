use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use crate::log::event::{LogEvent, LogSource};

pub async fn run(command: Vec<String>, sender: mpsc::Sender<LogEvent>) -> anyhow::Result<()> {
    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut stdout_lines = BufReader::new(stdout).lines();
    let mut stderr_lines = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            line = stdout_lines.next_line() => {
                if let Ok(Some(text)) = line {
                    let _ = sender.send(LogEvent {
                        source: LogSource::Stdout,
                        message: text,
                        timestamp: std::time::SystemTime::now(),
                    }).await;
                }
            }
            line = stderr_lines.next_line() => {
                if let Ok(Some(text)) = line {
                    let _ = sender.send(LogEvent {
                        source: LogSource::Stderr,
                        message: text,
                        timestamp: std::time::SystemTime::now(),
                    }).await;
                }
            }
            else => break,
        }
    }

    let _ = child.wait().await;

    Ok(())
}
