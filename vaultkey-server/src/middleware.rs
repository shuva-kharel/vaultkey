use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::future::{ready, Ready};
use crate::AppState;

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware { service }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let app_data = req.app_data::<actix_web::web::Data<AppState>>().cloned();
        let secret = match app_data {
            Some(state) => state.jwt_secret.clone(),
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Missing state"))
                })
            }
        };

        let token_valid = auth_header
            .as_deref()
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|token| {
                decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(&secret),
                    &Validation::new(Algorithm::HS256),
                )
                .ok()
            })
            .flatten();

        match token_valid {
            Some(token_data) => {
                req.extensions_mut().insert(token_data.claims.sub.clone());
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            }
            None => {
                Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                })
            }
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}