import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(root, "../../../../../../..");

/** 🧪️ Vitest for `@semio-tech/framework-os-shell`: the reducer and schema suites import their modules and register themselves. */
export default defineConfig({
  root: testRoot,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "os-shell"),
  resolve: {
    alias: {
      "@semio-tech/framework-os-shell": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-os-shell",
    environment: "node",
    include: ["../../🧪️tests/🧪️semio-tech-framework-os-shell-reduce/🟦️.ts", "../../🧬️schema/🧪️tests/🧪️os-shell-schema-module/🟦️.ts"],
    coverage: { include: ["../../🟦️.ts", "../../🧬️schema/🟦️.ts"] },
    passWithNoTests: false,
  },
});
