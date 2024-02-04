use actix_web::{get, http::StatusCode, HttpResponse, Responder};
use askama::Template;
use tracing::{debug, info};

#[get("/about_button")]
pub async fn about_button() -> HttpResponse {
    let request_span = tracing::info_span!("about_button");

    let _enter = request_span.enter();

    info!("about button");

    debug!("defining the style and structure of the request");

    let response: &str = "
	<button style=\"background-color: #4CAF50; /* Green */
  border: none;
  border-radius: 8px;
  color: white;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;\"
		>About</button>
	";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[get("/investment_button")]
pub async fn investment_button() -> HttpResponse {
    let request_span = tracing::info_span!("investment_button");

    let _enter = request_span.enter();

    info!("investment button");

    debug!("defining the style and structure of the request");

    let response: &str = "
	<button style=\"background-color: #4CAF50; /* Green */
  border: none;
  border-radius: 8px;
  color: white;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;\"
		>Investment</button>
	";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[get("/income_button")]
pub async fn income_button() -> impl Responder {
    let request_span = tracing::info_span!("income_button");

    let _enter = request_span.enter();

    info!("income button");

    debug!("defining the style and structure of the request");

    let response: &str = "
	<button style=\"background-color: #4CAF50; /* Green */
  border: none;
  border-radius: 8px;
  color: white;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;\"
		>Income</button>
	";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[get("/retirement_button")]
pub async fn retirement_button() -> HttpResponse {
    let request_span = tracing::info_span!("retirement_button");

    let _enter = request_span.enter();

    info!("retirement button");

    debug!("defining the style and structure of the request");

    let response: &str = "
	<button style=\"background-color: #4CAF50; /* Green */
  border: none;
  border-radius: 8px;
  color: white;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;\"
		>Retirement</button>
	";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[get("/debt_button")]
pub async fn debt_button() -> HttpResponse {
    let request_span = tracing::info_span!("debt_button");

    let _enter = request_span.enter();

    info!("debt button");

    debug!("defining the style and structure of the request");

    let response: &str = "
	<button style=\"background-color: #4CAF50; /* Green */
  border: none;
  border-radius: 8px;
  color: white;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;\"
		>Debt</button>
	";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[get("/budget_button")]
pub async fn budget_button() -> HttpResponse {
    let request_span = tracing::info_span!("budget_button");

    let _enter = request_span.enter();

    info!("budget button");

    debug!("defining the style and structure of the request");

    let response: &str = "
	<button style=\"background-color: #4CAF50; /* Green */
  border: none;
  border-radius: 8px;
  color: white;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;\"
		>Budget</button>
	";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[get("/savings_button")]
pub async fn savings_button() -> HttpResponse {
    let request_span = tracing::info_span!("savings_button");

    let _enter = request_span.enter();

    info!("savings button");

    debug!("defining the style and structure of the request");

    let response: &str = "
	<button style=\"background-color: #4CAF50; /* Green */
  border: none;
  border-radius: 8px;
  color: white;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;\"
		>Savings</button>
	";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub title: &'a str,
    pub index: &'a str,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct About<'a> {
    pub title: &'a str,
    pub about: &'a str,
}

#[derive(Template)]
#[template(path = "debt.html")]
pub struct Debt<'a> {
    pub title: &'a str,
    pub debt: &'a str,
}

#[derive(Template)]
#[template(path = "savings.html")]
pub struct Savings<'a> {
    pub title: &'a str,
    pub savings: &'a str,
}

#[derive(Template)]
#[template(path = "income.html")]
pub struct Income<'a> {
    pub title: &'a str,
    pub income: &'a str,
}

#[derive(Template)]
#[template(path = "retirement.html")]
pub struct Retirement<'a> {
    pub title: &'a str,
    pub retirement: &'a str,
}

#[derive(Template)]
#[template(path = "investment.html")]
pub struct Investment<'a> {
    pub title: &'a str,
    pub investment: &'a str,
}

#[derive(Template)]
#[template(path = "budget.html")]
pub struct Budget<'a> {
    pub title: &'a str,
    pub budget: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFound<'a> {
    pub title: &'a str,
    pub not_found: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::ContentType;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_about() {
        let app = test::init_service(App::new().service(about_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_debt() {
        let app = test::init_service(App::new().service(debt_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_savings() {
        let app = test::init_service(App::new().service(savings_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_income() {
        let app = test::init_service(App::new().service(income_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_retirement() {
        let app = test::init_service(App::new().service(retirement_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_investment() {
        let app = test::init_service(App::new().service(investment_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_budget() {
        let app = test::init_service(App::new().service(budget_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }
}
