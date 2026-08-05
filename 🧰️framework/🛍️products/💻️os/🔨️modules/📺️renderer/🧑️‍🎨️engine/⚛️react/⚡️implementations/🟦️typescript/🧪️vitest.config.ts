import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../../../../..");

const wasmEngineStub = resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts");

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx") },
      { find: "@semio-tech/assets", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/ui-styling", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/⚡️implementations/🟦️typescript") },
      { find: "@semio-tech/framework-core", replacement: resolve(repoRoot, "./🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/framework-os-core", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementations/🟦️typescript/📦️index.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/⚡️implementations/🟦️typescript/📦️index.tsx") },
      { find: "@semio-tech/framework-surface-node-graph-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-surface-paint-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-surface-tiled-map-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-surface-terrain-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-surface-board-2d-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-editor-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/flow-core", replacement: wasmEngineStub },
    ],
  },
  test: {
    name: "@semio-tech/framework-renderer-react",
    environment: "jsdom",
    coverage: { include: ["index.tsx"] },
  },
});
