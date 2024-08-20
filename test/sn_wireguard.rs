use flate2::{write::ZlibEncoder, Compression};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Write};

fn encode_sn_str(s: &str) -> Vec<u8> {
    if s.is_empty() {
        return vec![0x81];
    }
    if s.len() == 1 {
        let mut bytes = vec![0x82];
        bytes.extend(s.as_bytes());
        return bytes;
    }
    let mut bytes = s.as_bytes().to_vec();
    let last = bytes.pop().unwrap() | 0x80;
    bytes.push(last);
    bytes
}

fn p32(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

#[allow(dead_code)]
fn p8(n: u8) -> [u8; 1] {
    [n]
}

#[derive(Serialize, Deserialize, Debug)]
struct SnServer {
    server_address: String,
    server_port: u32,
}

impl Default for SnServer {
    fn default() -> Self {
        SnServer {
            server_address: "162.159.192.1".to_string(),
            server_port: 2408,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WireguardConfig {
    version: u32,
    server: SnServer,
    local_address: String,
    private_key: String,
    peer_public_key: String,
    peer_pre_shared_key: String,
    mtu: u32,
    reserved: String,
}

impl Default for WireguardConfig {
    fn default() -> Self {
        WireguardConfig {
            version: 2,
            server: SnServer::default(),
            local_address: "172.16.0.2/32".to_string(),
            private_key: "".to_string(),
            peer_public_key: "bmXOC+F1FxEMF9dyiK2H5/1SUtzH0JuVo51h2wPfgyo=".to_string(),
            peer_pre_shared_key: "".to_string(),
            mtu: 1420,
            reserved: "".to_string(),
        }
    }
}

impl WireguardConfig {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(p32(self.version));
        bytes.extend(encode_sn_str(&self.server.server_address));
        bytes.extend(p32(self.server.server_port));
        bytes.extend(encode_sn_str(&self.local_address));
        bytes.extend(encode_sn_str(&self.private_key));
        bytes.extend(encode_sn_str(&self.peer_public_key));
        bytes.extend(encode_sn_str(&self.peer_pre_shared_key));
        bytes.extend(p32(self.mtu));
        bytes.extend(encode_sn_str(&self.reserved));
        bytes
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SnMeta {
    extra_version: u32,
    name: String,
    custom_outbound_json: String,
    custom_config_json: String,
}

impl Default for SnMeta {
    fn default() -> Self {
        SnMeta {
            extra_version: 1,
            name: "".to_string(),
            custom_outbound_json: "".to_string(),
            custom_config_json: "".to_string(),
        }
    }
}

impl SnMeta {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(p32(self.extra_version));
        bytes.extend(encode_sn_str(&self.name));
        bytes.extend(encode_sn_str(&self.custom_outbound_json));
        bytes.extend(encode_sn_str(&self.custom_config_json));
        bytes
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Wireguard {
    wireguard_config: WireguardConfig,
    sn_meta: SnMeta,
}

impl Default for Wireguard {
    fn default() -> Self {
        Wireguard {
            wireguard_config: WireguardConfig::default(),
            sn_meta: SnMeta::default(),
        }
    }
}

impl Wireguard {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.wireguard_config.serialize());
        bytes.extend(self.sn_meta.serialize());
        bytes
    }

    fn to_string(&self) -> String {
        let mut compressed_data = Vec::new();
        let mut encoder = ZlibEncoder::new(Cursor::new(compressed_data), Compression::default());
        encoder
            .write_all(&self.serialize())
            .expect("Failed to compress data");
        compressed_data = encoder
            .finish()
            .expect("Failed to finish compression")
            .into_inner();

        let encoded_data = base64::encode_config(&compressed_data, base64::URL_SAFE_NO_PAD);
        format!("sn://wg?{}", encoded_data)
    }
}

fn main() {
    let address = "162.159.192.11";
    let port = 854;
    let local_address_value =
        "172.16.0.2/32,2606:4700:110:88f9:54b8:120e:d01d:c77e/128".to_string();
    let private_key_value = "GKaNRx+KVRL3F1sguZHO8wh70yUprNsPjmUapCGUsGk=".to_string();
    let public_key_value = "bmXOC+F1FxEMF9dyiK2H5/1SUtzH0JuVo51h2wPfgyo=".to_string();
    let mtu_value: u32 = 1280;
    let reserved_value = "eWZI".to_string();

    // ————————————————————————————————————————————————————————————————————————————————————————————————————————————————

    let server = SnServer {
        server_address: address.to_string(),
        server_port: port,
    };

    let config = WireguardConfig {
        version: 2,
        server,
        local_address: local_address_value,
        private_key: private_key_value,
        peer_public_key: public_key_value,
        peer_pre_shared_key: "".to_string(),
        mtu: mtu_value,
        reserved: reserved_value, // https://gchq.github.io/CyberChef/#recipe=From_Decimal('Comma',false)To_Base64('A-Za-z0-9%2B/%3D')&input=MTIxLDEwMiw3Mg
    };

    let meta = SnMeta {
        extra_version: 1,
        name: format!("warp-{}:{}", address, port),
        custom_outbound_json: "".to_string(),
        custom_config_json: "".to_string(),
    };

    let wireguard = Wireguard {
        wireguard_config: config,
        sn_meta: meta,
    };

    // ————————————————————————————————————————————————————————————————————————————————————————————————————————————————

    println!("{}", wireguard.to_string());
}
