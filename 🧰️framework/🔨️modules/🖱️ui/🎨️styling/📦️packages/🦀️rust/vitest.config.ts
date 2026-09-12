// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** @emoji 🧪️ Vitest for the styling projection's inline `import.meta.vitest` contract. */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/ui-styling": resolve(root, "📦️packages/🟦️typescript/🟦️.ts"),
    },
  },
  test: {
    name: "@semio-tech/ui-styling",
    mode: "test",
    environment: "node",
    include: ["📽️projection/🟦️.ts"],
    coverage: { include: ["📽️projection/🟦️.ts"] },
    includeSource: ["📽️projection/🟦️.ts"],
    passWithNoTests: false,
  },
};
