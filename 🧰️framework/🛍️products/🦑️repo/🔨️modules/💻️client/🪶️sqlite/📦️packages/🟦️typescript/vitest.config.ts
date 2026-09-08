// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/repo-sqlite`: the owner module's per-case suites under
 * `🪶️sqlite/🧪️tests`, which assert that `🧬️schema/🔣️.json` and the native `🧬️schema/🗄️.sql`
 * declare the same columns with the same nullability. */
export default {
  root,
  test: {
    name: "@semio-tech/repo-sqlite",
    mode: "test",
    environment: "node",
    include: [resolve(root, "../../🧪️tests/*/🟦️.ts")],
    passWithNoTests: false,
  },
};
