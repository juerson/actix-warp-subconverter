use super::yaml::read_yaml_data; // 相对路径引用
use rand::Rng;

pub fn generate_wireguard_nodes(
    file_path: &str,
    ip_with_port_vec: Vec<String>,
    mtu_value: u16,
) -> String {
    let mut result: Vec<String> = Vec::new();
    match read_yaml_data(file_path) {
        Ok(items) => {
            for ip_with_port in ip_with_port_vec {
                /* 随机选择一个 warp_parameters 元素（warp账号信息） */
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..items.get_warp_parameters().len());
                let random_item = &items.get_warp_parameters()[random_index];
                let private_key = random_item.get_private_key().clone();
                let public_key = random_item.get_public_key().clone();
                let reserved_vec = random_item.get_reserved().clone().unwrap_or(vec![]);
                let reserved_string = reserved_vec
                    .iter()
                    .map(|&byte| byte.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                let v4 = random_item.get_v4().clone();
                let v6 = random_item.get_v6().clone();

                let mut local_address_string = String::new();
                if v4.contains("/") {
                    local_address_string.push_str(&v4);
                } else {
                    local_address_string.push_str(&v4);
                    local_address_string.push_str("/32");
                }
                if !v6.is_empty() {
                    local_address_string.push(',');
                    if v6.contains("/") {
                        local_address_string.push_str(&v6);
                    } else {
                        local_address_string.push_str(&v6);
                        local_address_string.push_str("/128");
                    }
                }
                // 对字符串进行url编码
                let remarks = format!("warp-{ip_with_port}");
                let encoded_remarks = urlencoding::encode(&remarks);
                let encoded_privatekey = urlencoding::encode(&private_key);
                let encoded_publickey = urlencoding::encode(&public_key);
                let encoded_local_address = urlencoding::encode(&local_address_string);

                let wireguardlinks;
                if reserved_string.is_empty() {
                    wireguardlinks = format!("wireguard://{encoded_privatekey}@{ip_with_port}/?publickey={encoded_publickey}&address={encoded_local_address}&mtu={mtu_value}#{encoded_remarks}");
                } else {
                    let encodeed_reserved = urlencoding::encode(&reserved_string); // 对reserved_string进行url编码
                    wireguardlinks = format!("wireguard://{encoded_privatekey}@{ip_with_port}/?publickey={encoded_publickey}&reserved={encodeed_reserved}&address={encoded_local_address}&mtu={mtu_value}#{encoded_remarks}");
                }

                result.push(wireguardlinks);
            }
        }
        Err(e) => {
            println!("Error reading YAML file: {}", e);
        }
    }
    result.join("\n")
}
