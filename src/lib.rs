//! This crate allows you to easily send messages using Twilio.
//!
//! # Getting started
//!
//! ```no_run
//! # let account_sid: String = "".into();
//! # let auth_token: String = "".into();
//! # let message = "";
//! # let message_destination = "";
//! # let twilio_number = "";
//! use fullsend::{Client, Message};
//!
//! # tokio_test::block_on(async {
//! let client = Client::builder()
//!     .account_sid(account_sid)
//!     .auth_token(auth_token)
//!     .build()
//!     .unwrap();
//! let message = Message::builder()
//!     .to(message_destination)
//!     .from(twilio_number)
//!     .body(message)
//!     .build()
//!     .unwrap();
//! client.send_message(&message).await;
//! # })
//! ```

mod auth;
pub mod client;
pub mod message;

pub use client::Client;
pub use message::Message;
