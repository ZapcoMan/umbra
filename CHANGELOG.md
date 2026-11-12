# 更新日志

本项目遵循[保留式更新日志](https://keepachangelog.com/zh-CN/1.0.0/)规范，版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 改动

- **许可文件**：`LICENSE.md` 重命名为 `LICENSE`，恢复上游 ip2region 许可原文（Apache-2.0 全文 + MIT 部分版权署上游作者 Lionsoul）；README、AGENT.md 与 Release 工作流中的文件名引用同步更新。

## [0.1.1] - 2026-11-12

发布产物平台收敛：Release 工作流不再构建 macOS 产物。`v0.1.1` 是首个实际产出 GitHub Release 的版本。

### 改动

- **CLI / Server 归档**：覆盖 Linux x86_64-gnu 与 Windows x64-msvc 两个 target，仍连同 `data/` 一起打包，解压即用。
- **桌面安装包**：覆盖 Windows（NSIS + MSI）与 Linux（deb）。
- README 与官网的平台表述同步收敛，只列出实际提供的平台。

### 说明

- `v0.1.0` 的 Release 未生成：同一运行中三个 macOS 作业未能分配到 runner（无 step、无日志），导致汇总 Release 的 job 未执行。按"不重指已推送 tag"的约定，以本版本作为发布点。
- `v0.1.0` 条目所述 macOS 产物未对外提供；该历史条目保留原文以保真。

## [0.1.0] - 2026-11-11

首个版本：基于上游 ip2region 的离线 IP 地理定位套件。

### 新增

- **umbra-core（Rust 内核）**：加载 ip2region v4/v6 xdb，按 IP 族自动路由，region 五段结构化解析（含 `0` 占位），统一错误类型；12 项单元测试通过。
- **umbra-cli（命令行）**：`umbra` 支持单/多 IP、stdin 管道批量、NDJSON 输出、`--db-dir` 指定数据、黑客风 banner；任一失败退码 1。
- **umbra-server（Web 后端）**：axum + 类 Spring Boot 三层（controller/service/mapper），`/api/health`、`/api/lookup`、`/api/batch`，非法 IP 返回 400，批量逐项独立，CORS 放开。
- **apps/desktop（桌面客户端）**：Tauri 2 + Vue3 + Vite + Element Plus + TS，三主题（极客/暗色/亮色，设置页切换持久化），经 IPC 调用 Rust 内核；产出 Windows MSI 与 NSIS 安装包。
- **apps/web（网页端）**：Vue3 + Vite + Element Plus + TS，三主题，经 HTTP 调用后端，含后端连通测试与本地历史。
- **site（官网）**：零构建静态站，暗黑极客视觉，呈现三形态/快速上手/使用须知与上游致谢。
- **数据与合规**：随附从上游复制的 `data/ip2region_v4.xdb`、`ip2region_v6.xdb`；保留上游 Apache-2.0/MIT 许可全文；UMBRA 采用相同双许可。
- **离线库定位优先级链**：`UMBRA_DB_DIR` → 可执行文件同级 `data/` → 源码树 `data/`（开发态），使分发产物解压即用；桌面端在此之前先尝试 Tauri 资源目录，xdb 经 `bundle.resources` 随包分发。
- **发布与 CI**：`v*` tag 触发的 Release 工作流——四平台（Linux x86_64-gnu、macOS aarch64、macOS x86_64、Windows x64-msvc）构建 `umbra` 与 `umbra-server` 并连同 `data/` 归档；三平台（Windows NSIS+MSI、macOS dmg、Linux deb）构建桌面安装包；`site/` 经 GitHub Pages 部署至 <https://zapcoman.github.io/umbra/>。

### 说明

- 数据访问全部在 Rust 内核完成，前端不解析任何 `.xdb`。
- 根 `Cargo.toml` 的 `repository` 由上游地址修正为本仓库地址；上游致敬保留在文档与 `vendor/UPSTREAM-LICENSE-ip2region.md`。
- 前端新增 `pnpm-workspace.yaml` 显式声明 `allowBuilds`（esbuild / vue-demi 不运行安装脚本），使 `pnpm install --frozen-lockfile` 在 CI 上以 0 退出（pnpm 11+ 的 `strictDepBuilds` 默认会把未审查的构建脚本判为失败）。
- 新增的 4 项用例覆盖离线库定位优先级链与缺数据文件报错分支。
- 桌面与网页端 GUI 运行时交互需人工最终确认。
