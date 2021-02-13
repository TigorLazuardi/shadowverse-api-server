use actix_server::Server;
use actix_web::{get, rt::System, App, HttpResponse, HttpServer, Result};
use clap::{crate_name, crate_version};
use serde::Serialize;
use std::{sync::mpsc, thread};

pub mod card;

pub fn new_server(port: String) -> Server {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");
        let server = HttpServer::new(|| App::new().service(health_check))
            .bind(format!("0.0.0.0:{}", port))
            .unwrap_or_else(|err| panic!("failed to bind on port {}: {}", port, err))
            .shutdown_timeout(30)
            .run();

        tx.send(server)
            .expect("failed to send server variable to handler thread");
        sys.run()
    });

    rx.recv().expect("failed to receive server from sub thread")
}

#[get("/")]
async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthCheck {
        message: "ok",
        version: crate_version!(),
        description: "shadowverse backend service",
        app: crate_name!(),
    }))
}

#[derive(Serialize)]
struct HealthCheck<'a> {
    message: &'a str,
    version: &'a str,
    description: &'a str,
    app: &'a str,
}
