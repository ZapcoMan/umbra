<script setup lang="ts">
import { useQueryStore } from "../stores/query";
import type { LookupResult } from "../services/geo";

const query = useQueryStore();

function cell(r: LookupResult, key: keyof NonNullable<LookupResult["region"]>) {
  return r.region?.[key] ?? "—";
}
</script>

<template>
  <div class="rt">
    <el-table
      v-loading="query.loading"
      :data="query.results"
      size="small"
      height="320"
      empty-text="尚无结果，输入 IP 后查询"
      border
    >
      <el-table-column prop="ip" label="IP" min-width="200" />
      <el-table-column label="状态" width="70">
        <template #default="{ row }">
          <el-tag :type="row.ok ? 'success' : 'danger'" size="small" effect="dark">
            {{ row.ok ? "命中" : "失败" }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="国家" min-width="120">
        <template #default="{ row }">{{ cell(row, "country") }}</template>
      </el-table-column>
      <el-table-column label="省份" min-width="110">
        <template #default="{ row }">{{ cell(row, "province") }}</template>
      </el-table-column>
      <el-table-column label="城市" min-width="110">
        <template #default="{ row }">{{ cell(row, "city") }}</template>
      </el-table-column>
      <el-table-column label="ISP" min-width="120">
        <template #default="{ row }">{{ cell(row, "isp") }}</template>
      </el-table-column>
      <el-table-column label="ISO" width="70">
        <template #default="{ row }">{{ cell(row, "iso") }}</template>
      </el-table-column>
      <el-table-column label="备注 / 错误" min-width="180">
        <template #default="{ row }">
          <span v-if="!row.ok" class="err">{{ row.error }}</span>
          <span v-else class="dim">—</span>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<style scoped>
.rt {
  background: var(--umbra-panel);
  border: 1px solid var(--umbra-border);
  border-radius: 4px;
  padding: 8px;
}
.err {
  color: var(--el-color-danger);
}
.dim {
  opacity: 0.4;
}
</style>
