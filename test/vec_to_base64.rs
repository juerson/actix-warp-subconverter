use base64::{encode_config, URL_SAFE_NO_PAD};
use regex::Regex;

fn main() {
    let input = "[18, 15, 251]";
    let base64_encoded = from_decimal_to_base64(input);

    println!("Base64 编码: {}", base64_encoded);
}

fn from_decimal_to_base64(input: &str) -> String {
    // Step 1: 从字符串中提取数字
    let re = Regex::new(r"\d+").unwrap();
    let numbers: Vec<u8> = re
        .captures_iter(input)
        .filter_map(|cap| cap[0].parse::<u8>().ok())
        .collect();

    // Step 2: 将数字转换为字节并进行 Base64 编码
    let base64_encoded = encode_config(&numbers, URL_SAFE_NO_PAD);

    base64_encoded.replace("_", "/").replace("-", "+")
}
