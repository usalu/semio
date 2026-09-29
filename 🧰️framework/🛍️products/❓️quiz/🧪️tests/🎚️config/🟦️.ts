// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🧪️ Vitest for `@semio-tech/quiz`: the unit suites of the core modules (the Protocol v2 cases run through the repo test harness). */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/quiz": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    root,
    name: "@semio-tech/quiz",
    mode: "test",
    environment: "node",
    include: [
      "../../🧪️tests/🌀️mt19937-generator/🟦️.ts",
      "../../🧪️tests/🎴️sheet-randomization/🟦️.ts",
      "../../🧪️tests/🩺️document-validation/🟦️.ts",
      "../../🧪️tests/⚖️partial-credit-scoring/🟦️.ts",
      "../../🧪️tests/🎖️badge-awards/🟦️.ts",
      "../../🧪️tests/🔁️run-lifecycle/🟦️.ts",
      "../../🧪️tests/👁️read-views/🟦️.ts",
      "../../🧪️tests/🗃️shared-vectors/🟦️.ts",
      "../../🧪️tests/🫂️presence-roster/🟦️.ts",
      "../../🧪️tests/🗳️crowd-answers/🟦️.ts",
    ],
    coverage: { include: ["🟦️.ts", "../../🧬️schema/🟦️.ts", "../../🔨️modules/**/🟦️.ts"] },
    passWithNoTests: false,
  },
};
