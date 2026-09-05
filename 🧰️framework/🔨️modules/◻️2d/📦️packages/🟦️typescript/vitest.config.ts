// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/flow-core": resolve(root, "../../../🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.js"),
    },
  },
  test: {
    name: "@semio-tech/s-2d-js",
    mode: "test",
    environment: "node",
    // 🩹️ In-source (`import.meta.vitest`) suite in `../../🟦️.ts` — `include` names ACTUAL TEST FILES,
    // and no file named literally "index.ts" exists here (the real file is `../../🟦️.ts`), so this was
    // silently collecting zero tests while `nx test` reported success. See the os-dev/replication
    // configs' note on why `include` must stay empty for an in-source suite.
    include: [],
    includeSource: ["../../🟦️.ts"],
    coverage: { include: ["../../🟦️.ts"] },
    passWithNoTests: false,
  },
};
