use futures::future::{self, Future};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use neon::prelude::*;
mod routes;

fn setup_server(mut cx: FunctionContext) -> JsResult<JsObject> {
    let application_port: Handle<JsNumber> = cx.argument(0)?;

    // @todo chose port
    let addr = ([127, 0, 0, 1], 3001).into();

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| {
            if req.uri().path().starts_with("/") {
                // @todo: concat port and test /* route
                return hyper_reverse_proxy::call(remote_addr.ip(), "http://127.0.0.1:3000", req);
            } else if req.uri().path().starts_with("/stats") {
                routes::stats()
            } else if req.uri().path().starts_with("/nodes") {
                routes::nodes()
            } else if req.uri().path().starts_with("/join-request") {
                routes::join()
            } else if req.uri().path().starts_with("/update-node-info") {
                routes::update()
            } else {
                Box::new(future::ok(Response::new(Body::from("Not found!"))))
            }
        })
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running server on {:?}", addr);

    hyper::rt::run(server);
    let obj = cx.empty_object();

    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("setup_server", setup_server)?;
    Ok(())
}
