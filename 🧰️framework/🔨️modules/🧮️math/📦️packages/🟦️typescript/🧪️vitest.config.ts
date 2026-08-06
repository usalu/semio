// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));
const sources = ["../../🕸️graph/🗣️dsl/🟦️component.ts"];

/** @emoji 🧪️ Vitest for `@semio-tech/framework-math-js` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-math": resolve(root, "📦️index.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-math-js",
    mode: "test",
    environment: "node",
    include: sources,
    coverage: { include: sources },
    includeSource: sources,
    passWithNoTests: false,
  },
});
