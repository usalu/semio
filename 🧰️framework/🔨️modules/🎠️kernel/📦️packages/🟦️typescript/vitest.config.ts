// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
// #endregion 🔌️Adapters

const configDir = dirname(fileURLToPath(import.meta.url));
const root = resolve(configDir, "../.."); // 🎠️kernel module root — owner of 🟦️.ts
const repoRoot = resolve(configDir, "../../../../..");

/** 🧪️ Runs canonical kernel cases and existing production-to-test registration bridges. */
export default defineConfig({
  root,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "framework-kernel"),
  test: {
    name: "@semio-tech/framework-kernel",
    mode: "test",
    environment: "jsdom",
    include: ["🧪️tests/🔬️scope-contributions/🟦️.ts"],
    coverage: { include: ["*.ts"] },
    includeSource: ["*.ts", "📤️return/📦️content/🟦️.ts"],
    passWithNoTests: false,
  },
});
