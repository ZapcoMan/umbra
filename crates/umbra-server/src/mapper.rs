//! 数据操作层（Mapper）：全服务唯一持有 ip2region 引擎、真正读取 .xdb 的地方。
//! 上层（service/controller）一律经此取数，不得绕过。

use std::path::PathBuf;
use std::sync::Arc;

use umbra_core::{Region, Umbra};

use crate::error::AppError;

pub struct IpGeoMapper {
    umbra: Arc<Umbra>,
}

impl IpGeoMapper {
    /// 从显式路径打开 v4/v6 两份 xdb。
    pub fn open(v4: impl Into<PathBuf>, v6: impl Into<PathBuf>) -> Result<Self, AppError> {
        let v4 = v4.into();
        let v6 = v6.into();
        let umbra = Umbra::open(
            v4.to_str().unwrap_or_default(),
            v6.to_str().unwrap_or_default(),
        )
        .map_err(|e| AppError::internal(format!("打开离线库失败: {e}")))?;
        Ok(Self {
            umbra: Arc::new(umbra),
        })
    }

    /// 使用内置 data 目录（开发/测试）。
    pub fn open_default() -> Result<Self, AppError> {
        let umbra = Umbra::open_default()
            .map_err(|e| AppError::internal(format!("打开内置离线库失败: {e}")))?;
        Ok(Self {
            umbra: Arc::new(umbra),
        })
    }

    /// 按内核优先级链定位数据目录：`UMBRA_DB_DIR` → 可执行文件同级 `data/` → 源码树 `data/`。
    /// 前两级供分发场景（解压即用），最后一级供开发/测试。
    pub fn open_from_env() -> Result<Self, AppError> {
        let umbra =
            Umbra::open_auto().map_err(|e| AppError::internal(format!("打开离线库失败: {e}")))?;
        Ok(Self {
            umbra: Arc::new(umbra),
        })
    }

    /// 查询单个 IP，返回结构化 region。
    pub fn query(&self, ip: &str) -> Result<Region, AppError> {
        self.umbra.lookup(ip).map_err(AppError::from)
    }
}
