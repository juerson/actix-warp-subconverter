use super::data::read_ip_with_port_from_files; // 相对路径引用
use ipnetwork::IpNetwork;
use rand::{prelude::SliceRandom, rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::{
    collections::HashSet,
    net::{IpAddr, Ipv6Addr},
};

pub fn selected_ip_with_port<'a>(
    ip_count: &'a usize,
    port_count: &'a usize,
    node_count: &'a usize,
    loc: &'a String,
    file_data: &'a bool,
    ip_type: &'a u8,
) -> Vec<String> {
    if *file_data {
        let dir_path = "data"; // 自定义ip:port数据文件存在的文件夹里面
        let ip_with_port_vec = if let Ok(ip_with_port_vec) = read_ip_with_port_from_files(&dir_path)
        {
            if ip_with_port_vec.is_empty() {
                generate_ip_with_port_vec(loc, ip_count, port_count, ip_type) /* 生成的数据，防止读取到空数据 */
            } else {
                ip_with_port_vec /* 读取的文件数据 */
            }
        } else {
            generate_ip_with_port_vec(loc, ip_count, port_count, ip_type) /* 生成的数据 */
        };
        // 使用切边的方法：获取向量的前面node_count个元素（不足node_count个元素就获取全部元素），这个切片操作更为直接
        return ip_with_port_vec[..std::cmp::min(ip_with_port_vec.len(), *node_count)].to_vec();
    } else {
        /* 下面是生成的数据的相关代码 */
        let ip_with_port_vec = generate_ip_with_port_vec(loc, ip_count, port_count, ip_type);
        // 打乱地址向量并选择前node_count个元素
        let mut rng = thread_rng();
        let mut shuffled_addresses = ip_with_port_vec.clone(); // 克隆一份地址向量以免改变原始向量
        shuffled_addresses.shuffle(&mut rng);

        // 获取向量的前面node_count个元素（不足node_count个元素就获取全部元素）
        return shuffled_addresses
            .clone()
            .iter()
            .take(*node_count)
            .cloned()
            .collect();
    }
}

fn generate_ip_with_port_vec(
    loc: &String,
    ip_count: &usize,
    port_count: &usize,
    ip_type: &u8,
) -> Vec<String> {
    let cidrs: Vec<&str> = match ip_type {
        4 => match loc.as_str() {
            "us" => vec!["162.159.192.0/24", "162.159.193.0/24", "162.159.195.0/24"],
            "gb" => vec![
                "188.114.96.0/24",
                "188.114.97.0/24",
                "188.114.98.0/24",
                "188.114.99.0/24",
            ],
            _ => vec![
                "162.159.192.0/24",
                "162.159.193.0/24",
                "162.159.195.0/24",
                "188.114.96.0/24",
                "188.114.97.0/24",
                "188.114.98.0/24",
                "188.114.99.0/24",
            ],
        },
        6 => vec!["2606:4700:d0::/48", "2606:4700:d1::/48"],
        _ => vec![
            "162.159.192.0/24",
            "162.159.193.0/24",
            "162.159.195.0/24",
            "188.114.96.0/24",
            "188.114.97.0/24",
            "188.114.98.0/24",
            "188.114.99.0/24",
            "2606:4700:d0::/48",
            "2606:4700:d1::/48",
        ],
    };

    let ips = generate_random_ip_in_cidrs(cidrs, *ip_count);

    let mut ports: Vec<u16> = vec![
        854, 859, 864, 878, 880, 890, 891, 894, 903, 908, 928, 934, 939, 942, 943, 945, 946, 955,
        968, 987, 988, 1002, 1010, 1014, 1018, 1070, 1074, 1180, 1387, 1843, 2371, 2506, 3138,
        3476, 3581, 3854, 4177, 4198, 4233, 5279, 5956, 7103, 7152, 7156, 7281, 7559, 8319, 8742,
        8854, 8886, 2408, 500, 4500, 1701,
    ];

    let mut rng = thread_rng();
    ports.shuffle(&mut rng);
    let selected_ports: Vec<u16> = ports.iter().take(*port_count).cloned().collect();

    // 组合成网络地址
    let addresses: Vec<String> = ips
        .iter()
        .flat_map(|ip| {
            selected_ports.iter().map(move |port| {
                if ip.contains(":") {
                    format!("[{}]:{}", ip, port)
                } else {
                    format!("{}:{}", ip, port)
                }
            })
        })
        .collect();
    addresses
}

fn generate_random_ip_in_cidrs(cidrs: Vec<&str>, count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut generated_ips = HashSet::new();

    let all_ips: Vec<String> = cidrs
        .iter()
        .flat_map(|&cidr| generate_ips_from_cidr(&cidr))
        .collect();

    while generated_ips.len() < count && generated_ips.len() < all_ips.len() {
        if let Some(ip) = all_ips.choose(&mut rng) {
            generated_ips.insert(ip.clone());
        }
    }
    generated_ips.into_iter().collect()
}

fn generate_ips_from_cidr(cidr: &str) -> Vec<String> {
    let mut ips = HashSet::new();

    match cidr.parse::<IpNetwork>() {
        Ok(cidr) => match cidr {
            IpNetwork::V4(_) => {
                for ip in cidr.iter() {
                    ips.insert(ip.to_string());
                }
            }
            IpNetwork::V6(_) => {
                let ipaddress_vec = generate_random_ipv6_addresses(cidr, 1000);
                ips.extend(ipaddress_vec.iter().map(|ip| ip.to_string()));
            }
        },
        Err(e) => eprintln!("CIDR 解析错误: {}", e),
    }

    ips.into_iter().collect()
}

fn generate_random_ipv6_addresses(cidr: IpNetwork, count: usize) -> Vec<String> {
    let mut rng = StdRng::from_entropy();
    let mut ips = HashSet::new();
    for _ in 0..count {
        if let IpAddr::V6(addr) = cidr.network() {
            // 将前四段保持不变
            let prefix_segments = &addr.segments()[..4];

            // 生成后四段的随机部分
            let random_segments: Vec<u16> = (0..4).map(|_| rng.gen()).collect();

            // 组合前四段和后四段
            let mut full_segments = [0u16; 8];
            full_segments[..4].copy_from_slice(prefix_segments);
            full_segments[4..].copy_from_slice(&random_segments);

            // 生成最终的IPv6地址
            let generated_addr = Ipv6Addr::from(full_segments);
            ips.insert(generated_addr.to_string());
        }
    }
    ips.into_iter().collect()
}
