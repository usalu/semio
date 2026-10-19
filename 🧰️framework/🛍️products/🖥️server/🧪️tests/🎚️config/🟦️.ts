// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🧪️ Vitest for `@semio-tech/framework-server` (owned standalone wire tests). */
export default defineConfig({
  root: testRoot,
  resolve: {
    alias: [{ find: /^@semio-tech\/framework-server$/u, replacement: resolve(root, "🟦️.ts") }],
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-server",
    cache: false,
    environment: "node",
    include: ["../../🧪️tests/🔬️wire/🟦️.ts"],
    coverage: { include: ["../../🟦️.ts"] },
    passWithNoTests: false,
  },
});
