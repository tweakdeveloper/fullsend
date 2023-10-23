# fullsend

![build status](https://img.shields.io/github/actions/workflow/status/tweakdeveloper/fullsend/build-and-test.yaml?style=flat-square)

A Rust library to interact with the Twilio API

## Getting started

```rust
use fullsend::{Client, Message};

let client = Client::builder()
    .account_sid(account_sid)
    .auth_token(auth_token)
    .build();
let message = Message::builder()
    .to(message_destination)
    .from(twilio_number)
    .body(message)
    .build();
client.send_message(&message).await;
```
