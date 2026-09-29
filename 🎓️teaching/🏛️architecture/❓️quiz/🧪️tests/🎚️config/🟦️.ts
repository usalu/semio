import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(siteRoot, "../../..");

/** 🎚️ Vitest configuration of `@teaching/architecture-quiz`: node-only content tests over the site's catalog. */
export default defineConfig({
  root: siteRoot,
  resolve: { alias: [{ find: /^@semio-tech\/quiz$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts") }] },
  test: { root: siteRoot, name: "@teaching/architecture-quiz", environment: "node", include: ["🧪️tests/🧪️catalog/🟦️.ts"], passWithNoTests: false },
});
