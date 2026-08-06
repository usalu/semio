// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/flow-module-brep": resolve(root, "../../flow/module/brep/pkg/flow_extension_brep.js"),
    },
  },
  assetsInclude: ["**/*.wasm"],
  test: {
    name: "@semio-tech/s-3d-js",
    mode: "test",
    environment: "node",
    include: ["index.ts"],
    coverage: { include: ["index.ts"] },
  },
});
