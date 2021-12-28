use futures::future::{self, Future};
use hyper::{Body, Response};
use std::env;
use serde_json::json;

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
  let json_data = json!({ "nodes": ["node-1"] });

  Box::new(future::ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .header("Access-Control-Allow-Origin", "*") // @todo: think about this
      .body(Body::from(json_data.to_string()))
      .unwrap(),
  ))
}

// @todo -> DB implementation to finish this
pub fn join() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
  let json_data = json!({ "status": "PENDING" });

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
  
  Box::new(future::ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .body(Body::from(json_data.to_string()))
      .unwrap(),
  ))
}
