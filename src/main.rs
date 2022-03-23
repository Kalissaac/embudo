mod config;

use config::Config;
use hyper::{
    client::HttpConnector,
    service::{make_service_fn, service_fn},
    Body, Request, Response, StatusCode,
};
use hyper::{Client, Server};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref CONFIG: Config = {
        let config_path = env::args().nth(1).expect("no config path given");
        config::get_config(Some(&config_path))
    };
    static ref HTTP_CLIENT: Client<HttpConnector, Body> = Client::new();
}

#[tokio::main]
async fn main() {
    if env::args().len() < 2 {
        println!("Usage: embudo <path to config file>");
        return;
    }

    let service = make_service_fn(move |_| async { Ok::<_, hyper::Error>(service_fn(handle)) });

    let addr = CONFIG.listen_addr.unwrap_or(([127, 0, 0, 1], 80).into());
    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_host = {
        let headers = req.headers();
        let mut host = "127.0.0.1";
        if let Some(h) = headers.get("host") {
            host = h.to_str().unwrap_or("127.0.0.1");
        }
        host
    };

    if let Some(h) = CONFIG.hosts.iter().find(|h| h.source == request_host) {
        let uri_string = format!(
            "http://{}{}",
            h.destination,
            req.uri()
                .path_and_query()
                .map(|x| x.as_str())
                .unwrap_or("/")
        );
        let uri = uri_string.parse().unwrap();
        *req.uri_mut() = uri;
        HTTP_CLIENT.request(req).await
    } else {
        let mut response = Response::new(Body::from("Could not find host"));
        *response.status_mut() = StatusCode::BAD_GATEWAY;
        Ok(response)
    }
}
