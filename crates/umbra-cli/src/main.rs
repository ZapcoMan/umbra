//! UMBRA 命令行工具（黑客 / 极客 / 暗黑风格）。
//!
//! 用法：
//!   umbra 8.8.8.8 1.1.1.1 2408:4003::1     # 直接查询若干 IP
//!   cat ips.txt | umbra                     # 从 stdin 逐行批量（跳过空行与 # 注释）
//!   umbra --json 8.8.8.8                    # NDJSON 输出，便于管道
//!   umbra --db-dir ./data 114.114.114.114   # 指定 xdb 目录（生产打包场景）

use std::io::{self, IsTerminal, Write};

use clap::{CommandFactory, Parser};
use serde_json::json;
use umbra_core::Umbra;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";

#[derive(Parser)]
#[command(
    name = "umbra",
    version,
    about = "UMBRA — 离线 IP 地理定位 (基于 ip2region，Rust 内核)",
    after_help = "示例:\n  umbra 8.8.8.8 2408:4003::1\n  cat ips.txt | umbra --json\n  umbra --db-dir ./data 114.114.114.114"
)]
struct Cli {
    /// 要查询的 IP（IPv4/IPv6）。省略时从 stdin 逐行读取
    ips: Vec<String>,

    /// 以 NDJSON 逐行输出（适合管道 / 脚本）
    #[arg(long)]
    json: bool,

    /// 存放 ip2region_v4.xdb / ip2region_v6.xdb 的目录
    #[arg(long, value_name = "DIR")]
    db_dir: Option<String>,

    /// 关闭启动 banner
    #[arg(long)]
    no_banner: bool,
}

fn banner() {
    let out = io::stdout();
    let mut h = out.lock();
    let art = [
        "██╗   ██╗██╗    ██╗██████╗ ██╗    ██╗██████╗  █████╗",
        "██║   ██║██║    ██║██╔══██╗██║    ██║██╔══██╗██╔══██╗",
        "██║   ██║██║ █╗ ██║██████╔╝██║ █╗ ██║██████╔╝███████║",
        "╚██╗ ██╔╝██║███╗██║██╔═══╝ ██║███╗██║██╔══██╗██╔══██║",
        " ╚████╔╝ ╚███╔███╔╝██║     ╚███╔███╔╝██║  ██║██║  ██║",
        "  ╚═══╝   ╚══╝╚══╝ ╚═╝      ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝",
    ];
    for line in art {
        let _ = writeln!(h, "{BOLD}{MAGENTA}{line}{RESET}");
    }
    let _ = writeln!(
        h,
        "{DIM}  umbra v{} · 离线 IP 地理定位 · 内核: ip2region(Rust){RESET}",
        env!("CARGO_PKG_VERSION")
    );
}

/// 读取 stdin 的批量 IP（跳过空行与 # 注释）。
fn read_stdin() -> Vec<String> {
    let mut v = Vec::new();
    for line in io::stdin().lines() {
        let l = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let l = l.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        v.push(l.to_string());
    }
    v
}

fn main() {
    let cli = Cli::parse();

    // 收集待查 IP
    let mut queries = cli.ips.clone();
    if queries.is_empty() {
        if !io::stdin().is_terminal() {
            queries = read_stdin();
        } else {
            let mut err = io::stderr();
            let _ = Cli::command().write_help(&mut err);
            eprintln!("\n{RED}umbra: 需要至少一个 IP 或通过管道传入批量{RESET}");
            std::process::exit(2);
        }
    }
    if queries.is_empty() {
        eprintln!("{RED}umbra: 没有可查询的 IP{RESET}");
        std::process::exit(2);
    }

    // 打开引擎
    let umbra = match &cli.db_dir {
        Some(d) => Umbra::open(
            &format!("{d}/ip2region_v4.xdb"),
            &format!("{d}/ip2region_v6.xdb"),
        ),
        None => Umbra::open_default(),
    };
    let umbra = match umbra {
        Ok(u) => u,
        Err(e) => {
            eprintln!("{RED}umbra: 打开离线库失败 — {e}{RESET}");
            std::process::exit(1);
        }
    };

    if !cli.json && !cli.no_banner && io::stdout().is_terminal() {
        banner();
    }

    let mut failures = 0usize;
    for ip in &queries {
        match umbra.lookup(ip) {
            Ok(region) => {
                if cli.json {
                    let v = json!({ "ip": ip, "ok": true, "region": region });
                    println!("{}", serde_json::to_string(&v).unwrap());
                } else {
                    println!(
                        "  {CYAN}{:<44}{RESET} {DIM}│{RESET} {GREEN}{}{RESET}",
                        ip,
                        region.as_raw()
                    );
                }
            }
            Err(e) => {
                failures += 1;
                let msg = e.to_string();
                if cli.json {
                    let v = json!({ "ip": ip, "ok": false, "error": msg });
                    println!("{}", serde_json::to_string(&v).unwrap());
                } else {
                    println!("  {CYAN}{:<44}{RESET} {DIM}│{RESET} {RED}✗ {msg}{RESET}", ip);
                }
            }
        }
    }

    if failures > 0 {
        std::process::exit(1);
    }
}
