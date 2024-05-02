use crate::utils::yaml::read_yaml_data;
use rand::Rng;
use serde_json::{json, Number, Value};
use regex::Regex;

pub fn generate_nekoray_nodes(ip_with_port_vec: Vec<String>, mtu_value: u16) -> String {
    let mut result: Vec<String> = Vec::new();
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
                    let private_key_str = random_item.get_private_key().clone();
                    let public_key = random_item.get_public_key().clone();
                    let reserved_vec = random_item.get_reserved().clone().unwrap_or(vec![]);
                    let v4 = random_item.get_v4().clone();
                    let v6 = random_item.get_v6().clone();

                    let nekoray_name = format!("warp-{}", ip_with_port);

                    /* 处理nekoray节点中cs键名对应的value值 */
                    // local_address地址
                    let mut local_address_vec: Vec<String> = Vec::new();
                    if !v4.contains("/") {
                        local_address_vec.push(format!("{}/32", v4));
                    }
                    if !v6.is_empty() && !v6.contains("/") {
                        let count = v6.chars().filter(|&c| c == ':').count();
                        if count > 4 {
                            local_address_vec.push(format!("{}/128", v6));
                        }
                    }
                    let cs_json_str = r#"{
                        "type": "wireguard",
                        "tag": "proxy",
                        "server": "162.159.192.1",
                        "server_port": 2408,
                        "system_interface": false,
                        "interface_name": "warp",
                        "local_address": [],
                        "private_key": "",
                        "peer_public_key": "",
                        "pre_shared_key": "",
                        "reserved": [],
                        "mtu": 1280
                    }"#;
                    let mut cs_json_value: Value = serde_json::from_str(cs_json_str).unwrap();

                    /* 修改cs特定键的值 */
                    if let Some(private_key) = cs_json_value.get_mut("private_key") {
                        *private_key = json!(private_key_str);
                    }
                    if let Some(peer_public_key) = cs_json_value.get_mut("peer_public_key") {
                        *peer_public_key = json!(public_key);
                    }
                    if let Some(server) = cs_json_value.get_mut("server") {
                        *server = json!(ip);
                    }
                    if let Some(server_port) = cs_json_value.get_mut("server_port") {
                        *server_port = json!(port.parse::<Number>().unwrap_or(2408.into()));
                    }
                    if let Some(local_address) = cs_json_value.get_mut("local_address") {
                        *local_address = json!(local_address_vec);
                    }
                    if let Some(reserved) = cs_json_value.get_mut("reserved") {
                        *reserved = json!(reserved_vec);
                    }
                    if let Some(mtu) = cs_json_value.get_mut("mtu") {
                        *mtu = json!(mtu_value);
                    }
                    // 将修改后的 Value 对象转换为格式化的 JSON 字符串
                    let modified_cs_json_string =
                        serde_json::to_string_pretty(&cs_json_value).unwrap();

                    // 构造 nekoray 配置
                    let nekoray_json_str = r#"{
                        "_v": 0,
                        "addr": "127.0.0.1",
                        "cmd": [""],
                        "core": "internal",
                        "cs": "",
                        "mapping_port": 0,
                        "name": "",
                        "port": 1080,
                        "socks_port": 0
                    }"#;
                    let mut nekoray_json_value: Value =
                        serde_json::from_str(nekoray_json_str).unwrap();

                    /* 修改nekoray_json_str特定键的值 */
                    if let Some(cs) = nekoray_json_value.get_mut("cs") {
                        *cs = json!(modified_cs_json_string);
                    }
                    if let Some(name) = nekoray_json_value.get_mut("name") {
                        *name = json!(nekoray_name);
                    }
                    let node: String = serde_json::to_string(&nekoray_json_value).unwrap();
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
