use actix_web::{get, HttpResponse, Responder};
use reqwest::StatusCode;
use tracing::{debug, info};

#[get("/home_button")]
pub async fn home_button() -> impl Responder {
    let request_span = tracing::info_span!("home_button");

    let _enter = request_span.enter();

    info!("home button");

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
        >Home</button>
    ";

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response.to_string())
}

#[cfg(test)]
mod test {
    use actix_web::{http::header::ContentType, test, App};

    use crate::logic::index::home_button;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(home_button)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }
}
