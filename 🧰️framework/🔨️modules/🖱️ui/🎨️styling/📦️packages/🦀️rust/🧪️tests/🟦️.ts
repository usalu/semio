// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji 🧪️ Vitest for `@semio-tech/ui-styling` (inline `import.meta.vitest` in `🟦️.ts`:
 * `tileProxyVitePlugin`/`staticDirVitePlugin`/`meshCollectionVitePlugin`/`playgroundAssetVitePlugins`,
 * every other in-source `describe` block in that file, and `script.ts`'s 🌓️Levels generator tests). */
export default {
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
    include: ["🟦️.ts", "📜️script.ts"],
    coverage: { include: ["🟦️.ts", "📜️script.ts"] },
    includeSource: ["🟦️.ts", "📜️script.ts"],
    passWithNoTests: false,
  },
};
