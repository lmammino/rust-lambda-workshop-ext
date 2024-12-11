# 07-api-sam

This example demonstrates how to create a simple API with AWS SAM.

## Local testing

```bash
sam build --beta-features
```

```bash
sam local start-api
```

Now you can open the following URL in your browser:
[http://127.0.0.1:3000](http://127.0.0.1:3000/)

Try to set the `?name` parameter!

## Deploy

Validate your template with:

```bash
sam validate
```

then build your project:

```bash
sam build --beta-features
```

(always build if you have done changes)

The first time, you can follow a guided deployment procedure:

```bash
sam deploy --guided
```

After the first deployment, you can use the following command:

```bash
sam deploy
```
