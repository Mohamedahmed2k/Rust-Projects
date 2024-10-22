# Rust Utilities Repo

This repository contains three Rust-based utilities: a **Calculator**, a **SHA-256 Hash Cracker**, and a **URL Shortener**. These programs demonstrate different features of the Rust programming language, from basic mathematical operations to security functions and URL management.

## Table of Contents
- [Installation](#installation)
- [Utilities Overview](#utilities-overview)
    - [1. Rust Calculator](#1-rust-calculator)
    - [2. SHA-256 Hash Cracker](#2-sha-256-hash-cracker)
    - [3. URL Shortener](#3-url-shortener)
- [Usage](#usage)
- [References](#references)

---

## Installation

To get started, ensure you have Rust installed on your Linux machine. You can install Rust by running the following commands:

1. Open a terminal and enter:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Follow the instructions to install Rust and set up your environment.
3. Once installed, make sure that Rust is correctly installed by running:

    ```bash
    rustc --version
    ```

For more details, refer to the official [Rust documentation](https://www.rust-lang.org/tools/install).

---

## Utilities Overview

### 1. Rust Calculator
The **Rust Calculator** is a simple command-line calculator capable of performing basic operations like addition, subtraction, multiplication, division, and modulo. It supports both integers and floating-point numbers.

#### Features:
- Operations: `+`, `-`, `*`, `/`, `%`
- Input validation for integers

### 2. SHA-256 Hash Cracker
The **SHA-256 Hash Cracker** attempts to match a given SHA-256 hash with a password from a wordlist. It uses the popular "rockyou.txt" password list as a dictionary.

#### Features:
- Command-line interface for hash input
- Attempts to match the input hash with passwords from a wordlist

### 3. URL Shortener
The **URL Shortener** converts long URLs into short, randomly generated URLs and stores the mappings in a file (`mapping.txt`). You can also input a short URL to retrieve the corresponding long URL.

#### Features:
- Generates an 8-character alphanumeric short URL
- Saves URL mappings to a file
- Retrieves long URLs from short ones

---

## Usage

Each of these utilities can be run independently via the command line.

### 1. Rust Calculator

To run the calculator, navigate to the calculator directory and execute the application.

Follow the on-screen prompts to input numbers and operators.

Here's an example of how it works:

Rust calculator
Enter value of X
5
Enter value of Y
3
choose operator: + ,* ,- ,/ ,%
+
5 + 3 = 8

### 2. SHA-256 Hash Cracker

To run the hash cracker, provide the SHA-256 hash as an argument.

For example:

e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

The program will attempt to match the hash with passwords from the rockyou.txt wordlist. Ensure you have the wordlist file (rockyou.txt) in the correct location (e.g., the src/ directory of the project).

The output will show the number of attempts and whether the password was found.

### 3. URL Shortener

To shorten a URL, provide the long URL as an argument.

For example:

http://example.com

To retrieve a long URL from a short one, provide the short URL.

The URL mappings are stored in a file called mapping.txt in the src folder.
---
## References

- Rust Official Documentation: https://doc.rust-lang.org/
- Rust Installation Guide: https://www.rust-lang.org/tools/install
- sha2 Rust crate documentation: https://docs.rs/sha2/latest/sha2/


