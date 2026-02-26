pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod service;

pub use hello_world::greeter_client::GreeterClient;
pub use hello_world::{HelloReply, HelloRequest};
