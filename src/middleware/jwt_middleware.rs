use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;

use crate::models::error::{AppError, AppErrorType};
use crate::utils::jwt::{Jwt, Jwtoken};
pub struct JwtHandle;

pub struct JwtMiddleware<S> {
    pub service: Rc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for JwtHandle
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Error = Error;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddleware {
            service: Rc::new(service),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        Box::pin(async move {
            if let Some(header) = req.headers().get("Authorization") {
                if let Ok(token) = header.to_str() {
                    match Jwtoken::verify_jwt(token) {
                        Ok(claim) => {
                            req.extensions_mut().insert::<String>(claim.sub);
                            return svc.call(req).await 
                        }
                        Err(error) => {
                            return Err(Error::from(AppError {
                                error_type: AppErrorType::Authentication,
                                message: Some(error),
                            }))
                        }
                    }
                }
                return svc.call(req).await;
            }
            Err(Error::from(AppError {
                error_type: AppErrorType::NotAllowed,
                message: Some("Not permitted to visit this".to_string()),
            }))
        })
    }
}
