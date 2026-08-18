// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-server` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-server": resolve(root, "🟦️glue.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-server",
    mode: "test",
    environment: "node",
    include: [],
    coverage: { include: ["../../🟦️component.ts"] },
    includeSource: ["../../🟦️component.ts"],
    passWithNoTests: true,
  },
});
