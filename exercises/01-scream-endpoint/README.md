# 01-scream-endpoint

**✏️ Exercise**

Write a Lambda that implements an HTTP API that receives a message via query
string parameters and responds with that messages capitalized (in the response
body).

**Bonus**: if no message is provided in the request, return a help message in
the response body.

## Example

`GET https://...?message=Hello`

Should return:

```plain
HTTP/1.1
Content-Type: text/plain
Content-Length: 5

HELLO
```

## Solution

To build and deploy the provided solution, run:

```bash
cargo lambda build --release --arm64
cargo lambda deploy --enable-function-url
```

Then you can query the given URL with a message.
