// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework` (inline `import.meta.vitest`). */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/framework": resolve(root, "🟦️.ts"),
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
    coverage: { include: ["🟦️.ts"] },
    includeSource: ["🟦️.ts"],
    passWithNoTests: false,
  },
};
