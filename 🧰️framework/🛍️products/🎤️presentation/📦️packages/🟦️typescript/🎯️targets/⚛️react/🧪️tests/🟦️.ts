// #region 🔌️Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(root, "../../../../../../..");

/** @emoji 🧪️ Vitest for `@semio-tech/presentation-react`. */
export default defineConfig({
  root,
  plugins: [react()],
  resolve: {
    alias: [
      { find: "@semio-tech/presentation-react", replacement: resolve(root, "🟦️.tsx") },
      { find: "@semio-tech/presentation", replacement: resolve(root, "../../🟦️.ts") },
      { find: "@semio-tech/framework", replacement: resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") },
      {
        find: "@semio-tech/mit-bestand-praesentation-projektetage-spec",
        replacement: resolve(repoRoot, "./♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/🟦️.ts"),
      },
    ],
  },
  test: {
    name: "@semio-tech/presentation-react",
    environment: "jsdom",
    include: [],
    coverage: { include: ["🟦️.tsx"] },
    includeSource: ["🟦️.tsx", "🔨️modules/📝️markdown-html-compiler/🟦️.ts", "🔨️modules/🔌️pdf-canvas-port/🟦️.ts"],
    passWithNoTests: false,
    setupFiles: [resolve(root, "🧰️vitest.setup.ts")],
  },
});
