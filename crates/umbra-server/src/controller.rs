//! 控制层：axum handler，仅负责请求解析与响应封装，业务一律委派 [`IpGeoService`]。

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;

use crate::dto::{BatchRequest, BatchResponse, HealthResponse, LookupQuery, LookupResponse};
use crate::error::AppError;
use crate::service::IpGeoService;

/// GET /api/lookup?ip=<IPv4|IPv6>
pub async fn lookup(
    State(svc): State<Arc<IpGeoService>>,
    Query(q): Query<LookupQuery>,
) -> Result<Json<LookupResponse>, AppError> {
    svc.lookup(&q.ip).map(Json)
}

/// POST /api/batch  body: {"ips": ["8.8.8.8", ...]}
pub async fn batch(
    State(svc): State<Arc<IpGeoService>>,
    Json(req): Json<BatchRequest>,
) -> Json<BatchResponse> {
    Json(svc.batch(req))
}

/// GET /api/health
pub async fn health() -> Json<HealthResponse<'static>> {
    Json(IpGeoService::health())
}
