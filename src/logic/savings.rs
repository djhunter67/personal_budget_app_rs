use actix_web::{web, Responder};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    information: String,
    more_data: String,
}

// Post request from /finances form
pub async fn savings_post(payload: web::Json<Info>) -> impl Responder {
    let info = payload.into_inner();
    println!("info: {}", info.information);
    println!("data: {}", info.more_data);

    "savings post"
}
