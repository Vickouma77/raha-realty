use std::future::{Ready, ready};
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::Service;
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use futures::future::LocalBoxFuture;
use log::warn;
use serde_json::json;

pub struct AuthMiddleware {
    pub auth_token: String,
}

impl AuthMiddleware {
    pub fn new(auth_token: String) -> Self {
        AuthMiddleware { auth_token }
    }
}

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            auth_token: self.auth_token.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    auth_token: String,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let svc = self.service.clone();
        let auth_token = self.auth_token.clone();

        Box::pin(async move {
            if path == "/health" || path == "/login" || path == "/register" {
                return svc.call(req).await;
            }

            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str == format!("Bearer {}", auth_token) {
                        return svc.call(req).await;
                    }
                }
            }

            warn!("Unauthorized access to: {}", path);
            let res = HttpResponse::Unauthorized().json(json!({
                "error": "Unauthorized",
                "redirect": "/login"
            }));

            Ok(req.into_response(res.map_into_boxed_body()))
        })
    }
}
