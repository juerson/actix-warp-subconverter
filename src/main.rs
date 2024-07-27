mod utils;

use crate::utils::clash::generate_clash_config;
use crate::utils::data::read_ip_with_port_from_files;
use crate::utils::hiddify::generate_hiddify_config;
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
        // 使用迭代的方法：获取向量的前面node_count个元素（不足node_count个元素就获取全部元素），这个更具 Rust 的习惯和风格
        let selected_ip_with_port: Vec<String> = shuffled_addresses
            .clone()
            .iter()
            .take(*node_count)
            .cloned()
            .collect();
        return selected_ip_with_port;
    }
}

fn generate_ip_with_port_vec(
    loc: &String,
    ip_count: &usize,
    port_count: &usize,
    ip_type: &u8,
) -> Vec<String> {
    let cidrs: Vec<&str> = if loc.to_lowercase() == "gb" && ip_type.clone() == 4 {
        vec![
            "188.114.96.0/24",
            "188.114.97.0/24",
            "188.114.98.0/24",
            "188.114.99.0/24",
        ]
    } else if loc.to_lowercase() == "us" && ip_type.clone() == 4 {
        vec!["162.159.192.0/24", "162.159.193.0/24", "162.159.195.0/24"]
    } else if loc == "" && ip_type.clone() == 6 {
        vec!["2606:4700:d0::/48", "2606:4700:d1::/48"]
    } else if loc == "" && ip_type.clone() == 4 {
        vec![
            "162.159.192.0/24",
            "162.159.193.0/24",
            "162.159.195.0/24",
            "188.114.96.0/24",
            "188.114.97.0/24",
            "188.114.98.0/24",
            "188.114.99.0/24",
        ]
    } else {
        vec![
            "162.159.192.0/24",
            "162.159.193.0/24",
            "162.159.195.0/24",
            "188.114.96.0/24",
            "188.114.97.0/24",
            "188.114.98.0/24",
            "188.114.99.0/24",
            "2606:4700:d0::/48",
            "2606:4700:d1::/48",
        ]
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

// 定义带有查询参数的路由
#[get("/sub")]
async fn subconverter(req: HttpRequest) -> impl Responder {
    // 获取查询字符串
    let query_string = req.query_string();

    // 使用 serde_urlencoded 库解析查询字符串
    let params: Vec<(String, String)> =
        serde_urlencoded::from_str(&query_string).expect("Failed to parse query string");

    // 查询参数
    let mut target = "".to_string();
    let mut ip_count = 1000;
    let mut port_count = 10;
    let mut node_count: usize = 300;
    let mut mtu_value: u16 = 1280;
    let mut detour = false;
    let mut loc: String = "".to_string(); // us/gb
    let mut fake_packets: String = "5-10".to_string(); // 用于修改hiddify的json数据
    let mut fake_packets_size: String = "40-100".to_string(); // 用于修改hiddify的数据
    let mut fake_packets_delay: String = "".to_string(); // 用于修改hiddify的数据
    let mut file_data: bool = false; // 用于控制是否使用data文件夹下的txt、csv数据文件，默认false不使用
    let mut ip_type: u8 = 4;

    for (key, value) in params {
        if key.to_lowercase() == "target" {
            target = value.to_string();
        } else if key.to_lowercase() == "ip_count" || key.to_lowercase() == "ipcount" {
            ip_count = value.parse().unwrap_or(1000);
        } else if key.to_lowercase() == "port_count" || key.to_lowercase() == "portcount" {
            let port_number: usize = value.parse().unwrap_or(10);
            if port_number > 0 && port_number <= 54 {
                port_count = port_number;
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
                file_data = true; // 用于控制是否使用data文件夹下的txt、csv数据文件
            }
        } else if key.to_lowercase() == "iptype" {
            if [4, 6].contains(&value.parse().unwrap_or(4)) {
                ip_type = value.parse().unwrap_or(4);
            } else {
                ip_type = 4;
            }
        } else {
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
    if target.to_lowercase() == "clash" {
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
    } else if target.to_lowercase() == "hiddify" {
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(generate_hiddify_config(
                selected_ip_with_port_vec,
                mtu_value,
                detour, // 是否构建链式代理
                fake_packets,
                fake_packets_size,
                fake_packets_delay,
            ))
    } else {
        HttpResponse::Ok().body("404 Not Found")
    }
}

// 首页内容
#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    /* 这里的代码不重要，只是该程序的使用方法，备忘录。显示效果在浏览器中查看！ */
    // 从HttpRequest中获取请求的域名地址/主机地址
    let host_address = req.connection_info().host().to_owned();

    let title = format!(
        "软件功能：WARP 转换为 Clash、NekoBox for PC、Hiddify、v2rayN/v2rayNG 客户端的订阅!\n\n"
    );

    let web_address = format!("web服务地址：http://{}\n\n", host_address);
    let syntax_info1 = format!("订阅地址格式：\nhttp://{}/sub?target=[clash,nekobox/nekoray,hiddify,v2rayn/wireguard]&ipCount=[1..?]&portCount=[1..54]&nodeCount=[1..?]&mtu=[1280..1500]&loc=[gb,us]&iptype=[4 or 6]\n\n",host_address);
    let syntax_info2 = format!("target：要转换为的目标客户端（必须的），其它参数为可选参数。\n");
    let parma_info = format!("ipCount: 从内置的CIDRs段中，选择随机生成多少个IP；\nportCount：从WARP支持的54个UDP端口中，选择随机多个端口；\nnodeCount：您想要生成多少个节点(最多节点数)；\nmtu：修改WireGuard节点的MTU值；\nloc：选择哪组CIDRs段(gb/us)的IP;\niptype：选择IPV4地址为wireguard的端点，还是选择IPv6地址为wireguard的端点？默认是IPv4地址。\n\n");
    let loc_info1 =
        format!("loc=gb -> 188.114.96.0/24,188.114.97.0/24,188.114.98.0/24,188.114.99.0/24\n");
    let loc_info2 = format!("loc=us -> 162.159.192.0/24,162.159.193.0/24,162.159.195.0/24\n");
    let loc_info3 = format!("\n注意：loc参数与filedata=1的参数(使用文件里面的优选IP)不能同时使用，同时使用会忽略loc参数。\n");
    let example1 = format!("http://{host_address}/sub?target=clash&loc=us\n");
    let example2 = format!("http://{host_address}/sub?target=nekobox&loc=gb\n");
    let example3 = format!("http://{host_address}/sub?target=wireguard&loc=us\n");
    let example4 = format!("http://{host_address}/sub?target=wireguard&iptype=6\n");
    let example5 = format!("http://{host_address}/sub?target=clash&iptype=6\n");
    let example_str = format!("{example1}{example2}{example3}{example4}{example5}");
    let hiddify1 = format!("\nHiddify相关\n\n");
    let hiddify2 = format!("【1】启用detour字段（detour=[1/true/on]，记住数字1即可）\n");
    let hiddify3 = format!("http://{}/sub?target=hiddify&detour=1\n", host_address);
    let hiddify4 = format!(
        "http://{}/sub?target=hiddify&detour=1&loc=gb\n",
        host_address
    );
    let hiddify5 = format!(
        "http://{}/sub?target=hiddify&detour=1&loc=us\n\n",
        host_address
    );
    let hiddify6 =
        format!("【2】修改字段 fake_packets、fake_packets_size、fake_packets_delay 的值（如果网络无法连接，网速慢，可以尝试修改这些参数）\n");
    let hiddify7 = format!(
        "http://{}/sub?target=hiddify&fake_packets_delay=10-100\n",
        host_address
    );
    let hiddify8 = format!(
        "http://{}/sub?target=hiddify&fake_packets=10-20&fake_packets_delay=30-200\n",
        host_address
    );
    let hiddify9 = format!("http://{}/sub?target=hiddify&detour=1&loc=us&fake_packets=10-20&fake_packets_delay=30-200\n\n",host_address);

    let hiddify_info = format!(
        "{}{}{}{}{}{}{}{}{}",
        hiddify1, hiddify2, hiddify3, hiddify4, hiddify5, hiddify6, hiddify7, hiddify8, hiddify9
    );
    let example = &format!("     例如：http://{host_address}/sub?target=hiddify&filedata=1");
    let file_data_info = vec!["支持使用WARP的优选IP，制作订阅链接：","\n使用方法：",
        "【1】将优选IP的文件(txt、csv文件)放到data文件夹中，文件名称随意。",
        "【2】打开本地web服务(您能看到这个页面就是打开了本地web服务)，在目标客户端的订阅地址中添加参数 filedata=[1/true/on]，记住数字1即可)。",
        example,
        "注意：",
        "【1】支持热更新，不需要重启服务，也就是，web服务一直打开，优选IP后，更新订阅就能使用新优选的IP。", 
        "【2】\"ips-v\"开头的文件被忽略了，比如：ips-v4.txt、ips-v6.txt，可以放心地将WARP优选IP的程序放到这个data文件夹里面，随时优选IP。",
        "【3】默认使用读取到的前300个IP:PORT，如果文件不存在、数据为空，则使用内置CIDRs段中生成的随机IP:PORT。",
        "【4】一定要记住：使用文件的优选IP数据，数据格式必须含IP:PORT格式的数据，才能被正则匹配到，否则读取不到数据，使用内置CIDRs段中生成的随机IP:PORT。"];

    let content = format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        title,
        web_address,
        syntax_info1,
        syntax_info2,
        parma_info,
        loc_info1,
        loc_info2,
        loc_info3,
        example_str,
        hiddify_info,
        file_data_info.join("\n")
    );
    // 返回响应
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(content)
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
        // 创建应用程序并注册路由
        actix_web::App::new()
            .service(index)
            .service(subconverter) // 注册路由
            .default_service(actix_web::web::route().to(default_route)) // 设置通配符路由处理函数
    })
    .bind(format!("0.0.0.0:{}", port))? // 监听所有 IPv4 地址
    .run() // 启动服务器
    .await // 等待服务器运行完成
}
