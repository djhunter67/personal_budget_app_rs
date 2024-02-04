use actix_web::{post, web, Responder};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    information: String,
    more_data: String,
}

#[post("/savings")]
pub async fn savings_post(payload: web::Json<Info>) -> impl Responder {
    let info = payload.into_inner();

    let request_span = tracing::info_span!("savings_post");

    let _enter = request_span.enter();

    tracing::info!("info: {}", info.information);
    tracing::info!("data: {}", info.more_data);

    "savings post"
}
