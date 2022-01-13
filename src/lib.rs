use futures::future::Future;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use neon::prelude::*;
use std::env;
use std::thread;

mod storage;
mod routes;

fn setup_server(mut cx: FunctionContext) -> JsResult<JsUndefined> {

    // setup sqlite
    storage::setup();

    let port_fd = env::var("PORT_FD").expect("$PORT_FD is not set");
    let addr = ([127, 0, 0, 1], port_fd.parse::<u16>().unwrap()).into();

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| {
            if req.uri().path().starts_with("/stats") {
                routes::stats()
            } else if req.uri().path().starts_with("/nodes") {
                routes::nodes()
            } else if req.uri().path().starts_with("/join-request") {
                routes::join()
            } else if req.uri().path().starts_with("/update-node-info") {
                routes::update()
            } else {
                let app_port = env::var("PORT").expect("$PORT is not set");
                let url = format!("http://localhost:{}", app_port);
                println!("fd: reverse proxy to: {}", url);

                return hyper_reverse_proxy::call(remote_addr.ip(), &url, req);
            }
        })
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // this move hyper to another thread and unlock nodeJS process
    // @todo: need improve and test better 
    thread::spawn(move || {
        hyper::rt::run(server);
    });

    println!("fd: running on {:?}", addr);

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("setup_server", setup_server)?;
    Ok(())
}
