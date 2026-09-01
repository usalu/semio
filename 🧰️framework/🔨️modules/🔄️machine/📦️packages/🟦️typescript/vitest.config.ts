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
  test: {
    name: "@semio-tech/machine",
    mode: "test",
    environment: "node",
    // 🩹️ In-source (`import.meta.vitest`) suite in `../../🟦️.ts` — see the 3d module's vitest
    // config for why `include` must stay empty for an in-source suite named something other than
    // literally "index.ts".
    include: [],
    includeSource: ["../../🟦️.ts"],
    coverage: { include: ["../../🟦️.ts"] },
    passWithNoTests: false,
  },
});
