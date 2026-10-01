import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(root, "../../../../../../..");

/** 🛰️ The agent-bridge rendezvous every gateway these suites spawn meets shells in — a fresh one per
 * run. The per-user default is where a developer's live `dev s` sessions wait, and a test gateway
 * offered there is dialled by them: `ui_focus` then succeeds against a real shell and the tier-2 law
 * measures a stranger's browser instead of the binary. */
const SUITE_AGENT_BRIDGE_DIR = mkdtempSync(join(tmpdir(), "semio-mcp-suite-bridge-"));

/** 🧪️ Vitest for `@semio-tech/framework-os-mcp` — in-source tests (`import.meta.vitest`) on
 * the pure surface in `../../🟦️.ts`, plus three real-process integration suites that spawn
 * the compiled `semio-os-mcp` binary directly: legacy era (real `@modelcontextprotocol/sdk`
 * `Client`), modern era (hand-rolled raw JSON-RPC, `📓️design-decisions.md` D1), stdio hygiene, and the
 * end-to-end surface + progressive-enhancement gate (ticket `26/08/29/AI-MCP-END-TO-END`).
 * A generous `testTimeout` covers real process spawn/build-adjacent latency, not network flakiness. */
export default defineConfig({
  root: testRoot,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "os-mcp"),
  resolve: {
    alias: {
      "@semio-tech/framework-os-mcp": resolve(root, "./🟦️.ts"),
    },
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-os-mcp",
    environment: "node",
    include: [resolve(root, "../*/🟦️.ts")],
    exclude: [
      resolve(root, "../🧪️resolvemcpbinarypath/🟦️.ts"),
      resolve(root, "./🟦️.ts"),
    ],
    env: { S_AGENT_BRIDGE_DIR: SUITE_AGENT_BRIDGE_DIR },
    coverage: { include: ["../../🟦️.ts"] },
    includeSource: ["../../🟦️.ts"],
    testTimeout: 30_000,
    hookTimeout: 30_000,
    passWithNoTests: false,
  },
});
