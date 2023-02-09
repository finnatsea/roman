# Roman

This is a simple program that converts arabic numbers to roman numerals. It's written in rust and has zero dependencies (as expected).

Binaries are available for mac os x (x86_64 and aarch64). To run on other platforms clone and build from source.

## Usage

```bash
# running the binary
./roman

# running from source from /roman directory
cargo run

# running tests
cargo test

# building the binary
cargo build --release

# building for release
cargo build --release --target aarch64-apple-darwin --target x86_64-apple-darwin

```

## Example

```bash
$ ./roman
$ Enter a number to convert to a Roman numeral: 1999
$ The Roman numeral equivalent of 1999 is: MCMXCIX
```