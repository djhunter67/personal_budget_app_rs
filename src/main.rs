mod logic;

use actix_web::{web, App, HttpServer};

use personal_budget_app_rs::{
    home_button, About, Budget, Debt, HelloTemplate, Income, Investment, NotFound, Retirement,
    Savings,
};

use crate::logic::savings::savings_post;

use simplelog::{CombinedLogger, TermLogger, WriteLogger};
use std::fs::File;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let titled: &str = "Home";
    let abouted: &str = "About";
    let debted: &str = "Debt";
    let savinged: &str = "Savings";
    let incomed: &str = "Income";
    let invested: &str = "Investment";
    let retired: &str = "Retirement";
    let budgeted: &str = "Budget";
    let not_founded: &str = "404";

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

    // The web server
    let _ = HttpServer::new(|| {
        App::new()
            .route("/savings_input", web::post().to(savings_post)) // GET endpoint for htmx
            .service(web::resource("/").to(|| async {
                HelloTemplate {
                    title: "Hunter Home",
                    index: titled,
                } // Accessable in the HTML template; askama
            }))
            .service(home_button) // When home button is pressed.
            .service(web::resource("/about").to(|| async {
                About {
                    title: "Aboot",
                    about: abouted,
                } // Accessable in the HTML template; askama
            }))
            .service(web::resource("/debt").to(|| async {
                Debt {
                    title: "Debt",
                    debt: debted,
                } // Accessable in the HTML template; askama
            }))
            .service(web::resource("/savings").to(|| async {
                Savings {
                    title: "Savings",
                    savings: savinged,
                } // Accessable in the HTML template; askama
            }))
            .service(web::resource("/income").to(|| async {
                Income {
                    title: "Income",
                    income: incomed,
                } // Accessable in the HTML template; askama
            }))
            .service(web::resource("/investment").to(|| async {
                Investment {
                    title: "Investment",
                    investment: invested,
                } // Accessable in the HTML template; askama
            }))
            .service(web::resource("/retirement").to(|| async {
                Retirement {
                    title: "Retirement",
                    retirement: retired,
                } // Accessable in the HTML template; askama
            }))
            .service(web::resource("/budget").to(|| async {
                Budget {
                    title: "Budget",
                    budget: budgeted,
                } // Accessable in the HTML template; askama
            }))
            .service(web::resource("/404").to(|| async {
                NotFound {
                    title: "404",
                    not_found: not_founded,
                } // Accessable in the HTML template; askama
            }))
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
