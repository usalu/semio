// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/ui-styling` (inline `import.meta.vitest` in `🟦️vite-elements-assets.ts`:
 * `tileProxyVitePlugin`/`staticDirVitePlugin`/`meshCollectionVitePlugin`/`playgroundAssetVitePlugins`,
 * every other in-source `describe` block in that file, and `script.ts`'s 🌓️Levels generator tests). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/ui-styling": resolve(root, "js/index.ts"),
    },
  },
  test: {
    name: "@semio-tech/ui-styling",
    mode: "test",
    environment: "node",
    include: ["🟦️vite-elements-assets.ts", "📜️script.ts"],
    coverage: { include: ["🟦️vite-elements-assets.ts", "📜️script.ts"] },
    includeSource: ["🟦️vite-elements-assets.ts", "📜️script.ts"],
    passWithNoTests: false,
  },
});
