import { serviceMcpTestArtifactRoot } from "../📁️artifact-root/🟦️.ts";
import { defineConfig } from "vitest/config";
import { mkdtempSync } from "node:fs";
import { join, resolve } from "node:path";

export default defineConfig({
  root: resolve(import.meta.dir, "../.."),
  test: {
    name: "@semio-tech/s-services-native",
    environment: "node",
    include: ["🧪️tests/🧷️untrusted-content/🟦️.ts"],
    env: { S_AGENT_BRIDGE_DIR: mkdtempSync(join(serviceMcpTestArtifactRoot(), "semio-services-mcp-")) },
    testTimeout: 240_000,
    hookTimeout: 240_000,
    passWithNoTests: false,
  },
});
