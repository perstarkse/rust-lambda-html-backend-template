mod helloworld;

use helloworld::hello_world_handler;
use lambda_http::{run, service_fn, Body, Error, Request, Response};

async fn router(event: Request) -> Result<Response<Body>, Error> {
    match event.uri().path() {
        "/hello-world" => hello_world_handler(event).await,
        _ => {
            let resp = Response::builder()
                .status(404)
                .body("Not Found".into())
                .map_err(Box::new)?;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(router)).await
}
