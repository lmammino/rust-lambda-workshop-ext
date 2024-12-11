use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use std::env;

struct HandlerConfig {
    bucket_name: String,
}

fn normalize_key(key: &str) -> String {
    key.replace("%20", " ")
}

async fn function_handler(
    config: &HandlerConfig,
    event: LambdaEvent<S3Event>,
) -> Result<(), Error> {
    let (event, context) = event.into_parts();
    println!("{}", context.request_id);

    for record in event.records {
        tracing::info!(
            "{} - {} \"{}\"",
            config.bucket_name,
            record.event_name.unwrap_or_default(),
            normalize_key(&record.s3.object.key.unwrap_or_default())
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME environment variable is not set");

    let config = &HandlerConfig { bucket_name };

    run(service_fn(move |event| async move {
        function_handler(config, event).await
    }))
    .await
}
