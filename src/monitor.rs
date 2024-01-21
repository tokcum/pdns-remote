use std::time::Duration;
use crate::state::State;

/// Run the monitor.
///
pub async fn run(state: State) {
    let fn_name = "run";

    log::info!("{}, monitor started .", fn_name);

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

    log::debug!("{}: run monitor", fn_name);
    state.incr("www.google.de".to_string());
    tokio::time::sleep(Duration::from_millis(1000)).await;
    log::debug!("{}: monitor run", fn_name);

    Ok(())
}
