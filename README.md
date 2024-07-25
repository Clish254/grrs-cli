# grrs

`grrs` is a simple command-line tool written in Rust that searches for a pattern in a file and displays the lines that contain it.
I built this while going though [this](https://rust-cli.github.io/book/index.html) tutorial on how to build rust clis.

## Features

- Search for a specified pattern in a given file.
- Display lines that match the pattern.

## Installation

To install the `grrs` CLI tool, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/your-username/grrs.git
cd grrs
cargo build --release
