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
  cacheDir: repoCacheDirectory(repoRoot, "vite", "framework-machine"),
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
