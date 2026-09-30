import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { defineConfig } from "vitest/config";
import rendererConfig from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

export default defineConfig({
  ...rendererConfig,
  root,
  cacheDir: `${rendererConfig.cacheDir}-puzzle`,
  test: {
    ...rendererConfig.test,
    root,
    name: "puzzle-renderer-contract",
    include: ["🧪️tests/🪪️session-factory/🟦️.ts", "🧪️tests/🥽️brush-mesh-upload/🟦️.ts"],
    includeSource: [],
    passWithNoTests: false,
    testNamePattern: undefined,
  },
});
