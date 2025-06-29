use crate::config::AppState;
use actix_web::{HttpResponse, web};

pub async fn login(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "token": state.auth_token }))
}

pub async fn register(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "token": state.auth_token }))
}
