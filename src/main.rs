use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use simplelog::{CombinedLogger, TermLogger, WriteLogger};
use std::fs::File;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    CombinedLogger::init(vec![
        TermLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("budget_rs.log").unwrap(),
        ),
    ])
    .unwrap();

    let _ = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
    })
    .bind(("127.0.0.1", 8123))
    .unwrap_or_else(|_| {
        log::warn!("Can not bind to port 8123");
        std::process::exit(1);
    })
    .run()
    .await;

    Ok(())
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
