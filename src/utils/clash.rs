use super::yaml::read_yaml_data;
use rand::Rng;
use regex::Regex;
use serde_json::{json, Number, Value};
use std::{collections::BTreeMap, fs};

pub fn generate_clash_config(files: Vec<&str>, ip_with_port_vec: Vec<String>, mtu: u16) -> String {
    let clash_template = fs::read_to_string(files[1]).expect("读取clash配置模板文件失败");

    let proxies_name_vec: Vec<String> = ip_with_port_vec
        .iter()
        .map(|address| {
            let proxy_name = if address.chars().filter(|&c| c == ':').count() > 4 {
                format!("      - {}", address.replace("[", "【").replace("]", "】"))
            } else {
                format!("      - warp-{}", address)
            };
            proxy_name.to_string()
        })
        .collect();

    let mut json_nodes = Vec::new();

    // 匹配 IP 地址和端口
    let re = Regex::new(r#"\[?([^\]]+)\]?:([^:]+)"#).unwrap();
    match read_yaml_data(files[0]) {
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
                    let proxy_name = if ip.chars().filter(|&c| c == ':').count() > 4 {
                        format!("【{}】:{}", ip, port)
                    } else {
                        format!("warp-{}", ip_with_port)
                    };
                    wireguard_map.insert("name".to_string(), json!(proxy_name));
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

    // 替换模板中的占位符
    let clash_config = clash_template
        .replace("[]", format!("\n{}", proxies_json_data).as_str())
        .replace(r"      - node_name", proxies_name_string.as_str());

    clash_config
}
