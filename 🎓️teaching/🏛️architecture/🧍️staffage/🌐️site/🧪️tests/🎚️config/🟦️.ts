// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🧪️ Vitest for `@teaching/staffage-site`: every case under `🧪️tests/<case>/🟦️.ts`. */
export default {
  root,
  test: {
    root,
    name: "@teaching/staffage-site",
    mode: "test",
    environment: "node",
    include: ["../../🧪️tests/*/🟦️.ts"],
    exclude: ["../../🧪️tests/🎚️config/🟦️.ts", "**/node_modules/**"],
    passWithNoTests: false,
  },
};
