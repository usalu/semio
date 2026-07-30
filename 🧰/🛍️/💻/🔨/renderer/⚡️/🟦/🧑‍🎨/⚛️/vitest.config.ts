import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../../..");

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "framework/module/ui/js/react/index.tsx") },
      { find: "@semio-tech/ui-asset", replacement: resolve(repoRoot, "framework/module/ui/asset/js/index.ts") },
      { find: "@semio-tech/ui-styling", replacement: resolve(repoRoot, "framework/module/ui/styling/js/index.ts") },
      { find: "@semio-tech/framework-core", replacement: resolve(repoRoot, "framework/js/index.ts") },
      { find: "@semio-tech/infinite-cavas-react-renderer", replacement: resolve(repoRoot, "framework/product/os/module/infinite/canvas/react-renderer/index.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "framework/product/os/module/infinite/world/r3f/index.tsx") },
    ],
  },
  test: {
    name: "@semio-tech/framework-renderer-react",
    environment: "jsdom",
    coverage: { include: ["index.tsx"] },
  },
});
