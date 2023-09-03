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
            .route("/about", web::get().to(about))
            .route("/debt", web::get().to(debt))
            .route("/finance", web::get().to(finance))
            .route("/income", web::get().to(income))
            .route("/retirement", web::get().to(retirement))
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
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../frontend/index.html"))
}

async fn about() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../frontend/about.html"))
}

async fn debt() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../frontend/debt.html"))
}

async fn finance() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../frontend/finances.html"))
}

async fn income() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../frontend/income.html"))
}

async fn retirement() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../frontend/retirement.html"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_about() {
        let app = test::init_service(App::new().route("/about", web::get().to(about))).await;

        let req = test::TestRequest::get().uri("/about").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_debt() {
        let app = test::init_service(App::new().route("/debt", web::get().to(debt))).await;

        let req = test::TestRequest::get().uri("/debt").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_finance() {
        let app = test::init_service(App::new().route("/finance", web::get().to(finance))).await;

        let req = test::TestRequest::get().uri("/finance").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_income() {
        let app = test::init_service(App::new().route("/income", web::get().to(income))).await;

        let req = test::TestRequest::get().uri("/income").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_retirement() {
        let app =
            test::init_service(App::new().route("/retirement", web::get().to(retirement))).await;

        let req = test::TestRequest::get().uri("/retirement").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
