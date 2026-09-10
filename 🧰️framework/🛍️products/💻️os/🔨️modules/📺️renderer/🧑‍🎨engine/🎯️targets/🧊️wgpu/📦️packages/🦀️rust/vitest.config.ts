import { defineConfig } from "vitest/config";

export default defineConfig({
  root: "../../../..",
  test: {
    name: "@semio-tech/framework-renderer-wgpu",
    environment: "node",
    include: ["🧪️tests/📨️browser-frame-transport/🟦️.ts", "🧪️tests/🎮️browser-interactive-job-port/🟦️.ts", "🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts", "🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts", "🧪️tests/🧩️package-integration/🟦️.ts"],
    coverage: { include: ["🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts"] },
    passWithNoTests: false,
  },
});
