/** 🌉️ TypeScript surface of the `🌉️mcp` module (packet `P5-conformance-tests`) — the pieces the
 * conformance test suite under `📦️packages/🟦️typescript` shares: where the real `semio-os-mcp`
 * stdio binary lives, a minimal raw newline-delimited JSON-RPC client for the modern era the
 * installed `@modelcontextprotocol/sdk` (1.30.0, legacy-only — `📓️design-decisions.md` D1) cannot
 * speak, and a JSON Schema 2020-12 validator wrapper. Never touches the Rust crate directly — this
 * module only ever crosses the process boundary over real stdio, exactly like a real IDE client.
 */

import { type ChildProcessByStdio, spawn } from "node:child_process";
import { createInterface } from "node:readline";
import type { Readable, Writable } from "node:stream";
import Ajv2020 from "ajv/dist/2020.js";

//#region 🔖️BinaryPath
/** 📁️ This ticket's own scratch `CARGO_TARGET_DIR` (packet brief §3.1) — every packet in
 * `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` builds here instead of the shared workspace
 * `target/`, so concurrent tickets' cargo builds never collide with this one's. */
export const TICKET_TARGET_REL = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/🎯️target";

/** 📁️ `<repoRoot>/<TICKET_TARGET_REL>/debug/semio-os-mcp[.exe]` — overridable via `SEMIO_OS_MCP_BIN`
 * (brief §3.1), the seam a later wave uses once the crate graduates to the shared `target/debug`. */
export function resolveMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env): string {
  const override = env.SEMIO_OS_MCP_BIN;
  if (override) return override;
  const name = process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp";
  return `${repoRoot}/${TICKET_TARGET_REL}/debug/${name}`;
}
//#endregion 🔖️BinaryPath

//#region 🔖️RawJsonRpc
export type RawJsonRpcRequest = { readonly jsonrpc: "2.0"; readonly id?: number | string | null; readonly method: string; readonly params?: unknown };
export type RawJsonRpcResponse = { readonly jsonrpc: "2.0"; readonly id: number | string | null; readonly result?: unknown; readonly error?: { readonly code: number; readonly message: string; readonly data?: unknown } };

export type RawMcpProcess = {
  readonly stdoutLines: () => readonly string[];
  readonly stderrText: () => string;
  readonly pid: number | undefined;
  request(method: string, params?: unknown): Promise<RawJsonRpcResponse>;
  writeRaw(line: string): void;
  nextLine(timeoutMs?: number): Promise<string>;
  waitForExit(timeoutMs?: number): Promise<number | null>;
  close(): Promise<void>;
};

/** 🚀️ Spawns the real `semio-os-mcp` binary (default argv `["stdio"]`) and wires the ~50-line raw
 * newline-delimited JSON-RPC client the packet brief §3.3 calls for — the installed SDK is
 * legacy-only and cannot send a per-request `_meta` modern request or talk to a fresh, unhandshaked
 * connection, so this hand-rolled client is the only way to independently exercise the modern era.
 * Captures EVERY raw stdout line (hygiene suite, §3.4 — a single stray non-JSON byte on stdout
 * breaks every real MCP client) and the full stderr text (diagnostics only, never asserted as
 * JSON). Queue/waiter shape mirrors `os-hub-ts`'s `openFrameSocket` — the established pattern in
 * this repo for "await the next line from a live child process" over an event-based stream. */
export function spawnRawMcp(bin: string, args: readonly string[] = ["stdio"]): RawMcpProcess {
  const child = spawn(bin, [...args], { stdio: ["pipe", "pipe", "pipe"] }) as ChildProcessByStdio<Writable, Readable, Readable>;
  const allLines: string[] = [];
  const queue: string[] = [];
  let waiters: Array<(error: Error | null, line?: string) => void> = [];
  let stderr = "";
  let closedWith: Error | null = null;
  let exitCode: number | null = null;

  const rl = createInterface({ input: child.stdout });
  rl.on("line", (line) => {
    allLines.push(line);
    const waiter = waiters.shift();
    if (waiter) waiter(null, line);
    else queue.push(line);
  });
  child.stderr.on("data", (chunk: Buffer) => {
    stderr += chunk.toString("utf8");
  });
  child.once("exit", (code) => {
    exitCode = code;
    closedWith = new Error(`spawnRawMcp: process exited (code ${code})`);
    for (const waiter of waiters.splice(0)) waiter(closedWith);
  });

  const nextLine = (timeoutMs = 10_000): Promise<string> => {
    const queued = queue.shift();
    if (queued !== undefined) return Promise.resolve(queued);
    if (closedWith) return Promise.reject(closedWith);
    return new Promise((resolveLine, rejectLine) => {
      const timer = setTimeout(() => {
        waiters = waiters.filter((waiter) => waiter !== onLine);
        rejectLine(new Error(`spawnRawMcp: timed out waiting for a stdout line after ${timeoutMs}ms`));
      }, timeoutMs);
      const onLine = (error: Error | null, line?: string): void => {
        clearTimeout(timer);
        if (error) rejectLine(error);
        else resolveLine(line as string);
      };
      waiters.push(onLine);
    });
  };

  const writeRaw = (line: string): void => {
    child.stdin.write(`${line}\n`);
  };

  let nextId = 1;
  const request = async (method: string, params?: unknown): Promise<RawJsonRpcResponse> => {
    const id = nextId++;
    writeRaw(JSON.stringify({ jsonrpc: "2.0", id, method, params }));
    const line = await nextLine();
    return JSON.parse(line) as RawJsonRpcResponse;
  };

  const waitForExit = (timeoutMs = 5_000): Promise<number | null> => {
    if (exitCode !== null || closedWith) return Promise.resolve(exitCode);
    return new Promise((resolveExit, rejectExit) => {
      const timer = setTimeout(() => rejectExit(new Error(`spawnRawMcp: process did not exit within ${timeoutMs}ms`)), timeoutMs);
      child.once("exit", (code) => {
        clearTimeout(timer);
        resolveExit(code);
      });
    });
  };

  const close = async (): Promise<void> => {
    if (closedWith) return;
    await new Promise<void>((resolveClose) => {
      child.once("exit", () => resolveClose());
      child.stdin.end();
      setTimeout(() => {
        if (exitCode === null) child.kill("SIGKILL");
      }, 5_000);
    });
  };

  return { stdoutLines: () => allLines, stderrText: () => stderr, pid: child.pid, request, writeRaw, nextLine, waitForExit, close };
}
//#endregion 🔖️RawJsonRpc

//#region 🔖️SchemaValidation
/** ✅️ True iff `schema` is a structurally valid JSON Schema draft 2020-12 document — compiling it
 * with a real `Ajv2020` instance (`ajv` 8.20, already vendored as a transitive dependency of the
 * installed `@modelcontextprotocol/sdk` — packet brief §3.2 names `ajv` explicitly) is itself the
 * proof: Ajv rejects anything that doesn't parse as a schema under that draft. A fresh instance per
 * call avoids `$id` collisions across unrelated tool schemas sharing no registry. */
export function isValidJsonSchema2020_12(schema: unknown): { readonly valid: true } | { readonly valid: false; readonly error: string } {
  try {
    new Ajv2020({ strict: false }).compile(schema as object);
    return { valid: true };
  } catch (error) {
    return { valid: false, error: String(error) };
  }
}
//#endregion 🔖️SchemaValidation

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("resolveMcpBinaryPath", () => {
    it("defaults to this ticket's scratch target/debug, platform-named", () => {
      const path = resolveMcpBinaryPath("/repo", {});
      const expected = process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp";
      expect(path).toBe(`/repo/${TICKET_TARGET_REL}/debug/${expected}`);
    });

    it("prefers SEMIO_OS_MCP_BIN when set", () => {
      const path = resolveMcpBinaryPath("/repo", { SEMIO_OS_MCP_BIN: "/custom/semio-os-mcp" });
      expect(path).toBe("/custom/semio-os-mcp");
    });
  });

  describe("isValidJsonSchema2020_12", () => {
    it("accepts a well-formed object schema", () => {
      const result = isValidJsonSchema2020_12({ type: "object", properties: { x: { type: "string" } }, required: ["x"] });
      expect(result.valid).toBe(true);
    });

    it("rejects a schema whose keyword values are structurally invalid", () => {
      const result = isValidJsonSchema2020_12({ type: "object", properties: "not-an-object" });
      expect(result.valid).toBe(false);
    });
  });
}
//#endregion 🧪️Tests
