use actix_web::{web, HttpRequest, HttpResponse, Error};
use rand::prelude::*;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use crate::config::{ServiceConfig, ServiceInstance, InstanceStatus};
use log::{info, error};
use url::Url;

pub async fn activate_service(
    services: Arc<RwLock<HashMap<String, ServiceConfig>>>,
    service_name: &str,
) {
    let mut services = services.write().await;
    if let Some(service) = services.get_mut(service_name) {
        for instance in service.instances.iter_mut() {
            if instance.active != InstanceStatus::Healthy {
                instance.active = InstanceStatus::Healthy;
                info!("Activated service instance: {} for {}", instance.url, service_name);
                break;
            }
        }
    } else {
        error!("Service not found for activation: {}", service_name);
    }
}

pub fn select_instance(service: &ServiceConfig) -> Option<&ServiceInstance> {
    let mut rng = rand::rng();
    let healthy_instances: Vec<&ServiceInstance> = service
        .instances
        .iter()
        .filter(|instance| instance.active == InstanceStatus::Healthy)
        .collect();
    healthy_instances.choose(&mut rng).copied()
}

pub async fn proxy(
    req: HttpRequest,
    body: web::Bytes,
    path: web::Path<(String, String)>,
    data: web::Data<crate::config::AppState>,
) -> Result<HttpResponse, Error> {
    let (service_name, tail) = path.into_inner();
    let services = data.services.clone();
    let client = &data.client;

    // Generate a request ID for logging
    let request_id = uuid::Uuid::new_v4().to_string();
    info!("[{}] Proxying request for service: {}", request_id, service_name);

    // Check and activate service if needed
    {
        let services_read = services.read().await;
        if let Some(service) = services_read.get(&service_name) {
            if !service.instances.iter().any(|i| i.active == InstanceStatus::Healthy) {
                drop(services_read); // Release read lock before write
                info!("[{}] No healthy instances found, attempting to activate service: {}", request_id, service_name);
                activate_service(services.clone(), &service_name).await;
            }
        } else {
            error!("[{}] Service not found: {}", request_id, service_name);
            return Err(actix_web::error::ErrorNotFound("Service not found"));
        }
    }

    // Fetch service and select instance
    let services_read = services.read().await;
    let service = services_read
        .get(&service_name)
        .ok_or_else(|| {
            error!("[{}] Service not found after activation: {}", request_id, service_name);
            actix_web::error::ErrorNotFound("Service not found")
        })?;

    let instance = select_instance(service)
        .ok_or_else(|| {
            error!("[{}] No healthy instances available for service: {}", request_id, service_name);
            actix_web::error::ErrorServiceUnavailable("No healthy instances")
        })?;

    // Construct URI safely
    let base_url = Url::parse(&instance.url).map_err(|e| {
        error!("[{}] Invalid instance URL {}: {}", request_id, instance.url, e);
        actix_web::error::ErrorBadGateway("Invalid upstream URL")
    })?;
    let uri = base_url.join(&tail).map_err(|e| {
        error!("[{}] Failed to construct URI with tail {}: {}", request_id, tail, e);
        actix_web::error::ErrorBadGateway("Invalid URI")
    })?;

    info!("[{}] Proxying to: {}", request_id, uri);

    // Create forwarded request with selective headers
    let mut forwarded_req = client.request(req.method().clone(), uri.as_str());
    for (key, value) in req.headers().iter() {
        if key != "Host" && key != "Connection" { // Skip sensitive headers
            forwarded_req = forwarded_req.header(key, value);
        }
    }

    // Send request with timeout
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        forwarded_req.send_body(body),
    )
        .await
        .map_err(|_| {
            error!("[{}] Upstream request timed out for: {}", request_id, uri);
            actix_web::error::ErrorRequestTimeout("Upstream timeout")
        })?
        .map_err(|e| {
            error!("[{}] Failed to forward request to {}: {}", request_id, uri, e);
            actix_web::error::ErrorBadGateway("Upstream error")
        })?;

    let status = response.status();
    let headers = response.headers();
    let body = response
        .bytes()
        .await
        .map_err(|e| {
            error!("[{}] Failed to read upstream response: {}", request_id, e);
            actix_web::error::ErrorInternalServerError("Error reading upstream response")
        })?;

    let mut client_resp = HttpResponse::build(status);
    for (key, value) in headers.iter() {
        client_resp.append_header((key.clone(), value.clone()));
    }

    Ok(client_resp.body(body))
}