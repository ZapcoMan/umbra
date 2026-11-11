use std::sync::Arc;

use serde::Serialize;
use tauri::State;
use umbra_core::{Region, Umbra};

/// Tauri 托管状态：全局共享的只读查询引擎（Rust 内核，前端经 IPC 调用）。
pub struct AppState {
    pub umbra: Arc<Umbra>,
}

#[derive(Serialize)]
pub struct BatchItem {
    pub ip: String,
    pub ok: bool,
    pub region: Option<Region>,
    pub error: Option<String>,
}

/// 单 IP 查询。错误以字符串返回，前端 invoke 侧会 reject。
#[tauri::command]
fn lookup(state: State<'_, AppState>, ip: String) -> Result<Region, String> {
    state.umbra.lookup(&ip).map_err(|e| e.to_string())
}

/// 批量查询：逐项独立，绝不因单条非法中断整批。
#[tauri::command]
fn lookup_batch(state: State<'_, AppState>, ips: Vec<String>) -> Vec<BatchItem> {
    ips.into_iter()
        .map(|ip| match state.umbra.lookup(&ip) {
            Ok(region) => BatchItem {
                ip,
                ok: true,
                region: Some(region),
                error: None,
            },
            Err(e) => BatchItem {
                ip,
                ok: false,
                region: None,
                error: Some(e.to_string()),
            },
        })
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let umbra =
        Arc::new(Umbra::open_default().expect("无法打开内置离线库 data/*.xdb"));

    tauri::Builder::default()
        .manage(AppState { umbra })
        .invoke_handler(tauri::generate_handler![lookup, lookup_batch])
        .run(tauri::generate_context!())
        .expect("error while running UMBRA application");
}
