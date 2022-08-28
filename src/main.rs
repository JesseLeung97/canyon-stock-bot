#![feature(decl_macro)]

extern crate line_bot_sdk_rust as line;
extern crate dotenv;

use std::env;
use line::bot::LineBot;
use line::messages::{SendMessageType, TextMessage};
use dotenv::dotenv;

const PAGE_URL: &str = "https://www.canyon.com/en-jp/road-bikes/race-bikes/ultimate/cf-sl/ultimate-cf-sl-8-wmn-disc/3069.html?dwvar_3069_pv_rahmenfarbe=BU%2FTQ";
// "https://www.canyon.com/en-jp/road-bikes/race-bikes/ultimate/cf-sl/ultimate-cf-sl-7-wmn-disc/3068.html?dwvar_3068_pv_rahmenfarbe=BU%2FTQ",
const LI_SELECTOR: &str = "li.productConfiguration__optionListItem";
const BUTTON_SELECTOR: &str = "button.productConfiguration__selectVariant";
const ATTR: &str = "data-product-size";
const TARGET_SIZE: &str = "2XS";

fn main() {
    dotenv().ok();

    let channel_secret: &str = &env::var("CHANNEL_SECRET").expect("Failed to get channel secret.");
    let channel_token: &str = &env::var("CHANNEL_TOKEN").expect("Failed to get channel token");
    
    let bot = LineBot::new(channel_secret, channel_token);
    
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
        println!("In stock: {}", in_stock);
        if in_stock {
            send_message(&bot);
        }
    }
}

fn send_message(bot: &LineBot) {
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