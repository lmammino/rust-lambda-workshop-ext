# 02-s3-event

**✏️ Exercise**

Write a Lambda that processes S3 events. For every object in the event, print
the bucket name, the type of operation, and the object name.

## Example

If you have an event that contains 3 Put object events for the following
resources:

- **Bucket**: `MyBucket`, **Key**: `Happy Face.jpg`
- **Bucket**: `MyBucket`, **Key**: `lolz.gif`
- **Bucket**: `MyBucket`, **Key**: `secrets/passwords.txt`

This lambda should log the following lines:

```plain
MyBucket - ObjectCreated:Put "Happy Face.jpg"
MyBucket - ObjectCreated:Put "lolz.gif"
MyBucket - ObjectCreated:Put "secrets/passwords.txt"
```

## Test locally

```bash
cargo lambda watch
```

And, in another terminal

```bash
cargo lambda invoke --data-example s3-event
```

## Deploy the solution

To build and deploy the provided solution, run:

```bash
cargo lambda build --release --arm64
cargo lambda deploy --enable-function-url
```

Then you need to _MANUALLY_ create a bucket and the trigger for the Lambda. 😟
