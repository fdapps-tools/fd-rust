use futures::future::{self, Future};
use hyper::{Body, Response};

pub fn stats() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
  let data = r#"{ "url": "process.env.TUNNEL_URL" }"#;
  Box::new(future::ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .body(Body::from(data))
      .unwrap(),
  ))
}

pub fn nodes() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
  let data = r#"{ "nodes": ["node-1"] }"#;
  Box::new(future::ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .header("Access-Control-Allow-Origin", "*") // @todo: think about this
      .body(Body::from(data))
      .unwrap(),
  ))
}

pub fn join() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
  let data = r#"{ "status": "PENDING" }"#;
  Box::new(future::ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .body(Body::from(data))
      .unwrap(),
  ))
}

pub fn update() -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
  let data = r#"{ "status": "true" }"#;
  Box::new(future::ok(
    Response::builder()
      .header("Content-Type", "application/json")
      .body(Body::from(data))
      .unwrap(),
  ))
}
