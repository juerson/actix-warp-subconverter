mod utils;

use crate::utils::clash::generate_clash_config;
use crate::utils::ips::generate_random_ip_in_cidrs;
use crate::utils::nekoray::generate_nekoray_nodes;
use crate::utils::wiregurad::generate_wireguard_nodes;

use actix_web::{get, HttpRequest, HttpResponse, Responder};
use rand::{seq::SliceRandom, thread_rng};

// 定义通配符路由处理函数
async fn default_route() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}

fn selected_ip_with_port<'a>(
    ip_count: &'a usize,
    port_count: &'a usize,
    node_count: &'a usize,
) -> Vec<String> {
    let cidrs = vec![
        "162.159.192.0/24",
        "162.159.193.0/24",
        "162.159.195.0/24",
        "188.114.96.0/24",
        "188.114.97.0/24",
        "188.114.98.0/24",
        "188.114.99.0/24",
    ];
    let ips = generate_random_ip_in_cidrs(cidrs, *ip_count);

    let mut ports: Vec<u16> = vec![
        854, 859, 864, 878, 880, 890, 891, 894, 903, 908, 928, 934, 939, 942, 943, 945, 946, 955,
        968, 987, 988, 1002, 1010, 1014, 1018, 1070, 1074, 1180, 1387, 1843, 2371, 2506, 3138,
        3476, 3581, 3854, 4177, 4198, 4233, 5279, 5956, 7103, 7152, 7156, 7281, 7559, 8319, 8742,
        8854, 8886, 2408, 500, 4500, 1701,
    ];
    let mut rng = thread_rng();
    ports.shuffle(&mut rng);
    let selected_ports: Vec<u16> = ports.iter().take(*port_count).cloned().collect(); // 获取port_count个随机端口

    // 组合成网络地址
    let addresses: Vec<String> = ips
        .iter()
        .flat_map(|ip| {
            selected_ports
                .iter()
                .map(move |port| format!("{}:{}", ip, port))
        })
        .collect();

    // 打乱地址向量并选择前node_count个元素
    let mut rng = thread_rng();
    let mut shuffled_addresses = addresses.clone(); // 克隆一份地址向量以免改变原始向量
    shuffled_addresses.shuffle(&mut rng);
    let selected_ip_with_port: Vec<String> = shuffled_addresses
        .clone()
        .iter()
        .take(*node_count)
        .cloned()
        .collect();
    return selected_ip_with_port;
}

// 定义带有查询参数的路由
#[get("/sub")]
async fn subconverter(req: HttpRequest) -> impl Responder {
    // 获取查询字符串
    let query_string = req.query_string();

    // 使用 serde_urlencoded 库解析查询字符串
    let params: Vec<(String, String)> =
        serde_urlencoded::from_str(&query_string).expect("Failed to parse query string");

    // 查询参数
    let mut target = "wireguard".to_string();
    let mut ip_count = 1000;
    let mut port_count = 10;
    let mut node_count: usize = 300;
    let mut mtu_value: u16 = 1387; // 1387、1342、1304、1280
    for (key, value) in params {
        if key == "target" {
            target = value.to_string();
        } else if key == "ip_count" || key.to_lowercase() == "ipcount" {
            ip_count = value.parse().unwrap_or(1000);
        } else if key == "port_count" || key.to_lowercase() == "portcount" {
            let port_number: usize = value.parse().unwrap_or(10);
            if port_number > 0 && port_number <= 54 {
                port_count = port_number;
            }
        } else if key == "node_count" || key.to_lowercase() == "nodecount" {
            node_count = value.parse().unwrap_or(300);
        } else if key == "mtu" {
            mtu_value = value.parse().unwrap_or(1280);
        }
    }
    let selected_ip_with_port_vec = selected_ip_with_port(&ip_count, &port_count, &node_count);
    if target.to_lowercase() == "clash" {
        // 返回响应
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(generate_clash_config(selected_ip_with_port_vec, mtu_value))
    } else if target.to_lowercase() == "nekoray" || target.to_lowercase() == "nekobox" {
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(generate_nekoray_nodes(selected_ip_with_port_vec, mtu_value))
    } else if target.to_lowercase() == "wireguard" || target.to_lowercase() == "v2rayn" {
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(generate_wireguard_nodes(
                selected_ip_with_port_vec,
                mtu_value,
            ))
    } else {
        // 返回响应
        HttpResponse::Ok().body("404 Not Found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "127.0.0.1:8081";
    println!("web服务地址：http://{}/sub?", bind);
    // 启动 HTTP 服务器
    actix_web::HttpServer::new(|| {
        // 创建应用程序并注册路由
        actix_web::App::new()
            .service(subconverter) // 注册路由
            .default_service(actix_web::web::route().to(default_route)) // 设置通配符路由处理函数
    })
    .bind(bind)? // 绑定监听地址和端口
    .run() // 启动服务器
    .await // 等待服务器运行完成
}
