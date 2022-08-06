use core::panic;

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};

use chrono::{prelude::*, Duration};
use reqwest::{Url};
use clap::Parser;

mod logging;
mod args;

use args::ProgramArgs;

pub struct FuzzResult {
    fuzz_word: String,
    url: String,
    status_code: reqwest::StatusCode,
    body_length: usize,
    time: DateTime<Local>,
    request_duration: Duration
}

fn get_url(url: &str, fuzz_word: &str) -> (Url, String) {
    let new_url = url.replace("FUZZ", fuzz_word);
    
    match Url::parse(&new_url) {
        Ok(u) => (u, new_url),
        Err(_e) => panic!("Error while building URL ({new_url}) with word {fuzz_word}")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = ProgramArgs::parse();
    
    if !args.url.doescontains("FUZZ") {
        println!("The URL provided does not contain a fuzzable area.");
        std::process::exit(1);
    }

    logging::print_args(&args);

    let client = reqwest::Client::new();
    
    let file = File::open(&args.wordlist)?;
    let reader = BufReader::new(file);
    
    let mut file_save: Option<File> = None;

    if args.file_save != "" {
        file_save = Some(OpenOptions::new()
            .append(true)
            .open(&args.file_save)
            .expect("Unable to open file for saving results"));
    }

    for line in reader.lines() {
        let fuzz_word = line?;
        let (url, url_string) = get_url(&args.url, &fuzz_word);

        let request_builder = client.get(url);
        let request = request_builder.build()?;
        
        let time_before_res = Local::now();
        let response = client.execute(request).await?;
        let time_after_res = Local::now();

        let duration = time_after_res - time_before_res;
        let status = response.status();
        let body = response.text().await?;
        
        let fuzz_result = FuzzResult {
            fuzz_word: fuzz_word.clone(),
            url: url_string.clone(),
            status_code: status,
            body_length: body.len(),
            time: Local::now(),
            request_duration: duration,
        };

        if logging::print_fuzz_result(&args, &fuzz_result) {
            if let Some(file) = &mut file_save {
                fuzz_result.save(file)
            }
        }
    }

    Ok(())
}
