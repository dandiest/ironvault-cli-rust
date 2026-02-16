<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Stable-brightgreen.svg" /> 
  <img src="https://img.shields.io/badge/Library-Serde-blue.svg" />
  <img src="https://img.shields.io/badge/Library-Chrono-red.svg" />
</p>

<h1 align="center">🛡️ IronVault CLI - Persistent Finance & Task Tracker</h1>

<p align="center">
  A robust Command Line Interface (CLI) engine demonstrating advanced Rust patterns: JSON Serialization and Time Management.
</p>

---

## 🎓 Educational Disclaimer
This repository is part of my **Personal Apprenticeship** and learning journey with the Rust programming language. 
* **Purpose**: This code is strictly for educational purposes. It serves as a practical sandbox to master Rust's ownership system, external crate integration, and file system interaction.
* **Evolution**: As I progress in my apprenticeship, this code will be refactored to implement more advanced concepts (Error Handling, Traits, Generics).
* **Feedback**: Constructive feedback is welcome as I work towards becoming a proficient Rust developer!

## 🌟 Features
* **JSON Persistence**: Uses `Serde` to save and load data from `data.json`, ensuring your information survives program restarts.
* **Dynamic Collections**: Manages a `Vec<VaultEntry>` to handle multiple financial records simultaneously.
* **Time Tracking**: Integrates `Chrono` for automatic, high-precision UTC timestamping of every entry.
* **Scalable IDs**: Automatic ID generation based on collection length for unique entry identification.

## 🛠️ Technical Deep Dive
* **Serialization/Deserialization**: Leverages `serde_json` to bridge the gap between Rust's strong types and the flexible JSON format.
* **Crate Features**: Configured `chrono` with the `serde` feature in `Cargo.toml` to allow seamless integration between time types and JSON.
* **File System (std::fs)**: Uses `fs::read_to_string` and `fs::write` to manage the local database with safety checks via `.expect()`.
* **Pattern Matching**: Implements `match` on file reading to handle "File Not Found" scenarios gracefully by initializing a new vector.



---

## 🚀 How to Run
1. Clone the repository.
2. Ensure you have [Rust](https://www.rust-lang.org/) installed.
3. Run the following command in your terminal:
   ```bash
   cargo run

## ⚖️ License & Copyright

Copyright (c) 2026 **[dandiest]**

This project is licensed under the MIT License.

*You are free to use, study, and modify this code for educational purposes. Please provide attribution if you use significant portions of this logic in your own learning journey.*
