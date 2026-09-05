/** 🧪️ Stdio hygiene suite (packet `P5-conformance-tests`, brief §3.4) — a black-box process-level
 * proof, independent of `🚚️transport/🦀️.rs`'s own in-memory `Cursor`-based unit tests
 * (`one_request_line_produces_exactly_one_response_line_on_stdout`,
 * `malformed_json_logs_to_the_log_writer_and_never_pollutes_stdout_with_non_json_text`,
 * `eof_ends_the_loop_cleanly`): those exercise `StdioTransport` in memory, never the REAL process's
 * REAL file descriptors. This suite spawns the actual compiled binary and inspects its actual
 * stdout/stderr/exit code — a stray byte on real stdout breaks every real MCP client, which an
 * in-memory `Cursor` test structurally cannot observe. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { describe, expect, it } from "vitest";
import { requireMcpBinary, spawnRawMcp } from "../../🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const bin = requireMcpBinary(repoRoot);

describe("document descriptor schema oracle", () => {
  it("accepts the shared Rust and TypeScript fixture with AJV", () => {
    const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🔣️.json"), "utf8")) as object;
    const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧫️fixtures", "📇️directory", "🪪️document-descriptor.json"), "utf8")) as { valid: object; conflictingSchemaHash: object; crossSpaceSameDocument: object };
    const validate = new Ajv2020({ strict: false }).compile({ ...schema, $ref: "#/$defs/DocumentDescriptor" });
    expect(validate(fixture.valid), JSON.stringify(validate.errors)).toBe(true);
    expect(validate(fixture.conflictingSchemaHash), JSON.stringify(validate.errors)).toBe(true);
    expect(validate(fixture.crossSpaceSameDocument), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture.valid, packSchemaHash: "0".repeat(64) })).toBe(false);
  });

  it("validates structural artifact authority fixtures and rejects fixed-hash and overflow boundaries with AJV", () => {
    const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🔣️.json"), "utf8")) as object;
    const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧫️fixtures", "📇️directory", "🛡️artifact-authority.json"), "utf8")) as {
      checkpoint: Record<string, unknown>;
      retention: object;
      scopeBoundaries: Record<string, object>;
      invalidBoundaries: { shortHash: number[]; overflowHashByte: number[]; zeroHash: number[]; unsafeInteger: number };
    };
    const compile = (reference: string) => new Ajv2020({ strict: false }).compile({ ...schema, $ref: reference });
    const checkpoint = compile("#/$defs/ArtifactCheckpoint");
    const retention = compile("#/$defs/ArtifactRetention");
    const scope = compile("#/$defs/DocumentScope");
    expect(checkpoint(fixture.checkpoint), JSON.stringify(checkpoint.errors)).toBe(true);
    expect(retention(fixture.retention), JSON.stringify(retention.errors)).toBe(true);
    expect(scope(fixture.scopeBoundaries.nonAscii), JSON.stringify(scope.errors)).toBe(true);
    expect(scope(fixture.scopeBoundaries.emptySpace)).toBe(false);
    expect(scope(fixture.scopeBoundaries.emptyDocument)).toBe(false);
    expect(checkpoint({ ...fixture.checkpoint, checkpointId: fixture.invalidBoundaries.shortHash })).toBe(false);
    expect(checkpoint({ ...fixture.checkpoint, checkpointId: fixture.invalidBoundaries.overflowHashByte })).toBe(false);
    expect(checkpoint({ ...fixture.checkpoint, checkpointId: fixture.invalidBoundaries.zeroHash })).toBe(false);
    expect(checkpoint({ ...fixture.checkpoint, publishedAtMs: fixture.invalidBoundaries.unsafeInteger })).toBe(false);
    expect(checkpoint({ ...fixture.checkpoint, pack: { ...(fixture.checkpoint.pack as object), byteLength: fixture.invalidBoundaries.unsafeInteger } })).toBe(false);
  });
});

describe("semio-os-mcp — stdio hygiene", () => {
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
