use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscripeData {
    name: String,
    email: String,
}

pub async fn subscripe(form: web::Form<SubscripeData>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscriped_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
