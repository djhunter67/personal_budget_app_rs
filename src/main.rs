mod logic;
mod telemetry;

use actix_web::{web, App, HttpServer};

use logic::index::home_button;
use personal_budget_app_rs::{
    about_button, budget_button, debt_button, income_button, investment_button, retirement_button,
    savings_button, About, Budget, Debt, HelloTemplate, Income, Investment, NotFound, Retirement,
    Savings,
};
use telemetry::{get_subscriber, init_subscriber};

use crate::logic::savings::savings_post;

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

    let subscriber = get_subscriber("personal_budget_app_rs".into(), "info".into());
    init_subscriber(subscriber);

    // The web server
    let _ = HttpServer::new(|| {
        App::new()
            .service(savings_post) // POST request
            .service(web::resource("/").to(|| async {
                HelloTemplate {
                    title: "Hunter Home",
                    index: titled,
                } // Accessable in the HTML template; askama
            }))
            .service(home_button) // When home button is pressed.
            .service(web::resource("/about").to(|| async {
                About {
                    title: "About",
                    about: abouted,
                } // Accessable in the HTML template; askama
            }))
            .service(about_button)
            .service(web::resource("/debt").to(|| async {
                Debt {
                    title: "Debted",
                    debt: debted,
                } // Accessable in the HTML template; askama
            }))
            .service(debt_button)
            .service(web::resource("/savings").to(|| async {
                Savings {
                    title: "Savings",
                    savings: savinged,
                } // Accessable in the HTML template; askama
            }))
            .service(savings_button)
            .service(web::resource("/income").to(|| async {
                Income {
                    title: "Income",
                    income: incomed,
                } // Accessable in the HTML template; askama
            }))
            .service(income_button)
            .service(web::resource("/investment").to(|| async {
                Investment {
                    title: "Investment",
                    investment: invested,
                } // Accessable in the HTML template; askama
            }))
            .service(investment_button)
            .service(web::resource("/retirement").to(|| async {
                Retirement {
                    title: "Retirement",
                    retirement: retired,
                } // Accessable in the HTML template; askama
            }))
            .service(retirement_button)
            .service(web::resource("/budget").to(|| async {
                Budget {
                    title: "Budget",
                    budget: budgeted,
                } // Accessable in the HTML template; askama
            }))
            .service(budget_button)
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
