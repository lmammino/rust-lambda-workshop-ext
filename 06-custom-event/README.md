# 06-custom-event

An example of a Lambda function that can process custom events.

## Test locally

```bash
cargo lambda watch
```

And, in another terminal

```bash
cargo lambda invoke --data-ascii '{"url": "https://us15.campaign-archive.com/home/?u=b015626aa6028495fe77c75ea&id=55ace33899"}'
```
