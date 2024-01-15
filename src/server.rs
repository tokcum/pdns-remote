use std::io::ErrorKind;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{UnixListener, UnixStream};
use crate::state::State;

const SOCKET: &str = "/tmp/pdns-rust.socket";

/// Run the server.
///
/// Accepts connections from the supplied listener. For each inbound connection,
/// a task is spawned to handle that connection.
///
pub async fn run(listener: UnixListener, state: State) {
    let fn_name = "run";

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                // Connection succeeded.
                let state = state.clone();
                tokio::spawn(async move {
                    // Process each client separately.
                    handle_request(stream, state).await
                });
            }
            Err(e) => {
                // Connection failed.
                log::error!("{}, connection failed with {} .", fn_name, e);
            }
        }
    }
}

async fn handle_request(mut stream: UnixStream, state: State) -> std::io::Result<()> {
    let fn_name = "handle_request";

    log::info!("{}: client connected.", fn_name);

    let (read, mut write) = stream.split();

    // Ensure that socket is read- and writeable.
    // todo: implement proper timeout by measuring duration
    let i = 0;
    while read.readable().await.is_err() || write.writable().await.is_err() {
        log::warn!("{}: socket not yet read- or writeable.", fn_name);
        tokio::time::sleep(Duration::from_millis(100)).await;

        if i >= 5 {
            log::error!("{}: socket timed out.", fn_name);
            return Err(std::io::Error::new(
                ErrorKind::BrokenPipe,
                "read or write failed",
            ));
        }
    }

    let mut reader = BufReader::new(read);
    let mut writer = BufWriter::new(write);
    let mut lines = reader.lines();

    // todo: implement timeout, count down when no new lines are received, reset counter when new line received
    while let Some(line) = lines.next_line().await? {
        log::info! {"{} data read: {}", fn_name, line}

        let response = match state.get("www.google.de".to_string()) {
            None => b"{\"result\":false}\n".to_vec(),
            Some(value) => {
                if value % 2 == 0 {
                    b"{\"result\":even}\n".to_vec()
                } else {
                    b"{\"result\":odd}\n".to_vec()
                }
            },
        };

        match writer.write_all(&*response).await {
            Ok(_) => {
                writer.flush().await;
            },
            Err(_) => {
                log::error!("{}: write to socket failed.", fn_name);
                return Err(std::io::Error::new(
                    ErrorKind::BrokenPipe,
                    "read or write failed",
                ));
            }
        }
    }

    //

    log::info!("{}: client disconnected.", fn_name);

    Ok(())
}