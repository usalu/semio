// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** 🧪️ Runs the canonical fixed-slot and continuation scheduler test implementations. */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-async": resolve(root, "../../🟦️.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-async",
    mode: "test",
    environment: "node",
    include: ["../../🧪️tests/🧱️boxed-fixed-slots/🟦️.ts", "../../🪃️continuation/🧪️tests/🪃️scheduler/🟦️.ts"],
    coverage: { include: ["../../🟦️.ts", "../../🪃️continuation/🟦️.ts"] },
    passWithNoTests: false,
  },
};
