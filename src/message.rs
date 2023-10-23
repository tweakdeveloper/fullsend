//! This module provides an interface for interacting with Twilio messages.

/// The `Message` struct is the interface for interacting with Twilio messages.
///
/// # Creating
///
/// Use a `MessageBuilder`:
///
/// ```rust
/// use fullsend::Message;
///
/// # let phone_num: String = "".into();
/// let message = Message::builder()
///     .to(phone_num)
///     .build();
/// ```
#[derive(Debug, PartialEq)]
pub struct Message {
    to: String,
}

impl Message {
    /// This function returns a `MessageBuilder` to use to create a `Message`.
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }
}

/// The `MessageBuilderError` enum represents the various types of errors that
/// can arise when attempting to build a `MessageBuilder`.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum MessageBuilderError {
    /// This error occurs when you attempt to build a `MessageBuilder` without
    /// setting the `to` field by calling the `to` function during the builder
    /// chain.
    #[error("no `to` field set in builder")]
    NoToSet,
}

/// The `MessageBuilder` struct is used to create a `Message`.
#[derive(Default)]
pub struct MessageBuilder {
    to: Option<String>,
}

impl MessageBuilder {
    /// This function creates a `MessageBuilder`.
    pub fn new() -> Self {
        Self { to: None }
    }

    /// This function validates the builder chain and returns a `Message` that
    /// you can then use to interact with Twilio messages.
    pub fn build(&self) -> Result<Message, MessageBuilderError> {
        if self.to.is_none() {
            return Err(MessageBuilderError::NoToSet);
        }
        let to = self.to.clone().unwrap();
        Ok(Message { to })
    }

    /// This function sets the destination (i.e. recipient's phone number) of
    /// the message.
    pub fn to(&mut self, to: String) -> &mut Self {
        self.to = Some(to);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_to() {
        let builder_result = Message::builder().build();
        assert_eq!(Err(MessageBuilderError::NoToSet), builder_result);
    }

    #[test]
    fn valid_builder_returns_client() {
        let client = Message::builder().to("".into()).build();
        assert!(client.is_ok());
    }
}
