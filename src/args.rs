use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author="d4rckh", 
    version="v1.0", 
    about="a fuzzer written in rust", 
    long_about="if you want to contribute to this project, check out the github repo: https://github.com/d4rckh/rustfuzz",
)]
pub struct ProgramArgs {
    #[clap(
        short='u', 
        long="url", 
        help="URL to be fuzzed contaning the string 'FUZZ'"
    )]
    pub url: String,

    #[clap(
        short='w', 
        long="wordlist", 
        help="Path to file from which fuzz words will be read"
    )]
    pub wordlist: String,

    #[clap(
        short='s', 
        long="status-code", 
        help="Specify a status code to show (flag can be used multiple times)"
    )]
    pub status_codes: Vec<String>,

    #[clap(
        default_value="", 
        short='o', 
        long="output", 
        help="Specify the file in which the results will be saved")
    ]
    pub file_save: String
}
