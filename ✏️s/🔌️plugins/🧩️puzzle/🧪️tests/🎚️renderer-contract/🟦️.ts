import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { defineConfig } from "vitest/config";
import rendererConfig from "@semio-tech/framework-renderer-react/testing/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

export default defineConfig({
  ...rendererConfig,
  root,
  cacheDir: `${rendererConfig.cacheDir}-puzzle`,
  test: {
    ...rendererConfig.test,
    root,
    name: "puzzle-renderer-contract",
    include: ["🗿️artifacts/**/🧪️tests/🪪️session-factory/🟦️.ts", "🗿️artifacts/**/🧪️tests/🥽️brush-mesh-upload/🟦️.ts", "🧪️tests/🚀️renderer-boot-variants/🟦️.ts"],
    includeSource: [],
    passWithNoTests: false,
    testNamePattern: undefined,
  },
});
