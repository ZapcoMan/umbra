<script setup lang="ts">
import { ref } from "vue";
import QueryBar from "./components/QueryBar.vue";
import ResultTable from "./components/ResultTable.vue";
import HistoryPanel from "./components/HistoryPanel.vue";
import SettingsView from "./views/SettingsView.vue";
import { API_BASE } from "./api/geo";

const view = ref<"query" | "settings">("query");
</script>

<template>
  <div class="web-shell">
    <header class="web-header">
      <div class="umbra-brand">◈ UMBRA</div>
      <nav class="tabs">
        <button :class="{ active: view === 'query' }" @click="view = 'query'">查询</button>
        <button :class="{ active: view === 'settings' }" @click="view = 'settings'">
          设置
        </button>
      </nav>
      <span class="be">后端 {{ API_BASE }}</span>
    </header>

    <main v-if="view === 'query'" class="web-main">
      <QueryBar />
      <ResultTable />
      <HistoryPanel />
    </main>
    <main v-else class="web-main">
      <SettingsView />
    </main>

    <footer class="web-foot">
      UMBRA · 离线 IP 地理定位 · 内核 ip2region(Rust) · 数据不出本机后端
    </footer>
  </div>
</template>

<style scoped>
.web-shell {
  display: flex;
  flex-direction: column;
  min-height: 100%;
  max-width: 1040px;
  margin: 0 auto;
}
.web-header {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 14px 18px;
  border-bottom: 1px solid var(--umbra-border);
}
.tabs {
  display: flex;
  gap: 4px;
}
.tabs button {
  border: 1px solid var(--umbra-border);
  background: transparent;
  color: var(--umbra-fg);
  padding: 5px 18px;
  cursor: pointer;
  border-radius: 3px;
}
.tabs button.active {
  background: var(--umbra-accent);
  color: var(--umbra-bg);
  border-color: var(--umbra-accent);
}
.be {
  margin-left: auto;
  font-size: 12px;
  opacity: 0.55;
}
.web-main {
  flex: 1;
  padding: 18px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.web-foot {
  padding: 12px 18px;
  font-size: 12px;
  opacity: 0.5;
  border-top: 1px solid var(--umbra-border);
}
</style>
