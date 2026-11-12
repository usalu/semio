// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { testCacheDirectoryV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

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
    name: "@semio-tech/framework-3d-js",
    environment: "node",
    include: ["../../🗿️model/🧪️tests/*/🟦️.ts", "../../🗿️model/*/🧪️tests/*/🟦️.ts", "../../🗿️model/*/*/🧪️tests/*/🟦️.ts"],
    exclude: ["../../**/🧰️support/**", "**/node_modules/**"],
    includeSource: ["../../🟦️.ts"],
    coverage: { include: ["../../🟦️.ts"] },
    passWithNoTests: false,
  },
});
