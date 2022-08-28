#![feature(decl_macro)]

extern crate line_bot_sdk_rust as line;
extern crate dotenv;

use std::{env, time, thread};
use line::bot::LineBot;
use line::messages::{SendMessageType, TextMessage};
use dotenv::dotenv;

const PAGE_URL: &str = "https://www.canyon.com/en-jp/road-bikes/race-bikes/ultimate/cf-sl/ultimate-cf-sl-7-wmn-disc/3068.html?dwvar_3068_pv_rahmenfarbe=BU%2FTQ";
const LI_SELECTOR: &str = "li.productConfiguration__optionListItem";
const BUTTON_SELECTOR: &str = "button.productConfiguration__selectVariant";
const ATTR: &str = "data-product-size";
const TARGET_SIZE: &str = "2XS";
const UPDATE_INTERVAL_SECONDS: u64 = 600;

fn main() {
    dotenv().ok();
    loop {
        println!("Checking for stock");
        let interval = time::Duration::from_secs(UPDATE_INTERVAL_SECONDS);
        if check_stock() {
            send_message();
        }
        thread::sleep(interval);
    }
}

fn check_stock() -> bool {
    let response = reqwest::blocking::get(PAGE_URL)
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let li_selector = scraper::Selector::parse(LI_SELECTOR).unwrap();
    let button_selector = scraper::Selector::parse(BUTTON_SELECTOR).unwrap();

    for element in document.select(&li_selector) {
        let button_element = element.select(&button_selector).next();
        let size_name = match button_element {
            Some(item) => item.value().attr(ATTR),
            None => continue
        };
        let in_stock = match size_name {
            Some(name) => name == TARGET_SIZE,
            None => false
        }; 
        if in_stock {
            return in_stock;
        }
    }
    return false;
}

fn send_message() {
    let channel_secret: &str = &env::var("CHANNEL_SECRET").expect("Failed to get channel secret.");
    let channel_token: &str = &env::var("CHANNEL_TOKEN").expect("Failed to get channel token");
    
    let bot = LineBot::new(channel_secret, channel_token);

    let message_text = format!("There's a 2XS ultimate in stock!\n{}", PAGE_URL);

    let message = SendMessageType::TextMessage(TextMessage {
        text: message_text,
        emojis: None,
    });

    match bot.broadcast(vec![message]) {
        Ok(_) => (),
        Err(err) => println!("{}", err)
    };
}