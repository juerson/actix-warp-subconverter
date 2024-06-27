use crate::utils::config::BASIC_INFO;
use crate::utils::config::PROXY_GROUPS1;
use crate::utils::config::PROXY_GROUPS2;
use crate::utils::config::RULES;
use crate::utils::yaml::read_yaml_data;

use rand::Rng;
// use serde::Deserialize;
use serde_json::Number;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use regex::Regex;

/** 这里注释的全部代码，已经抽离到yaml.rs中 */
/*
    // #[derive(Debug, Deserialize)]
    // struct WarpDate {
    //     private_key: String,
    //     public_key: String,
    //     reserved: Option<Vec<u8>>,
    //     v4: String,
    //     v6: String,
    // }

    // #[derive(Debug, Deserialize)]
    // struct WarpDates {
    //     warp_parameters: Vec<WarpDate>,
    // }

    // fn read_yaml_data(yaml_file: &str) -> Result<WarpDates, Box<dyn std::error::Error>> {
    //     let file = File::open(yaml_file)?;
    //     let reader = BufReader::new(file);
    //     let items: WarpDates = serde_yaml::from_reader(reader)?;
    //     Ok(items)
    // }
*/
pub fn generate_clash_config(ip_with_port_vec: Vec<String>, mtu: u16) -> String {
    // 复制一份数据并在每个元素前添加 "- "
    let proxies_name_vec: Vec<String> = ip_with_port_vec
        .iter()
        .map(|address| format!("      - warp-{}", address))
        .collect();
    // 读取warp.yaml文件并生成json数据
    let mut json_nodes = Vec::new();
    let yaml_file = "config/warp.yaml";
    // 定义用于匹配 IP 地址和端口的正则表达式
    let re = Regex::new(r#"\[?([^\]]+)\]?:([^:]+)"#).unwrap();
    match read_yaml_data(&yaml_file) {
        Ok(items) => {
            for ip_with_port in ip_with_port_vec {
                if let Some(captures) = re.captures(&ip_with_port) {
                    let ip: &str = captures.get(1).map_or("", |m| m.as_str());
                    let port = captures.get(2).map_or("", |m| m.as_str());
                    /* 随机选择一个 warp_parameters 元素（warp账号信息） */
                    let mut rng = rand::thread_rng();
                    let random_index = rng.gen_range(0..items.get_warp_parameters().len());
                    let random_item = &items.get_warp_parameters()[random_index];
                    let private_key = random_item.get_private_key().clone();
                    let public_key = random_item.get_public_key().clone();
                    let reserved = random_item.get_reserved().clone().unwrap_or(vec![]);
                    let v4 = random_item.get_v4().clone();
                    let v6 = random_item.get_v6().clone();
                    /* 将数据写入json中 */
                    let mut wireguard_map = BTreeMap::new();
                    wireguard_map
                        .insert("name".to_string(), json!(format!("warp-{}", ip_with_port)));
                    wireguard_map.insert("type".to_string(), json!("wireguard"));
                    wireguard_map.insert("private-key".to_string(), json!(private_key));
                    wireguard_map.insert("server".to_string(), json!(ip));
                    wireguard_map.insert(
                        "port".to_string(),
                        json!(port.parse::<Number>().unwrap_or(2408.into())),
                    );
                    wireguard_map.insert("ip".to_string(), json!(v4));
                    if !v6.is_empty() {
                        wireguard_map.insert("ipv6".to_string(), json!(v6));
                    }
                    wireguard_map.insert("public-key".to_string(), json!(public_key));
                    if !reserved.is_empty() {
                        wireguard_map.insert("reserved".to_string(), json!(reserved));
                    }
                    wireguard_map.insert("mtu".to_string(), json!(mtu)); // 1280
                    wireguard_map.insert("udp".to_string(), json!(true));
                    let json_value: Value = serde_json::to_value(wireguard_map).unwrap();
                    let json_str = serde_json::to_string(&json_value).unwrap();
                    let node_string = format!("  - {}", json_str);
                    json_nodes.push(node_string);
                }
            }
        }
        Err(err) => eprintln!("Failed to read YAML data: {}", err),
    }
    let proxies_json_data = json_nodes.join("\n");
    let proxies_name_string = proxies_name_vec.join("\n");

    let mut clash_proxy_groups = Vec::new();
    for group in PROXY_GROUPS1 {
        clash_proxy_groups.push(format!("{}{}", group, proxies_name_string));
    }
    for group in PROXY_GROUPS2 {
        clash_proxy_groups.push(group.to_string());
    }
    let proxy_groups = clash_proxy_groups.join("\n");
    // 构建clash配置文件信息
    let clash_config_data = format!(
        "{}proxies:\n{}\nproxy-groups:\n{}\n{}",
        BASIC_INFO, proxies_json_data, proxy_groups, RULES
    );
    clash_config_data
}
