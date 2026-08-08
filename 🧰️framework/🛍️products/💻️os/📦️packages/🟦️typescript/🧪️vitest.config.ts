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
    include: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts"],
    coverage: { include: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts"] },
    includeSource: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts"],
    passWithNoTests: false,
  },
});
