//! This module provides an interface for interacting with Twilio messages.

use std::collections::HashMap;

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
/// # let message = "";
/// let message = Message::builder()
///     .to(phone_num)
///     .from(sender_num)
///     .body(message)
///     .build();
/// ```
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub(crate) body: Option<&'a str>,
    pub(crate) content_sid: Option<&'a str>,
    pub(crate) content_variables: Option<HashMap<&'a str, &'a str>>,
    pub(crate) from: Option<&'a str>,
    pub(crate) media_urls: Option<Vec<&'a str>>,
    pub(crate) messaging_service_sid: Option<&'a str>,
    pub(crate) to: &'a str,
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
    /// setting a body for the message. This can be done by passing the desired
    /// message to the `body` function, passing a Twilio Content SID to the
    /// `content_sid` function, or passing URL(s) to the `media_url` function.
    #[error("no message set in builder")]
    NoMessageSet,
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
    body: Option<&'a str>,
    content_sid: Option<&'a str>,
    content_variables: Option<HashMap<&'a str, &'a str>>,
    from: Option<&'a str>,
    media_urls: Option<Vec<&'a str>>,
    messaging_service_sid: Option<&'a str>,
    to: Option<&'a str>,
}

impl<'a> MessageBuilder<'a> {
    /// This function creates a `MessageBuilder`.
    pub fn new() -> Self {
        Self {
            body: None,
            content_sid: None,
            content_variables: None,
            from: None,
            media_urls: None,
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
        // validate that we have content: any of body, media URL, or Content SID
        if self.body.is_none() && self.media_urls.is_none() && self.content_sid.is_none() {
            return Err(MessageBuilderError::NoMessageSet);
        }
        // all necessary fields are set, let's return the message
        Ok(Message {
            body: self.body,
            content_sid: self.content_sid,
            content_variables: self.content_variables,
            from: self.from,
            media_urls: self.media_urls,
            messaging_service_sid: self.messaging_service_sid,
            to,
        })
    }

    /// This function sets the content of the message (in this case, the body).
    pub fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    /// This function sets the Twilio Content SID of the message
    pub fn content_sid(mut self, content_sid: &'a str) -> Self {
        self.content_sid = Some(content_sid);
        self
    }

    /// This function sets the Content Variables of the message.
    pub fn content_variables(mut self, content_variables: HashMap<&'a str, &'a str>) -> Self {
        self.content_variables = Some(content_variables);
        self
    }

    /// This function sets the sender (in this case, the Twilio phone number
    /// you're using to send the message) of the message.
    pub fn from(mut self, from: &'a str) -> Self {
        self.from = Some(from);
        self
    }

    /// This function sets the media URL of the message.
    ///
    /// # Setting multiple media URLs
    ///
    /// This function will only store a single media URL. If you need to send
    /// multiple, use `media_urls` instead.
    pub fn media_url(mut self, media_url: &'a str) -> Self {
        self.media_urls = Some(vec![media_url]);
        self
    }

    /// This function sets the media URLs of the message.
    pub fn media_urls(mut self, media_urls: Vec<&'a str>) -> Self {
        self.media_urls = Some(media_urls);
        self
    }

    /// This function sets the sender (in this case, the Twilio Messaging
    /// Service you're using to send the message) of the message.
    pub fn messaging_service_sid(mut self, messaging_service_sid: &'a str) -> Self {
        self.messaging_service_sid = Some(messaging_service_sid);
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
    fn builder_requires_message() {
        let builder_result = Message::builder().to("").from("").build();
        assert_eq!(Err(MessageBuilderError::NoMessageSet), builder_result);
    }

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
        let message = Message::builder().to("").from("").body("").build();
        assert!(message.is_ok());
    }
}
