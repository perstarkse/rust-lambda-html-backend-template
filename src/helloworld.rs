use askama::Template;
use lambda_http::{Body, Error, Request, RequestExt, Response};

#[derive(Template)]
#[template(path = "helloworld.html")]
struct HelloWorldTemplate {
    name: String,
}

pub async fn hello_world_handler(event: Request) -> Result<Response<Body>, Error> {
    let name = HelloWorldTemplate {
        name: event
            .query_string_parameters_ref()
            .and_then(|params| params.first("name"))
            .unwrap_or("world")
            .to_string(),
    };

    let html_content = name.render().map_err(Box::new)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(html_content.into())
        .map_err(Box::new)?;
    Ok(resp)
}
