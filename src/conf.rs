use serde::{
 Deserialize,
 Serialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UnafSvrConf
{
 pub http: UnafHttpConf,
 pub grpc: UnafGrpcConf
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnafHttpConf
{
 pub bind_to: String,
 pub workers: Option<usize>
}

impl Default for UnafHttpConf
{
 fn default() -> Self
 {
  Self {
   bind_to: "[::1]:50000".into(),
   workers: None
  }
 }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnafGrpcConf
{
 pub bind_to: String,
 pub workers: Option<usize>
}

impl Default for UnafGrpcConf
{
 fn default() -> Self
 {
  Self {
   bind_to: "[::1]:50001".into(),
   workers: None
  }
 }
}
