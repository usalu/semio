// #region 🔌️Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const target = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const root = resolve(target, "📦️packages/🟦️typescript");
const product = resolve(target, "../..");

/** 🧪️ Vitest for `@semio-tech/pets-react`: every React suite under the product's `🧪️tests`, found by a glob so a new suite needs no edit here, run in jsdom. */
export default defineConfig({
  root,
  plugins: [react()],
  resolve: {
    alias: [
      { find: /^@semio-tech\/pets-react$/, replacement: resolve(root, "🟦️.tsx") },
      { find: /^@semio-tech\/pets$/, replacement: resolve(product, "📦️packages/🟦️typescript/🟦️.ts") },
    ],
  },
  test: {
    root,
    name: "@semio-tech/pets-react",
    environment: "jsdom",
    include: ["../../../../🧪️tests/*/🟦️.tsx"],
    coverage: { include: ["../../🟦️.tsx", "../../🔨️modules/**/🟦️.ts", "../../🔨️modules/**/🟦️.tsx"] },
    passWithNoTests: false,
  },
});
