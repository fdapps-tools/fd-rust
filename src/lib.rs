use neon::prelude::*;
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};
use hyper::service::{service_fn, make_service_fn};
use futures::future::{self, Future};

fn setup_server(mut cx: FunctionContext) -> JsResult<JsObject> {
    let application_port: Handle<JsNumber> = cx.argument(0)?;

    // @todo setup network layer routes
    
    // @todo chose port
    let addr = ([127, 0, 0, 1], 3000).into();

    // @todo: concat port and test /* route
    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| {

            if req.uri().path().starts_with("/app") {

                // will forward requests to port application_port
                return hyper_reverse_proxy::call(remote_addr.ip(), "http://127.0.0.1:13901", req)

            } else {
                debug_request(req)
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


type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn debug_request(req: Request<Body>) -> BoxFut {
    let body_str = format!("{:?}", req);
    let response = Response::new(Body::from(body_str));
    Box::new(future::ok(response))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("setup_server", setup_server)?;
    Ok(())
}

