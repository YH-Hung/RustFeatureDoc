pub mod routeguide {
    tonic::include_proto!("routeguide");
}

pub mod data;
pub mod service;

pub use routeguide::route_guide_server::{RouteGuide, RouteGuideServer};
pub use routeguide::route_guide_client::RouteGuideClient;
pub use routeguide::{Feature, Point, Rectangle, RouteNote, RouteSummary};
pub use service::RouteGuideService;
