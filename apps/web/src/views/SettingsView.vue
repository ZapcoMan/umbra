<script setup lang="ts">
import { ref } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import { useThemeStore, type Theme } from "../stores/theme";
import { useHistoryStore } from "../stores/history";
import { useQueryStore } from "../stores/query";
import { API_BASE, pingHealth } from "../api/geo";

const theme = useThemeStore();
const history = useHistoryStore();
const query = useQueryStore();
const checking = ref(false);

async function testConn() {
  checking.value = true;
  const ok = await pingHealth();
  checking.value = false;
  if (ok) ElMessage.success(`后端在线: ${API_BASE}`);
  else ElMessage.error(`无法连接后端: ${API_BASE}`);
}

async function clearHistory() {
  try {
    await ElMessageBox.confirm("确认清空全部历史记录？此操作不可撤销。", "UMBRA", {
      type: "warning",
      confirmButtonText: "清空",
      cancelButtonText: "取消",
    });
  } catch {
    return;
  }
  history.clear();
  ElMessage.success("已清空历史记录");
}

function clearResults() {
  query.clear();
  ElMessage.success("已清空当前结果");
}
</script>

<template>
  <div class="settings">
    <h3>外观主题</h3>
    <el-radio-group
      :model-value="theme.theme"
      @update:model-value="(v: string) => theme.setTheme(v as Theme)"
    >
      <el-radio-button value="light">亮色</el-radio-button>
      <el-radio-button value="dark">暗色</el-radio-button>
      <el-radio-button value="geek">极客</el-radio-button>
    </el-radio-group>

    <h3>后端连接</h3>
    <div class="row">
      <code class="api">{{ API_BASE }}</code>
      <el-button size="small" :loading="checking" @click="testConn">测试连接</el-button>
    </div>
    <p class="desc">
      网页端不直接读取数据：所有查询经 HTTP 发往 umbra-server，由其 Rust 内核(umbra-core)
      在本机离线解析 ip2region xdb。可通过构建时环境变量 <code>VITE_API_BASE</code> 指定后端地址。
    </p>

    <h3>数据管理</h3>
    <div class="row">
      <el-button @click="clearResults">清空当前结果</el-button>
      <el-button type="danger" plain @click="clearHistory">清空历史记录</el-button>
    </div>
  </div>
</template>

<style scoped>
.settings {
  max-width: 720px;
}
h3 {
  font-size: 14px;
  margin: 18px 0 10px;
  color: var(--umbra-accent);
}
.desc {
  font-size: 13px;
  line-height: 1.7;
  opacity: 0.8;
  margin-top: 10px;
}
.row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.api {
  padding: 2px 8px;
  border: 1px solid var(--umbra-border);
  border-radius: 3px;
  font-size: 12px;
}
</style>
