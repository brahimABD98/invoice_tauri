#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::io::Write;

use lib::invoice::Invoice;

mod lib;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn new_invoice() -> String {
    return serde_json::to_string(&Invoice::make()).unwrap();
}
#[tauri::command]
fn resolve_invoice(invoice: String) -> String {
    let res = serde_json::from_str(&invoice);
    if res.is_ok() {
        let mut invoice: Invoice = res.unwrap();
        invoice.resolve();

        let response = serde_json::to_string(&invoice);
        if response.is_ok() {
            let str = response.unwrap();
            return str;
        } else {
            return "error seriliazing response".to_owned();
        }
    } else {
        return "error deserialize".to_owned();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            new_invoice,
            resolve_invoice
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn write(line: &str) -> std::io::Result<()> {
//     let path = "results.json";
//     let mut output = std::fs::File::create(path)?;
//     // let line = "hello";
//     write!(output, "{}", line)
// }
