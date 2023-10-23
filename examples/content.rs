use std::{collections::HashMap, env, error::Error, io::Write};

use fullsend::{Client, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // set up the client
    let account_sid =
        env::var("TWILIO_ACCOUNT_SID").map_err(|e| format!("account SID env var error: {}", e))?;
    let account_tkn = env::var("TWILIO_ACCOUNT_TKN")
        .map_err(|e| format!("account auth token env var error: {}", e))?;
    let client = Client::builder()
        .account_sid(account_sid)
        .auth_token(account_tkn)
        .build()?;
    // get the destination phone number
    let phone_num = prompt("phone number")?;
    // get the sender phone number
    let sender_num = env::var("TWILIO_SENDER_NUM")
        .map_err(|e| format!("twilio sender number env var error: {}", e))?;
    // get the content template's SID
    let content_sid = env::var("TWILIO_CONTENT_SID")
        .map_err(|_| "twilio content template SID env var not set")?;
    // get the value to be inserted into the content
    // in this case, a name.
    let name = prompt("name")?;
    // set up the variables to be inserted into the content template
    // the content template in Twilio could look something like:
    // Hello, {{name}}!
    let mut content_variables = HashMap::<&str, &str>::new();
    content_variables.insert("name", &name);
    // set up the message
    let message = Message::builder()
        .to(phone_num.trim())
        .content_sid(&content_sid)
        .content_variables(content_variables)
        .from(&sender_num)
        .build()?;
    // send the message
    client.send_message(&message).await?;
    Ok(())
}

fn prompt(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}> ", prompt);
    std::io::stdout().flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf)
}
