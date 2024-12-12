# 04-s3-thumbnail

**✏️ Exercise**

Update the previous exercise (process files from S3) to use SAM.

Use a SAM template to define the S3 bucket and the Lambda function. Pass the
bucket name to the lambda function using an environment variable.

## Local testing

You'll need to have deployed the stack to have created the bucket, first!

```bash
export SRC_BUCKET_NAME=src-your-bucket-name
export DEST_BUCKET_NAME=dest-your-bucket-name
cargo lambda watch
```

Copy some files in the source bucket and edit the `events/s3.json` file to match
one of the added files.

Then, in another terminal:

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
