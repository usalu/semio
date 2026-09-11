import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

/** 🧪️ Module-local vitest project for `framework.schema`: it runs the third-party draft-07 oracle
 * spec at its taxonomy location `🧪️tests/✅️draft07-oracle/🟦️.ts`, which vitest's default
 * `**‍/*.{test,spec}.?(c|m)[jt]s?(x)` include glob does not match — without an explicit `include`
 * vitest exits 1 with "No test files found", which a shell ignoring the exit code reads as green.
 * Root `📜️script.ts` `schema test` runs it with `bunx vitest run --config <this file>`.
 * @see https://vitest.dev/config/#include */
const moduleRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(moduleRoot, "../../..");

export default defineConfig({
  cacheDir: repoCacheDirectory(repoRoot, "vite", "framework-schema"),
  test: {
    root: moduleRoot,
    include: ["🧪️tests/**/🟦️.ts"],
    passWithNoTests: false,
  },
});
