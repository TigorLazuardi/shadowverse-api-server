use crate::services::http;
use signal_hook::{consts::TERM_SIGNALS, iterator::Signals};
use std::env;

/// run http server
#[actix_web::main]
pub async fn run() {
    let port = env::var("SHADOWVERSE_PORT").unwrap_or("3000".to_string());
    println!("http server running on port {}", port);
    let server = http::new_server(port);
    signal_waiter();
    println!("received interrupt signal");
    server.stop(true).await;
    println!("closed server");
}

/// waits for sigint or sigterm
fn signal_waiter() {
    let mut signals = Signals::new(TERM_SIGNALS).expect("failed to register term signals");
    signals.wait();
}
