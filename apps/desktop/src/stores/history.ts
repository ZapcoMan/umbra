import { defineStore } from "pinia";
import { ref, watch } from "vue";
import type { LookupResult } from "../services/geo";

const KEY = "umbra.history";
const MAX = 200;

function load(): LookupResult[] {
  try {
    return JSON.parse(localStorage.getItem(KEY) || "[]") as LookupResult[];
  } catch {
    return [];
  }
}

export const useHistoryStore = defineStore("history", () => {
  const items = ref<LookupResult[]>(load());

  watch(
    items,
    (v) => localStorage.setItem(KEY, JSON.stringify(v)),
    { deep: true },
  );

  function push(result: LookupResult) {
    items.value = [result, ...items.value.filter((x) => x.ip !== result.ip)].slice(
      0,
      MAX,
    );
  }

  function clear() {
    items.value = [];
  }

  return { items, push, clear };
});
