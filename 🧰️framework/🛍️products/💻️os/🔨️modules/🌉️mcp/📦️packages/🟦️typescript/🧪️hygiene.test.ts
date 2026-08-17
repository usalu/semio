/** 🧪️ Stdio hygiene suite (packet `P5-conformance-tests`, brief §3.4) — a black-box process-level
 * proof, independent of `🚚️transport/🦀️component.rs`'s own in-memory `Cursor`-based unit tests
 * (`one_request_line_produces_exactly_one_response_line_on_stdout`,
 * `malformed_json_logs_to_the_log_writer_and_never_pollutes_stdout_with_non_json_text`,
 * `eof_ends_the_loop_cleanly`): those exercise `StdioTransport` in memory, never the REAL process's
 * REAL file descriptors. This suite spawns the actual compiled binary and inspects its actual
 * stdout/stderr/exit code — a stray byte on real stdout breaks every real MCP client, which an
 * in-memory `Cursor` test structurally cannot observe. */
import { existsSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { resolveMcpBinaryPath, spawnRawMcp } from "../../🟦️component.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const repoRoot = getWorkspaceRoot();
const bin = resolveMcpBinaryPath(repoRoot);
const BIN_PRESENT = existsSync(bin);

if (!BIN_PRESENT) {
  console.warn(`[@semio-tech/framework-os-mcp] hygiene suite SKIPPED — binary not found at ${bin}. Build it first: CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp`);
}

describe.skipIf(!BIN_PRESENT)("semio-os-mcp — stdio hygiene", () => {
  it("every stdout line across a whole mixed-traffic session parses as JSON, even around a malformed line", async () => {
    const proc = spawnRawMcp(bin);
    try {
      await proc.request("ping");
      proc.writeRaw("not json at all, a stray diagnostic-looking line");
      // The malformed line still produces exactly one PARSE_ERROR response line — wait for it
      // before sending the next well-formed request, so line order stays deterministic.
      await proc.nextLine();
      await proc.request("server/discover", {});
      await proc.request("tools/list", {});

      for (const line of proc.stdoutLines()) {
        expect(() => JSON.parse(line), `stdout line was not valid JSON: ${line}`).not.toThrow();
      }
      expect(proc.stdoutLines().length).toBeGreaterThanOrEqual(4);
    } finally {
      await proc.close();
    }
  });

  it("malformed input yields a proper JSON-RPC parse error (not a crash) and the process answers the next request", async () => {
    const proc = spawnRawMcp(bin);
    try {
      proc.writeRaw("{ this is not valid json");
      const errorLine = JSON.parse(await proc.nextLine()) as { error?: { code: number } };
      expect(errorLine.error?.code).toBe(-32700);

      const pong = await proc.request("ping");
      expect(pong.error).toBeUndefined();
      expect(pong.result).toBeDefined();
    } finally {
      await proc.close();
    }
  });

  it("the malformed-line diagnostic lands on stderr, never on stdout", async () => {
    const proc = spawnRawMcp(bin);
    try {
      proc.writeRaw("also not json");
      await proc.nextLine();
      for (const line of proc.stdoutLines()) {
        expect(line).not.toContain("malformed JSON-RPC line rejected");
      }
    } finally {
      await proc.close();
    }
  });

  it("the process exits cleanly (code 0) on stdin EOF", async () => {
    const proc = spawnRawMcp(bin);
    await proc.request("ping");
    await proc.close();
    const exitCode = await proc.waitForExit(1_000);
    expect(exitCode).toBe(0);
  });
});
