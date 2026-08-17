import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-os-mcp` — in-source tests (`import.meta.vitest`) on
 * the pure surface in `../../🟦️component.ts`, plus three real-process integration suites that spawn
 * the compiled `semio-os-mcp` binary directly: legacy era (real `@modelcontextprotocol/sdk`
 * `Client`), modern era (hand-rolled raw JSON-RPC, `📓️design-decisions.md` D1), and stdio hygiene.
 * A generous `testTimeout` covers real process spawn/build-adjacent latency, not network flakiness. */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-os-mcp": resolve(root, "🟦️glue.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-os-mcp",
    mode: "test",
    environment: "node",
    include: ["../../🟦️component.ts", "🧪️legacy-conformance.test.ts", "🧪️modern-era.test.ts", "🧪️hygiene.test.ts"],
    coverage: { include: ["../../🟦️component.ts"] },
    includeSource: ["../../🟦️component.ts"],
    testTimeout: 30_000,
    hookTimeout: 30_000,
    passWithNoTests: false,
  },
});
