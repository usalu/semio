import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../../../../..");

const wasmEngineStub = resolve(repoRoot, "./🧰️framework/🔨️module/🖱️ui/🎨️styling/⚡️implementation/🦀️rust/🟦️vite-elements-assets.ts");

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "./🧰️framework/🔨️module/🖱️ui/⚛️react/⚡️implementation/🟦️typescript/📦️index.tsx") },
      { find: "@semio-tech/asset", replacement: resolve(repoRoot, "./🧰️framework/🔨️module/🖼️asset/⚡️implementation/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/ui-styling", replacement: resolve(repoRoot, "./🧰️framework/🔨️module/🖱️ui/🎨️styling/⚡️implementation/🟦️typescript") },
      { find: "@semio-tech/framework-core", replacement: resolve(repoRoot, "./🧰️framework/⚡️implementation/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/framework-os-core", replacement: resolve(repoRoot, "./🧰️framework/🛍️product/💻️os/⚡️implementation/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: resolve(repoRoot, "./🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementation/🟦️typescript/📦️index.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "./🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🌍️world/🎨️r3f/⚡️implementation/🟦️typescript/📦️index.tsx") },
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
