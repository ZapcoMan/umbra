//! 数据契约（DTO）：请求/响应的序列化结构。

use serde::{Deserialize, Serialize};
use umbra_core::Region;

/// GET /api/lookup 的查询参数。
#[derive(Debug, Deserialize)]
pub struct LookupQuery {
    pub ip: String,
}

/// 单 IP 查询响应。
#[derive(Debug, Serialize)]
pub struct LookupResponse {
    pub ok: bool,
    pub ip: String,
    pub region: Option<Region>,
}

/// POST /api/batch 的请求体。
#[derive(Debug, Deserialize)]
pub struct BatchRequest {
    pub ips: Vec<String>,
}

/// 批量结果中的单项。
#[derive(Debug, Serialize)]
pub struct BatchItem {
    pub ip: String,
    pub ok: bool,
    pub region: Option<Region>,
    pub error: Option<String>,
}

/// POST /api/batch 的响应体。
#[derive(Debug, Serialize)]
pub struct BatchResponse {
    pub results: Vec<BatchItem>,
}

/// GET /api/health 的响应。
#[derive(Debug, Serialize)]
pub struct HealthResponse<'a> {
    pub status: &'a str,
    pub engine: &'a str,
    pub version: &'a str,
}
