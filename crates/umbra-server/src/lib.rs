//! UMBRA Web 后端库入口：装配三层并暴露路由。
//!
//! 分层（仿 Spring Boot）：
//! - [`controller`]：控制层，axum handler 与路由
//! - [`service`]：服务层，业务编排与 DTO 组装
//! - [`mapper`]：数据操作层，唯一持有 ip2region 引擎访问 xdb
//! - [`dto`] / [`error`]：数据契约与统一错误

pub mod controller;
pub mod dto;
pub mod error;
pub mod mapper;
pub mod service;

use std::sync::Arc;

use axum::{routing::get, routing::post, Router};
use tower_http::cors::CorsLayer;

use controller::{batch, health, lookup};
use service::IpGeoService;

/// 组装路由。State 为线程共享的只读服务实例。
pub fn build_router(state: Arc<IpGeoService>) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/lookup", get(lookup))
        .route("/api/batch", post(batch))
        // 开发期放开跨域；生产按前端实际来源收紧
        .layer(CorsLayer::permissive())
        .with_state(state)
}
