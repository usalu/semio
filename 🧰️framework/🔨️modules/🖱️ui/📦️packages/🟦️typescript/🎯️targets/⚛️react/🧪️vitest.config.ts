// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/ui-react` and its owned React modules. */
export default defineConfig({
  root,
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(root, "📦️index.tsx") }],
  },
  test: {
    name: "@semio-tech/ui-react",
    environment: "jsdom",
    include: ["../../../../🔨️modules/⌨️control-keybinding-context/🧪️component.test.tsx", "../../../../🔨️modules/🏷️style-variants/🧪️component.test.ts"],
    includeSource: ["📦️index.tsx"],
    coverage: { include: ["📦️index.tsx"] },
    passWithNoTests: true,
    setupFiles: [resolve(root, "🟦️vitest.setup.ts")],
  },
});
