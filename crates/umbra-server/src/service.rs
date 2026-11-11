//! 服务层：业务编排。负责校验、批量处理与 DTO 组装；
//! 不直接触碰 ip2region，所有取数都通过 [`IpGeoMapper`]。

use umbra_core::Region;

use crate::dto::{BatchItem, BatchRequest, BatchResponse, HealthResponse, LookupResponse};
use crate::error::AppError;
use crate::mapper::IpGeoMapper;

pub struct IpGeoService {
    mapper: IpGeoMapper,
}

impl IpGeoService {
    pub fn new(mapper: IpGeoMapper) -> Self {
        Self { mapper }
    }

    /// 单 IP 查询。非法 IP 等错误经 `?` 冒泡为 [`AppError`]，由控制层转成 HTTP 状态码。
    pub fn lookup(&self, ip: &str) -> Result<LookupResponse, AppError> {
        let region: Region = self.mapper.query(ip)?;
        Ok(LookupResponse {
            ok: true,
            ip: ip.trim().to_string(),
            region: Some(region),
        })
    }

    /// 批量查询：逐项独立成功/失败，绝不因单条非法中断整批。
    pub fn batch(&self, req: BatchRequest) -> BatchResponse {
        let results = req
            .ips
            .iter()
            .map(|ip| match self.mapper.query(ip) {
                Ok(region) => BatchItem {
                    ip: ip.trim().to_string(),
                    ok: true,
                    region: Some(region),
                    error: None,
                },
                Err(e) => BatchItem {
                    ip: ip.trim().to_string(),
                    ok: false,
                    region: None,
                    error: Some(e.to_string()),
                },
            })
            .collect();
        BatchResponse { results }
    }

    /// 健康检查。
    pub fn health() -> HealthResponse<'static> {
        HealthResponse {
            status: "ok",
            engine: "ip2region",
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
