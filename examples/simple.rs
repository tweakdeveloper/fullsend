use std::{env, error::Error, io::Write};

use fullsend::{Client, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // set up the client
    let account_sid = env::var("TWILIO_ACCOUNT_SID")?;
    let account_tkn = env::var("TWILIO_ACCOUNT_TKN")?;
    let client = Client::builder()
        .account_sid(account_sid)
        .auth_token(account_tkn)
        .build()?;
    // get the destination phone number
    print!("phone number> ");
    std::io::stdout().flush()?;
    let mut phone_num = String::new();
    std::io::stdin().read_line(&mut phone_num)?;
    // get the sender phone number
    let sender_num = env::var("TWILIO_SENDER_NUM")?;
    // set up the message
    let message = Message::builder()
        .to(phone_num.trim())
        .body("howdy from fullsend!")
        .from(&sender_num)
        .build()?;
    // send the message
    client.send_message(&message).await?;
    Ok(())
}
