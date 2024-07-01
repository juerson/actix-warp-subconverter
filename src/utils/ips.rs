use ipnetwork::IpNetwork;
use rand::{prelude::SliceRandom, rngs::StdRng, Rng, SeedableRng};
use std::{
    collections::HashSet,
    net::{IpAddr, Ipv6Addr},
};

/**
 * 在给定的 CIDR 范围内随机生成指定数量的 IPv6 地址。
 * 仅随机生成 IPv6 地址的后四段、 而前四段则来自CIDR网络范围的前四段。
 *
 * @param cidr - IPv6 CIDR 网络范围。
 * @param count - 要生成的 IPv6 地址的数量。
 * @returns - 生成的 IPv6 地址列表。
 */
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

fn generate_ips(cidr: &str) -> Vec<String> {
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

pub fn generate_random_ip_in_cidrs(cidrs: Vec<&str>, count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut generated_ips = HashSet::new();

    // Generate all possible IP addresses from CIDRs
    let all_ips: Vec<String> = cidrs.iter().flat_map(|&cidr| generate_ips(&cidr)).collect();
    // Generate 300 unique random IP addresses
    while generated_ips.len() < count && generated_ips.len() < all_ips.len() {
        if let Some(ip) = all_ips.choose(&mut rng) {
            generated_ips.insert(ip.clone());
        }
    }
    generated_ips.into_iter().collect()
}
