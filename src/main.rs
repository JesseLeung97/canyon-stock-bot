extern crate line_bot_sdk_rust as line;
extern crate dotenv;

use std::{env, time, thread};
use line::bot::LineBot;
use line::messages::{SendMessageType, TextMessage};
use dotenv::dotenv;

const PAGE_URL: &str = "https://www.canyon.com/en-jp/road-bikes/race-bikes/ultimate/cf-sl/ultimate-cf-sl-7-wmn-disc/3068.html?dwvar_3068_pv_rahmenfarbe=BU%2FTQ";
const OUTLET_PAGE_URL: &str = "https://www.canyon.com/en-jp/outlet-bikes/?hideSelectedFilters=true&prefn1=pc_outlet&prefn2=pc_rahmengroesse&prefv1=true&prefv2=3XS%7C2XS%7CXS";
const LI_SELECTOR: &str = "li.productConfiguration__optionListItem";
const BUTTON_SELECTOR: &str = "button.productConfiguration__selectVariant";
const PRODUCT_TILE_SELECTOR: &str = "div.js-productTileWrapper";
const PRODUCT_NAME_SELECTOR: &str = "div.productTile__productName";
const PRODUCT_SIZE_SELECTOR: &str = "div.eyebrow";
const ATTR: &str = "data-product-size";
const TARGET_SIZE: &str = "2XS";
const TARGET_NAME: &str = "Ultimate CF SL 7 WMN Disc";
const UPDATE_INTERVAL_SECONDS: u64 = 600;

fn main() {
    dotenv().ok();
    loop {
        println!("Checking for stock");
        let interval = time::Duration::from_secs(UPDATE_INTERVAL_SECONDS);
        if check_stock() {
            send_message(false);
        }
        if check_outlet_stock() {
            send_message(true);
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

fn check_outlet_stock() -> bool {
    let response = reqwest::blocking::get(OUTLET_PAGE_URL)
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let product_tile_selector = scraper::Selector::parse(PRODUCT_TILE_SELECTOR).unwrap();
    let product_name_selector = scraper::Selector::parse(PRODUCT_NAME_SELECTOR).unwrap();
    let product_size_selector = scraper::Selector::parse(PRODUCT_SIZE_SELECTOR).unwrap();

    for element in document.select(&product_tile_selector) {
        let name_element = element.select(&product_name_selector).next();
        let size_element = element.select(&product_size_selector).next();

        let is_name = match name_element {
            Some(name) => name.inner_html().contains(TARGET_NAME),
            None => continue
        };
        let is_size = match size_element {
            Some(size) => size.inner_html().contains(TARGET_SIZE),
            None => continue
        };
        if is_name && is_size {
            return true;
        }
    }
    return false;
}

fn send_message(isOutlet: bool) {
    let channel_secret: &str = &env::var("CHANNEL_SECRET").expect("Failed to get channel secret.");
    let channel_token: &str = &env::var("CHANNEL_TOKEN").expect("Failed to get channel token");
    
    let bot = LineBot::new(channel_secret, channel_token);

    let message_text = format!("There's a 2XS ultimate in stock!\n{}", if isOutlet { PAGE_URL } else { OUTLET_PAGE_URL} );

    let message = SendMessageType::TextMessage(TextMessage {
        text: message_text,
        emojis: None,
    });

    match bot.broadcast(vec![message]) {
        Ok(_) => (),
        Err(err) => println!("{}", err)
    };
}