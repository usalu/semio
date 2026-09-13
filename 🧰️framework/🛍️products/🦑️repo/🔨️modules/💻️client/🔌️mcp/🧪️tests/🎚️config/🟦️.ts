// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** @emoji 🧪️ Vitest for `@semio-tech/repo-mcp-schema`: the owner module's `🔬️schema` case, which asserts
 * that `🧬️schema/🔗️.graphql` and `🧬️schema/🔣️.json` declare the same exports with the same field
 * nullability, and that the JSON facet compiles under an independent draft-07 validator. */
export default {
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/repo-mcp-schema",
    mode: "test",
    environment: "node",
    include: [resolve(root, "../../🧪️tests/🔬️schema/🟦️.ts")],
    passWithNoTests: false,
  },
};
