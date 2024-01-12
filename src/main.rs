use env_logger::Target;
use log::LevelFilter;

use std::io::{BufRead, BufReader};
use tokio::net;
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

const SOCKET: &str = "/tmp/pdns-rust.socket";

fn handle_request(stream: UnixStream) -> std::io::Result<()> {
    let fn_name = "handle_request";

    log::info!("{}: client connected.", fn_name);

    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                log::info!("{}: request received: \"{}\"", fn_name, line);

                //let mut stream = tokio::net::UnixStream::connect(SOCKET).await?;
                // Split the UnixStream into read and write halves
                //let (mut read_half, mut write_half) = stream.split();
                //write_half.try_write("Hello".as_bytes());
            },
            Err(_) => log::error!("{}, read failed.", fn_name)
        }
    }

    log::info!("{}: client disconnected.", fn_name);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut builder = env_logger::builder();
    builder.target(Target::Stdout);
    builder.filter_module("pdns_remote", LevelFilter::Trace);
    builder.filter_level(LevelFilter::Warn);
    builder.init();

    let fn_name = "main";

    let listener = UnixListener::bind(SOCKET)?;

    // Accept connections and process them, spawning a new thread for each request.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Connection succeeded.
                thread::spawn(|| handle_request(stream));
            }
            Err(_err) => {
                // Connection failed.
                log::error!("{}, connection to {} failed.", fn_name, SOCKET);
                break;
            }
        }
    }

    Ok(())
}
