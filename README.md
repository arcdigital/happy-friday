# Happy Friday!

Slack notifier/ping application. The goal of this project is to produce a
single, (mostly) standalone binary which is able to run on most modern Linux
distributions (and by extension, AWS Lambda).

It is scheduled to run on Fridays at 07:00 eastern time.

# System requirements

 - [Rust][rust] `=1.12.0`
 - [Cargo][cargo] `=0.13.0`
 - make

# Getting started

```bash
# Navigate to the src/ directory
$ cd src

# Copy the example configuration file; edit where appropriate
$ cp config.example.rs config.rs

# Compile the application
$ cargo build

# Run the application
$ cargo run

# Package the application for AWS lambda
$ make
```

[rust]: https://www.rust-lang.org/
[cargo]: https://github.com/rust-lang/cargo
