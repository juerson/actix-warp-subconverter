use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};
use regex::Regex;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

pub fn read_ip_with_port_from_files(folder_path: &str) -> io::Result<Vec<String>> {
    // 读取指定文件夹下的所有文件
    let paths = fs::read_dir(folder_path)?;
    // 定义用于匹配 IP 地址和端口的正则表达式
    let re = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}:[0-9]{1,5}\b|\[([0-9a-fA-F]{0,4}:){2,7}[0-9a-fA-F]{0,4}\]:[0-9]{1,5}").unwrap();
    // 存储匹配到的 IP 地址和端口的向量
    let mut ip_with_port_vec: Vec<String> = Vec::new();

    // 遍历文件夹中的每个文件
    for path in paths {
        let file_path = path?.path();
        let file_extension = file_path.extension().unwrap_or_default();
        // 获取文件名
        let file_name = file_path.file_name().unwrap().to_string_lossy();
        // 排除以 "ips-v" 开头的文件
        if file_name.starts_with("ips-v") {
            continue;
        }
        // 如果文件扩展名是 txt 或 csv，则读取文件内容并匹配IP:PORT
        if let Some(ext) = file_extension.to_str() {
            if ext == "txt" || ext == "csv" {
                if let Ok(bytes) = fs::read(&file_path) {
                    if let Ok(content) = std::str::from_utf8(&bytes) {
                        for cap in re.captures_iter(&content) {
                            if let Some(ip_port) = cap.get(0) {
                                ip_with_port_vec.push(ip_port.as_str().to_string());
                            }
                        }
                    } else {
                        //使用GBK编码读取csv文件，并使用正则匹配IP:PORT
                        let file = File::open(file_path).expect("File not found");
                        let reader = BufReader::new(file);
                        for line in reader.split(b'\n').map(|l| l.unwrap()) {
                            let decoded_string = GBK.decode(&line, DecoderTrap::Strict).unwrap();
                            for cap in re.captures_iter(&decoded_string) {
                                if let Some(ip_port) = cap.get(0) {
                                    ip_with_port_vec.push(ip_port.as_str().to_string());
                                }
                            }
                        }
                    }
                } else {
                    println!("Failed to read file: {:?}", file_path);
                }
            }
        }
    }
    // 使用 HashSet 去重
    let mut set: HashSet<String> = HashSet::new();
    let mut unique_ip_with_port_vec: Vec<String> = Vec::new();

    for ip_with_port in ip_with_port_vec {
        if set.insert(ip_with_port.clone()) {
            // 如果成功插入，说明是第一次出现，将其添加到新的 Vec 中
            unique_ip_with_port_vec.push(ip_with_port);
        }
    }
    Ok(unique_ip_with_port_vec)
}
