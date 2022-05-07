use super::*;
use actix_web::{
 dev::{
  Service,
  ServiceRequest,
  ServiceResponse,
  Transform
 },
 Error,
 HttpMessage
};
use std::{
 future::{
  ready,
  Ready
 },
 task::{
  Context,
  Poll
 }
};

#[doc(hidden)]
pub struct AddMsgService<S>
{
 service: S,
 enabled: bool
}

impl<S, B> Service<ServiceRequest> for AddMsgService<S>
where
 S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>
{
 type Error = Error;
 type Future = S::Future;
 type Response = ServiceResponse<B>;

 fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>
 {
  self.service.poll_ready(ctx)
 }

 fn call(&self, req: ServiceRequest) -> Self::Future
 {
  log::info!("request is passing through the AddMsg middleware");

  if self.enabled
  {
   // insert data into extensions if enabled
   req.extensions_mut().insert(IndexOutput("Hello from Middleware!".to_owned()));
  }

  self.service.call(req)
 }
}

#[derive(Clone, Debug, Default)]
pub struct IndexProcessor
{
 enabled: bool
}

impl IndexProcessor
{
 pub fn enabled() -> Self
 {
  Self {
   enabled: true
  }
 }

 pub fn disabled() -> Self
 {
  Self {
   enabled: false
  }
 }
}

impl<S, B> Transform<S, ServiceRequest> for IndexProcessor
where
 S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>
{
 type Error = Error;
 type Future = Ready<Result<Self::Transform, Self::InitError>>;
 type InitError = ();
 type Response = ServiceResponse<B>;
 type Transform = AddMsgService<S>;

 fn new_transform(&self, service: S) -> Self::Future
 {
  ready(Ok(AddMsgService {
   service,
   enabled: self.enabled
  }))
 }
}
