import { defineStore } from "pinia";
import { ref, watch } from "vue";

export type Theme = "light" | "dark" | "geek";

const KEY = "umbra.theme";

function applyTheme(t: Theme) {
  const el = document.documentElement;
  el.dataset.theme = t;
  if (t === "dark" || t === "geek") {
    el.classList.add("dark");
  } else {
    el.classList.remove("dark");
  }
}

export const useThemeStore = defineStore("theme", () => {
  const stored = (localStorage.getItem(KEY) as Theme) || null;
  const theme = ref<Theme>(stored ?? "geek");

  watch(
    theme,
    (t) => {
      applyTheme(t);
      localStorage.setItem(KEY, t);
    },
    { immediate: true },
  );

  function setTheme(t: Theme) {
    theme.value = t;
  }

  return { theme, setTheme };
});
