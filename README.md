# Happy Friday!

Slack notifier/ping application. The goal of this project is to produce a
single, (mostly) standalone binary which is able to run on most modern Linux
distributions (and by extension, AWS Lambda).

It is scheduled to run on Fridays at 07:00 eastern time.

# Example

![friday](https://cloud.githubusercontent.com/assets/3905798/19406168/ecebd63c-924e-11e6-8a48-cc32f998e5a4.png)

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
```

# Compiling on Amazon Linux

By compiling this application on an Amazon Linux AMI, you have a much better
chance at binary compatibility with the AWS Lambda service.

```bash
# Install development tools
$ sudo yum install git
$ sudo yum groupinstall "Development Tools"
$ sudo yum --enablerepo=amzn-preview install rust

# Download and install Cargo using binaries from https://crates.io/install
$ wget https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz
$ tar xvf cargo-nightly-x86_64-unknown-linux-gnu.tar.gz
$ cd cargo-nightly-x86_64-unknown-linux-gnu
$ sudo ./install.sh

# Navigate to your cloned, configured repo
$ cd happy-friday

# Package the application for AWS lambda
$ make
```

[rust]: https://www.rust-lang.org/
[cargo]: https://github.com/rust-lang/cargo
