// pub mod core;
// pub mod utils;
// use core::rce;
// use reqwest::{self, Client};
// use utils::request;

// #[cfg(test)]
// mod tests {

//     use super::*;
//     // #[test]
//     // fn test_rand() {
//     //     println!("{:?}", rce::gen_random_key());
//     // }

//     // #[tokio::test]
//     // async fn test_get() {
//     //     let client = reqwest::Client::new();
//     //     let res = request::get_with_key(
//     //         "http://127.0.0.1/webshell.php".to_string(),
//     //         "1",
//     //         "phpinfo();",
//     //         &client,
//     //     )
//     //     .await
//     //     .unwrap();
//     //     println!("res: {:?}", res);
//     // }

//     // #[tokio::test]
//     // async fn test_echo_system_command() -> Result<(), Box<dyn std::error::Error>> {
//     //     let text = rce::echo_base64_encode_system_command(
//     //         "http://127.0.0.1/webshell.php".to_string(),
//     //         "1",
//     //         "dir".to_string(),
//     //     )
//     //     .await?;
//     //     println!("{:?}", text);
//     //     Ok(())
//     // }
// }
