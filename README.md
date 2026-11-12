# UMBRA

<div align="center">

![Version](https://img.shields.io/badge/version-v0.1.1-00ff9c?style=flat-square)
![Release](https://img.shields.io/github/v/release/ZapcoMan/umbra?style=flat-square&label=release&color=ff2d95)
![Rust](https://img.shields.io/badge/Rust-1.95-orange?style=flat-square)
![Vue](https://img.shields.io/badge/Vue-3.x-blue?style=flat-square)
![Tests](https://img.shields.io/badge/Tests-12%20passed-success?style=flat-square)
![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blueviolet?style=flat-square)

**基于 Rust 内核的离线 IP 地理定位套件 · 不联网、不上传**

[快速开始](#-快速开始) • [项目结构](#-项目结构) • [测试说明](#-测试说明) • [API 文档](#-api-文档) • [常见问题](#-常见问题)

</div>

---

## 📋 项目简介

UMBRA 是一款**离线、本地化**的 IP 地理定位套件：把 IP 地理位置数据编译进本地 `.xdb` 二进制文件，查询全程在本机完成，**不联网、不上传**。它提供三种形态——Windows 桌面客户端、命令行工具、网页端 + HTTP API，且数据访问统一由 **Rust 内核**实现。

> UMBRA **基于开源项目 [ip2region](https://github.com/lionsoul2014/ip2region) 构建**：数据格式、xdb 解析与查询能力均来自上游 ip2region（本项目 vendor 其 Rust 绑定）。详见[关于上游](#-关于上游)。

### ✨ 核心特性

- 🔌 **完全离线**：内置 ip2region v4/v6 xdb，查询不产生任何网络请求
- 🦀 **Rust 内核**：`umbra-core` 是唯一读取 `.xdb` 的地方，前端只做界面
- 🌐 **IPv4 + IPv6**：按地址族自动路由到对应库，批量查询逐项独立不中断
- 🧩 **region 结构化解析**：五段 `国家|省份|城市|ISP|ISO` 解析为结构化字段，缺失以 `0` 占位
- 🎨 **三主题界面**：极客 / 暗色 / 亮色，设置页切换并本地持久化
- 📦 **多端复用**：一套内核支撑桌面(Tauri)、CLI、Web(HTTP API)
- 🚀 **解压即用**：Release 归档与安装包内嵌离线库，无需额外配置

### 🛠️ 技术栈

| 分类         | 技术                                                                   |
|--------------|------------------------------------------------------------------------|
| **内核**     | Rust 1.95, ip2region 0.2.1（vendored）, maker 0.2.0（vendored）, serde |
| **命令行**   | clap 4.6（derive）                                                     |
| **Web 后端** | axum 0.8, tokio 1, tower-http 0.6（CORS）, tracing                     |
| **前端**     | Vue 3.5, Vite 6, Element Plus 2.14, Pinia 2, TypeScript 5.6            |
| **桌面**     | Tauri 2（IPC 直连 Rust 内核）                                          |
| **官网**     | 零构建静态 HTML/CSS                                                    |
| **工具**     | cargo（rsproxy 镜像）, pnpm 12（npmmirror）, GitHub Actions            |

---

## 🚀 快速开始

### ⚡ 方式一：下载 Release 产物（推荐，解压即用）

CI 在 `v*` tag 上构建并发布到 [GitHub Release](https://github.com/ZapcoMan/umbra/releases)：

| 产物 | 内容 |
|------|------|
| `umbra-<tag>-<target>.tar.gz` / `.zip` | `umbra` + `umbra-server` + 内嵌 `data/*.xdb`，解压即用。target 覆盖 Linux x86_64-gnu、Windows x64-msvc |
| `UMBRA_<version>_x64-setup.exe` / `.msi` | Windows 桌面客户端安装包 |
| `umbra_<version>_amd64.deb` | Linux 桌面客户端 |

✅ 归档内二进制与 `data/` 同级，命中内核定位链的"可执行文件同级 `data/`"，无需任何配置
🌐 官网：<https://zapcoman.github.io/umbra/>（由 `site/` 经 GitHub Pages 部署）

---

### 💻 方式二：源码构建（开发模式）

#### 前置要求

- Rust 1.95+（cargo/rustc）
- Node.js 24+、pnpm 12+（仅前端需要）
- 离线库已随仓库提供（`data/ip2region_v4.xdb`、`data/ip2region_v6.xdb`），无需下载

#### 1. 命令行

```bash
# 查询单个或批量 IP（IPv4 / IPv6 自动路由）
cargo run -p umbra-cli -- 8.8.8.8 2408:4003::1

# 实测输出示例
#   8.8.8.8        │ United States|California|0|Google LLC|US
#   2408:4003::1   │ 中国|广东省|深圳市|阿里|CN

# 管道批量 + NDJSON 输出
cat ips.txt | cargo run -q -p umbra-cli -- --json
```

#### 2. HTTP API 后端

```bash
cargo run -p umbra-server        # 默认监听 127.0.0.1:8080
```

✅ 后端地址：http://127.0.0.1:8080
✅ 健康检查：http://127.0.0.1:8080/api/health

```bash
curl "http://127.0.0.1:8080/api/lookup?ip=8.8.8.8"
curl -X POST -H "Content-Type: application/json" \
     -d '{"ips":["8.8.8.8","114.114.114.114"]}' \
     "http://127.0.0.1:8080/api/batch"
```

#### 3. 桌面客户端

```bash
cd apps/desktop
pnpm install
pnpm tauri dev       # 开发模式
pnpm tauri build     # 产出 src-tauri/target/release/bundle 下的 .msi 与 setup.exe
```

#### 4. 网页端

```bash
cd apps/web
pnpm install
pnpm build           # VITE_API_BASE 指定后端地址；需先启动 umbra-server
```

---

## 📁 项目结构

```
umbra/
├── 📄 README.md                    # 项目说明（本文件）
├── 📄 CHANGELOG.md                 # 更新日志（保留式更新日志规范）
├── 📄 AGENT.md                     # 会话记录与决策追溯
├── 📄 LICENSE                      # Apache-2.0 OR MIT 双许可全文
├── 📄 Cargo.toml                   # workspace（members: core/cli/server；exclude: vendor,apps）
├── ⚙️ .cargo/config.toml           # rsproxy sparse 镜像
├── 📂 crates/
│   ├── umbra-core/                 # 🦀 Rust 查询内核（唯一读 xdb）+ 12 项测试
│   ├── umbra-cli/                  # ⌨️ 命令行工具 umbra
│   └── umbra-server/               # 🌐 axum 三层 Web 后端
│       └── src/
│           ├── controller.rs       # 控制层（路由）
│           ├── service.rs          # 服务层（业务编排）
│           ├── mapper.rs           # 数据操作层（唯一持有查询引擎）
│           ├── dto.rs              # 请求/响应数据契约
│           └── error.rs            # 统一错误 → HTTP 语义
├── 📂 apps/
│   ├── desktop/                    # 🖥️ Tauri 桌面客户端（src 前端 + src-tauri Rust）
│   └── web/                        # 🌐 网页端查询应用（fetch 调后端）
├── 📂 site/                        # 🏠 官网静态站（零构建）
├── 📂 data/                        # 🗄️ ip2region_v4.xdb / ip2region_v6.xdb 离线库
├── 📂 vendor/                      # 📦 上游 ip2region + maker vendored 副本与上游许可
└── 📂 .github/workflows/           # 🚀 Release 构建与 GitHub Pages 部署
```

---

## 🧪 测试说明

内核包含完整单元测试，CLI / 后端 / 前端均经真实构建与查询验证。

| 层级 | 验证内容 | 状态 |
|------|---------|------|
| umbra-core | 12 项单元测试（真实 xdb 查询、双栈路由、非法 IP、region 解析、序列化、离线库定位优先级链） | ✅ 100% |
| umbra-cli / umbra-server | 真实 IP 查询、错误码、批量逐项 | ✅ 实测通过 |
| 桌面 / 网页端 | `vue-tsc` + `vite build`；桌面已产出 Windows 安装包 | ✅ 构建通过 |
| 官网 | 浏览器实测渲染 | ✅ 通过 |
| GUI 运行时交互 | 主题切换、IPC/HTTP 端到端 | ⏳ 需人工打开安装包 / 浏览器最终确认 |

**快速运行：**
```bash
cargo build --workspace          # 三个 Rust crate
cargo test  --workspace          # 内核单元测试
cargo clippy --workspace         # 代码检查（零告警）
```

前端各自独立：`apps/desktop`、`apps/web` 用 `pnpm install && pnpm build`。
Rust 依赖通过 `.cargo/config.toml` 走 rsproxy 镜像；npm 依赖走 npmmirror。

---

## 📖 API 文档

后端采用类 Spring Boot 三层架构（controller / service / mapper），数据访问全部经 Rust 内核。

### 接口一览

| 模块 | 接口路径 | 说明 |
|------|---------|------|
| ❤️ 健康检查 | `GET /api/health` | 服务状态、引擎与版本 |
| 🔍 单 IP 查询 | `GET /api/lookup?ip=` | IPv4 / IPv6 自动路由 |
| 📦 批量查询 | `POST /api/batch` | 请求体 `{"ips":[...]}`，逐项独立不中断 |

### 响应格式

单 IP 查询（`GET /api/lookup?ip=8.8.8.8`）：

```json
{
  "ok": true,
  "ip": "8.8.8.8",
  "region": {
    "country": "United States",
    "province": "California",
    "city": null,
    "isp": "Google LLC",
    "iso": "US"
  }
}
```

批量查询（`POST /api/batch`）：

```json
{
  "results": [
    { "ip": "8.8.8.8", "ok": true, "region": { "country": "United States", "province": "California", "city": null, "isp": "Google LLC", "iso": "US" }, "error": null },
    { "ip": "not-an-ip", "ok": false, "region": null, "error": "非法 IP 地址: not-an-ip" }
  ]
}
```

⚠️ **注意**：缺失字段在原始 region 中以 `0` 占位，结构化后为 `null`；错误响应统一为 `{"ok": false, "error": "..."}`。

### 错误码

| HTTP 状态码 | 场景 |
|------------|------|
| 400 | 非法 IP 字面量 |
| 422 | 请求合法但数据中查不到该 IP |
| 500 | 服务内部错误（细节不外泄，仅记录日志） |

### region 数据格式

查询结果按 5 段返回，缺失字段以 `0` 占位：

```text
国家|省份|城市|ISP|ISO-3166 alpha2
中国|广东省|深圳市|电信|CN
United States|New York|0|0|US
```

---

## ⚙️ 配置与环境变量

| 配置项 | 作用域 | 默认值 | 说明 |
|--------|--------|--------|------|
| `UMBRA_DB_DIR` | 内核通用 | 未设置 | 显式指定离线库目录，优先级最高 |
| `UMBRA_ADDR` | umbra-server | `127.0.0.1:8080` | 后端监听地址 |
| `--db-dir` | umbra-cli | 未设置 | 命令行显式覆盖数据目录 |
| `VITE_API_BASE` | apps/web | 本机后端 | 网页端指向的后端地址 |

**使用示例：**
```bash
# 指定数据目录启动后端，并监听所有网卡
UMBRA_DB_DIR=/opt/umbra/data UMBRA_ADDR=0.0.0.0:9000 ./umbra-server

# CLI 显式指定数据目录
./umbra --db-dir /opt/umbra/data 8.8.8.8
```

---

## 🔒 离线与隐私机制

- **完全离线查询**：所有查询在本机离线完成，不会向任何外部服务发送你查询的 IP
- **数据边界**：`umbra-core` 是全项目唯一读取 `.xdb` 之处；CLI/后端/桌面 IPC 都经它，网页端经 HTTP 到 `umbra-server`，前端(Node/浏览器)绝不解析数据
- **CORS 跨域**：后端已配置全局跨域支持（当前 `permissive()`，公网部署需按实际前端来源收紧）
- **异常处理**：统一错误类型归类为 HTTP 语义响应，内部错误细节不外泄
- **批量容错**：批量查询逐项独立，单项失败不影响其余项

### 前端架构说明

**技术栈：**
- Vue 3 + Vite + TypeScript
- Element Plus UI 组件库
- Pinia 状态管理
- 桌面端经 Tauri IPC（`invoke`）直连内核；网页端用原生 `fetch`（不引 axios，保持最小依赖）

**目录结构（两端一致）：**
```
src/
├── components/     # QueryBar / ResultTable / HistoryPanel（桌面另有 ThemeSwitch）
├── services|api/   # 桌面 geo.ts 走 IPC；网页 geo.ts 走 HTTP
├── stores/         # query / history / theme（Pinia）
├── styles/         # base.css + themes.css（三主题基座）
├── views/          # SettingsView（主题偏好与治理动作归此页）
└── App.vue         # 查询页只做查询
```

**关键特性：**

1. **三主题**（geek / dark / light）：设置页切换，本地持久化；命令行工具不做主题
2. **页面职责分离**：查询页只做查询；主题切换、清空历史/结果等治理与偏好归设置页
3. **本地历史**：查询历史存于本机，网页端另含后端连通测试

---

## 🗄️ 离线库与查询性能

### 数据文件

`data/` 随附从上游复制的离线库：

| 文件 | 说明 |
|------|------|
| `ip2region_v4.xdb` | IPv4 查询数据 |
| `ip2region_v6.xdb` | IPv6 查询数据 |

数据头版本为 v3。`umbra-core` 默认以 **VectorIndex 缓存**加载这两份文件（内存与速度折中，推荐默认）。

### 离线库定位顺序

内核按以下优先级查找 `data/ip2region_v{4,6}.xdb`，命中即停：

| 优先级 | 位置 | 适用场景 |
|--------|------|---------|
| 1 | 环境变量 `UMBRA_DB_DIR` 指定的目录 | 显式覆盖 |
| 2 | 可执行文件同级的 `data/` | Release 归档与桌面安装包即此布局，解压/安装即用 |
| 3 | 编译期源码树 `data/` | 仅 `cargo run` / `tauri dev` 开发态有效 |

CLI 可用 `--db-dir` 显式覆盖；桌面客户端在此之前还会先尝试 Tauri 资源目录（xdb 经 `bundle.resources` 随包分发）。三级都未命中时，错误信息会列出已尝试的目录。

**性能说明：**
- ⚡ VectorIndex 缓存：一次加载、常驻内存，查询为纯内存/本地文件操作
- ⚡ 双栈引擎以 `Arc` 共享只读实例，多线程并发查询安全
- ⚡ 无网络往返，查询延迟仅取决于本地 IO

---

## ❓ 常见问题

### 1. 启动报"未找到离线库"

错误信息会列出已尝试的目录。确认 `data/ip2region_v4.xdb` 与 `ip2region_v6.xdb` 位于可执行文件同级的 `data/` 下，或用环境变量显式指定：

```bash
UMBRA_DB_DIR=/path/to/data ./umbra-server
```

### 2. 后端端口被占用

用 `UMBRA_ADDR` 更换监听地址：

```bash
UMBRA_ADDR=127.0.0.1:9000 ./umbra-server
```

### 3. 网页端连不上后端

先启动 `umbra-server`，再确认网页端 `VITE_API_BASE` 指向正确的后端地址；后端 CORS 已放开，跨域不是问题源。

### 4. `pnpm install` 退出码非 0

pnpm 12 的安全默认会拦截 esbuild/vue-demi 的构建脚本，但原生二进制走 optionalDependencies，`pnpm build` 仍成功——退出码非 0 不代表前端构建失败。仓库已用 `pnpm-workspace.yaml` 显式声明 `allowBuilds` 使 CI 上以 0 退出。

### 5. 查询结果精度有限

IP 地理数据由第三方渠道汇总，**精度与时效有限**，不作为执法、金融风控等高置信场景的唯一依据。请在符合所在地法律法规的前提下使用，不得用于侵犯他人隐私或其他违法违规用途。

---

## 🤝 贡献指南

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/AmazingFeature`
3. 提交更改：`git commit -m 'Add some AmazingFeature'`
4. 推送分支：`git push origin feature/AmazingFeature`
5. 提交 Pull Request

提交前请确保 `cargo build/test/clippy --workspace` 全绿，前端 `pnpm build` 通过。

---

## 📄 许可证

UMBRA 采用 **Apache License 2.0 与 MIT License 双许可**（与上游 ip2region 一致），你可以任选其一使用、分发和修改。协议全文见 [LICENSE](LICENSE)（沿用上游许可文本，MIT 部分版权署上游作者 Lionsoul）。

### 🙏 关于上游

UMBRA 的数据与查询能力来自 **ip2region**（作者 lionsoul2014 / chenxin619315@gmail.com）。本项目 vendor 其 Rust 绑定（`vendor/rust/ip2region` 及依赖 `vendor/maker`），并保留上游许可全文于 `vendor/UPSTREAM-LICENSE-ip2region.md`。上游采用 **Apache License 2.0 与 MIT License 双许可**。

---

## 📞 联系方式

如有问题或建议，欢迎提 [Issue](https://github.com/ZapcoMan/umbra/issues)

---

*最后更新时间：2026-11-12*
*最新更新：许可文件重命名为 LICENSE 并恢复上游许可原文，README 按统一模板重写*