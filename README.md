# rustsockets
Websockets with warp in Rust. Built using a tutorial I found, with an example repository from a LogRocket aricle. Definitely didn't write all of this but I added the tests.

Run with

```bash
make dev
```

Register/unregister a client:

```bash
curl -X POST 'http://localhost:8000/register' -H 'Content-Type: application/json' -d '{ "user_id": 1 }' 

curl -X DELETE 'http://localhost:8000/register/<clientId>' 
```

Or connect to the WebSocket using the returned URL: `ws://127.0.0.1:8000/ws/<socketId>`.

Publish messages using

```bash
curl -X POST 'http://localhost:8000/publish' \
    -H 'Content-Type: application/json' \
    -d '{"user_id": 1, "topic": "rust", "message": "rust is awesome"}'
```

