#[macro_use] extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate regex;
extern crate select;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use std::io::{stdin};

mod reporter;
mod data_manager;

use data_manager::{Data};

fn main() {
    let mut url = String::new();

    println!("Enter a URL:");

    stdin().read_line(&mut url)
        .expect("Failed to grab URL");

    println!("Grabbing data...");

    let mut data_parser = Data {
        original_link: url,
        images: Vec::new(),
        links: Vec::new(),
        code: 0
    };

    let body = data_parser.grab_body();
    data_parser
        .parse_to_self(body)
        .write_to("out")
        .expect("Cannot write...");
}
