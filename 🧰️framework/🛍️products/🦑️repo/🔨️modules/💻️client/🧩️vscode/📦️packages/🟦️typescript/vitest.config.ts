// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/repo-vscode`: the owner module's `🔬️schema` case, which asserts
 * the `repo.client.vscode` parsers and the ajv draft-07 oracle agree on `🗂️technologies.json`.
 * `🧩️extension` is the extension-host Mocha case and runs under `vscode-test`, not here. */
export default {
  root,
  test: {
    name: "@semio-tech/repo-vscode",
    mode: "test",
    environment: "node",
    include: [resolve(root, "../../🧪️tests/🔬️schema/🟦️.ts")],
    passWithNoTests: false,
  },
};
