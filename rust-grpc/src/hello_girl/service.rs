use std::pin::Pin;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::hello_girl::hello_girl::girl_greeter_server::GirlGreeter;
use crate::hello_girl::hello_girl::{HelloGirlReply, HelloGirlRequest};

#[derive(Debug)]
pub struct GirlGreeterService {}

#[tonic::async_trait]
impl GirlGreeter for GirlGreeterService {
    async fn say_hello(&self, request: Request<HelloGirlRequest>) -> Result<Response<HelloGirlReply>, Status> {
        let req = request.get_ref();
        let reply = HelloGirlReply {
            message: format!("Hello, {} & {}! (round {})", req.name, req.spouse, req.first_round),
            marriage: format!("{} married {}", req.name, req.spouse),
            size: req.first_round * 2,
        };

        Ok(Response::new(reply))
    }

    type SayHelloStreamReplyStream = ReceiverStream<Result<HelloGirlReply, Status>>;

    async fn say_hello_stream_reply(&self, request: Request<HelloGirlRequest>) -> Result<Response<Self::SayHelloStreamReplyStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let req = request.into_inner();

        tokio::spawn(async move {
            for i in 0..5 {
                let reply = HelloGirlReply {
                    message: format!("Hello, {} & {}! ({})", req.name, req.spouse, i),
                    marriage: format!("{} married {}", req.name, req.spouse),
                    size: req.first_round + i,
                };
                tx.send(Ok(reply)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type SayHelloBidiStreamStream = Pin<Box<dyn Stream<Item = Result<HelloGirlReply, Status>> + Send + 'static>>;

    async fn say_hello_bidi_stream(&self, request: Request<Streaming<HelloGirlRequest>>) -> Result<Response<Self::SayHelloBidiStreamStream>, Status> {
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(req) = stream.next().await {
                let req = req?;
                let reply = HelloGirlReply {
                    message: format!("Hello, {} & {}!", req.name, req.spouse),
                    marriage: format!("{} married {}", req.name, req.spouse),
                    size: req.first_round,
                };
                yield reply;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::SayHelloBidiStreamStream))
    }
}
