#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate regex;
extern crate select;
extern crate reqwest;
extern crate serde_json;

use regex::Regex;

use select::document::Document;
use select::predicate::{Name, Predicate};

use std::net::{TcpListener, TcpStream};
use std::io::{stdin, Read, Write};
use std::fs::{File, OpenOptions};

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

fn parse_to(document: String, url: String) -> Result<(), &'static str> {
    let mut page_links: Vec<String> = Vec::new();
    let mut page_images: Vec<String> = Vec::new();
    let new_document = Document::from(document.as_str());

    for node in new_document.find(Name("a")) {
        let mut page_link = String::new();
        let mut link = node.attr("href").unwrap_or("Cannot grab a link...");

        if (link.starts_with("http://") != true) && (link.starts_with("https://") != true) {
            // Just going to assume if the HTTP prefixes are missing it must be a backend proxy...
            // Basically if the original link was like http://example.com/some/file but instead they give me /some/file...
            // We can assume that `/some/file` is paramters towards the original link/url... hence we add the missing parts to the paramaters...

            // I'm just going to use https:// in this case because most websites support it already... might make a function that supports http:// soon...
            println!("Neither http or https");

            let mut domain_name = String::new();
            let potential_name = Regex::new(r"//(?:[^./]+[.])*([^/.]+[.][^/.]+)/?").unwrap();

            match potential_name.captures(url.as_str()) {
                Some(domain) => {
                    let temp = domain.get(0).unwrap();

                    let text = temp.as_str();

                    domain_name.push_str(text.replace("/", "").as_str());
                },
                None => println!("No matches we're available")
            }

            let new_link = format!("https://{}/{}", domain_name, link);

            page_link.push_str(new_link.as_str());
        } else {
            page_link.push_str(link)
        }

        page_links.push(String::from(page_link));
    }

    for node in new_document.find(Name("img")) {
        println!("Image link? {:?}", node.attr("src"));
    }

    let data = AppendData {
        new_images: page_images,
        new_links: page_links
    };

    write_to_file(data);
    Ok(println!("PARSE: Pass..."))
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
        if file.links.is_empty() != true {
            for check_link in file.links.clone() {
                if check_link == link {
                    println!("{} is already inserted", check_link);
                } else {
                    println!("Do something else... it's not inserted");
                }
            } 
        };

        file.links.push(link)
    }

    for image in data.new_images {
        if file.images.is_empty() != true {
            for check_image in file.images.clone() {
                if check_image == image {
                    println!("{} is already inserted", check_image);
                } else {
                    println!("Do something else... it's not inserted");
                }
            }
        }

        file.images.push(image)
    }

    let new_file = serde_json::to_string(&file)
        .unwrap();

    let mut f = OpenOptions::new()
        .write(true)
        .open("temp/data.json")
        .unwrap();
    
    f.write_all(new_file.as_bytes()).unwrap();
}

fn main() {
    let mut url = String::new();

    println!("Enter a URL:");

    stdin().read_line(&mut url)
        .expect("Failed to grab URL");

    println!("Grabbing data...");

    let html_body = grab_body(url.clone());

    parse_to(html_body, url.clone()).unwrap();
}
