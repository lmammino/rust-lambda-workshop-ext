# 04-s3-event

An example of a Lambda function that can process S3 events.

## Test locally

```bash
cargo lambda watch
```

And, in another terminal

```bash
cargo lambda invoke --data-example s3-event
```
