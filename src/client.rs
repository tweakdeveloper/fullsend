//! This module provides an interface for interacting with Twilio.

use crate::{auth::AuthMethod, Message};

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

/// The `SendError` enum represents the various types of errors that can arise
/// when attempting to send a `Message`.
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    /// This error occurs when there was an error communicating with Twilio.
    /// The `reqwest` error is contained in this error.
    #[error("couldn't communicate with twilio")]
    Network(#[from] reqwest::Error),
    /// This error occurs when Twilio was able to be contacted, but the request
    /// was unsuccessful. The HTTP response code is contained in this error.
    #[error("Twilio returned reponse code {0}")]
    Twilio(u16),
}

impl Client {
    /// This function returns a `ClientBuilder` to use to create a `Client`.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn send_message(&self, message: &Message<'_>) -> Result<(), SendError> {
        // in order to avoid having our params map reallocate every time we push
        // one, we're going to count the number we need, then allocate once.
        // we know for sure we have one: the message destination, so we'll start
        // with one.
        let mut num_params = 1;
        // now let's sort out the sender items
        if message.from.is_some() {
            num_params += 1;
        }
        if message.messaging_service_sid.is_some() {
            num_params += 1;
        }
        // now, the content items. i'll save the media URL(s) for last because
        // they're a bit more tricky
        if message.body.is_some() {
            num_params += 1;
        }
        if message.content_sid.is_some() {
            num_params += 1;
        }
        if message.media_urls.is_some() {
            // like i said
            num_params += message.media_urls.as_ref().unwrap().len();
        }
        // now that we know how many params we need, let's create our params
        let mut params = Vec::<(&str, &str)>::with_capacity(num_params);
        params.push(("To", message.to));
        if let Some(from) = message.from {
            params.push(("From", from));
        }
        if let Some(messaging_service_sid) = message.messaging_service_sid {
            params.push(("MessagingServiceSid", messaging_service_sid));
        }
        if let Some(body) = message.body {
            params.push(("Body", body));
        }
        if let Some(content_sid) = message.content_sid {
            params.push(("ContentSid", content_sid));
        }
        if let Some(media_urls) = &message.media_urls {
            for media_url in media_urls {
                params.push(("MediaUrl", media_url));
            }
        }
        // let's get our auth situation sorted
        let auth_user: &str;
        let auth_pass: &str;
        match &self.auth {
            AuthMethod::AccountAuthToken(token) => {
                auth_user = &self.account_sid;
                auth_pass = &token;
            }
            AuthMethod::APIKey(key, secret) => {
                auth_user = &key;
                auth_pass = &secret;
            }
        };
        // now that we have our params and auth sorted, we can send the request
        let client = reqwest::Client::new();
        let twilio_result = client
            .post(format!(
                "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
                self.account_sid
            ))
            .form(&params)
            .basic_auth(auth_user, Some(auth_pass))
            .send()
            .await;
        let twilio_response = match twilio_result {
            Ok(response) => response,
            Err(error) => return Err(SendError::Network(error)),
        };
        if twilio_response.status().is_success() {
            Ok(())
        } else {
            Err(SendError::Twilio(twilio_response.status().as_u16()))
        }
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
