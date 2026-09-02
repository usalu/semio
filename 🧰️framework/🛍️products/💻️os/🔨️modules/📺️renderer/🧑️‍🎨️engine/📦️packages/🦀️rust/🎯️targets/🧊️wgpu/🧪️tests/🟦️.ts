import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

export default defineConfig({
  root,
  test: {
    name: "@semio-tech/framework-renderer-wgpu",
    environment: "node",
    coverage: { include: ["index.ts"] },
  },
});
