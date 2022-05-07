use retrieve::mod_pub_use;
#[mod_pub_use(helper_for_middleware, helper_for_app, api_processor_unit, teapot)]
type __ = ();

use super::*;
use actix_web::{
 dev::{
  Service,
  ServiceRequest,
  ServiceResponse,
  Transform
 },
 http::Method,
 Error,
 HttpMessage
};
use either::Either;
use futures_util::{
 future::LocalBoxFuture,
 stream::StreamExt
};
use std::{
 future::{
  ready,
  Ready
 },
 rc::Rc
};

#[doc(hidden)]
pub struct ApiMiddleware<S>
{
 // service: S
 service: Rc<S>
}

impl<S, B> Service<ServiceRequest> for ApiMiddleware<S>
where
 S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
 S::Future: 'static,
 B: 'static
{
 type Error = Error;
 type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
 type Response = ServiceResponse<B>;

 actix_web::dev::forward_ready!(service);

 fn call(&self, mut req: ServiceRequest) -> Self::Future
 {
  log::trace!("unaf/api");

  let svc = self.service.clone();
  Box::pin(async move {
   if req.head().method != Method::PUT
   {
    req
     .extensions_mut()
     .insert(ApiOutput(Either::Left(Err(ApiOutputError::MethodIsNotPut))));
    return Ok(svc.call(req).await?);
   }

   let api_id = match req.headers().get("x-unaf-api-id")
   {
    Some(api_id) => api_id.to_str().unwrap().to_owned(),
    _ =>
    {
     req
      .extensions_mut()
      .insert(ApiOutput(Either::Left(Err(ApiOutputError::ApiIdIsNotFound))));
     return Ok(svc.call(req).await?);
    }
   };
   // log::debug!("api_id={api_id:?}");

   let content_type = req.content_type().to_ascii_lowercase();
   // log::debug!("content_type={content_type:?}");
   if !content_type::is_valid(content_type.as_str())
   {
    req
     .extensions_mut()
     .insert(ApiOutput(Either::Left(Err(ApiOutputError::UnknownContentType))));
    return Ok(svc.call(req).await?);
   };

   let serialized_params = {
    // body を完全に回収する
    let mut body = actix_web::web::BytesMut::new();
    let mut stream = req.take_payload();
    while let Some(chunk) = stream.next().await
    {
     body.extend_from_slice(&chunk?);
    }
    body.to_vec()
   };

   // ここまで来たら API として文法上は正しいのでとりあえず log::debug
   log::debug!(
    "unaf/api api_id={api_id:?} content_type={content_type} serialized_params.len={}",
    serialized_params.len()
   );

   // unaf またはユーザカスタムのAPIの呼び出し様式は満たしているので pp_in はここから使える
   let pp_in = PreprocessedPayload {
    content_type,
    serialized_params: serialized_params.to_vec()
   };

   process_for_mw!(api_id, pp_in, req, svc, [UnafTeapot])
  })
 }
}

#[derive(Clone, Debug, Default)]
pub struct ApiProcessor;

impl<S: 'static, B> Transform<S, ServiceRequest> for ApiProcessor
where
 S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
 S::Future: 'static,
 B: 'static
{
 type Error = Error;
 type Future = Ready<Result<Self::Transform, Self::InitError>>;
 type InitError = ();
 type Response = ServiceResponse<B>;
 type Transform = ApiMiddleware<S>;

 fn new_transform(&self, service: S) -> Self::Future
 {
  ready(Ok(ApiMiddleware {
   service: Rc::new(service)
  }))
 }
}
