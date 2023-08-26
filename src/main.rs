// #![allow(unused_variables,unused_imports,dead_code)]
use base64::{Engine as _, engine::general_purpose};
use std::env;

use chrono::Utc;
use tungstenite::{connect, Message};
use url::Url;

use ring::hmac;


struct StreamInfo {
    url: String,
    init_message: String,
}

fn make_auth_message(api_key: &str, secret_key: &str, passphrase: &str) -> String {
    let timestamp = Utc::now().timestamp().to_string();
    // let timestamp = "1538054050".to_string();
    let sign_input = format!(r"{}GET/users/self/verify", timestamp);

    let key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_bytes());
    let tag = hmac::sign(&key, sign_input.as_bytes());
    let sign = general_purpose::STANDARD.encode(tag.as_ref());
    let full_message = format!(
        r##"{{"op":"login","args":[{{"apiKey":"{}","passphrase":"{}","timestamp":"{}","sign":"{}"}}]}}"##,
        api_key, passphrase, timestamp, sign
    );

    full_message
    // sign
}

fn main() {
    // runner();
    dotenvy::dotenv().expect("Failed# to read .env file");
    let stream_info = StreamInfo {
        url: "wss://ws.okx.com:8443/ws/v5/private".to_string(),
        init_message: "Hello".to_string(),
    };
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let passphrase = env::var("PASSPHRASE").expect("PASSPHRASE not set");
    let con_msg = make_auth_message(&api_key, &secret_key, &passphrase);

    let url = Url::parse(&stream_info.url).expect("not a valid url");

    let (mut socket, _res) = match connect(url) {
        Ok(s) => s,
        Err(e) => panic!("Can't connect: {}", e),
    };

    socket
        .send(Message::Text(con_msg))
        .expect("Error sending message");
    let res = socket.read().expect("Error reading message");
    println!("{:?}", res);
}