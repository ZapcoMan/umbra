<script setup lang="ts">
import { useThemeStore, type Theme } from "../stores/theme";

const store = useThemeStore();
</script>

<template>
  <el-select
    :model-value="store.theme"
    size="small"
    style="width: 120px"
    @update:model-value="(v: string) => store.setTheme(v as Theme)"
  >
    <el-option label="极客" value="geek" />
    <el-option label="暗色" value="dark" />
    <el-option label="亮色" value="light" />
  </el-select>
</template>
