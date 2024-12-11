use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    let (event, context) = event.into_parts();
    println!("{}", context.request_id);
    
    for record in event.records {
        tracing::info!(
            "[{}] Bucket={} Key={}",
            record.event_name.unwrap_or_default(),
            record.s3.bucket.name.unwrap_or_default(),
            record.s3.object.key.unwrap_or_default()
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
