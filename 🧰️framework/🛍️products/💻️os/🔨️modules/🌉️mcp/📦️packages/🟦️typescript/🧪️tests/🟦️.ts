import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji 🧪️ Vitest for `@semio-tech/framework-os-mcp` — in-source tests (`import.meta.vitest`) on
 * the pure surface in `../../🟦️.ts`, plus three real-process integration suites that spawn
 * the compiled `semio-os-mcp` binary directly: legacy era (real `@modelcontextprotocol/sdk`
 * `Client`), modern era (hand-rolled raw JSON-RPC, `📓️design-decisions.md` D1), stdio hygiene, and the
 * end-to-end surface + progressive-enhancement gate (ticket `26/08/29/AI-MCP-END-TO-END`).
 * A generous `testTimeout` covers real process spawn/build-adjacent latency, not network flakiness. */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-os-mcp": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-os-mcp",
    mode: "test",
    environment: "node",
    include: ["🧪️legacy-conformance.test.ts", "🧪️modern-era.test.ts", "🧪️hygiene.test.ts", "🧪️end-to-end.test.ts", "🧪️authenticated-hub-workspace.test.ts"],
    coverage: { include: ["../../🟦️.ts"] },
    includeSource: ["../../🟦️.ts"],
    testTimeout: 30_000,
    hookTimeout: 30_000,
    passWithNoTests: false,
  },
});
