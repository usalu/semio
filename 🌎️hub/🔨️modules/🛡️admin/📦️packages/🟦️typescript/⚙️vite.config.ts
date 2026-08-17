// #region 🔌️Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { semioEmojiIndexHtmlVitePlugin } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts";
// #endregion 🔌️Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../../../../..");

/** 🎯️ Contract §C0 "hub admin vite dev": `/directory`, `/admin/api`, `/auth`, `/spaces` all proxy to
 * the hub (`OS_HUB_URL`, else the loopback default `bin.rs`'s own dev default binds to), `ws: true`
 * so `/directory/ws` and `/spaces/{id}/documents/{id}/ws` tunnel through the dev server exactly like
 * the production `/admin` deployment (same-origin, no CORS). */
const hubProxyTarget = process.env.OS_HUB_URL ?? "http://127.0.0.1:8787";
const hubProxy = { target: hubProxyTarget, changeOrigin: true, ws: true };

export default defineConfig({
  root: dir,
  base: "/admin/",
  plugins: [semioEmojiIndexHtmlVitePlugin(dir), react(), tailwindcss()],
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx") },
      { find: "@semio-tech/framework-os", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts") },
    ],
  },
  server: {
    port: Number(process.env.OS_HUB_ADMIN_DEV_PORT ?? 8790),
    strictPort: true,
    fs: { allow: [repoRoot] },
    proxy: {
      "/directory": hubProxy,
      "/admin/api": hubProxy,
      "/auth": hubProxy,
      "/spaces": hubProxy,
    },
  },
  build: {
    outDir: "📤️dist",
    emptyOutDir: true,
  },
});
