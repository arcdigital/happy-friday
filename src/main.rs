extern crate hyper;
extern crate rand;
extern crate rustc_serialize;

use hyper::Client;
use rustc_serialize::json;
use rand::Rng;

mod config;

// Incoming webhook message format
#[derive(RustcEncodable)]
struct SlackMessage {
    text: String
}

fn main() {
    // Select a random video from this list of fun fun fun videos
    let videos = [
        "https://www.youtube.com/watch?v=kfVsfOSbJY0",
        "https://www.youtube.com/watch?v=I1188GO4p1E",
        "https://www.youtube.com/watch?v=KlyXNRrsk4A",
        "https://www.youtube.com/watch?v=d9TpRfDdyU0"
    ];
    let video = rand::thread_rng().choose(&videos).unwrap();

    // Message data
    let msg = SlackMessage {
        text: format!("Happy Friday! {}", video).to_string()
    };
    let encoded_msg = json::encode(&msg).unwrap();

    // Send to Slack via HTTP POST
    let client = Client::new();
    let res = client.post(config::WEBHOOK_URL)
        .body(&encoded_msg)
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);
}
