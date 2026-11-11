// 前端与 Tauri(Rust) 后端的唯一边界：所有数据查询都经 invoke 走到 umbra-core，
// 前端不直接读取或解析任何 .xdb 数据。
import { invoke } from "@tauri-apps/api/core";

export interface Region {
  country: string;
  province: string | null;
  city: string | null;
  isp: string | null;
  iso: string | null;
}

export interface LookupResult {
  ip: string;
  ok: boolean;
  region: Region | null;
  error: string | null;
}

/** 单 IP 查询：底层 invoke 失败时归一为结果对象，便于界面统一呈现。 */
export async function lookupOne(ip: string): Promise<LookupResult> {
  try {
    const region = await invoke<Region>("lookup", { ip });
    return { ip, ok: true, region, error: null };
  } catch (e) {
    return { ip, ok: false, region: null, error: String(e) };
  }
}

/** 批量查询：由 Rust 侧逐项处理，互不影响。 */
export async function lookupBatch(ips: string[]): Promise<LookupResult[]> {
  return invoke<LookupResult[]>("lookup_batch", { ips });
}
