import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import rendererConfig from "@semio-tech/framework-renderer-react/testing/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** 🖼️ Vitest for the laws where one plugin meets the real renderer; plugins may not import renderer internals, so these live where plugins may meet. */
export default defineConfig({
  ...rendererConfig,
  root,
  cacheDir: `${rendererConfig.cacheDir}-composition`,
  test: {
    ...rendererConfig.test,
    root,
    name: "plugin-composition-renderer-contract",
    include: ["🛍️layout-canvas-catalogue/🟦️.ts", "🪪️puzzle-board-session-factory/🟦️.ts", "🥽️puzzle-brush-mesh-upload/🟦️.ts", "🗺️gismap-mounted-probe/🟦️.ts", "🗺️gismap-inference-panel/🟦️.tsx"],
    includeSource: [],
    passWithNoTests: false,
    testNamePattern: undefined,
  },
});
