// #region 🔌Adapters
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/repo-coordinator` (Next.js API routes; no in-source tests yet). */
export default defineConfig({
  root,
  test: {
    name: "@semio-tech/repo-coordinator",
    mode: "test",
    environment: "node",
    passWithNoTests: true,
    coverage: { include: ["app/**/*.ts", "app/**/*.tsx"] },
  },
});
