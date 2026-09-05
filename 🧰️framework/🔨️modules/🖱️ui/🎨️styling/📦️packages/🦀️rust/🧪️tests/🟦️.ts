// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

/** @emoji 🧪️ Vitest for `@semio-tech/ui-styling` (inline `import.meta.vitest` in `🟦️.ts`:
 * `tileProxyVitePlugin`/`staticDirVitePlugin`/`meshCollectionVitePlugin`/`playgroundAssetVitePlugins`,
 * every other in-source `describe` block in that file, and `script.ts`'s 🌓️Levels generator tests). */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/ui-styling": resolve(root, "📦️packages/🟦️typescript/🟦️.ts"),
    },
  },
  test: {
    name: "@semio-tech/ui-styling",
    mode: "test",
    environment: "node",
    include: ["🟦️.ts", "📦️packages/🦀️rust/📜️script.ts"],
    coverage: { include: ["🟦️.ts", "📦️packages/🦀️rust/📜️script.ts"] },
    includeSource: ["🟦️.ts", "📦️packages/🦀️rust/📜️script.ts"],
    passWithNoTests: false,
  },
};
