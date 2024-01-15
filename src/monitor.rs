use std::time::Duration;
use tokio::net::{UnixListener, UnixStream};
use crate::state::State;

/// Run the monitor.
///
pub async fn run(state: State) {
    let fn_name = "run";

    loop {
        let state_m = state.clone();
        tokio::spawn(async move {
            // Process each client separately.
            handle_monitor(state_m).await
        });

        tokio::time::sleep(Duration::from_millis(5000)).await;
    }
}

async fn handle_monitor(state: State) -> std::io::Result<()> {
    let fn_name = "handle_request";

    log::info!("{}: run monitor", fn_name);
    state.incr("www.google.de".to_string());
    tokio::time::sleep(Duration::from_millis(1000)).await;
    log::info!("{}: monitor run", fn_name);

    Ok(())
}