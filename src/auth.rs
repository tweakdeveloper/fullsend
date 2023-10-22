#[derive(Clone, Debug, PartialEq)]
pub enum AuthMethod {
    AccountAuthToken(String),
    APIKey(String, String),
}
