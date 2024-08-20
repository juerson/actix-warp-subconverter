mod utils;

use crate::utils::{
    clash::generate_clash_config, hiddify::generate_hiddify_config, ips::selected_ip_with_port,
    nekoray::generate_nekoray_nodes, singbox::generate_singbox_nodes,
    sn_wgs::generate_more_sn_links, wiregurad::generate_wireguard_nodes,
}; // 绝对路径引用
use actix_web::{get, HttpRequest, HttpResponse, Responder};

const SPECIFICATION: &str = include_str!("../使用说明.txt");

async fn default_route() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    // 从HttpRequest中获取请求的域名地址/主机地址
    let host_address = req.connection_info().host().to_owned();

    let html_doc = SPECIFICATION.replace("127.0.0.1:18081", &host_address);

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(html_doc)
}

#[get("/sub")]
async fn subconverter(req: HttpRequest) -> impl Responder {
    // 获取查询字符串
    let query_string = req.query_string();

    let params: Vec<(String, String)> =
        serde_urlencoded::from_str(&query_string).expect("Failed to parse query string");

    // 查询参数
    let mut target = "".to_string();
    let mut ip_count = 1000;
    let mut port_count = 10;
    let mut node_count: usize = 300;
    let mut mtu_value: u16 = 1280;
    let mut ip_type: u8 = 4;
    let mut loc: String = "".to_string(); // us/gb
    let mut file_data: bool = false; // 是否使用data文件夹下的txt、csv文件
    let mut detour = false;
    let mut fake_packets: String = "5-10".to_string(); // 用于修改hiddify的json数据
    let mut fake_packets_size: String = "40-100".to_string(); // 用于修改hiddify的json数据
    let mut fake_packets_delay: String = "".to_string(); // 用于修改hiddify的json数据

    let files = vec![
        "config.yaml",
        "template/Clash.yaml",
        "template/Hiddify.json",
    ];

    for (key, value) in params {
        if key.to_lowercase() == "target" {
            target = value.to_string();
        } else if key.to_lowercase() == "ip_count" || key.to_lowercase() == "ipcount" {
            ip_count = value.parse().unwrap_or(1000);
        } else if key.to_lowercase() == "port_count" || key.to_lowercase() == "portcount" {
            let port_number: usize = value.parse().unwrap_or(10);
            match port_number {
                1..54 => port_count = port_number,
                _ => port_count = 10,
            }
        } else if key.to_lowercase() == "node_count" || key.to_lowercase() == "nodecount" {
            node_count = value.parse().unwrap_or(300);
        } else if key.to_lowercase() == "mtu" {
            mtu_value = value.parse().unwrap_or(1280);
        } else if key.to_lowercase() == "detour" {
            if ["on", "1", "true"].contains(&value.to_lowercase().as_str()) {
                detour = true;
            }
        } else if key.to_lowercase() == "loc" || key.to_lowercase() == "location" {
            loc = value.to_string();
        } else if key.to_lowercase() == "fake_packets" {
            fake_packets = value.to_string();
        } else if key.to_lowercase() == "fake_packets_size" {
            fake_packets_size = value.to_string();
        } else if key.to_lowercase() == "fake_packets_delay" {
            fake_packets_delay = value.to_string();
        } else if key.to_lowercase() == "filedata" {
            if ["on", "1", "true"].contains(&value.to_lowercase().as_str()) {
                file_data = true;
            }
        } else if key.to_lowercase() == "iptype" {
            if [4, 6].contains(&value.parse().unwrap_or(4)) {
                ip_type = value.parse().unwrap_or(4);
            }
        } else {
            //
        }
    }
    let selected_ip_with_port_vec = selected_ip_with_port(
        &ip_count,
        &port_count,
        &node_count,
        &loc,
        &file_data,
        &ip_type,
    );

    // 输出订阅内容
    if target.to_lowercase() == "clash" {
        let node_count_take = if node_count == 300 { 200 } else { node_count };
        let ip_with_port_data: Vec<String> = selected_ip_with_port_vec
            .iter()
            .take(node_count_take)
            .cloned()
            .collect();
        let clash_config = generate_clash_config(
            vec![files[0], files[1]], // warp的参数配置文件，clash模板文件
            ip_with_port_data,
            mtu_value,
        );
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(clash_config)
    } else if target.to_lowercase() == "nekoray" || target.to_lowercase() == "nekobox" {
        let nekoray_links = generate_nekoray_nodes(files[0], selected_ip_with_port_vec, mtu_value);
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(nekoray_links)
    } else if target.to_lowercase() == "wireguard"
        || target.to_lowercase() == "v2rayn"
        || target.to_lowercase() == "v2ray"
    {
        let wiregurad_links =
            generate_wireguard_nodes(files[0], selected_ip_with_port_vec, mtu_value);
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(wiregurad_links)
    } else if target.to_lowercase() == "hiddify" {
        // 控制hiddify的节点数量
        let node_count_take = if node_count == 300 && detour == true {
            50
        } else if node_count != 300 {
            node_count
        } else {
            200
        };
        let ip_with_port_data: Vec<String> = selected_ip_with_port_vec
            .iter()
            .take(node_count_take)
            .cloned()
            .collect();
        let hiddify_config = generate_hiddify_config(
            vec![files[0], files[2]], // warp的参数配置文件，hiddify模板文件
            ip_with_port_data,
            mtu_value,
            detour, // 是否构建链式代理
            fake_packets,
            fake_packets_size,
            fake_packets_delay,
        );
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(hiddify_config)
    } else if target.to_lowercase() == "singbox" {
        // 控制sing-box的节点数量
        let node_count_take = if node_count == 300 { 100 } else { node_count };
        // 另一种写法(切片)
        let ip_with_port_data: Vec<String> = selected_ip_with_port_vec
            [..std::cmp::min(selected_ip_with_port_vec.len(), node_count_take)]
            .to_vec();
        let singbox_config = generate_singbox_nodes(files[0], ip_with_port_data, mtu_value);
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(singbox_config)
    } else if target.to_lowercase() == "sn" {
        let sn_links = generate_more_sn_links(files[0], selected_ip_with_port_vec, mtu_value);
        HttpResponse::Ok().body(sn_links)
    } else {
        HttpResponse::Ok().body("404 Not Found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 获取本机的私有IP地址
    let local_ip = match local_ip_address::local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Failed to get local IP address: {}", e);
            return Ok(());
        }
    };
    // 绑定的端口
    let port = 18081;
    println!(
        "使用方法，打开: http://{}:{} 或 http://127.0.0.1:{}",
        local_ip.to_string(),
        port,
        port
    );
    // 启动 HTTP 服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(index)
            .service(subconverter)
            .default_service(actix_web::web::route().to(default_route))
    })
    .bind(format!("0.0.0.0:{}", port))? // 监听所有 IPv4 地址
    .run()
    .await
}
