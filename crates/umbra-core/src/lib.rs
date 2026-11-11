//! UMBRA 离线 IP 地理查询内核。
//!
//! 本 crate 是全项目**唯一**真正读取 `.xdb` 数据的地方：
//! CLI、Web 后端、Tauri 桌面客户端都必须经由它取数，前端(Node/浏览器)绝不解析数据。
//!
//! 查询能力来自上游 ip2region 的 Rust 绑定（本项目 vendor 副本），
//! 本层在其之上做三件事：双栈(v4/v6)自动路由、region 字符串解析为结构化字段、统一错误类型。

use std::fmt;
use std::net::IpAddr;
use std::sync::Arc;

use ip2region::{CachePolicy, Searcher};
use serde::Serialize;

/// region 字符串固定段数：国家|省份|城市|ISP|ISO-3166 alpha2
const REGION_FIELDS: usize = 5;
/// 缺失字段的占位值
const NULL_FIELD: &str = "0";

/// 解析后的结构化地理信息。除国家外，缺失字段以 `None` 表示（源数据用 `0` 占位）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Region {
    pub country: String,
    pub province: Option<String>,
    pub city: Option<String>,
    pub isp: Option<String>,
    /// ISO-3166 alpha-2 国家码，如 `CN` / `US`
    pub iso: Option<String>,
}

impl Region {
    /// 将 ip2region 返回的 `国家|省份|城市|ISP|ISO` 解析为结构化字段。
    pub fn parse(raw: &str) -> Result<Region, UmbraError> {
        let parts: Vec<&str> = raw.split('|').collect();
        if parts.len() != REGION_FIELDS {
            return Err(UmbraError::MalformedRegion(raw.to_string()));
        }
        let opt = |v: &str| -> Option<String> {
            if v == NULL_FIELD || v.is_empty() {
                None
            } else {
                Some(v.to_string())
            }
        };
        Ok(Region {
            country: parts[0].to_string(),
            province: opt(parts[1]),
            city: opt(parts[2]),
            isp: opt(parts[3]),
            iso: opt(parts[4]),
        })
    }

    /// 还原为 `国家|省份|城市|ISP|ISO` 形式的原始字符串，缺失字段回填 `0`。
    pub fn as_raw(&self) -> String {
        let f = |v: &Option<String>| v.clone().unwrap_or_else(|| NULL_FIELD.to_string());
        format!(
            "{}|{}|{}|{}|{}",
            self.country,
            f(&self.province),
            f(&self.city),
            f(&self.isp),
            f(&self.iso)
        )
    }
}

/// UMBRA 统一错误类型。
#[derive(Debug)]
pub enum UmbraError {
    /// 输入不是合法的 IPv4 / IPv6 字面量
    InvalidIp(String),
    /// region 字符串结构异常（段数不符）
    MalformedRegion(String),
    /// 底层 xdb 读取 / 查询失败
    Search(String),
}

impl fmt::Display for UmbraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UmbraError::InvalidIp(ip) => write!(f, "非法 IP 地址: {ip}"),
            UmbraError::MalformedRegion(s) => write!(f, "region 格式异常: {s}"),
            UmbraError::Search(e) => write!(f, "查询失败: {e}"),
        }
    }
}

impl std::error::Error for UmbraError {}

/// 双栈查询引擎：持有 v4、v6 两个 ip2region Searcher，按输入 IP 族自动路由。
///
/// `Searcher` 的查询以不可变引用进行，故本类型可安全地被多线程共享（外层用 `Arc` 包裹即可）。
pub struct Umbra {
    v4: Arc<Searcher>,
    v6: Arc<Searcher>,
}

impl Umbra {
    /// 打开两份 xdb，使用 VectorIndex 缓存策略（内存与速度折中，推荐默认）。
    pub fn open(v4_path: &str, v6_path: &str) -> Result<Umbra, UmbraError> {
        let v4 = Searcher::new(v4_path.to_string(), CachePolicy::VectorIndex)
            .map_err(|e| UmbraError::Search(format!("{v4_path}: {e}")))?;
        let v6 = Searcher::new(v6_path.to_string(), CachePolicy::VectorIndex)
            .map_err(|e| UmbraError::Search(format!("{v6_path}: {e}")))?;
        Ok(Umbra {
            v4: Arc::new(v4),
            v6: Arc::new(v6),
        })
    }

    fn searcher_for(&self, addr: IpAddr) -> &Searcher {
        if addr.is_ipv4() {
            &self.v4
        } else {
            &self.v6
        }
    }

    /// 查询单个 IP，返回结构化 region。IPv4 / IPv6 均由本方法自动路由到对应库。
    pub fn lookup(&self, ip: &str) -> Result<Region, UmbraError> {
        let raw = self.lookup_raw(ip)?;
        Region::parse(&raw)
    }

    /// 查询单个 IP，返回原始 `国家|省份|城市|ISP|ISO` 字符串。
    pub fn lookup_raw(&self, ip: &str) -> Result<String, UmbraError> {
        let trimmed = ip.trim();
        let addr: IpAddr = trimmed
            .parse()
            .map_err(|_| UmbraError::InvalidIp(ip.to_string()))?;
        let searcher = self.searcher_for(addr);
        let raw = searcher
            .search(trimmed)
            .map_err(|e| UmbraError::Search(format!("{ip}: {e}")))?;
        Ok(raw)
    }

    /// 便捷构造：默认使用与源码树同包内置的 `data/ip2region_v{4,6}.xdb`
    /// （路径在编译期由 CARGO_MANIFEST_DIR 固定，供开发/测试；生产请用 [`Umbra::open`] 指定实际路径）。
    pub fn open_default() -> Result<Umbra, UmbraError> {
        let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");
        Umbra::open(
            base.join("ip2region_v4.xdb").to_str().unwrap(),
            base.join("ip2region_v6.xdb").to_str().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- 纯解析逻辑：可脱离 xdb 断言，能红 ----

    #[test]
    fn parse_full_region() {
        let r = Region::parse("中国|广东省|深圳市|电信|CN").unwrap();
        assert_eq!(r.country, "中国");
        assert_eq!(r.province.as_deref(), Some("广东省"));
        assert_eq!(r.city.as_deref(), Some("深圳市"));
        assert_eq!(r.isp.as_deref(), Some("电信"));
        assert_eq!(r.iso.as_deref(), Some("CN"));
    }

    #[test]
    fn parse_zero_placeholders_become_none() {
        let r = Region::parse("United States|New York|0|0|US").unwrap();
        assert_eq!(r.country, "United States");
        assert_eq!(r.province.as_deref(), Some("New York"));
        assert_eq!(r.city, None);
        assert_eq!(r.isp, None);
        assert_eq!(r.iso.as_deref(), Some("US"));
    }

    #[test]
    fn parse_wrong_field_count_is_error() {
        // 少一段应报错，而不是静默通过
        assert!(matches!(
            Region::parse("中国|广东省|深圳市|电信"),
            Err(UmbraError::MalformedRegion(_))
        ));
    }

    #[test]
    fn as_raw_roundtrips() {
        let raw = "中国|0|杭州市|阿里云|CN";
        let r = Region::parse(raw).unwrap();
        assert_eq!(r.as_raw(), raw);
    }

    #[test]
    fn region_serializes_to_json() {
        let r = Region::parse("中国|浙江省|杭州市|阿里云|CN").unwrap();
        let j = serde_json::to_string(&r).unwrap();
        assert!(j.contains("\"country\":\"中国\""));
        assert!(j.contains("\"city\":\"杭州市\""));
    }

    // ---- 真实 xdb 集成：验证路由与非法输入 ----

    fn umbra() -> Umbra {
        Umbra::open_default().expect("打开默认 xdb 失败：确认 data/*.xdb 存在")
    }

    #[test]
    fn invalid_ip_is_error() {
        let u = umbra();
        assert!(matches!(
            u.lookup("999.999.999.999"),
            Err(UmbraError::InvalidIp(_))
        ));
        assert!(matches!(
            u.lookup("not-an-ip"),
            Err(UmbraError::InvalidIp(_))
        ));
    }

    #[test]
    fn lookup_ipv4_public_dns() {
        let u = umbra();
        let r = u.lookup("8.8.8.8").expect("v4 查询应成功");
        assert_eq!(r.iso.as_deref(), Some("US"));
    }

    #[test]
    fn lookup_ipv6_routes_to_v6_lib() {
        let u = umbra();
        let r = u.lookup("2408:4003::1").expect("v6 查询应成功且路由到 v6 库");
        assert!(!r.country.is_empty());
    }
}
