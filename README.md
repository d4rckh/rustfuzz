# rustfuzz

basic web fuzzer

## building

```
cargo build --release
```

## usage

```usage
USAGE:
    rustfuzz.exe [OPTIONS] --url <URL> --wordlist <WORDLIST>

OPTIONS:
    -h, --help                          Print help information
    -o, --output <FILE_SAVE>            Specify the file in which the results will be saved
                                        [default: ]
    -s, --status-code <STATUS_CODES>    Specify a status code to show (flag can be used multiple
                                        times)
    -u, --url <URL>                     URL to be fuzzed contaning the string 'FUZZ'
    -V, --version                       Print version information
    -w, --wordlist <WORDLIST>           Path to file from which fuzz words will be read```