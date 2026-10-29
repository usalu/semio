// #region 🔌️Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const target = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const root = resolve(target, "📦️packages/🟦️typescript");
const staffage = resolve(target, "../..");

/** 🧪️ Vitest for `@teaching/staffage-react`: every case under `🧪️tests/<case>/🟦️.tsx`, run in jsdom. */
export default defineConfig({
  root,
  plugins: [react()],
  resolve: {
    alias: [
      { find: /^@teaching\/staffage-react\/app$/u, replacement: resolve(root, "📚️app/🟦️.tsx") },
      { find: /^@teaching\/staffage-react$/u, replacement: resolve(root, "🟦️.tsx") },
      { find: /^@teaching\/staffage$/u, replacement: resolve(staffage, "📦️packages/🟦️typescript/🟦️.ts") },
    ],
    dedupe: ["react", "react-dom"],
  },
  test: {
    root,
    name: "@teaching/staffage-react",
    environment: "jsdom",
    include: ["../../🧪️tests/*/🟦️.tsx"],
    coverage: { include: ["🟦️.tsx", "📚️app/**/🟦️.tsx"] },
    passWithNoTests: false,
  },
});
