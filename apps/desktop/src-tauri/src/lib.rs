use std::path::PathBuf;
use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Manager, State};
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

/// 定位离线库：`UMBRA_DB_DIR` → 随包资源 `<resource_dir>/data` → 内核兜底链
/// （可执行文件同级 `data/`，最后才是源码树 `data/`，仅 `tauri dev` 有效）。
fn open_engine(app: &AppHandle) -> Result<Umbra, String> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(dir) = std::env::var("UMBRA_DB_DIR") {
        candidates.push(PathBuf::from(dir));
    }
    if let Ok(resource) = app.path().resource_dir() {
        candidates.push(resource.join("data"));
    }

    let mut missed = Vec::new();
    for dir in candidates {
        match Umbra::open_dir(&dir) {
            Ok(umbra) => return Ok(umbra),
            Err(e) => missed.push(format!("{}: {e}", dir.display())),
        }
    }
    Umbra::open_auto().map_err(|e| format!("{e}；随包资源亦未命中 — {}", missed.join(" | ")))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let umbra =
                open_engine(app.handle()).map_err(|e| format!("无法打开内置离线库 — {e}"))?;
            app.manage(AppState {
                umbra: Arc::new(umbra),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![lookup, lookup_batch])
        .run(tauri::generate_context!())
        .expect("error while running UMBRA application");
}
