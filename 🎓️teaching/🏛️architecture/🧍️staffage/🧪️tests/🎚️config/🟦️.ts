// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🧪️ Vitest for `@teaching/staffage`: every case under `🧪️tests/<case>/🟦️.ts`, found by a glob so a new case needs no edit here. */
export default {
  root,
  resolve: {
    alias: [{ find: /^@teaching\/staffage$/u, replacement: resolve(root, "🟦️.ts") }],
  },
  test: {
    root,
    name: "@teaching/staffage",
    mode: "test",
    environment: "node",
    include: ["../../🧪️tests/*/🟦️.ts"],
    exclude: ["../../🧪️tests/🎚️config/🟦️.ts", "**/node_modules/**"],
    coverage: { include: ["🟦️.ts", "../../🧬️schema/🟦️.ts", "../../🔨️modules/**/🟦️.ts"] },
    passWithNoTests: false,
  },
};
