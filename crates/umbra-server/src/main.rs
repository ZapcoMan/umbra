//! UMBRA Web 后端入口：装配 mapper→service→router 并启动监听。

use std::sync::Arc;

use umbra_server::{build_router, mapper::IpGeoMapper, service::IpGeoService};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mapper = match IpGeoMapper::open_from_env() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("umbra-server 启动失败: {e}");
            std::process::exit(1);
        }
    };
    let service = Arc::new(IpGeoService::new(mapper));
    let app = build_router(service);

    let addr =
        std::env::var("UMBRA_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("umbra-server 绑定 {addr} 失败: {e}");
            std::process::exit(1);
        }
    };

    tracing::info!("UMBRA server 已启动: http://{addr}/api/health");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("umbra-server 运行错误: {e}");
        std::process::exit(1);
    }
}
