// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-os` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-os": resolve(root, "🟦️glue.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-os",
    mode: "test",
    environment: "node",
    // 🩹️ `include` MUST stay empty: these are in-source (`import.meta.vitest`) suites collected via
    // `includeSource`. Listing the same files in BOTH keys made vitest collect each twice and report
    // double the real test count. Add new in-source files to `includeSource`/`coverage.include` only.
    include: [],
    coverage: { include: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts", "../../🟦️effect-backbone.ts"] },
    includeSource: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts", "../../🟦️effect-backbone.ts"],
    passWithNoTests: false,
  },
});
