use actix_web::{web, HttpResponse};
use crate::config::AppState;

pub async fn login(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "token": state.auth_token }))
}

pub async fn register(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "token": state.auth_token }))
}