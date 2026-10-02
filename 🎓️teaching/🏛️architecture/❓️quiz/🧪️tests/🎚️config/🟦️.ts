import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(siteRoot, "../../..");

/** 🎚️ Vitest configuration of `@teaching/architecture-quiz`: node-only tests of the catalog, of the deployment, of the
 * local stack and of the host document. The browser specs beside them run in the end-to-end gate
 * (`../../🎭️e2e/🎚️config/🟦️.ts`). */
export default defineConfig({
  root: siteRoot,
  resolve: {
    alias: [
      { find: /^@semio-tech\/quiz$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/pets$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts") },
    ],
  },
  test: { root: siteRoot, name: "@teaching/architecture-quiz", environment: "node", include: ["🧪️tests/🧪️catalog/🟦️.ts", "🧪️tests/🧪️deploy/🟦️.ts", "🧪️tests/🧱️local-stack/🟦️.ts", "🧪️tests/📰️host-document/🟦️.ts", "🧪️tests/🐾️pet-cast/🟦️.ts"], passWithNoTests: false },
});
