use actix_web::{get, HttpResponse, Responder, Result};
use crate::services::FlightService;

#[get("/api/flights")]
pub async fn flights_api() -> Result<impl Responder> {
    let flights = FlightService::fetch_todays_flights().await?;
    Ok(HttpResponse::Ok().json(flights))
}

#[get("/health")]
pub async fn health_check() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Local::now().to_rfc3339()
    })))
}
