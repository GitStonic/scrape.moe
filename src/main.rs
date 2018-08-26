#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate select;
extern crate reqwest;
extern crate serde_json;

use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

use std::net::{TcpListener, TcpStream};
use std::io::{stdin, Read, Write};
use std::fs::{File};

#[derive(Debug, Serialize, Deserialize)]
struct EndFile {
    images: Vec<String>,
    links: Vec<String>
}

struct AppendData {
    new_images: Vec<String>,
    new_links: Vec<String>
}

fn grab_body(url: String) -> String {
    let mut body = String::new();

    reqwest::get(url.as_str())
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();
    
    body
}

fn parse_to(document: String, _to: String) {
    let new_document = Document::from(document.as_str());

    for node in new_document.find(Name("body")) {
        // Grab images and links from body document...
    }
}

fn grab_file() -> serde_json::Result<EndFile> {
    let mut data = String::new();

    let mut file = File::open("temp/data.json").unwrap();
    file.read_to_string(&mut data).unwrap();
    

    let end_file: EndFile = serde_json::from_str(&data)?;

    Ok(end_file)
}

fn write_to_file(data: AppendData) {
    let mut file = grab_file().unwrap();

    for link in data.new_links {
        file.links.push(link)
    }

    for image in data.new_images {
        file.images.push(image)
    }

    let new_file = serde_json::to_string(&file)
        .unwrap();

    println!("{}", new_file);
}

fn main() {
    let mut url = String::new();

    println!("Enter a URL:");

    stdin().read_line(&mut url)
        .expect("Failed to grab URL");

    println!("Grabbing data...");

    let html_body = grab_body(url);

    parse_to(html_body, String::from("Something"));
}
