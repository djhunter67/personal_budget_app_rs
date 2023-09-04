mod logic;

use actix_web::{web, App, HttpServer};

use personal_budget_app_rs::{
    About, Budget, Debt, Finances, HelloTemplate, Income, Investment, NotFound, Retirement,
};

use crate::logic::finances::finances_post;

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
            .route("/savings_input", web::post().to(savings_post))
            .service(web::resource("/").to(|| async {
                HelloTemplate {
                    name: "Hunter",
                    title: "Home",
                }
            }))
            .service(web::resource("/about").to(|| async { About { title: "About" } }))
            .service(web::resource("/debt").to(|| async { Debt { title: "Debt" } }))
            .service(web::resource("/savings").to(|| async { Finances { title: "Savings" } }))
            .service(web::resource("/income").to(|| async { Income { title: "Income" } }))
            .service(web::resource("/investment").to(|| async {
                Investment {
                    title: "Investment",
                }
            }))
            .service(web::resource("/retirement").to(|| async {
                Retirement {
                    title: "Retirement",
                }
            }))
            .service(web::resource("/budget").to(|| async { Budget { title: "Budget" } }))
            .service(web::resource("/404").to(|| async { NotFound { title: "404" } }))
    })
    .workers(1)
    .bind(("127.0.0.1", 8123))
    .unwrap_or_else(|_| {
        log::warn!("Can not bind to port 8123");
        std::process::exit(1);
    })
    .run()
    .await;

    Ok(())
}
