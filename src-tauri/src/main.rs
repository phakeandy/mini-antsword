// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

use mini_antsword_ts::echo_base64_encode_system_command;

#[tauri::command]
fn my_custom_command() -> String {
    String::from("I am a String.  was invoked from JS!")
} // test

#[tauri::command]
fn submit_form(url: String, password: String) -> String {
    format!(
        "Recive Url: {}\
    Recive Password: {}",
        url, password
    )
} // test

#[tauri::command]
async fn base64_encode_system_command(
    url: String,
    key: String,
    command: String,
) -> Result<String, String> {
    let base64_echo = echo_base64_encode_system_command(url, key, command).await;
    match base64_echo {
        Ok(base64_echo) => Ok(base64_echo),
        Err(err) => Err(String::from("error in echo encoded result.")),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            submit_form,
            base64_encode_system_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
