// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root,
  resolve: {
    alias: {
    },
  },
  assetsInclude: ["**/*.wasm"],
  test: {
    name: "@semio-tech/s-3d-js",
    mode: "test",
    environment: "node",
    // 🩹️ In-source (`import.meta.vitest`) suite in `📦️index.ts` — `include` names ACTUAL TEST FILES,
    // and no file named literally "index.ts" exists here (the real file is `📦️index.ts`), so this was
    // silently collecting zero tests while `nx test` reported success. See the os-dev/replication
    // configs' note on why `include` must stay empty for an in-source suite.
    include: [],
    includeSource: ["📦️index.ts"],
    coverage: { include: ["📦️index.ts"] },
    passWithNoTests: false,
  },
});
