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
      "../🌀️mt19937-generator/🟦️.ts",
      "../🎴️sheet-randomization/🟦️.ts",
      "../🩺️document-validation/🟦️.ts",
      "../⚖️partial-credit-scoring/🟦️.ts",
      "../🎖️badge-awards/🟦️.ts",
      "../🔁️run-lifecycle/🟦️.ts",
      "../👁️read-views/🟦️.ts",
      "../🗃️shared-vectors/🟦️.ts",
      "../🫂️presence-roster/🟦️.ts",
      "../🗳️crowd-answers/🟦️.ts",
    ],
    coverage: { include: ["🟦️.ts", "../../🧬️schema/🟦️.ts", "../../🔨️modules/**/🟦️.ts"] },
    passWithNoTests: false,
  },
};
