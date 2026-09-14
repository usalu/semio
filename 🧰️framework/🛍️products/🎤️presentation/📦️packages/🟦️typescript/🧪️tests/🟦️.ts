// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji 🧪️ Vitest for `@semio-tech/presentation` (case suite plus inline `import.meta.vitest`). */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/presentation": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    name: "@semio-tech/presentation",
    mode: "test",
    environment: "node",
    include: ["🧪️tests/🧭️slide-glob-assembly/🟦️.ts"],
    coverage: { include: ["🟦️.ts"] },
    includeSource: ["🟦️.ts"],
    passWithNoTests: false,
  },
};
