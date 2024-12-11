use chrono::prelude::*;
use notify_rust::Notification;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let data = read_json("data.json");
    let today = get_today_date();

    if !check_and_notify(&data, &today) {
        println!("No birthdays today.");
    }
}

// Retrieve information from data.json file
fn read_json(filename: &str) -> HashMap<String, String> {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    serde_json::from_str(&contents).expect("JSON parsing error")
}

// Get today's date
fn get_today_date() -> String {
    let today = Local::now();
    today.format("%d.%m").to_string()
}

// Check if there are any birthdays today and create a pop-up notification
fn check_and_notify(data: &HashMap<String, String>, today: &str) -> bool {
    let mut birthday_found = false;

    for (name, birthday) in data {
        if birthday == today {
            println!("{} has a birthday today!", name);
            Notification::new()
                .summary("Birthday Reminder")
                .body(&format!("{} has a birthday today!", name))
                .show()
                .unwrap();
            birthday_found = true;
        }
    }

    birthday_found
}
