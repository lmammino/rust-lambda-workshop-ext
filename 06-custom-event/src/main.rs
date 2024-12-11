use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

#[derive(serde::Deserialize)]
struct Request {
    url: String,
}

#[derive(serde::Serialize)]
struct Response {
    issue_number: u32,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    println!("I am going to scrape {}", event.payload.url);
    // TODO: actual scraping logic here
    Ok(Response { issue_number: 333 })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
