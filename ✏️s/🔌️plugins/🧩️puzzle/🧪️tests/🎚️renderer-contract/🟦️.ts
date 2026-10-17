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
    include: ["🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🪪️session-factory/🟦️.ts", "🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🥽️brush-mesh-upload/🟦️.ts", "🧪️tests/🚀️renderer-boot-variants/🟦️.ts"],
    includeSource: [],
    passWithNoTests: false,
    testNamePattern: undefined,
  },
});
