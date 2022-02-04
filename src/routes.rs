use futures::future::{self, Future};
use hyper::{Body, Response};
use serde_json::json;
use std::env;

use crate::storage;

pub fn stats() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    let tunnel_url = env::var("TUNNEL_URL").expect("$TUNNEL_URL is not set");
    let json_data = json!({ "url": tunnel_url });

    Box::new(future::ok(
        Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_data.to_string()))
            .unwrap(),
    ))
}

// @todo -> DB implementation to finish this
pub fn nodes() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    // @todo: after make list ono storage, convert to json in here
    let nodes = match storage::node_list() {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    Box::new(future::ok(
        Response::builder()
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*") // @todo: think about this
            .body(Body::from(serde_json::to_string(&nodes).unwrap()))
            .unwrap(),
    ))
}

// @todo -> DB implementation to finish this
pub fn join() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    let json_data = json!({ "status": "PENDING" });
    // let json_data = node_manager::join();

    Box::new(future::ok(
        Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_data.to_string()))
            .unwrap(),
    ))
}

// @todo -> DB implementation to finish this
pub fn update() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    let json_data = json!({ "status": "true" });
    // node_manager::update();

    Box::new(future::ok(
        Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_data.to_string()))
            .unwrap(),
    ))
}
