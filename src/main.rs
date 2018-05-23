extern crate reqwest;
extern crate serde_json;
extern crate irc;
extern crate regex;

use serde_json::Value;
use irc::client::prelude::*;
use regex::Regex;

fn main () {
    let cfg = Config {
        nickname: Some("mbot".to_owned()),
        server: Some("localhost".to_owned()),
        channels: Some(vec!["#bots".to_owned(), "#tildetown".to_owned()]),
        ..Default::default()
    };

    let client = IrcClient::from_config(cfg).unwrap();

    client.identify().unwrap();

    client.for_each_incoming( |irc_msg| {
        if let Command::PRIVMSG(channel, message) = irc_msg.command {
            if message.starts_with("!wiki") {
                match message.split(" ").nth(1) {
                    Some(word) => client.send_privmsg(&channel, get_extract(String::from(word)).as_str()).unwrap(),
                    None => client.send_privmsg(&channel, "Usage \"!wiki ThingToSearch\"").unwrap()
                };
            };
        };
    }).unwrap();
}

fn get_extract(word: String) -> String {
    let text = reqwest::get(format!("https://en.wikipedia.org/w/api.php?action=query&prop=extracts&format=json&exintro=&titles={}", word).as_str()).expect("").text().expect("");

    let parsed:Value = serde_json::from_str(&text).expect("");
    let pages = &parsed["query"]["pages"];
    let key = pages.as_object().expect("").keys().last().expect("");
    let re = Regex::new(r"<[^>]*>").unwrap();
    String::from(re.replace_all(pages[key]["extract"].as_str().expect("").split(".").nth(0).unwrap(),""))
}
