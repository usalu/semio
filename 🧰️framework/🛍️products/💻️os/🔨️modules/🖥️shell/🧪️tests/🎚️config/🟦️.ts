import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/⚡️caching/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(root, "../../../../../../..");

/** 🧪️ Vitest for `@semio-tech/framework-os-shell`: the reducer and schema suites import their modules and register themselves. */
export default defineConfig({
  root: testRoot,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "os-shell"),
  resolve: {
    alias: [{ find: /^@semio-tech\/framework-os-shell$/u, replacement: resolve(root, "🟦️.ts") }],
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-os-shell",
    environment: "node",
    include: ["../../🧪️tests/🧪️semio-tech-framework-os-shell-reduce/🟦️.ts", "../../🧪️tests/🔬️rust-package-commands/🧪️tests/🟦️.ts", "../../🧬️schema/🧪️tests/🧪️os-shell-schema-module/🟦️.ts"],
    coverage: { include: ["../../../../../../🔨️modules/🐚️shell/🟦️.ts", "../../🧬️schema/🟦️.ts"] },
    passWithNoTests: false,
  },
});
