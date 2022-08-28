#![feature(decl_macro)]

extern crate line_bot_sdk_rust as line;
extern crate rocket;
extern crate dotenv;

use std::env;

use line::bot::LineBot;
use rocket::http::Status;
use rocket::routes;
use rocket::post;

use line::events::messages::MessageType as EventMessageType;
use line::events::{EventType, Events};
use line::messages::{SendMessageType, TextMessage};
use line::support::rocket_support::{Body, Signature};
use dotenv::dotenv;

const PAGE_URL: &str = "https://www.canyon.com/en-jp/road-bikes/race-bikes/ultimate/cf-sl/ultimate-cf-sl-8-wmn-disc/3069.html?dwvar_3069_pv_rahmenfarbe=BU%2FTQ";
// "https://www.canyon.com/en-jp/road-bikes/race-bikes/ultimate/cf-sl/ultimate-cf-sl-7-wmn-disc/3068.html?dwvar_3068_pv_rahmenfarbe=BU%2FTQ",
const LI_SELECTOR: &str = "li.productConfiguration__optionListItem";
const BUTTON_SELECTOR: &str = "div.productConfiguration__selectVariant";
const ATTR: &str = "data-product-size";
const TARGET_SIZE: &str = "2XS";



fn main() {
    dotenv().ok();
    rocket::ignite().mount("/", routes![callback]).launch();

    
    
    
    
    // let bot = LineBot::new(CHANNEL_SECRET, CHANNEL_TOKEN);
    
    // let response = reqwest::blocking::get(PAGE_URL)
    //     .unwrap()
    //     .text()
    //     .unwrap();

    // let document = scraper::Html::parse_document(&response);

    // let li_selector = scraper::Selector::parse(LI_SELECTOR).unwrap();
    // let button_selector = scraper::Selector::parse(BUTTON_SELECTOR).unwrap();

    // for element in document.select(&li_selector) {
    //     let button_element = element.select(&button_selector).next();
    //     let size_name = match button_element {
    //         Some(item) => item.value().attr(ATTR),
    //         None => continue
    //     };
    //     let in_stock = match size_name {
    //         Some(name) => name == TARGET_SIZE,
    //         None => false
    //     };

    //     println!("Stock: {:?}", in_stock);
        
    // }
}

#[post("/callback", data = "<body>")]
fn callback(signature: Signature, body: Body) -> Status {
    let channel_secret: &str = &env::var("CHANNEL_SECRET").expect("Failed to get channel secret.");
    let channel_token: &str = &env::var("CHANNEL_TOKEN").expect("Failed to get channel token");

    let bot = LineBot::new(channel_secret, channel_token);

    let result: Result<Events, &'static str> = bot.parse_event_request(&signature.key, &body.string);

    if let Ok(res) = result {
        for event in res.events {
            // MessageEvent only
            if let EventType::MessageEvent(message_event) = event.r#type {
                // TextMessageEvent only
                if let EventMessageType::TextMessage(text_message) = message_event.message.r#type {
                    // Create TextMessage
                    let message = SendMessageType::TextMessage(TextMessage {
                        text: text_message.text,
                        emojis: None,
                    });
                    // Reply message with reply_token
                    let _res = bot.reply_message(&message_event.reply_token, vec![message]);
                }
            }
        }
        return Status::new(200, "OK");
    }
    else if let Err(msg) = result {
        return Status::new(500, msg);
    }
    Status::new(500, "Internal Server Error")
}

// fn send_message(line_bot: &LineBot) {
//     line_bot.push_message(to, msgs)
// }
