use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

#[derive(Clone)]
pub struct ServiceInstance {
    pub url: String,
    pub active: InstanceStatus,
}

#[derive(Clone)]
pub struct ServiceConfig {
    pub instances: Vec<ServiceInstance>,
}

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub services: Arc<RwLock<HashMap<String, ServiceConfig>>>,
    pub auth_token: String,
}

impl ServiceInstance {
    pub fn new(url: &str, active: InstanceStatus) -> Self {
        ServiceInstance {
            url: url.to_string(),
            active,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut services = HashMap::new();
        services.insert(
            "properties".to_string(),
            ServiceConfig {
                instances: vec![ServiceInstance::new(
                    "http://localhost:8001/properties",
                    InstanceStatus::Unhealthy,
                )],
            },
        );
        AppState {
            client: Client::new(),
            services: Arc::new(RwLock::new(services)),
            auth_token: "super-secret-token".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_service_instance() {
        let instance =
            ServiceInstance::new("http://localhost:8001/properties", InstanceStatus::Healthy);
        assert_eq!(instance.url, "http://localhost:8001/properties");
        assert_eq!(instance.active, InstanceStatus::Healthy);
    }

    #[test]
    fn test_app_state_initialization() {
        let app_state = AppState::new();
        let services = app_state.services.clone();
        let services = futures::executor::block_on(services.read());

        assert!(services.contains_key("properties"));
        let config = services.get(&"properties".to_string()).unwrap();
        assert_eq!(config.instances.len(), 1);
        assert_eq!(config.instances[0].active, InstanceStatus::Unhealthy);
    }
}
