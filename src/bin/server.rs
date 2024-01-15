use env_logger::Target;
use log::LevelFilter;
use tokio::net::UnixListener;

use pdns_remote::{constants::*, monitor, server};
use pdns_remote::state::State;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut builder = env_logger::builder();
    builder.target(Target::Stdout);
    builder.filter_module("pdns_remote", LevelFilter::Trace);
    builder.filter_level(LevelFilter::Warn);
    builder.init();

    let fn_name = "main";

    // todo: write state to disk in a configurable interval
    let mut state = State::new();
    state.test();

    let state_m = state.clone();
    tokio::spawn(async move { monitor::run(state_m).await });

    let listener = UnixListener::bind(SOCKET)?;
    server::run(listener, state.clone()).await;

    Ok(())
}

// Read exactly one line and answer to it.
/*
let mut reader = BufReader::new(read);
    let mut data = String::new();

    match reader.read_line(&mut data).await {
        Ok(_) => log::info!{"{} data read: {}", fn_name, data},
        Err(_) => log::error!{"{} read on stream failed.", fn_name}
    }

    let default = r#"{"result":false}"#.as_bytes();
    let _ = write.write_all(default).await;
 */

// Read lines until there is an empty line. Answer to it after reading has ended.
/*
let mut reader = BufReader::new(read);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if ! line.is_empty() {
            log::info!{"{} data read: {}", fn_name, line}
        } else {
            break;
        }
    }

    let default = r#"{"result":false}"#.as_bytes();
    let _ = write.write_all(default).await;
 */
