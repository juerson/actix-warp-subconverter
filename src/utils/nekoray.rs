use super::yaml::read_yaml_data; // 相对路径引用
use rand::Rng;
use regex::Regex;
use serde_json::{json, Number};

pub fn generate_nekoray_nodes(
    file_path: &str,
    ip_with_port_vec: Vec<String>,
    mtu_value: u16,
) -> String {
    let mut result: Vec<String> = Vec::new();
    // 匹配 IP 地址和端口
    let re = Regex::new(r#"\[?([^\]]+)\]?:([^:]+)"#).unwrap();
    match read_yaml_data(file_path) {
        Ok(items) => {
            for ip_with_port in ip_with_port_vec {
                if let Some(captures) = re.captures(&ip_with_port) {
                    let ip: &str = captures.get(1).map_or("", |m| m.as_str());
                    let port = captures.get(2).map_or("", |m| m.as_str());

                    /* 随机选择一个 warp_parameters 元素（warp账号信息） */
                    let mut rng = rand::thread_rng();
                    let random_index = rng.gen_range(0..items.get_warp_parameters().len());
                    let random_item = &items.get_warp_parameters()[random_index];
                    let private_key_str = random_item.get_private_key().clone();
                    let public_key_str = random_item.get_public_key().clone();
                    let reserved_vec = random_item.get_reserved().clone().unwrap_or(vec![]);
                    let v4 = random_item.get_v4().clone();
                    let v6 = random_item.get_v6().clone();

                    let nekoray_name = format!("warp-{}", ip_with_port);

                    // local_address地址
                    let mut local_address_vec: Vec<String> = Vec::new();
                    if !v4.contains('/') {
                        local_address_vec.push(format!("{}/32", v4));
                    }
                    if !v6.is_empty()
                        && !v6.contains('/')
                        && v6.chars().filter(|&c| c == ':').count() > 4
                    {
                        local_address_vec.push(format!("{}/128", v6));
                    }

                    let cs_value = json!({
                        "type": "wireguard",
                        "tag": nekoray_name,
                        "server": ip,
                        "server_port": port.parse::<Number>().unwrap_or(2408.into()),
                        "system_interface": false,
                        "interface_name": "warp",
                        "local_address": local_address_vec,
                        "private_key": private_key_str,
                        "peer_public_key": public_key_str,
                        "reserved": reserved_vec,
                        "mtu": mtu_value
                    });

                    let cs_string = serde_json::to_string_pretty(&cs_value).unwrap();

                    let nekoray_config_value = json!({
                        "_v": 0,
                        "addr": "127.0.0.1",
                        "cmd": [""],
                        "core": "internal",
                        "cs": cs_string,
                        "mapping_port": 0,
                        "name": nekoray_name,
                        "port": 1080,
                        "socks_port": 0
                    });
                    let node: String = serde_json::to_string(&nekoray_config_value).unwrap();

                    /* base64编码、拼接协议 */
                    let encoded = base64::encode(node);
                    let transport_protocol = "nekoray://custom#";
                    let nekoray_node = format!("{}{}", transport_protocol, encoded);

                    result.push(nekoray_node);
                }
            }
        }
        Err(err) => eprintln!("Failed to read YAML data: {}", err),
    }
    result.join("\n")
}
