<script setup lang="ts">
import { ElMessage, ElMessageBox } from "element-plus";
import { useThemeStore, type Theme } from "../stores/theme";
import { useHistoryStore } from "../stores/history";
import { useQueryStore } from "../stores/query";

const theme = useThemeStore();
const history = useHistoryStore();
const query = useQueryStore();

async function clearHistory() {
  try {
    await ElMessageBox.confirm("确认清空全部历史记录？此操作不可撤销。", "UMBRA", {
      type: "warning",
      confirmButtonText: "清空",
      cancelButtonText: "取消",
    });
  } catch {
    return; // 用户取消
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

    <h3>数据与引擎</h3>
    <p class="desc">
      离线数据源：ip2region <code>v4 / v6</code> xdb（上游 Apache-2.0 / MIT
      双许可）。所有查询在本地 Rust 内核（umbra-core）完成，不联网、不上传任何 IP。
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
  max-width: 680px;
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
}
.row {
  display: flex;
  gap: 12px;
}
</style>
