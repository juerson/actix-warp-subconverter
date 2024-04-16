use ipnetwork::IpNetwork;
use rand::{prelude::SliceRandom, rngs::StdRng, Rng, SeedableRng};
use std::{
    collections::HashSet,
    net::{IpAddr, Ipv6Addr},
};

fn generate_random_ipv6_addresses(cidr: IpNetwork, count: usize) -> Vec<String> {
    let mut rng = StdRng::from_entropy();
    let mut ips = HashSet::new();
    for _ in 0..count {
        if let IpAddr::V6(addr) = cidr.network() {
            let random: u128 = rng.gen();
            let mask = !((1u128 << (128 - cidr.prefix())) - 1);
            let generated_addr =
                Ipv6Addr::from((u128::from_be_bytes(addr.octets()) & mask) | (random & !mask));
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
