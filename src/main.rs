use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct VaultEntry {
    id: u32,
    description: String,
    amount: f64,
    category: String,
    date: DateTime<Utc>,
    is_completed: bool,
}

fn main() {
    let new_expense = VaultEntry {
        id: 1,
        description: "Proteine".to_string(),
        amount: 45.50,
        category: "Gym".to_string(),
        date: Utc::now(),
        is_completed: true,
    };

    let json = serde_json::to_string_pretty(&new_expense).unwrap();

    fs::write("data.json", &json).expect("Impossibile writing the file.");
    println!("Successfully saved your data in data.json!");
}
