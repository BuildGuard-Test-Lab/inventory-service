use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        service: "inventory-service".to_string(),
    })
}

async fn list_inventory() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"items": [], "total": 0}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/api/v1/inventory", web::get().to(list_inventory))
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}
