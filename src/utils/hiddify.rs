use crate::utils::yaml::read_yaml_data;
use rand::Rng;
use serde_json::{from_str, json, to_string_pretty};
use std::fs::File;
use std::io::{BufReader, Read};
use regex::Regex;

use super::yaml::WarpDate;

pub fn generate_hiddify_config(
    ip_with_port_vec: Vec<String>,
    mtu_value: u16,
    detour: bool,
    fake_packets: String,
    fake_packets_size: String,
    fake_packets_delay: String,
) -> String {
    let [yaml_file, hiddify_file] = ["config/warp.yaml", "config/hiddify.json"];
    let mut results = Vec::new();
    let mut proxy_name_vec = Vec::new();
    let selected_vec: &[String] = if detour {
        // 选择前50个元素（如果元素少于50个，将选择整个向量）
        &ip_with_port_vec[..std::cmp::min(ip_with_port_vec.len(), 50)]
    } else {
        &ip_with_port_vec
    };
    let fake_packets_delay_str = if !detour && fake_packets_delay.is_empty() {
        "15-100".to_string() // 不是detour模式，且fake_packets_delay为空，则使用默认值
    } else if detour && fake_packets_delay.is_empty(){
        "20-100".to_string() // 是detour模式，且fake_packets_delay为空，则使用默认值
    } else {
        fake_packets_delay.clone() // 传入值不为空，则使用传入值
    };
    // 定义用于匹配 IP 地址和端口的正则表达式
    let re = Regex::new(r#"\[?([^\]]+)\]?:([^:]+)"#).unwrap();
    match read_yaml_data(yaml_file) {
        Ok(items) => {
            for ip_with_port in selected_vec {
                if let Some(captures) = re.captures(&ip_with_port) {
                    let ip: &str = captures.get(1).map_or("", |m| m.as_str());
                    let port = captures.get(2).map_or("", |m| m.as_str());
                    /* 随机选择一个 warp_parameters 元素（warp账号信息） */
                    let mut rng = rand::thread_rng();
                    let random_key_index = rng.gen_range(0..items.get_warp_parameters().len());
                    let random_item = &items.get_warp_parameters()[random_key_index];
                    let (private_key, public_key, reserved_vec, v4, v6) =
                        get_wireguard_params(random_item);
                    // 节点的名称
                    let tag_name = format!("warp-{}", ip_with_port);
                    proxy_name_vec.push(tag_name.clone());
                    let node_json = json!({
                      "type": "wireguard",
                      "tag": tag_name,
                      "local_address": [v4, v6],
                      "private_key": private_key,
                      "server": ip,
                      "server_port": port.parse().unwrap_or(2408),
                      "peer_public_key": public_key,
                      "reserved": reserved_vec,
                      "mtu": mtu_value,
                      "fake_packets": fake_packets,
                      "fake_packets_size": fake_packets_size,
                      "fake_packets_delay": fake_packets_delay_str
                    });
                    results.push(node_json);

                    let flag = if ip.starts_with("162.159.192")
                        || ip.starts_with("162.159.193")
                        || ip.starts_with("162.159.195")
                    {
                        "🇺🇲"
                    } else if ip.starts_with("188.114.96")
                        || ip.starts_with("188.114.97")
                        || ip.starts_with("188.114.98")
                        || ip.starts_with("188.114.99")
                    {
                        "🇬🇧"
                    } else {
                        "❓"
                    }
                    .to_string();
                    // 链式代理
                    if detour && items.get_warp_parameters().len() > 1 {
                        /* 随机选择一个 warp_parameters 元素（warp账号信息） */
                        let mut random_item: &WarpDate;
                        loop {
                            let mut rng = rand::thread_rng();
                            let random_index = rng.gen_range(0..items.get_warp_parameters().len());
                            random_item = &items.get_warp_parameters()[random_index];
                            let random_private_key = random_item.get_private_key().clone();
                            if private_key != random_private_key {
                                break;
                            }
                        }
                        let (private_key_str, public_key, reserved_vec, v4, v6) =
                            get_wireguard_params(random_item);
                        let detour_name: String = format!("warp-{}{}", ip_with_port, flag);
                        proxy_name_vec.push(detour_name.clone());
                        let node_json = json!({
                          "type": "wireguard",
                          "tag": detour_name,
                          "detour": tag_name,
                          "local_address": [v4, v6],
                          "private_key": private_key_str,
                          "server": ip,
                          "server_port": port.parse().unwrap_or(2408),
                          "peer_public_key": public_key,
                          "reserved": reserved_vec,
                          "mtu": mtu_value,
                          "fake_packets": fake_packets,
                          "fake_packets_size": fake_packets_size,
                          "fake_packets_delay": fake_packets_delay_str
                        });
                        results.push(node_json);
                    }
                }
            }
            .into()
        }
        Err(err) => eprintln!("Error reading YAML file: {}", err),
    }
    // 读取hiddify.json文件
    let file = File::open(hiddify_file).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let mut value: serde_json::Value = from_str(&contents).unwrap();

    let mut select_group_outbounds = vec!["auto".to_string()];
    proxy_name_vec
        .iter()
        .for_each(|x| select_group_outbounds.push(x.clone()));
    value["outbounds"][0]["outbounds"] = select_group_outbounds.into();
    value["outbounds"][1]["outbounds"] = json!(proxy_name_vec.clone());

    // 合并到尾部
    // value["outbounds"].as_array_mut().unwrap().extend(results.clone());

    // 合并到指定位置serde_json::Value
    let index = 2;
    value["outbounds"]
        .as_array_mut()
        .unwrap()
        .splice(index..index, results.clone());

    // 使用to_string_pretty()方法漂亮地输出json字符串
    let json_string = to_string_pretty(&value).unwrap();

    json_string
}

fn get_wireguard_params(
    random_item: &super::yaml::WarpDate,
) -> (String, String, Vec<u8>, String, String) {
    let private_key_str = random_item.get_private_key().clone();
    let public_key = random_item.get_public_key().clone();
    let reserved_vec = random_item.get_reserved().clone().unwrap_or(vec![]);
    let v4 = format!("{}/32", random_item.get_v4().clone());
    let v6 = format!("{}/128", random_item.get_v6().clone());
    (private_key_str, public_key, reserved_vec, v4, v6)
}
