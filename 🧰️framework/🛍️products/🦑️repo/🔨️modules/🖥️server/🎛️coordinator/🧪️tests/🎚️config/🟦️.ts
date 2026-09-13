// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** @emoji 🧪️ Vitest for `@semio-tech/repo-coordinator`: the owner module's per-case suites under
 * `🎛️coordinator/🧪️tests`, which assert the `repo.server.coordinator` parsers against the ajv draft-07
 * oracle and the `repo.server` `<Table>Row` exports against the native PostgreSQL DDL. */
export default {
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/repo-coordinator",
    mode: "test",
    environment: "node",
    include: [resolve(root, "../../🧪️tests/*/🟦️.ts")],
    passWithNoTests: false,
    coverage: { include: ["app/**/*.ts", "app/**/*.tsx"] },
  },
};
