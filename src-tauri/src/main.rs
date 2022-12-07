#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{fmt::Error, io::Write};

use lib::{invoice::Invoice, invoiceline::Invoiceline};

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
fn new_invoice_line() -> String {
    match serde_json::to_string(&Invoiceline::new()) {
        Ok(str) => return str,
        Err(error) => return error.to_string(),
    }
}
#[tauri::command]
fn resolve_invoice(invoice: String) -> String {
    let res: Result<Invoice, serde_json::Error> = serde_json::from_str(&invoice);

    match res {
        Ok(mut iv) => {
            iv.resolve();
            match serde_json::to_string(&iv) {
                Ok(str) => return str,
                Err(e) => e.to_string(),
            }
        }
        Err(e) => e.to_string(),
    }

    // if res.is_ok() {
    //     let mut invoice: Invoice = res.unwrap();
    //     invoice.resolve();

    //     let response = serde_json::to_string(&invoice);
    //     if response.is_ok() {
    //         let str = response.unwrap();
    //         return str;
    //     } else {
    //         return "error seriliazing response".to_owned();
    //     }
    // } else {
    //      panic!("{:?}",invoice);
    // }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            new_invoice,
            resolve_invoice,
            new_invoice_line
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
