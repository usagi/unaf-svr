// mod health;

// use futures::Future;
// use std::net::SocketAddr;
// use tonic::transport::Server;

// pub fn init(bind_to: SocketAddr) -> impl Future<Output = Result<(), tonic::transport::Error>>
// {
//  let health = health::UnafHealth::default();
//  let health = health::health_server::HealthServer::new(health);
//  Server::builder().add_service(health).serve(bind_to)
// }
