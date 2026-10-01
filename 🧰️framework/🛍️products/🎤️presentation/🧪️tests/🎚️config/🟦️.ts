// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🧪️ Vitest for `@semio-tech/presentation`: the owner's canonical cases. */
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
    include: ["../🧭️slide-glob-assembly/🟦️.ts", "../📽️presentation-core/🟦️.ts"],
    coverage: { include: ["🟦️.ts"] },
    passWithNoTests: false,
  },
};
