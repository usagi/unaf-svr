use tonic::{
 Request,
 Response,
 Status
};

tonic::include_proto!("health");

#[derive(Debug, Default)]
pub struct UnafHealth {}

#[tonic::async_trait]
impl health_server::Health for UnafHealth
{
 async fn reply_health(&self, req: Request<HealthRequest>) -> Result<Response<HealthReply>, Status>
 {
  log::debug!("Got a request: {:?}", req);

  let health_reply = HealthReply {
   res_test_param: format!("Health {}!", req.into_inner().req_test_param)
  };

  Ok(Response::new(health_reply))
 }
}
