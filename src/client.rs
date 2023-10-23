//! This module provides an interface for interacting with Twilio.

use crate::auth::AuthMethod;

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
/// # env::set_var("TWILIO_AUTH_TOKEN", "");
/// let client = Client::builder()
///     .account_sid(env::var("TWILIO_ACCOUNT_SID")?)
///     .auth_token(env::var("TWILIO_AUTH_TOKEN")?)
///     .build();
/// # Ok::<(), env::VarError>(())
/// ```
///
/// You can also use a Twilio API Key by replacing the `auth_token` with
/// `api_key`:
///
/// ```rust
/// # use std::env;
/// use fullsend::Client;
///
/// # env::set_var("TWILIO_ACCOUNT_SID", "");
/// # env::set_var("TWILIO_API_KEY", "");
/// # env::set_var("TWILIO_API_SECRET", "");
/// let client = Client::builder()
///     .account_sid(env::var("TWILIO_ACCOUNT_SID")?)
///     .api_key(env::var("TWILIO_API_KEY")?, env::var("TWILIO_API_SECRET")?)
///     .build();
/// # Ok::<(), env::VarError>(())
/// ```
#[derive(Debug, PartialEq)]
pub struct Client {
    account_sid: String,
    auth: AuthMethod,
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
    /// This error occurs when you attempt to build a `ClientBuilder` without
    /// setting an account SID.
    #[error("no account SID set in builder")]
    NoAccountSidSet,
    /// This error occurs when you attempt to build a `ClientBuilder` without
    /// setting an authentication method, either the account's auth token or an
    /// API key and secret.
    #[error("no auth method set in builder")]
    NoAuthMethodSet,
}

/// The `ClientBuilder` struct is used to create a `Client`.
#[derive(Default)]
pub struct ClientBuilder {
    account_sid: Option<String>,
    auth: Option<AuthMethod>,
}

impl ClientBuilder {
    /// This function creates a `ClientBuilder`.
    pub fn new() -> Self {
        ClientBuilder {
            account_sid: None,
            auth: None,
        }
    }

    /// This function validates the builder chain and returns a `Client` that
    /// you can then use to interact with Twilio.
    pub fn build(&self) -> Result<Client, ClientBuilderError> {
        if self.account_sid.is_none() {
            return Err(ClientBuilderError::NoAccountSidSet);
        }
        let account_sid = self.account_sid.clone().unwrap();
        if self.auth.is_none() {
            return Err(ClientBuilderError::NoAuthMethodSet);
        }
        let auth = self.auth.clone().unwrap();
        Ok(Client { account_sid, auth })
    }

    /// This function sets the account SID to be used by the `Client` when
    /// interacting with Twilio.
    pub fn account_sid(&mut self, account_sid: String) -> &mut Self {
        self.account_sid = Some(account_sid);
        self
    }

    pub fn api_key(&mut self, key: String, secret: String) -> &mut Self {
        self.auth = Some(AuthMethod::APIKey(key, secret));
        self
    }

    pub fn auth_token(&mut self, token: String) -> &mut Self {
        self.auth = Some(AuthMethod::AccountAuthToken(token));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_auth_method() {
        let builder_result = Client::builder().account_sid("".into()).build();
        assert_eq!(Err(ClientBuilderError::NoAuthMethodSet), builder_result);
    }

    #[test]
    fn builder_requires_sid() {
        let builder_result = Client::builder().auth_token("".into()).build();
        assert_eq!(Err(ClientBuilderError::NoAccountSidSet), builder_result);
    }

    #[test]
    fn valid_builder_returns_client() {
        let client = Client::builder()
            .account_sid("".into())
            .auth_token("".into())
            .build();
        assert!(client.is_ok());
    }
}
