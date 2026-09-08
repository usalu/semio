// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji 🧪️ Vitest for `@semio-tech/repo-sqlite` (parity of 🧬️schema/🔣️.json with the native 🗄️.sql). */
export default {
  root,
  test: {
    name: "@semio-tech/repo-sqlite",
    mode: "test",
    environment: "node",
    passWithNoTests: false,
  },
};
