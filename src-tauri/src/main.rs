#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lib::invoice::{Invoice};
use serde::Serialize;

mod lib;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn new_invoice()->String{
    return serde_json::to_string(&Invoice::make()).unwrap()  ;
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,new_invoice])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


