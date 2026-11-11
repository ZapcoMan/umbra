<script setup lang="ts">
import { ref } from "vue";
import QueryBar from "./components/QueryBar.vue";
import ResultTable from "./components/ResultTable.vue";
import HistoryPanel from "./components/HistoryPanel.vue";
import SettingsView from "./views/SettingsView.vue";

const view = ref<"query" | "settings">("query");
</script>

<template>
  <div class="umbra-shell">
    <header class="umbra-header">
      <div class="umbra-brand">◈ UMBRA</div>
      <nav class="umbra-tabs">
        <button :class="{ active: view === 'query' }" @click="view = 'query'">查询</button>
        <button
          :class="{ active: view === 'settings' }"
          @click="view = 'settings'"
        >
          设置
        </button>
      </nav>
      <span class="umbra-sub">离线 IP 地理定位 · Rust 内核</span>
    </header>

    <section v-if="view === 'query'" class="umbra-main">
      <QueryBar />
      <ResultTable />
      <HistoryPanel />
    </section>
    <section v-else class="umbra-main">
      <SettingsView />
    </section>
  </div>
</template>

<style scoped>
.umbra-shell {
  display: flex;
  flex-direction: column;
  height: 100%;
}
.umbra-header {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 10px 18px;
  border-bottom: 1px solid var(--umbra-border);
  background: var(--umbra-panel);
}
.umbra-tabs {
  display: flex;
  gap: 4px;
}
.umbra-tabs button {
  border: 1px solid var(--umbra-border);
  background: transparent;
  color: var(--umbra-fg);
  padding: 4px 16px;
  cursor: pointer;
  border-radius: 3px;
  font-size: 14px;
}
.umbra-tabs button.active {
  background: var(--umbra-accent);
  color: var(--umbra-bg);
  border-color: var(--umbra-accent);
}
.umbra-sub {
  margin-left: auto;
  font-size: 12px;
  opacity: 0.6;
}
.umbra-main {
  flex: 1;
  overflow: auto;
  padding: 16px 18px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
</style>
