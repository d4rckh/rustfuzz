# rustfuzz

basic web fuzzer

```usage
rustfuzz v1.0
d4rckh
a fuzzer written in rust

USAGE:
    rustfuzz.exe [OPTIONS] --url <URL> --wordlist <WORDLIST>

OPTIONS:
    -h, --help                          Print help information
    -s, --status-code <STATUS_CODES>    Specify a status code to show (flag can be used multiple
                                        times)
    -u, --url <URL>                     URL to be fuzzed contaning the string 'FUZZ'
    -V, --version                       Print version information
    -w, --wordlist <WORDLIST>           Path to file from which fuzz words will be read
```