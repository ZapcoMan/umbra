import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  server: {
    host: "127.0.0.1",
    port: 5173,
  },
  envPrefix: ["VITE_"],
  build: {
    target: "es2021",
    minify: "esbuild",
  },
});
