use super::UnafHttpResponseResult;
use actix_web::{
 web,
 HttpRequest
};
// use async_trait::async_trait;

// #[async_trait]
pub trait UnafHttpCustomizer
{
 // fn api(req: HttpRequest, payload: web::Payload) -> Result<HttpResponse, actix_web::Error>
 fn api(&self, req: HttpRequest, payload: web::Bytes) -> UnafHttpResponseResult
 {
  let id = req.headers().get("X-Unaf-Api-Id");
  log::debug!("/api -> x-unaf-api-id = {id:?}");
  let payload_size = payload.len();
  let payload = payload.to_vec();
  let payload_to_string = String::from_utf8(payload).ok();

  Ok(
   actix_web::HttpResponse::BadRequest()
    .content_type(mime::TEXT_PLAIN_UTF_8)
    .body(format!(
     "API/id: {id:?}\nAPI/payload.size: {payload_size}\nAPI/payload.to_string: {payload_to_string:?}"
    ))
  )
 }
}

#[derive(Default, Clone)]
pub struct UnafHttpCustomizerDefault {}

impl UnafHttpCustomizer for UnafHttpCustomizerDefault
{
  // fn api(&self, req: actix_web::HttpRequest, mut payload: actix_web::web::Bytes) -> UnafHttpResponseResult
  // {
  //  Ok(actix_web::HttpResponse::BadRequest().body("neko"))
  // }
}
