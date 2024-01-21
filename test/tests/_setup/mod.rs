use rand::distributions::{Alphanumeric, DistString};
use tokio::net::UnixListener;
use pdns_remote::state::State;

const SOCKET_DIR: &str = "/tmp";

pub fn listen() -> String {
    let rand = Alphanumeric.sample_string(&mut rand::thread_rng(), 5);
    let socket = format!("{SOCKET_DIR}/pdns-rust-{rand}.sock");
    let state = State::new();

    let listener = UnixListener::bind(&socket).expect("Failed to start listener.");
    let server = pdns_remote::server::run(listener, state.clone());
    let _ = tokio::spawn(server);

    socket
}
