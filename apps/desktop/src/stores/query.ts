import { defineStore } from "pinia";
import { ref } from "vue";
import type { LookupResult } from "../services/geo";

export const useQueryStore = defineStore("query", () => {
  const results = ref<LookupResult[]>([]);
  const loading = ref(false);

  function setResults(r: LookupResult[]) {
    results.value = r;
  }

  function clear() {
    results.value = [];
  }

  return { results, loading, setResults, clear };
});
