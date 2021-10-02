use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscripeData {
    name: String,
    email: String
}

pub async fn subscripe(form: web::Form<SubscripeData>) -> impl Responder {
    HttpResponse::Ok().finish()
}