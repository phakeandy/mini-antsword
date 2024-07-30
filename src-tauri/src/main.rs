// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod core;
pub mod utils;

use core::{file_upload, rce};
use reqwest::Client;
use std::fs::read_to_string;
use std::path::Path;
use std::result::Result;
use utils::request;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            base64_encode_system_command,
            upload_file,
            read_file_to_string,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn base64_encode_system_command(
    url: String,
    key: String,
    command: String,
) -> Result<String, String> {
    let base64_echo = rce::echo_base64_encode_system_command(url, key, command).await;
    match base64_echo {
        Ok(base64_echo) => Ok(base64_echo),
        Err(_) => Err(String::from("error in echo encoded result.")),
    }
}

/// Receive base64 encoded file content and upload it to url.
#[tauri::command]
async fn upload_file(
    file_base64_content: String,
    url: String,
    key: String,
    filename: String,
) -> bool {
    let client = Client::new();
    // rce::upload_file(file_base64_content, url, key).await;
    let _ = file_upload::file_upload(&url, &key, &file_base64_content, &filename, &client).await;

    let url = request::modify_url(&url, &filename);

    match url {
        Some(url) => request::is_url_return_ok(&url, &client).await,
        None => false,
    }
}

#[tauri::command]
fn read_file_to_string(path: String) -> Result<String, String> {
    match read_to_string(Path::new(&path)) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// #[tauri::command]
// fn my_custom_command() -> String {
//     String::from("I am a String.  was invoked from JS!")
// } // test

// #[tauri::command]
// fn submit_form(url: String, password: String) -> String {
//     format!(
//         "Recive Url: {}\
//     Recive Password: {}",
//         url, password
//     )
// } // test

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![
//             my_custom_command,
//             submit_form,
//             base64_encode_system_command
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
