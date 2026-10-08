// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

export default {
  root: testRoot,
  resolve: {
    alias: {},
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-2d-js",
    environment: "node",
    // 🧪️ The scene suite is in-source; line-layout fixtures have a dedicated test module.
    include: ["../../📝️text/🧪️tests/🔬️unit/🟦️.ts", "../../📝️text/🧪️tests/⚓️alignment/🟦️.ts", "../../🛤️path/📏️flatten/🧪️tests/🟦️.ts", "../../🛤️path/🖊️stroke/🧪️tests/🟦️.ts", "../../🔍️trace/🧪️tests/🟦️.ts", "../../🔀️booleans/🧪️tests/🟦️.ts", "../../🔀️booleans/🛤️paths/🧪️tests/🟦️.ts"],
    includeSource: ["../../🟦️.ts"],
    coverage: { include: ["../../🟦️.ts"] },
    passWithNoTests: false,
  },
};
