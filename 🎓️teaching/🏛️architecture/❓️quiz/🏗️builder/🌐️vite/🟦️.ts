// #region 🔌️Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { playgroundStaticSiteBuildOptions, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, semioReferencedAssetsVitePlugin, semioServeCloseVitePlugin, semioViteProductionBuild } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
// #endregion 🔌️Adapters

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const bundleRoot = resolve(siteRoot, "📦️packages/🟦️typescript");
const repoRoot = resolve(siteRoot, "../../..");
const proctor = `http://127.0.0.1:${process.env.PROCTOR_PORT ?? "8791"}`;
const gatewayRoutes = ["/instance", "/commands", "/queries", "/actors", "/scopes"];

/** ❓️ Vite configuration of `@teaching/architecture-quiz`: the quiz website of `quizze.architektur-und-technologie.de`.
 *
 * The site is served from the domain root by the proctor (SPA fallback to `index.html`), so assets resolve from `/`; the
 * build lands in the package's `dist`. In dev the proctor gateway routes are proxied to the dev proctor on `PROCTOR_PORT`
 * (default 8791), WebSocket event streams included.
 * @see ../../🚀️deploy/Dockerfile — the production image that serves this build */
export default defineConfig({
  root: siteRoot,
  base: "/",
  publicDir: false,
  plugins: [
    semioServeCloseVitePlugin(),
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "Quizze · Architektur und Technologie",
      entry: "./🟦️.ts",
      loading: { title: "Quizze · Architektur und Technologie" },
      cnameHost: "quizze.architektur-und-technologie.de",
    }),
    semioEmojiIndexHtmlVitePlugin(siteRoot),
    ...semioReferencedAssetsVitePlugin(repoRoot),
    tailwindcss(),
    react(),
  ],
  build: playgroundStaticSiteBuildOptions({ ...semioViteProductionBuild(), outDir: resolve(bundleRoot, "dist") }),
  define: { "import.meta.vitest": "undefined" },
  server: {
    fs: { allow: [repoRoot] },
    proxy: Object.fromEntries(gatewayRoutes.map((route) => [route, { target: proctor, ws: true }])),
  },
  resolve: {
    alias: [
      { find: /^@semio-tech\/quiz-react$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/quiz$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/ui-react$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/ui-react\/i18n$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts") },
      { find: /^@semio-tech\/framework-server$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/framework$/, replacement: resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
    ],
    dedupe: ["react", "react-dom"],
  },
});
