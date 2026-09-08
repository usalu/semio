// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const configDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(configDir, "../../../../../../../../..");
const componentSource = "../../🟦️.tsx";

/** @emoji 🧪️ Vitest for `@semio-tech/infinite-canvas-react-renderer` — in-source `import.meta.vitest` on `🟦️.tsx`. */
export default {
  root: configDir,
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") }],
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
};
