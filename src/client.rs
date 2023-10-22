//! This module provides an interface for interacting with Twilio.

/// The `Client` struct is the interface for interacting with Twilio.
///
/// # Creating
///
/// Use a `ClientBuilder`:
///
/// ```rust
/// # use std::env;
/// use fullsend::Client;
///
/// # env::set_var("TWILIO_ACCOUNT_SID", "");
/// let client = Client::builder()
///     .account_sid(env::var("TWILIO_ACCOUNT_SID")?)
///     .build();
/// # Ok::<(), env::VarError>(())
/// ```
#[derive(Debug, PartialEq)]
pub struct Client {
    account_sid: String,
}

impl Client {
    /// This function returns a `ClientBuilder` to use to create a `Client`.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

/// The `ClientBuilderError` enum represents the various types of errors that
/// can arise when attempting to build a `ClientBuilder`.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ClientBuilderError {
    #[error("no account SID set in builder")]
    NoAccountSidSet,
}

/// The `ClientBuilder` struct is used to create a `Client`.
#[derive(Default)]
pub struct ClientBuilder {
    account_sid: Option<String>,
}

impl ClientBuilder {
    /// This function creates a `ClientBuilder`.
    pub fn new() -> Self {
        ClientBuilder { account_sid: None }
    }

    /// This function validates the builder chain and returns a `Client` that
    /// you can then use to interact with Twilio.
    pub fn build(&self) -> Result<Client, ClientBuilderError> {
        if self.account_sid.is_none() {
            return Err(ClientBuilderError::NoAccountSidSet);
        }
        let account_sid = self.account_sid.clone().unwrap();
        Ok(Client { account_sid })
    }

    /// This function sets the account SID to be used by the `Client` when
    /// interacting with Twilio.
    pub fn account_sid(&mut self, account_sid: String) -> &mut Self {
        self.account_sid = Some(account_sid);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_sid() {
        let builder_result = Client::builder().build();
        assert_eq!(Err(ClientBuilderError::NoAccountSidSet), builder_result);
    }

    #[test]
    fn valid_builder_returns_client() {
        let client = Client::builder().account_sid("".into()).build();
        assert!(client.is_ok());
    }
}
