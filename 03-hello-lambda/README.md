# Hello World Lambda function (HTTP)

Scaffold with

```bash
cargo lambda new hello-lambda
```

(Pick HTTP)

## Local simulation

```bash
cargo lambda watch
```

And then, in another terminal:

```bash
cargo lambda invoke --data-example apigw-request
```

or, if you want to provide a custom JSON event:

- copy the content of
  [this file](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-events/src/fixtures/example-apigw-request.json)
  in a new file called `event.json`
- edit `event.json` as you like

```bash
cargo lambda invoke --data-file event.json
```

## Build

```bash
cargo lambda build --release --arm64
```

## Deploy

```bash
cargo lambda deploy --enable-function-url
```

## Invoke the deployed function

```bash
curl -v <your-function-url>?name=Luciano
```
