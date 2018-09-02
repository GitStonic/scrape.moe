use rand::random;
use reqwest::get;

use chrono::Utc;
use select::document::Document;
use select::predicate::{Name};

use std::io::{Read, Write};
use std::path::{Path};
use std::fs::{File};

use regex::Regex;

#[derive(Debug)]
pub struct Data {
    pub original_link: String,
    pub images: Vec<String>,
    pub links: Vec<String>,
    pub code: u32
}

impl Data {
    pub fn grab_body(&mut self) -> String {
        let mut body = String::new();

        get(self.original_link.as_str())
            .expect("Cannot grab body...")
            .read_to_string(&mut body)
            .expect("Cannot parse body to string...");

        body
    }

    pub fn parse_to_self(&mut self, body: String) -> Self {
        let new_document = Document::from(body.as_str());

        self.get_links(&new_document);
        self.get_images(&new_document);

        Data {
            original_link: self.original_link.clone(),
            images: self.images.clone(),
            links: self.links.clone(),
            code: random()
        }
    }

    fn get_links(&mut self, document: &Document) {
        for node in document.find(Name("a")) {
            let mut link = node.attr("href")
                .unwrap_or("N_LNK");

            if link == "N_LNK" {
                // Report\
                println!("No link");
            } else {
                self.push_new_item("link", String::from(link));
            }
        }
    }

    fn get_images(&mut self, document: &Document) {
        for node in document.find(Name("img")) {
            let mut link = node.attr("src")
                .unwrap_or("N_LNK");

            if link == "N_LNK" {
                // Report
                println!("No link");
            } else {
                self.push_new_item("image", String::from(link));
            }
        }
    }

    pub fn write_to(&mut self, path: &str) -> Result<(), ()> {
        let file = format!("{}/out_{}-{}.txt", path, self.code, Utc::now().timestamp());
        
        let contents = format!("Code : {}\nImages :\n{}\n-------\nLinks :\n{}\n-------",
            self.code, self.images.join("\n"), self.links.join("\n"));

        File::create(Path::new(file.as_str()))
            .expect("Cannot create file...")
            .write_all(contents.as_bytes())
            .expect("Cannot write to file...");
        

        Ok(println!("Wrote a new data-report."))
    }

    fn push_new_item(&mut self, to: &str, mut item: String) {
        if (item.starts_with("http://") != true) && (item.starts_with("https://") != true) {
            let mut domain_name = String::new();
            let potential_name = Regex::new(r"//(?:[^./]+[.])*([^/.]+[.][^/.]+)/?").unwrap();

            match potential_name.captures(self.original_link.as_str()) {
                Some(domain) => {
                    let temp = domain.get(0).unwrap();

                    let text = temp.as_str();

                    domain_name.push_str(text.replace("/", "").as_str());
                },
                None => println!("No matches we're available")
            }

            let new_item = format!("https://{}/{}", domain_name, item);

            item.push_str(new_item.as_str());
        }

        match to {
            "image" => self.images.push(item),
            "link" => self.links.push(item),
            _ => println!("Invalid 'to', try image or link"),
        }
    }
}
