# Birthday Checker

A simple Rust program that checks if today or tomorrow is someone's birthday based on a predefined list of birthdays. It sends desktop notifications for birthdays and runs on startup to remind you of upcoming birthdays.

## Features
- Reads birthday data from a JSON file.
- Checks if today or tomorrow is someone's birthday.
- Sends a desktop notification if there's a birthday.
- Runs on startup for periodic reminders.

## Requirements
- **Rust**: The program is written in Rust. Make sure to have Rust installed. Follow the installation instructions at [Rust's official website](https://www.rust-lang.org/).
- **Dependencies**:
  - `chrono`: For date and time manipulation.
  - `notify-rust`: For sending desktop notifications.
  - `serde` & `serde_json`: For parsing JSON data.

## Installation

### 1. Clone the repository
```bash
git clone https://github.com/your-username/birthday-checker.git
cd birthday-checker
