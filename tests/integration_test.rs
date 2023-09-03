use actix_web::{http, test, web, App};
use personal_budget_app_rs::{
    About, Budget, Debt, Finances, HelloTemplate, Income, Investment, NotFound, Retirement,
};

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

    assert_eq!(resp.status(), http::StatusCode::OK);
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
