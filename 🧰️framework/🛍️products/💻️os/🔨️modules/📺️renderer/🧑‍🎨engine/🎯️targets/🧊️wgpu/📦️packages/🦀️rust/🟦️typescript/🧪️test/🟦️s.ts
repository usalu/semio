import { defineConfig } from "vitest/config";

export default defineConfig({
  root: "../..",
  test: {
    name: "@semio-tech/framework-renderer-wgpu",
    environment: "node",
    include: ["🧪️tests/📨️browser-frame-transport.ts","🧪️tests/🎮️browser-interactive-job-port.ts","🧪️tests/🧩️package-integration.ts"],
    coverage: { include: ["🎬️renderer-boot/🟦️.ts"] },
  },
});
