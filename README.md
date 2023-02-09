# Roman

This is a simple program that converts arabic numbers to roman numerals. It's written in rust and has zero dependencies (as expected).

Release builds are available for mac os x aarch64 (M1/Apple Silicon). To run on other platforms clone and build from source.

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
```

## Example

```bash
$ ./roman
$ Enter a number to convert to a Roman numeral: 1999
$ The Roman numeral equivalent of 1999 is: MCMXCIX
```