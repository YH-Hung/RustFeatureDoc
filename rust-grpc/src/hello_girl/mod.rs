pub mod hello_girl {
    tonic::include_proto!("hellogirl");
}

pub mod service;

pub use hello_girl::girl_greeter_client::GirlGreeterClient;
pub use hello_girl::{HelloGirlReply, HelloGirlRequest};
