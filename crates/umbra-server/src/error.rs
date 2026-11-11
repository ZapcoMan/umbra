//! 统一错误类型：把内核 [`UmbraError`] 归类为带 HTTP 语义的响应。

use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use umbra_core::UmbraError;

#[derive(Debug)]
pub enum AppError {
    /// 请求里的 IP 字面量非法
    InvalidIp(String),
    /// 请求合法但数据中查不到该 IP
    NotFound(String),
    /// 服务端内部错误（打开库失败、数据异常等）
    Internal(String),
}

impl AppError {
    pub fn internal(msg: impl Into<String>) -> Self {
        AppError::Internal(msg.into())
    }
}

impl From<UmbraError> for AppError {
    fn from(e: UmbraError) -> Self {
        match e {
            UmbraError::InvalidIp(ip) => AppError::InvalidIp(ip),
            UmbraError::Search(s) => AppError::NotFound(s),
            UmbraError::MalformedRegion(s) => AppError::internal(s),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidIp(ip) => write!(f, "非法 IP 地址: {ip}"),
            AppError::NotFound(s) => write!(f, "未找到: {s}"),
            AppError::Internal(s) => write!(f, "服务内部错误: {s}"),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, msg) = match &self {
            AppError::InvalidIp(ip) => (StatusCode::BAD_REQUEST, format!("非法 IP 地址: {ip}")),
            AppError::NotFound(s) => (StatusCode::UNPROCESSABLE_ENTITY, format!("未找到: {s}")),
            AppError::Internal(s) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("服务内部错误: {s}"))
            }
        };
        // 内部错误细节不外泄给公网，仅记录
        if matches!(self, AppError::Internal(_)) {
            tracing::error!("{msg}");
        }
        let body = json!({ "ok": false, "error": msg });
        (code, Json(body)).into_response()
    }
}
