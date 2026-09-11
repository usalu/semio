// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../..");

export default defineConfig({
  root,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "framework-3d"),
  resolve: {
    alias: {
    },
  },
  assetsInclude: ["**/*.wasm"],
  test: {
    name: "@semio-tech/s-3d-js",
    mode: "test",
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
