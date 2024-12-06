use std::io::Read;
use base64::{Engine as _, engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD}};
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use crate::{Base64Format, get_reader};
use anyhow::Result;

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    println!("input {}, format: {}", input, format);

    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf);
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };
    Ok(decoded)
}



#[test]
fn test_process_decode() {
    let input = "cargo.toml" ;
    let format = Base64Format::Standard;
    assert!(process_decode(input, format).is_ok());
}