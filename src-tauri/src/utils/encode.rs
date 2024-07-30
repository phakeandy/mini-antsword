use base64::prelude::*;

/// result example 123 -> 202cb962ac59075b964b07152d234b70
pub fn md5(input: String) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

///# Example
/// ```
///let encoded_string = "dGVzdA==";
///assert_eq!(decode_base64(encoded_string).unwrap(), "test")
/// ```
pub fn decode_base64(encoded_string: &str) -> Result<String, base64::DecodeError> {
    let res = BASE64_STANDARD.decode(encoded_string.as_bytes());
    match res {
        Ok(decoded_string) => Ok(String::from_utf8(decoded_string).unwrap()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_base64() {
        let encoded_string = "dGVzdA==";
        assert_eq!(decode_base64(encoded_string).unwrap(), "test")
    }
}
