// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🧪️ Runs the canonical fixed-slot and continuation scheduler test implementations. */
export default {
  root: testRoot,
  resolve: {
    alias: {
      "@semio-tech/framework-async": resolve(root, "../../🟦️.ts"),
    },
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-async",
    mode: "test",
    environment: "node",
    include: ["../../🧪️tests/🧱️boxed-fixed-slots/🟦️.ts", "../../🪃️continuation/🧪️tests/🪃️scheduler/🟦️.ts"],
    coverage: { include: ["../../🟦️.ts", "../../🪃️continuation/🟦️.ts"] },
    passWithNoTests: false,
  },
};
