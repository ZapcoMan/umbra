// 网页端与后端的唯一边界：查询一律走 HTTP 到 umbra-server(Rust)，
// 前端不接触任何 .xdb 数据。
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

export const API_BASE: string =
  import.meta.env.VITE_API_BASE || "http://127.0.0.1:8080";

export async function lookupOne(ip: string): Promise<LookupResult> {
  try {
    const r = await fetch(`${API_BASE}/api/lookup?ip=${encodeURIComponent(ip)}`);
    const data = await r.json();
    if (data && data.ok) {
      return { ip, ok: true, region: data.region, error: null };
    }
    return { ip, ok: false, region: null, error: (data && data.error) || `HTTP ${r.status}` };
  } catch (e) {
    return { ip, ok: false, region: null, error: `请求失败(${API_BASE}): ${String(e)}` };
  }
}

export async function lookupBatch(ips: string[]): Promise<LookupResult[]> {
  try {
    const r = await fetch(`${API_BASE}/api/batch`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ ips }),
    });
    const data = await r.json();
    return data.results as LookupResult[];
  } catch (e) {
    return ips.map((ip) => ({
      ip,
      ok: false,
      region: null,
      error: `请求失败(${API_BASE}): ${String(e)}`,
    }));
  }
}

export async function pingHealth(): Promise<boolean> {
  try {
    const r = await fetch(`${API_BASE}/api/health`);
    return r.ok;
  } catch {
    return false;
  }
}
