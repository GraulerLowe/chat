use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.method() {
        &Method::GET => Ok(Response::new(Body::from("Received a GET request"))),
        &Method::POST => Ok(Response::new(Body::from("Received a POST request"))),
        &Method::PUT => Ok(Response::new(Body::from("Received a PUT request"))),
        &Method::DELETE => Ok(Response::new(Body::from("Received a DELETE request"))),
        _ => Ok(Response::new(Body::from("Received an unknown request"))),
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
