// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { testCacheDirectoryV1 } from "../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

export default defineConfig({
  root: testRoot,
  cacheDir: testCacheDirectoryV1(process.env, "framework-3d"),
  resolve: {
    alias: {
    },
  },
  assetsInclude: ["**/*.wasm"],
  test: {
    root: testRoot,
    name: "@semio-tech/s-3d-js",
    environment: "node",
    // 🩹️ In-source (`import.meta.vitest`) suite in `../../🟦️.ts` — `include` names ACTUAL TEST FILES,
    // and no file named literally "index.ts" exists here (the real file is `../../🟦️.ts`), so this was
    // silently collecting zero tests while `nx test` reported success. See the os-dev/replication
    // configs' note on why `include` must stay empty for an in-source suite.
    include: [],
    includeSource: ["../../🟦️.ts"],
    coverage: { include: ["../../🟦️.ts"] },
    passWithNoTests: false,
  },
});
