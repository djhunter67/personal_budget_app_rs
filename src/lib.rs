use actix_web::{get, http::StatusCode, HttpResponse};
use askama::Template;

#[get("/home_button")]
pub async fn home_button() -> HttpResponse {
    println!("Home button clicked");

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
        >Home</button>
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
