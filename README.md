<h1 align="left">Bcrypt - CLI Tool by reeth</h1>
<div align="center">

![https://www.rust-lang.org/](https://img.shields.io/badge/Rust-24273A.svg?style=flat&logo=rust&logoColor=fc8c03) 

</div>

## Table of Contents
- [About](#-about)
- [Flags](#-flags)
- [Installation](#-installation)


## 📖 About

This is a small script based on Rust, for comparing and encrypting Strings with bcrypt with a custom CLI tool.
It's designed for optimizing time on development and testing out of code blocks on the app.

## 🚩 Flags

| Flag                  | Behaviour                                                 |
|-----------------------|---------------------------------------------------------- |
| *encrypt*             | expects a string and a salt number to encrypt the string. |
| *compare*             | expects a string and a hash nto verify its integrity.     |


## 🚀 Installation

1. Clone [this repository](https://github.com/reethfx/bcrypt-cli-tool).

2. Exectue `cargo build --release` this will generate `/target/bcrypt_cli` directory.

3. Simply execute `./target/release/bcrypt_cli` on the specified directory to run the script. You can add it by alias on your shell to make it accesible.`