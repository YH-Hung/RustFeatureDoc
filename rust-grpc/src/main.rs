pub mod routeguide {
    tonic::include_proto!("routeguide");    // routeguide is the package name in proto file
}

use routeguide::route_guide_server::{RouteGuide, RouteGuideServer};
use routeguide::{Feature, Point, Rectangle, RouteNote, RouteSummary};

use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tokio_stream::{wrappers::ReceiverStream, Stream};

#[derive(Debug)]
struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}


#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, _request: Request<Point>) -> Result<Response<Feature>, Status> {
        unimplemented!()
    }

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        _request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        unimplemented!()
    }

    async fn record_route(
        &self,
        _request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        unimplemented!()
    }

    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send  + 'static>>;

    async fn route_chat(
        &self,
        _request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}
