// lib.rs
mod models;
mod api;
mod client;

use std::error::Error;
pub use client::Client;
pub use models::*;

#[tokio::test]
async fn trying() {
    let mut c = Client::new("key", "gemini-2.5-flash");
    let res = c.chat("Hello my name is Max").await;
    if res.is_err() {
    panic!("{}", res.err().unwrap());}
    let res2 = c.chat("Whats my name answer in single word only (answer containing more than 1 word will be treated as incorrect)").await;
    let name = res2.unwrap_or_else(|_| "Error".to_string());
    dbg!(&name);
    assert_eq!(&name, "Max");
}