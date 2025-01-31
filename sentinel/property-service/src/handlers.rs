use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::models::Property;
use crate::db::{self, PgPool};
use crate::kafka::{produce_event, PropertyEvent};

pub async fn  create_property(pool: web::Data<PgPool>, new_property: web::Json<Property>) -> HttpResponse {
    let property = new_property.into_inner();
    match db::create_property(&pool, &property).await {
        Ok(_) => {
            // Produce property created event
            let event = PropertyEvent::Created {
                id: property.id,
                address: property.address.clone(),
            };
            if let Err(e) = produce_event(event).await {
                eprintln!("Failed to produce event: {}", e);
            }
            HttpResponse::Created().json(property)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_property(pool: web::Data<PgPool>, property_id: web::Path<Uuid>) -> HttpResponse {
    match db::get_property(&pool, *property_id).await {
        Ok(Some(property)) => HttpResponse::Ok().json(property),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_properties(pool: web::Data<PgPool>) -> HttpResponse {
    match db::get_all_properties(&pool).await {
        Ok(properties ) => HttpResponse::Ok().json(properties),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_property(
    pool: web::Data<PgPool>,
    property_id: web::Path<Uuid>,
    updated_property: web::Json<Property>,
) -> HttpResponse {
    let mut property = updated_property.into_inner();
    property.id = *property_id;
    match db::update_property(&pool, &property).await {
        Ok(_) => HttpResponse::Ok().json(property),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_property(
    pool: web::Data<PgPool>,
    property_id: web::Path<Uuid>,
) -> HttpResponse {
    match db::delete_property(&pool, *property_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}