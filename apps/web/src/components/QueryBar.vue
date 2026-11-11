<script setup lang="ts">
import { ref } from "vue";
import { ElMessage } from "element-plus";
import { lookupOne, lookupBatch } from "../api/geo";
import { useQueryStore } from "../stores/query";
import { useHistoryStore } from "../stores/history";

const query = useQueryStore();
const history = useHistoryStore();
const text = ref("");

function parseIps(): string[] {
  return Array.from(
    new Set(text.value.split(/[\s,;]+/).map((s) => s.trim()).filter(Boolean)),
  );
}

async function run() {
  const ips = parseIps();
  if (ips.length === 0) {
    ElMessage.warning("请输入至少一个 IP");
    return;
  }
  query.loading = true;
  try {
    const results =
      ips.length === 1 ? [await lookupOne(ips[0])] : await lookupBatch(ips);
    query.setResults(results);
    results.forEach((r) => history.push(r));
  } finally {
    query.loading = false;
  }
}
</script>

<template>
  <div class="qbar">
    <el-input
      v-model="text"
      type="textarea"
      :rows="3"
      resize="none"
      placeholder="输入 IP，支持 IPv4 / IPv6；多个用空格、逗号或换行分隔，可粘贴批量"
      @keydown.ctrl.enter="run"
    />
    <div class="qbar-actions">
      <span class="hint">Ctrl+Enter 查询</span>
      <el-button type="primary" :loading="query.loading" @click="run">查询</el-button>
    </div>
  </div>
</template>

<style scoped>
.qbar {
  background: var(--umbra-panel);
  border: 1px solid var(--umbra-border);
  border-radius: 4px;
  padding: 12px;
}
.qbar-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 8px;
}
.hint {
  font-size: 12px;
  opacity: 0.5;
}
</style>
