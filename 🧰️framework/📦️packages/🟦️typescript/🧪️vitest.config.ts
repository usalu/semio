// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework": resolve(root, "🟦️glue.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework",
    mode: "test",
    environment: "node",
    // 🩹️ `include` MUST stay empty: this is an in-source (`import.meta.vitest`) suite collected via
    // `includeSource`. Listing the same file in BOTH keys made vitest collect it twice and report
    // double the real test count. Add new in-source files to `includeSource`/`coverage.include` only.
    include: [],
    coverage: { include: ["🟦️glue.ts"] },
    includeSource: ["🟦️glue.ts"],
    passWithNoTests: false,
  },
});
