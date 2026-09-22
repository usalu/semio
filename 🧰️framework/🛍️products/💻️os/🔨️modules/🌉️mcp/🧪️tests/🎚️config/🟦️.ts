import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(root, "../../../../../../..");

/** @emoji 🧪️ Vitest for `@semio-tech/framework-os-mcp` — in-source tests (`import.meta.vitest`) on
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
    include: [resolve(root, "../../🧪️tests/*/🟦️.ts")],
    // 🚫️ Four files live under `🧪️tests/` and are NOT vitest suites, so the glob above swept them in
    // and the project reported them as failures that measured nothing:
    //   · `🎚️config` is the configuration you are reading — vitest called it a suite with no tests;
    //   · `💬️agent-reply`, `🤖️live-agent-loop` and `🤖️hub-agent-participant` are LIVE GATES with
    //     their own `*-check` Nx targets and `.vscode/launch.json` rows (group `4_gate`). Each one
    //     needs an already-running `dev` serve or hub handed to it by environment, executes at
    //     import time and ends in `process.exit`. Run under `test quick` they crashed on an unset
    //     environment path instead of gating anything.
    exclude: [
      resolve(root, "../../🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts"),
      resolve(root, "../../🧪️tests/🎚️config/🟦️.ts"),
      resolve(root, "../../🧪️tests/💬️agent-reply/🟦️.ts"),
      resolve(root, "../../🧪️tests/🤖️live-agent-loop/🟦️.ts"),
      resolve(root, "../../🧪️tests/🤖️hub-agent-participant/🟦️.ts"),
    ],
    coverage: { include: ["../../🟦️.ts"] },
    includeSource: ["../../🟦️.ts"],
    testTimeout: 30_000,
    hookTimeout: 30_000,
    passWithNoTests: false,
  },
});
