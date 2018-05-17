extern crate github_rs;
extern crate serde_json;

use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::env;

pub const BOT_ENV_VAR: &'static str = "SHIBA_BOT_GITHUB_API_TOKEN";

fn main() {
    let api_token = env::var(BOT_ENV_VAR).expect(&format!("{} must be set", BOT_ENV_VAR));
    let client = Github::new(api_token).unwrap();
    let me = client.get().user().execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json {
                println!("{}", json);
            }
        }
        Err(e) => println!("{}", e),
    }
}
