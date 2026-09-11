import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const configDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(configDir, "../../../../../../../../../..");

export default defineConfig({
  root: "../../../..",
  cacheDir: repoCacheDirectory(repoRoot, "vite", "renderer-wgpu"),
  test: {
    name: "@semio-tech/framework-renderer-wgpu",
    environment: "node",
    include: ["🧪️tests/📨️browser-frame-transport/🟦️.ts", "🧪️tests/🎮️browser-interactive-job-port/🟦️.ts", "🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts", "🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts", "🧪️tests/🧩️package-integration/🟦️.ts", "🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts"],
    coverage: { include: ["🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts"] },
    passWithNoTests: false,
  },
});
