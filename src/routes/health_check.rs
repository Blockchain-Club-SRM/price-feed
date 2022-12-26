use actix_web::{HttpResponse};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Price-Feed-Server-v1 : Working")
}