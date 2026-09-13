// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
// #endregion 🔌️Adapters

const configDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(configDir, "../../../../../../../../..");
const componentSource = "../../🟦️.tsx";

/** @emoji 🧪️ Vitest for `@semio-tech/infinite-canvas-react-renderer` — in-source `import.meta.vitest` on `🟦️.tsx`. */
export default {
  root: testRoot,
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") }],
  },
  test: {
    root: testRoot,
    name: "@semio-tech/infinite-canvas-react-renderer",
    mode: "test",
    environment: "jsdom",
    include: [],
    includeSource: [componentSource],
    coverage: { include: [componentSource] },
    passWithNoTests: false,
  },
};
