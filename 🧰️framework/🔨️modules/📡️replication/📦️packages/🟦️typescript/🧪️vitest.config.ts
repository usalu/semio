// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-replication` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-replication": resolve(root, "🟦️glue.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-replication",
    mode: "test",
    environment: "node",
    // 🩹️ In-source (`import.meta.vitest`) suites only — see the os package's note on why `include`
    // must stay empty when `includeSource` already lists the same file.
    include: [],
    coverage: { include: ["../../🟦️component.ts"] },
    includeSource: ["../../🟦️component.ts"],
    passWithNoTests: false,
  },
});
