use std::error::Error;
use std::time::Duration;

use tokio::time;
use tonic::Request;
use tonic::transport::Channel;

use rust_grpc::hello_girl::{GirlGreeterClient, HelloGirlRequest};

async fn say_hello(client: &mut GirlGreeterClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = Request::new(HelloGirlRequest {
        name: "Alice".into(),
        spouse: "Bob".into(),
        first_round: 1,
    });

    let response = client.say_hello(request).await?;
    println!("RESPONSE = {response:?}");

    Ok(())
}

async fn say_hello_stream(client: &mut GirlGreeterClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = Request::new(HelloGirlRequest {
        name: "Alice".into(),
        spouse: "Bob".into(),
        first_round: 10,
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

async fn say_hello_bidi(client: &mut GirlGreeterClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let names = ["Alice", "Carol", "Eve", "Grace", "Heidi"];
        let spouses = ["Bob", "Dave", "Frank", "Hugo", "Ivan"];
        let mut interval = time::interval(Duration::from_secs(1));

        for (i, (name, spouse)) in names.iter().zip(spouses.iter()).enumerate() {
            interval.tick().await;
            let elapsed = start.elapsed();
            println!("SENDING \"{} & {}\" at {elapsed:?}", name, spouse);
            yield HelloGirlRequest {
                name: name.to_string(),
                spouse: spouse.to_string(),
                first_round: i as i32,
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
    let mut client = GirlGreeterClient::connect("http://[::1]:50051").await?;

    println!("*** UNARY ***");
    say_hello(&mut client).await?;

    println!("\n*** SERVER STREAMING ***");
    say_hello_stream(&mut client).await?;

    println!("\n*** BIDI STREAMING ***");
    say_hello_bidi(&mut client).await?;

    Ok(())
}
