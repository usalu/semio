// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { testCacheDirectoryV1 } from "../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
// #endregion 🔌️Adapters

const configDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const root = resolve(configDir, "../.."); // 🎠️kernel module root — owner of 🟦️.ts

/** 🧪️ Runs canonical kernel cases and existing production-to-test registration bridges. */
export default defineConfig({
  root: testRoot,
  cacheDir: testCacheDirectoryV1(process.env, "framework-kernel"),
  test: {
    root: testRoot,
    name: "@semio-tech/framework-kernel",
    environment: "jsdom",
    include: ["🧪️tests/🔬️scope-contributions/🟦️.ts", "🧪️tests/🏷️history-entry-label/🟦️.ts", "🧪️tests/🧪️history-patch/🟦️.ts", "🧪️tests/🧪️history-notices/🟦️.ts"],
    coverage: { include: ["*.ts"] },
    includeSource: ["*.ts", "📤️return/📦️content/🟦️.ts"],
    passWithNoTests: false,
  },
});
