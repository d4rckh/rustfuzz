mod logging;

use clap::Parser;

use reqwest::Url;
use reqwest::Method;
use core::panic;
use std::error::Error;

use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct FuzzResult {
    fuzz_word: String,
    url: String,
    status_code: reqwest::StatusCode,
    body_length: usize
}

#[derive(Parser, Debug)]
#[clap(author="d4rckh", version="v1.0", about="a fuzzer written in rust", long_about = None)]
pub struct Args {
    #[clap(short='u', long="url", help="URL to be fuzzed contaning the string 'FUZZ'")]
    url: String,

    #[clap(short='w', long="wordlist", help="Path to file from which fuzz words will be read")]
    wordlist: String,

    #[clap(short='s', long="status-code", help="Specify a status code to show (flag can be used multiple times)")]
    status_codes: Vec<String>
}

fn get_url(url: &str, fuzz_word: &str) -> (Url, String) {
    let new_url = url.replace("FUZZ", fuzz_word);
    
    match Url::parse(&new_url) {
        Ok(u) => return (u, new_url),
        Err(_e) => {
            panic!("Error while building URL: {}", new_url)
            // TODO: std::process::exit(1);
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    if !args.url.contains("FUZZ") {
        println!("The URL provided does not contain a fuzzable area.");
        std::process::exit(1);
    }

    logging::print_args(&args);


    let client = reqwest::Client::new();
    
    let file = File::open(&args.wordlist)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let fuzz_word = line?;
        let (url, url_string) = get_url(&args.url, &fuzz_word);

        let request = reqwest::Request::new(Method::GET, url);
        let response = client.execute(request).await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        let fuzz_result = FuzzResult {
            fuzz_word: fuzz_word.clone(),
            url: url_string.clone(),
            status_code: status,
            body_length: body.len(),
        };

        logging::print_fuzz_result(&args, &fuzz_result);
    }

    Ok(())
}
