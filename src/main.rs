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
    let mut entries: Vec<VaultEntry> = match fs::read_to_string("data.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    };
    let new_expense = VaultEntry {
        id: 1,
        description: "Proteine".to_string(),
        amount: 45.50,
        category: "Gym".to_string(),
        date: Utc::now(),
        is_completed: true,
    };

    entries.push(new_expense);
    let json_ready = serde_json::to_string_pretty(&entries).unwrap();
    fs::write("data.json", &json_ready).expect("Impossible file writing.");

    println!("Database updated. Total: {}", entries.len());
}
