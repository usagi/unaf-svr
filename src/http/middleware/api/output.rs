use super::content_type;
use either::Either;
use serde::{
 Deserialize,
 Serialize
};

/// Right: Processed => response: Vec<u8>
/// Left : Unprocessed => Ok( (api-id,pre-processed-payload) ), None; pre-process is failed.
#[derive(Debug, Clone)]
pub struct ApiOutput(pub ApiOutputEither);

pub type ApiOutputEitherLeft = Result<(String, PreprocessedPayload), ApiOutputError>;
pub type ApiOutputEitherRight = PreprocessedPayload;
pub type ApiOutputEither = Either<ApiOutputEitherLeft, PreprocessedPayload>;

#[derive(Debug, Clone)]
pub struct PreprocessedPayload
{
 pub content_type: String,
 pub serialized_params: Vec<u8>
}

impl<'de> PreprocessedPayload
{
 pub fn deserialize<T>(&'de self) -> Option<T>
 where
  T: Deserialize<'de> + std::fmt::Debug
 {
  match self.content_type.as_str()
  {
   content_type::APPLICATION_JSON => serde_json::from_slice(self.serialized_params.as_slice()).ok(),
   // mime::APPLICATION_MSGPACK =>,
   _ => None
  }
 }

 pub fn serialize<T>(&self, dst: T) -> Option<Vec<u8>>
 where
  T: Serialize + std::fmt::Debug
 {
  match self.content_type.as_str()
  {
   content_type::APPLICATION_JSON => serde_json::to_vec(&dst).ok(),
   // mime::APPLICATION_MSGPACK =>,
   _ => None
  }
 }

 pub fn map<T, F>(&'de self, fun: F) -> Option<Vec<u8>>
 where
  T: Deserialize<'de> + Serialize + std::fmt::Debug,
  F: Fn(T) -> Option<T>
 {
  let src = self.deserialize::<T>()?;
  let dst = fun(src);
  let r = self.serialize(dst);
  r
 }

 pub fn map_or<T, F>(&'de self, fun: F, default: T) -> Option<Vec<u8>>
 where
  T: Deserialize<'de> + Serialize + std::fmt::Debug,
  F: Fn(T) -> Option<T>
 {
  // let x = self.serialized_params.clone();
  // let x = String::from_utf8(x).unwrap();
  // log::debug!("de(s)=>{:?}", serde_json::from_str::<T>(x));
  log::debug!("de(u)=>{:?}", serde_json::from_slice::<T>(self.serialized_params.as_slice()));
  // log::debug!("de(u)=>{:?}", self.deserialize::<T>());
  let r = self.deserialize::<T>().and_then(fun).unwrap_or(default);
  self.serialize(r)
 }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ApiOutputError
{
 #[error("Method is not PUT.")]
 MethodIsNotPut,
 #[error("API-ID is not found.; unaf-svr's API attempts to get the API-ID from x-unaf-api-id in the request header.")]
 ApiIdIsNotFound,
 #[error("Unknown Content-type; unaf-svr's API attempts to deserialize the payload based on Content-type.")]
 UnknownContentType,
 #[error("Failed to process; Invalid params or bug of an implement of the ApiProcessorUnit, maybe.")]
 FailedToProcess
}
