use actix_web::{web, App, HttpServer};

use askama::Template;

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
            .service(web::resource("/").to(|| async {
                HelloTemplate {
                    name: "Hunter",
                    title: "Home",
                }
            }))
            .service(web::resource("/about").to(|| async { About { title: "About" } }))
            .service(web::resource("/debt").to(|| async { Debt { title: "Debt" } }))
            .service(web::resource("/finances").to(|| async { Finances { title: "Finances" } }))
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

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    title: &'a str,
    name: &'a str,
}

#[derive(Template)]
#[template(path = "about.html")]
struct About<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "debt.html")]
struct Debt<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "finances.html")]
struct Finances<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "income.html")]
struct Income<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "retirement.html")]
struct Retirement<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "investment.html")]
struct Investment<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "budget.html")]
struct Budget<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFound<'a> {
    title: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(App::new().route(
            "/",
            web::get().to(|| async {
                HelloTemplate {
                    name: "Hunter",
                    title: "Home",
                }
            }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_about() {
        let app = test::init_service(App::new().route(
            "/about",
            web::get().to(|| async { About { title: "About" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/about").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_debt() {
        let app = test::init_service(
            App::new().route("/debt", web::get().to(|| async { Debt { title: "Debt" } })),
        )
        .await;

        let req = test::TestRequest::get().uri("/debt").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_finances() {
        let app = test::init_service(App::new().route(
            "/finances",
            web::get().to(|| async { Finances { title: "Finances" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/finances").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_income() {
        let app = test::init_service(App::new().route(
            "/income",
            web::get().to(|| async { Income { title: "Income" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/income").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_investment() {
        let app = test::init_service(App::new().route(
            "/investment",
            web::get().to(|| async {
                Investment {
                    title: "Investment",
                }
            }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/investment").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_retirement() {
        let app = test::init_service(App::new().route(
            "/retirement",
            web::get().to(|| async {
                Retirement {
                    title: "Retirement",
                }
            }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/retirement").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_budget() {
        let app = test::init_service(App::new().route(
            "/budget",
            web::get().to(|| async { Budget { title: "Budget" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/budget").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_404() {
        let app = test::init_service(App::new().route(
            "/404",
            web::get().to(|| async { NotFound { title: "404" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/404").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_index_returns_html() {
        let app = test::init_service(App::new().route(
            "/",
            web::get().to(|| async {
                HelloTemplate {
                    name: "Hunter",
                    title: "Home",
                }
            }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_about_returns_html() {
        let app = test::init_service(App::new().route(
            "/about",
            web::get().to(|| async { About { title: "About" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/about").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_debt_returns_html() {
        let app = test::init_service(
            App::new().route("/debt", web::get().to(|| async { Debt { title: "Debt" } })),
        )
        .await;

        let req = test::TestRequest::get().uri("/debt").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_finances_returns_html() {
        let app = test::init_service(App::new().route(
            "/finances",
            web::get().to(|| async { Finances { title: "Finances" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/finances").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_income_returns_html() {
        let app = test::init_service(App::new().route(
            "/income",
            web::get().to(|| async { Income { title: "Income" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/income").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_retirement_returns_html() {
        let app = test::init_service(App::new().route(
            "/retirement",
            web::get().to(|| async {
                Retirement {
                    title: "Retirement",
                }
            }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/retirement").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_investment_returns_html() {
        let app = test::init_service(App::new().route(
            "/investment",
            web::get().to(|| async {
                Investment {
                    title: "Investment",
                }
            }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/investment").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_budget_returns_html() {
        let app = test::init_service(App::new().route(
            "/budget",
            web::get().to(|| async { Budget { title: "Budget" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/budget").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[actix_web::test]
    async fn test_404_returns_html() {
        let app = test::init_service(App::new().route(
            "/404",
            web::get().to(|| async { NotFound { title: "404" } }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/404").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }
}
