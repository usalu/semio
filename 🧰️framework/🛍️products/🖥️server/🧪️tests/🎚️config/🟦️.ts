// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** @emoji 🧪️ Vitest for `@semio-tech/framework-server` (inline `import.meta.vitest`). */
export default defineConfig({
  root: testRoot,
  resolve: {
    alias: {
      "@semio-tech/framework-server": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-server",
    mode: "test",
    environment: "node",
    include: [],
    coverage: { include: ["../../🟦️.ts"] },
    includeSource: ["../../🟦️.ts"],
    passWithNoTests: false,
  },
});
