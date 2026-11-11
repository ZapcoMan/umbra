import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// Tauri 期望前端固定端口、不自动打开浏览器、不清屏
export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: "127.0.0.1",
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2021",
    minify: "esbuild",
    sourcemap: false,
  },
});
