// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { testCacheDirectoryV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
// #endregion 🔌️Adapters

const configDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const root = resolve(configDir, "../.."); // 🎠️kernel module root — owner of 🟦️.ts

/** 🧪️ Runs canonical kernel cases and the kernel children's behaviour suites; the only in-source file left is the content return module. */
export default defineConfig({
  root: testRoot,
  cacheDir: testCacheDirectoryV1(process.env, "framework-kernel"),
  test: {
    root: testRoot,
    name: "@semio-tech/framework-kernel",
    environment: "jsdom",
    include: ["🧪️tests/🔬️scope-contributions/🟦️.ts", "🧪️tests/🏷️history-entry-label/🟦️.ts", "🧪️tests/🧪️history-patch/🟦️.ts", "🧪️tests/🧪️history-notices/🟦️.ts", "🧪️tests/🧪️framework-notices/🟦️.ts", "🧪️tests/🧪️history-reprojection/🟦️.ts", "🧪️tests/📦️package-assets/🟦️.ts", "🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts", "📤️return/📦️content/📥️input/🧪️tests/🔌️ui-input-host/🟦️.ts"],
    coverage: { include: ["*/🟦️.ts"] },
    includeSource: ["📤️return/📦️content/🟦️.ts"],
    passWithNoTests: false,
  },
});
