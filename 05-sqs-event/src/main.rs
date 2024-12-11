use aws_lambda_events::event::sqs::{SqsEvent, BatchItemFailure, SqsBatchResponse};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<SqsBatchResponse, Error> {
    let mut failed_jobs = Vec::with_capacity(event.payload.records.len());

    for record in event.payload.records {
        // process the job
        // ...
        // if the job failed, add it to the failed_jobs list
        failed_jobs.push(BatchItemFailure {
            item_identifier: record.message_id.unwrap_or_default(),
        });
    }

    Ok(SqsBatchResponse {
        batch_item_failures: failed_jobs,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
