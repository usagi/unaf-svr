use super::*;
use actix_web::web;

pub fn api_get_right(req_data: Option<web::ReqData<ApiOutput>>) -> Option<ApiOutputEitherRight>
{
 if let Some(req_data) = req_data
 {
  let ApiOutput(either) = req_data.into_inner();
  if let Some(right) = either.right()
  {
   return Some(right);
  }
 }
 None
}

pub fn api_get_left(req_data: Option<web::ReqData<ApiOutput>>) -> Option<ApiOutputEitherLeft>
{
 if let Some(req_data) = req_data
 {
  let ApiOutput(either) = req_data.into_inner();
  if let Some(left) = either.left()
  {
   return Some(left);
  }
 }
 None
}

pub fn api_get_either(
 req_data: Option<web::ReqData<ApiOutput>>
) -> Option<ApiOutputEither>
{
 if let Some(req_data) = req_data
 {
  let ApiOutput(either) = req_data.into_inner();
  return Some(either);
 }
 None
}
