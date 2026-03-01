use std::sync::Arc;
use tonic::transport::Server;
use rust_grpc::hello_world::hello_world::greeter_server::GreeterServer;
use rust_grpc::hello_world::service::GreeterService;
use rust_grpc::hello_girl::hello_girl::girl_greeter_server::GirlGreeterServer;
use rust_grpc::hello_girl::service::GirlGreeterService;
use rust_grpc::route_guide::{RouteGuideServer, RouteGuideService, data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let route_guide = RouteGuideService {
        features: Arc::new(data::load()),
    };

    let svc = RouteGuideServer::new(route_guide);
    
    let greeter = GreeterService {};
    let greeter_svc = GreeterServer::new(greeter);

    let girl_greeter = GirlGreeterService {};
    let girl_greeter_svc = GirlGreeterServer::new(girl_greeter);

    Server::builder()
        .add_service(svc)
        .add_service(greeter_svc)
        .add_service(girl_greeter_svc)
        .serve(addr)
        .await?;

    Ok(())
}
