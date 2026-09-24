// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** @emoji 🧪️ Vitest for `@semio-tech/presentation`: the owner's canonical cases. */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/presentation": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    root,
    name: "@semio-tech/presentation",
    mode: "test",
    environment: "node",
    include: ["../../🧪️tests/🧭️slide-glob-assembly/🟦️.ts", "../../🧪️tests/📽️presentation-core/🟦️.ts"],
    coverage: { include: ["🟦️.ts"] },
    passWithNoTests: false,
  },
};
