// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const dir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(dir, "../../..");

/** @emoji 🧪️ Vitest for `os-hub-ts` — the hub-owned `🤝️integration` and `🤝️two-client-document` cases, gated behind
 * `HUB_E2E=1` (see their own docs), and the ungated `📝️trace-record` and `📌️document-check-in` and `🛡️access-policy` oracles. Aliases `@semio-tech/framework-os` to its real source
 * file, matching every other vite/vitest config in this repo. */
export default defineConfig({
  root: testRoot,
  resolve: {
    alias: [{ find: "@semio-tech/framework-os", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts") }],
  },
  test: {
    root: testRoot,
    name: "os-hub-ts",
    environment: "node",
    include: [resolve(dir, "../../🧪️tests/🤝️integration/🟦️.ts"), resolve(dir, "../../🧪️tests/🤝️two-client-document/🟦️.ts"), resolve(dir, "../../🧪️tests/📝️trace-record/🟦️.ts"), resolve(dir, "../../🧪️tests/📌️document-check-in/🟦️.ts"), resolve(dir, "../../🧪️tests/🛡️access-policy/🟦️.ts")],
    passWithNoTests: false,
  },
});
