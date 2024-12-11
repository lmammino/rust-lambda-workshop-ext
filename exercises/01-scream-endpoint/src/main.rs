use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let message = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("message"));

    let resp_body = match message {
        Some(msg) => msg.to_uppercase(),
        None => "Send a message using a querystring parameter (e.g. ?message=Hello%20World)".to_string(),
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(resp_body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
