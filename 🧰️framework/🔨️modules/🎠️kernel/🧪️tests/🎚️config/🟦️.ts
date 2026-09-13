// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
// #endregion 🔌️Adapters

const configDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const root = resolve(configDir, "../.."); // 🎠️kernel module root — owner of 🟦️.ts
const repoRoot = resolve(configDir, "../../../../..");

/** 🧪️ Runs canonical kernel cases and existing production-to-test registration bridges. */
export default defineConfig({
  root: testRoot,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "framework-kernel"),
  test: {
    root: testRoot,
    name: "@semio-tech/framework-kernel",
    mode: "test",
    environment: "jsdom",
    include: ["🧪️tests/🔬️scope-contributions/🟦️.ts"],
    coverage: { include: ["*.ts"] },
    includeSource: ["*.ts", "📤️return/📦️content/🟦️.ts"],
    passWithNoTests: false,
  },
});
