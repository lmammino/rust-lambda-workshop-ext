use aws_config::BehaviorVersion;
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::primitives::ByteStream;
use image::{guess_format, ImageReader};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use std::{env, io::Cursor};
use tokio::io::AsyncReadExt;

struct HandlerConfig {
    s3_client: aws_sdk_s3::Client,
    src_bucket_name: String,
    dest_bucket_name: String,
}

fn normalize_key(key: &str) -> String {
    urlencoding::decode(key).unwrap().into_owned()
}

async fn function_handler(
    config: &HandlerConfig,
    event: LambdaEvent<S3Event>,
) -> Result<(), Error> {
    let (event, _context) = event.into_parts();

    for record in event.records {
        let key = normalize_key(&record.s3.object.key.unwrap_or_default());

        tracing::info!(
            "{} - {} \"{}\"",
            config.src_bucket_name,
            record.event_name.unwrap_or_default(),
            key
        );

        let object = config
            .s3_client
            .get_object()
            .bucket(&config.src_bucket_name)
            .key(&key)
            .send()
            .await?;

        let mut object_body = vec![];
        object
            .body
            .into_async_read()
            .read_to_end(&mut object_body)
            .await?;

        tracing::info!("Read s3://{}/{}", config.src_bucket_name, &key);

        let format = guess_format(&object_body);
        if format.is_err() {
            tracing::warn!("Unsupported format for {}", &key);
            continue;
        }
        let format = format.unwrap();

        let src_image = ImageReader::new(Cursor::new(object_body))
            .with_guessed_format()
            .unwrap()
            .decode()?;

        let dest_image = src_image.thumbnail(300, 300);
        let mut dest_buffer = Vec::new();
        dest_image.write_to(&mut Cursor::new(&mut dest_buffer), format)?;

        config
            .s3_client
            .put_object()
            .bucket(&config.dest_bucket_name)
            .key(&key)
            .body(ByteStream::new(dest_buffer.into()))
            .send()
            .await?;

        tracing::info!("Written s3://{}/{}", config.dest_bucket_name, &key);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let src_bucket_name =
        env::var("SRC_BUCKET_NAME").expect("SRC_BUCKET_NAME environment variable is not set");
    let dest_bucket_name =
        env::var("DEST_BUCKET_NAME").expect("DEST_BUCKET_NAME environment variable is not set");
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    let config: &HandlerConfig = &HandlerConfig {
        src_bucket_name,
        dest_bucket_name,
        s3_client,
    };

    run(service_fn(move |event| async move {
        function_handler(config, event).await
    }))
    .await
}
