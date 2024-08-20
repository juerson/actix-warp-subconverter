use super::sn_wg::generate_nekobox_sn_link;
use super::yaml::read_yaml_data;
use rand::Rng;
use regex::Regex;

pub fn generate_more_sn_links(
    file_path: &str,
    ip_with_port_vec: Vec<String>,
    mtu_value: u16,
) -> String {
    let mut result: Vec<String> = Vec::new();
    let shared_key_string = "".to_string();
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
                    let reserved_vec: Vec<u8> =
                        random_item.get_reserved().clone().unwrap_or(vec![]);
                    let v4 = random_item.get_v4().clone();
                    let v6 = random_item.get_v6().clone();

                    let config_name = format!("warp-{}", ip_with_port);

                    // local_address地址
                    let mut local_address_string = String::new();
                    if !v4.contains('/') {
                        local_address_string.push_str(format!("{}/32", v4).as_str());
                    }
                    if !v6.is_empty()
                        && !v6.contains('/')
                        && v6.chars().filter(|&c| c == ':').count() > 4
                    {
                        local_address_string.push_str(format!(",{}/128", v6).as_str());
                    }

                    // 【reserved值转换为4个字符的字符串】将 reserved_vec 转成 base64 安全编码字符串
                    let reserved_base64_encoded: String = if !reserved_vec.is_empty() {
                        base64::encode_config(&reserved_vec, base64::URL_SAFE_NO_PAD)
                            .replace("_", "/")
                            .replace("-", "+")
                    } else {
                        "".to_string()
                    };

                    let sn_link = generate_nekobox_sn_link(
                        ip.to_string(),
                        port.parse::<u32>().unwrap(),
                        config_name,
                        local_address_string,
                        private_key_str,
                        public_key_str,
                        shared_key_string.clone(),
                        mtu_value as u32,
                        reserved_base64_encoded,
                    );

                    result.push(sn_link);
                }
            }
        }
        Err(err) => eprintln!("Failed to read YAML data: {}", err),
    }
    result.join("\n")
}
