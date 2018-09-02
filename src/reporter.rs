use rand::random;
use chrono::prelude::*;

use std::fs::{File};
use std::io::{Write};

pub enum ReportType {
    Warning,
    Error,
    Pass,
}

pub struct Report {
    code: u32,
    report_type: String,
    message: String,
}

pub trait ReportMethods {
    fn new_report(report_type: ReportType, message: String) -> Self;
    fn write_report<E>(self, path: &str) -> Result<(), E>;
}

impl ReportMethods for Report {
    fn new_report(this_type: ReportType, message: String) -> Self {
        let code: u32 = random();
        let report_type = match this_type {
            ReportType::Warning => String::from("Type : Warning"),
            ReportType::Error => String::from("Type : Error"),
            ReportType::Pass => String::from("Type : Pass")
        };

        Report {
            code,
            report_type,
            message,
        }
    }

    fn write_report<E>(self, path: &str) -> Result<(), E> {
        let file_name = format!("{}/report-{}:{}.txt", path, self.code, Utc::now());

        let mut file = File::create(file_name.as_str())
            .expect("Cannot create file");

        let data = format!("Code : {}\nMessage : {}\nReport Type : {}",
            self.code, self.message, self.report_type);

        file.write_all(data.as_bytes())
            .expect_err("Cannot write data to report!");
        
        Ok(println!("New report added..."))
    }
}