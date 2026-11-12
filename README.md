# UMBRA

![Version](https://img.shields.io/badge/version-v0.1.1-00ff9c?style=flat-square)
![Release](https://img.shields.io/github/v/release/ZapcoMan/umbra?style=flat-square&label=release&color=ff2d95)
![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blueviolet?style=flat-square)

UMBRA 是一款**离线、本地化**的 IP 地理定位套件：把 IP 地理位置数据编译进本地 `.xdb` 二进制文件，查询全程在本机完成，**不联网、不上传**。它提供三种形态——Windows 桌面客户端、命令行工具、网页端 + HTTP API，且数据访问统一由 **Rust 内核**实现。

> UMBRA **基于开源项目 [ip2region](https://github.com/lionsoul2014/ip2region) 构建**：数据格式、xdb 解析与查询能力均来自上游 ip2region（本项目 vendor 其 Rust 绑定）。详见[关于上游](#关于上游)。

## 特性

- **完全离线**：内置 ip2region v4/v6 xdb，查询不产生任何网络请求。
- **Rust 内核**：`umbra-core` 是唯一读取 `.xdb` 的地方，双栈自动路由 + region 结构化解析；前端只做界面。
- **IPv4 + IPv6**：按地址族自动选择对应库，批量查询逐项独立不中断。
- **三主题界面**：极客 / 暗色 / 亮色，设置页切换并本地持久化。
- **多端复用**：一套内核支撑桌面(Tauri)、CLI、Web(HTTP API)。

## 三种形态

| 形态 | 位置 | 说明 |
| --- | --- | --- |
| 桌面客户端 | `apps/desktop` | Tauri 2 + Vue3 + Vite + Element Plus + TS，产出 Windows MSI/NSIS 安装包，经 IPC 调用 Rust 内核 |
| 命令行 | `crates/umbra-cli` | 二进制 `umbra`：单 IP、管道批量、NDJSON、`--db-dir` |
| Web 后端 | `crates/umbra-server` | axum HTTP API，类 Spring Boot 三层 controller/service/mapper |
| 网页端 | `apps/web` | Vue3 + Vite + Element Plus + TS，经 HTTP 调后端，同样三主题 |
| 官网 | `site` | 零构建静态站，暗黑极客视觉 |

## 下载

CI 在 `v*` tag 上构建并发布到 [GitHub Release](https://github.com/ZapcoMan/umbra/releases)：

| 产物 | 内容 |
| --- | --- |
| `umbra-<tag>-<target>.tar.gz` / `.zip` | `umbra` + `umbra-server` + 内嵌 `data/*.xdb`，解压即用。target 覆盖 Linux x86_64-gnu、Windows x64-msvc |
| `UMBRA_<version>_x64-setup.exe` / `.msi` | Windows 桌面客户端安装包 |
| `umbra_<version>_amd64.deb` | Linux 桌面客户端 |

官网：<https://zapcoman.github.io/umbra/>（由 `site/` 经 GitHub Pages 部署）

## 快速开始

### 命令行

```bash
# 查询单个或批量 IP（IPv4 / IPv6 自动路由）
cargo run -p umbra-cli -- 8.8.8.8 2408:4003::1

# 实测输出示例
#   8.8.8.8        │ United States|California|0|Google LLC|US
#   2408:4003::1   │ 中国|广东省|深圳市|阿里|CN

# 管道批量 + NDJSON 输出
cat ips.txt | cargo run -q -p umbra-cli -- --json
```

### HTTP API

```bash
cargo run -p umbra-server        # 默认监听 127.0.0.1:8080

curl "http://127.0.0.1:8080/api/lookup?ip=8.8.8.8"
curl -X POST -H "Content-Type: application/json" \
     -d '{"ips":["8.8.8.8","114.114.114.114"]}' \
     "http://127.0.0.1:8080/api/batch"
```

接口：`GET /api/health`、`GET /api/lookup?ip=`、`POST /api/batch`。非法 IP 返回 400。

### 桌面客户端

```bash
cd apps/desktop
pnpm install
pnpm tauri build      # 产出 src-tauri/target/release/bundle 下的 .msi 与 setup.exe
```

## region 数据格式

查询结果按 5 段返回，缺失字段以 `0` 占位：

```text
国家|省份|城市|ISP|ISO-3166 alpha2
中国|广东省|深圳市|电信|CN
United States|New York|0|0|US
```

## 数据文件

`data/` 随附从上游复制的离线库：

| 文件 | 说明 |
| --- | --- |
| `ip2region_v4.xdb` | IPv4 查询数据 |
| `ip2region_v6.xdb` | IPv6 查询数据 |

数据头版本为 v3。`umbra-core` 默认以 VectorIndex 缓存加载这两份文件。

## 离线库定位顺序

内核按以下优先级查找 `data/ip2region_v{4,6}.xdb`，命中即停：

1. 环境变量 `UMBRA_DB_DIR` 指定的目录；
2. 可执行文件同级的 `data/`——Release 归档与桌面安装包即此布局，故解压/安装即用；
3. 编译期源码树 `data/`，仅 `cargo run` / `tauri dev` 开发态有效。

CLI 可用 `--db-dir` 显式覆盖；桌面客户端在此之前还会先尝试 Tauri 资源目录。三级都未命中时，错误信息会列出已尝试的目录。

## 项目结构

```text
umbra/
├── crates/
│   ├── umbra-core/     # Rust 查询内核（唯一读 xdb）
│   ├── umbra-cli/      # 命令行工具 umbra
│   └── umbra-server/   # axum 三层 Web 后端
├── apps/
│   ├── desktop/        # Tauri 桌面客户端（含 src-tauri）
│   └── web/            # 网页端查询应用
├── site/               # 官网静态站
├── data/               # ip2region xdb 离线库
└── vendor/             # 上游 ip2region 及其依赖 maker 的 vendored 副本
```

## 开发与构建

```bash
cargo build --workspace          # 三个 Rust crate
cargo test  --workspace          # 内核单元测试
cargo clippy --workspace         # 代码检查
```

前端各自独立：`apps/desktop`、`apps/web` 用 `pnpm install && pnpm build`。
Rust 依赖通过 `.cargo/config.toml` 走 rsproxy 镜像；npm 依赖走 npmmirror。

## 测试与验证状态

- ✅ `umbra-core`：12 项单元测试（真实 xdb 查询、双栈路由、非法 IP、region 解析、序列化、离线库定位优先级链）全通过。
- ✅ `umbra-cli` / `umbra-server`：真实 IP 查询、错误码、批量逐项均实测。
- ✅ 桌面 / 网页端：`vue-tsc` + `vite build` 通过；桌面已产出 Windows 安装包；官网浏览器实测渲染正常。
- ⏳ 桌面与网页端的 GUI 运行时交互（主题切换、IPC/HTTP 端到端）需人工打开安装包 / 浏览器最终确认。

## 关于上游

UMBRA 的数据与查询能力来自 **ip2region**（作者 lionsoul2014 / chenxin619315@gmail.com）。本项目 vendor 其 Rust 绑定（`vendor/rust/ip2region` 及依赖 `vendor/maker`），并保留上游许可全文于 `vendor/UPSTREAM-LICENSE-ip2region.md`。上游采用 **Apache License 2.0 与 MIT License 双许可**。

## 使用须知

- IP 地理数据由第三方渠道汇总，**精度与时效有限**，不作为执法、金融风控等高置信场景的唯一依据。
- 请在符合所在地法律法规的前提下使用，不得用于侵犯他人隐私或其他违法违规用途。
- UMBRA 所有查询均在本机离线完成，不会向任何外部服务发送你查询的 IP。

## 开源协议

UMBRA 采用 **Apache License 2.0 与 MIT License 双许可**（与上游 ip2region 一致），你可以任选其一使用、分发和修改。协议全文见 [LICENSE](LICENSE)（沿用上游许可文本，MIT 部分版权署上游作者 Lionsoul）。
