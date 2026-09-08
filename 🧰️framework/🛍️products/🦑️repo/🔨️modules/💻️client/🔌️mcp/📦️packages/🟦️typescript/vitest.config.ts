// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/repo-mcp-schema`: the owner module's `🔬️schema` case, which asserts
 * that `🧬️schema/🔗️.graphql` and `🧬️schema/🔣️.json` declare the same exports with the same field
 * nullability, and that the JSON facet compiles under an independent draft-07 validator. */
export default {
  root,
  test: {
    name: "@semio-tech/repo-mcp-schema",
    mode: "test",
    environment: "node",
    include: [resolve(root, "../../🧪️tests/🔬️schema/🟦️.ts")],
    passWithNoTests: false,
  },
};
