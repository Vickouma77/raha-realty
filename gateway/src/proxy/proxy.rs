use crate::config::{InstanceStatus, ServiceConfig, ServiceInstance};
use actix_web::{Error, HttpRequest, HttpResponse, error, web};
use log::{error, info};
use rand::prelude::*;
use reqwest::Method;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
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
                info!(
                    "Activated service instance: {} for {}",
                    instance.url, service_name
                );
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
    info!(
        "[{}] Proxying request for service: {}",
        request_id, service_name
    );

    // Check and activate service if needed
    {
        let services_read = services.read().await;
        if let Some(service) = services_read.get(&service_name) {
            if !service
                .instances
                .iter()
                .any(|i| i.active == InstanceStatus::Healthy)
            {
                drop(services_read); // Release read lock before write
                info!(
                    "[{}] No healthy instances found, attempting to activate service: {}",
                    request_id, service_name
                );
                activate_service(services.clone(), &service_name).await;
            }
        } else {
            error!("[{}] Service not found: {}", request_id, service_name);
            return Err(error::ErrorNotFound("Service not found"));
        }
    }

    // Fetch service and select instance
    let services_read = services.read().await;
    let service = services_read.get(&service_name).ok_or_else(|| {
        error!(
            "[{}] Service not found after activation: {}",
            request_id, service_name
        );
        error::ErrorNotFound("Service not found")
    })?;

    let instance = select_instance(service).ok_or_else(|| {
        error!(
            "[{}] No healthy instances available for service: {}",
            request_id, service_name
        );
        error::ErrorServiceUnavailable("No healthy instances")
    })?;

    // Construct URI safely
    let base_url = Url::parse(&instance.url).map_err(|e| {
        error!(
            "[{}] Invalid instance URL {}: {}",
            request_id, instance.url, e
        );
        error::ErrorBadGateway("Invalid upstream URL")
    })?;
    let uri = base_url.join(&tail).map_err(|e| {
        error!(
            "[{}] Failed to construct URI with tail {}: {}",
            request_id, tail, e
        );
        error::ErrorBadGateway("Invalid URI")
    })?;

    info!("[{}] Proxying to: {}", request_id, uri);

    // Create forwarded request with selective headers
    let method = req.method().as_str().parse::<Method>().map_err(|e| {
        error!("Invalid HTTP method: {}", e);
        error::ErrorBadRequest("Invalid HTTP method")
    })?;
    let mut forwarded_req = client.request(method, uri.as_str());
    for (key, value) in req.headers().iter() {
        if key != "Host" && key != "Connection" {
            if let Ok(val_str) = value.to_str() {
                forwarded_req = forwarded_req.header(key.as_str(), val_str);
            }
        }
    }

    // Send request with timeout
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        forwarded_req.body(body.clone()).send(),
    )
    .await
    .map_err(|_| {
        error!("[{}] Upstream request timed out for: {}", request_id, uri);
        error::ErrorRequestTimeout("Upstream timeout")
    })?
    .map_err(|e| {
        error!(
            "[{}] Failed to forward request to {}: {}",
            request_id, uri, e
        );
        error::ErrorBadGateway("Upstream error")
    })?;

    let status =
        actix_web::http::StatusCode::from_u16(response.status().as_u16()).map_err(|e| {
            error!("[{}] Invalid status code: {}", request_id, e);
            error::ErrorInternalServerError("Invalid status code")
        })?;

    let headers = response.headers().clone();
    let body = response.bytes().await.map_err(|e| {
        error!("[{}] Failed to read upstream response: {}", request_id, e);
        error::ErrorInternalServerError("Error reading upstream response")
    })?;

    let mut client_resp = HttpResponse::build(status);
    for (key, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            client_resp.append_header((key.as_str(), val_str));
        }
    }

    Ok(client_resp.body(body))
}
