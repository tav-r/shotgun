# shotgun
A small tool to (time-)efficiently find reflected URL parameters. It reads URLs from stdin, replaces parameter values with random strings and makes requests to the modified URL to see if the values are reflected. This is not a very subtle approach, hence the name.

## Installation
Clone the repo, `cd` in it and run
```
cargo build --release
```
You need the rust toolchain for this.

## Usage
```
Read URLs from stdin and check for reflected parameters in responses

USAGE:
    shotgun [FLAGS] [OPTIONS]

FLAGS:
    -h, --help            Prints help information
    -p, --picky           Only report matches where the value not (only) reflected as part of the whole URL
    -s, --script-block    Only report matches inside HTML <script>-blocks
    -V, --version         Prints version information

OPTIONS:
    -j, --cookie-string <key1=value1; key2=value2; ...>    Set a cookie string for GET requests
```

Examples:
```
$ echo "https://ace21ffe1e887dac808b066f001b0033.web-security-academy.net/?search=test" | ./shotgun -p -s
[https://ace21ffe1e887dac808b066f001b0033.web-security-academy.net/?search=1IlXTlIWXs8FEaKlWncx] reflected search in script block
```
