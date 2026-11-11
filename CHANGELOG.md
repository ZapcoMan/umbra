# 更新日志

本项目遵循[保留式更新日志](https://keepachangelog.com/zh-CN/1.0.0/)规范，版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.1.0] - 2026-11-11

首个版本：基于上游 ip2region 的离线 IP 地理定位套件。

### 新增

- **umbra-core（Rust 内核）**：加载 ip2region v4/v6 xdb，按 IP 族自动路由，region 五段结构化解析（含 `0` 占位），统一错误类型；8 项单元测试通过。
- **umbra-cli（命令行）**：`umbra` 支持单/多 IP、stdin 管道批量、NDJSON 输出、`--db-dir` 指定数据、黑客风 banner；任一失败退码 1。
- **umbra-server（Web 后端）**：axum + 类 Spring Boot 三层（controller/service/mapper），`/api/health`、`/api/lookup`、`/api/batch`，非法 IP 返回 400，批量逐项独立，CORS 放开。
- **apps/desktop（桌面客户端）**：Tauri 2 + Vue3 + Vite + Element Plus + TS，三主题（极客/暗色/亮色，设置页切换持久化），经 IPC 调用 Rust 内核；产出 Windows MSI 与 NSIS 安装包。
- **apps/web（网页端）**：Vue3 + Vite + Element Plus + TS，三主题，经 HTTP 调用后端，含后端连通测试与本地历史。
- **site（官网）**：零构建静态站，暗黑极客视觉，呈现三形态/快速上手/使用须知与上游致谢。
- **数据与合规**：随附从上游复制的 `data/ip2region_v4.xdb`、`ip2region_v6.xdb`；保留上游 Apache-2.0/MIT 许可全文；UMBRA 采用相同双许可。

### 说明

- 数据访问全部在 Rust 内核完成，前端不解析任何 `.xdb`。
- 桌面与网页端 GUI 运行时交互需人工最终确认。
