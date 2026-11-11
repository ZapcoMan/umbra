<script setup lang="ts">
import { useQueryStore } from "../stores/query";
import { useHistoryStore } from "../stores/history";
import { lookupOne, type LookupResult } from "../services/geo";

const query = useQueryStore();
const history = useHistoryStore();

function summary(r: LookupResult): string {
  if (!r.ok) return r.error ?? "失败";
  const g = r.region!;
  return [g.country, g.province, g.city, g.isp].filter(Boolean).join(" · ");
}

async function replay(r: LookupResult) {
  const res = await lookupOne(r.ip);
  query.setResults([res]);
  history.push(res);
}
</script>

<template>
  <div class="hist">
    <div class="hist-title">历史记录</div>
    <div v-if="history.items.length === 0" class="empty">暂无</div>
    <ul v-else>
      <li v-for="r in history.items" :key="r.ip" @click="replay(r)">
        <span class="ip">{{ r.ip }}</span>
        <span class="sum" :class="{ bad: !r.ok }">{{ summary(r) }}</span>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.hist {
  background: var(--umbra-panel);
  border: 1px solid var(--umbra-border);
  border-radius: 4px;
  padding: 10px 12px;
}
.hist-title {
  font-size: 13px;
  opacity: 0.7;
  margin-bottom: 6px;
}
.empty {
  font-size: 12px;
  opacity: 0.4;
}
ul {
  list-style: none;
  margin: 0;
  padding: 0;
  max-height: 160px;
  overflow: auto;
}
li {
  display: flex;
  gap: 12px;
  padding: 3px 0;
  cursor: pointer;
  font-size: 13px;
}
li:hover {
  color: var(--umbra-accent);
}
.ip {
  min-width: 200px;
}
.sum {
  opacity: 0.75;
}
.sum.bad {
  color: var(--el-color-danger);
}
</style>
