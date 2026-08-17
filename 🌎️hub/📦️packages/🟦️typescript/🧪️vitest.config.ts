// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../../..");

/** @emoji 🧪️ Vitest for `os-hub-ts` — the whole suite lives in `🧪️index.test.ts`, gated behind
 * `HUB_E2E=1` (see that file's own doc). Aliases `@semio-tech/framework-os` to its real source
 * file, matching every other vite/vitest config in this repo. */
export default defineConfig({
  root: dir,
  resolve: {
    alias: [{ find: "@semio-tech/framework-os", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts") }],
  },
  test: {
    name: "os-hub-ts",
    environment: "node",
    include: ["🧪️index.test.ts"],
    passWithNoTests: false,
  },
});
