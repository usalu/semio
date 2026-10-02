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
  cacheDir: testCacheDirectoryV1(process.env, "framework-machine"),
  resolve: {
    alias: {
    },
  },
  test: {
    root: testRoot,
    name: "@semio-tech/machine",
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
