use super::{
 ApiProcessorUnit,
 PreprocessedPayload
};
use serde::{
 Deserialize,
 Serialize
};

/// API 動作試験用 UnafTeapot
/// unaf レベルの API が通っているか確認する用
#[derive(Serialize, Deserialize, Debug)]
pub struct UnafTeapot(String);

impl ApiProcessorUnit for UnafTeapot
{
 const API_ID: &'static str = "unaf/teapot";

 fn process(pp: PreprocessedPayload) -> Option<PreprocessedPayload>
 {
  let r_vec_opt = pp.map_or(
   |params: UnafTeapot| Some(UnafTeapot(format!("🍵unaf/{params:?}🍵"))),
   UnafTeapot("🍵unaf🍵".to_string())
  );
  let r_pp_opt = r_vec_opt.map(|serialized_params| {
   PreprocessedPayload {
    content_type: pp.content_type,
    serialized_params
   }
  });
  r_pp_opt
 }
}
