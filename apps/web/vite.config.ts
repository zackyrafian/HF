import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "path";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  // server: {
  //   middlewareMode: true,
  // },
  server: {
    port: 3000,
  },
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
});
