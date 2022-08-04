use colored::*;

use crate::{FuzzResult, Args};

pub fn print_fuzz_result(prog_args: &Args, fuzz_result: &FuzzResult) {
    let status_string = String::from(fuzz_result.status_code.as_str());

    if !prog_args.status_codes.contains(&status_string) && prog_args.status_codes.len() > 0 
    { return }

    let mut status_display: ColoredString = status_string.green();

    if fuzz_result.status_code.is_client_error() || 
        fuzz_result.status_code.is_server_error() {
        status_display = status_string.red()
    } else if fuzz_result.status_code.is_informational() ||
        fuzz_result.status_code.is_redirection() {
        status_display = status_string.blue()
    }

    println!("code {} size {}: {} ({})",
        status_display.bold(),
        fuzz_result.body_length.to_string().cyan(),
        fuzz_result.url,
        fuzz_result.fuzz_word.purple().bold()
    );
}

pub fn print_args(prog_args: &Args) {
    print!("rustfuzz: fuzzing {} using {}, ", 
        prog_args.url.blue(), 
        prog_args.wordlist.blue());
    if prog_args.status_codes.len() > 0 {
        println!("only displaying requests that return {}.",
            prog_args.status_codes.join(", ").blue())
    } else {
        println!("displaying all requests.")
    }
}