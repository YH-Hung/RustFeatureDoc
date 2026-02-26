use std::pin::Pin;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::hello_world::hello_world::greeter_server::Greeter;
use crate::hello_world::hello_world::{HelloReply, HelloRequest};

#[derive(Debug)]
pub struct GreeterService {}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(&self, request: tonic::Request<HelloRequest>) -> Result<tonic::Response<HelloReply>, tonic::Status> {
        let reply = HelloReply {
            message: format!("Hello, {}!", request.get_ref().name)
        };

        Ok(tonic::Response::new(reply))
    }

    type SayHelloStreamReplyStream = ReceiverStream<Result<HelloReply, Status>>;

    async fn say_hello_stream_reply(&self, request: Request<HelloRequest>) -> Result<Response<Self::SayHelloStreamReplyStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let name = request.into_inner().name;

        tokio::spawn(async move {
            for i in 0..5 {
                let reply = HelloReply {
                    message: format!("Hello, {}! ({})", name, i)
                };
                tx.send(Ok(reply)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type SayHelloBidiStreamStream = Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send + 'static>>;

    async fn say_hello_bidi_stream(&self, request: Request<Streaming<HelloRequest>>) -> Result<Response<Self::SayHelloBidiStreamStream>, Status> {
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(req) = stream.next().await {
                let req = req?;
                let reply = HelloReply {
                    message: format!("Hello, {}!", req.name)
                };
                yield reply;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::SayHelloBidiStreamStream))
    }
}
