# 03-s3-event-sam

**✏️ Exercise**

Update the previous exercise (process files from S3) to use SAM.

Use a SAM template to define the S3 bucket and the Lambda function. Pass the
bucket name to the lambda function using an environment variable.

## Local testing

```bash
export BUCKET_NAME=your-bucket-name
cargo lambda watch
```

in another terminal:

```bash
cargo lambda invoke --event events/s3.json
```

## Deploy the solution

To build and deploy the provided solution, run:

```bash
sam validate
sam build --beta-features
sam deploy --guided
```

If the deployment fails, because the bucket name is already taken try to change
the value for the `BucketNameParam`.
