//! UMBRA 离线 IP 地理查询内核。
//!
//! 本 crate 是全项目**唯一**真正读取 `.xdb` 数据的地方：
//! 上层 CLI / Web 服务 / Tauri 桌面客户端都通过它获取查询结果，
//! 前端(Node/浏览器)绝不直接解析数据文件。
//!
//! 数据能力来自上游 ip2region 的 Rust 绑定（本项目 vendor 副本）。
//! 具体查询与 region 解析在里程碑 2 实现。

// 里程碑 2 在此实现 Umbra 结构体、lookup 与 Region 解析。
