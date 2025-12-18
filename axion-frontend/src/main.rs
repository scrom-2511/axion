use std::fs::write;

use clap::Parser;
use ed25519_dalek::SigningKey;
use tokio_tungstenite::tungstenite::{connect, http::{Request, request}};
use url::Url;

#[derive(Parser, Debug)]
enum Args {
    Start,
    Keygen {
        private_key: Option<String>,
    },
    Send {
        rec_pub_key: String,
        amount_to_send: u64,
    },
    CheckBalance {
        pub_key: String,
    },
}

fn main() {
    let args = Args::parse();
    match args {
        Args::Start => {
            
        },
        Args::Keygen { private_key } => match private_key {
            Some(pri_key) => {
                keygen_using_pri_key(pri_key);
            }
            None => {
                keygen();
            }
        },
        Args::Send {
            rec_pub_key,
            amount_to_send,
        } => {}
        Args::CheckBalance { pub_key } => {}
    }
}

fn keygen_using_pri_key(pri_key: String) {
    let path = if let Some(home) = dirs::home_dir() {
        home.join("axion.txt")
    } else {
        {
            eprintln!("I think you are not running an os");
            return;
        };
    };

    match write(path, pri_key) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("There was an error: \n {}", e)
        }
    }
}

fn keygen() {
    let random_data: [u8; 32] = rand::random();

    let pri_key = SigningKey::from_bytes(&random_data);

    let pri_key_string = bs58::encode(pri_key.as_bytes()).into_string();

    keygen_using_pri_key(pri_key_string);
}