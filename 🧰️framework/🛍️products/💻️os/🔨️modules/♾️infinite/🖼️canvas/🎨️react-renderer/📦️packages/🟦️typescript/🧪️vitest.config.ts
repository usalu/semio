// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const configDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(configDir, "../../../../../../../../..");
const componentSource = "../../🟦️component.tsx";

/** @emoji 🧪️ Vitest for `@semio-tech/infinite-canvas-react-renderer` — in-source `import.meta.vitest` on `🟦️component.tsx`. */
export default defineConfig({
  root: configDir,
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx") }],
  },
  test: {
    name: "@semio-tech/infinite-canvas-react-renderer",
    mode: "test",
    environment: "jsdom",
    include: [],
    includeSource: [componentSource],
    coverage: { include: [componentSource] },
    passWithNoTests: false,
  },
});
