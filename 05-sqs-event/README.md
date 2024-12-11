# 05-sqs-event

An example of a Lambda function that can process SQS jobs.

## Test locally

```bash
cargo lambda watch
```

And, in another terminal

```bash
cargo lambda invoke --data-example sqs-event
```
