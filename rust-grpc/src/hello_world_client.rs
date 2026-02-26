use std::error::Error;
use std::time::Duration;

use tokio::time;
use tonic::Request;
use tonic::transport::Channel;

use rust_grpc::hello_world::{GreeterClient, HelloRequest};

async fn say_hello(client: &mut GreeterClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;
    println!("RESPONSE = {response:?}");

    Ok(())
}

async fn say_hello_stream(client: &mut GreeterClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let mut stream = client
        .say_hello_stream_reply(request)
        .await?
        .into_inner();

    while let Some(reply) = stream.message().await? {
        println!("REPLY = {reply:?}");
    }

    Ok(())
}

async fn say_hello_bidi(client: &mut GreeterClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let names = ["Tonic", "Prost", "Tokio", "Serde", "Hyper"];
        let mut interval = time::interval(Duration::from_secs(1));

        for name in names {
            interval.tick().await;
            let elapsed = start.elapsed();
            println!("SENDING \"{}\" at {elapsed:?}", name);
            yield HelloRequest {
                name: name.into(),
            };
        }
    };

    let response = client.say_hello_bidi_stream(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(reply) = inbound.message().await? {
        println!("REPLY = {reply:?}");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    println!("*** UNARY ***");
    say_hello(&mut client).await?;

    println!("\n*** SERVER STREAMING ***");
    say_hello_stream(&mut client).await?;

    println!("\n*** BIDI STREAMING ***");
    say_hello_bidi(&mut client).await?;

    Ok(())
}
