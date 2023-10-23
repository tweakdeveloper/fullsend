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
/// # let phone_num = "";
/// # let sender_num = "";
/// let message = Message::builder()
///     .to(phone_num)
///     .from(sender_num)
///     .build();
/// ```
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    from: Option<&'a str>,
    messaging_service_sid: Option<&'a str>,
    to: &'a str,
}

impl<'a> Message<'a> {
    /// This function returns a `MessageBuilder` to use to create a `Message`.
    pub fn builder() -> MessageBuilder<'a> {
        MessageBuilder::default()
    }
}

/// The `MessageBuilderError` enum represents the various types of errors that
/// can arise when attempting to build a `MessageBuilder`.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum MessageBuilderError {
    /// This error occurs when you attempt to build a `MessageBuilder` without
    /// setting a sender, either a "from" with the `from` function or a Twilio
    /// Messaging Service SID with the `messaging_service_sid` function.
    #[error("no sender set in the builder")]
    NoSenderSet,
    /// This error occurs when you attempt to build a `MessageBuilder` without
    /// setting the `to` field by calling the `to` function during the builder
    /// chain.
    #[error("no `to` field set in builder")]
    NoToSet,
}

/// The `MessageBuilder` struct is used to create a `Message`.
#[derive(Default)]
pub struct MessageBuilder<'a> {
    from: Option<&'a str>,
    messaging_service_sid: Option<&'a str>,
    to: Option<&'a str>,
}

impl<'a> MessageBuilder<'a> {
    /// This function creates a `MessageBuilder`.
    pub fn new() -> Self {
        Self {
            from: None,
            messaging_service_sid: None,
            to: None,
        }
    }

    /// This function validates the builder chain and returns a `Message` that
    /// you can then use to interact with Twilio messages.
    pub fn build(self) -> Result<Message<'a>, MessageBuilderError> {
        // validate that a destination is set and unwrap it if it is
        let to = match self.to {
            Some(to) => to,
            None => return Err(MessageBuilderError::NoToSet),
        };
        // validate that we have a sender: either a from or messaging service,
        // or both
        if self.from.is_none() && self.messaging_service_sid.is_none() {
            return Err(MessageBuilderError::NoSenderSet);
        }
        // all necessary fields are set, let's return the message
        Ok(Message {
            from: self.from,
            messaging_service_sid: self.messaging_service_sid,
            to,
        })
    }

    /// This function sets the sender (in this case, the Twilio phone number
    /// you're using to send the message) of the message.
    pub fn from(mut self, from: &'a str) -> Self {
        self.from = Some(from);
        self
    }

    /// This function sets the destination (i.e. recipient's phone number) of
    /// the message.
    pub fn to(mut self, to: &'a str) -> Self {
        self.to = Some(to);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_sender() {
        let builder_result = Message::builder().to("").build();
        assert_eq!(Err(MessageBuilderError::NoSenderSet), builder_result);
    }

    #[test]
    fn builder_requires_to() {
        let builder_result = Message::builder().build();
        assert_eq!(Err(MessageBuilderError::NoToSet), builder_result);
    }

    #[test]
    fn valid_builder_returns_message() {
        let message = Message::builder().to("").from("").build();
        assert!(message.is_ok());
    }
}
