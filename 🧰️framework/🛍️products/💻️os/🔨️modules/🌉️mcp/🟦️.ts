/** 🌉️ TypeScript surface of the `🌉️mcp` module (packet `P5-conformance-tests`) — the pieces the
 * conformance test suite under `📦️packages/🟦️typescript` shares: where the real `semio-os-mcp`
 * stdio binary lives, a minimal raw newline-delimited JSON-RPC client for the modern era the
 * installed `@modelcontextprotocol/sdk` (1.30.0, legacy-only — `📓️design-decisions.md` D1) cannot
 * speak, and a JSON Schema 2020-12 validator wrapper. Never touches the Rust crate directly — this
 * module only ever crosses the process boundary over real stdio, exactly like a real IDE client.
 */

import { type ChildProcessByStdio, spawn } from "node:child_process";
import { accessSync, constants as fsConstants, readFileSync, statSync } from "node:fs";
import { posix, win32 } from "node:path";
import { createInterface } from "node:readline";
import type { Readable, Writable } from "node:stream";

//#region 🔖️BinaryPath
/** 📁️ The shared workspace target directory the crate builds into, like every other crate in this
 * monorepo. Ticket `26/08/29/AI-MCP-END-TO-END` graduated it here from
 * `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`'s scratch `🎯️target` — that path is a
 * transient ticket artifact, so resolving against it made every conformance suite skip itself the
 * moment the ticket's scratch directory was cleaned. */
export const TARGET_DEBUG_REL = "target/debug";
export const MCP_CARGO_PACKAGE = "semio-framework-os-mcp";
export const MCP_BINARY_NAME = "semio-os-mcp";

function pathApi(platform: NodeJS.Platform): typeof posix {
  return platform === "win32" ? win32 : posix;
}

/** 📁️ Absolute Cargo target root shared by the build and black-box test gates. */
export function resolveMcpTargetDirectory(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  return pathApi(platform).resolve(repoRoot, env.CARGO_TARGET_DIR ?? "target");
}

/** 📦️ The exact debug artifact Cargo's MCP build command must produce, ignoring test overrides. */
export function resolveBuiltMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const filename = platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME;
  return pathApi(platform).join(resolveMcpTargetDirectory(repoRoot, env, platform), "debug", filename);
}

/** 📁️ `<repoRoot>/target/debug/semio-os-mcp[.exe]`, or `CARGO_TARGET_DIR`/`SEMIO_OS_MCP_BIN` when
 * either is set — the two seams a caller building somewhere else (a ticket scratch dir, CI) uses. */
export function resolveMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const override = env.SEMIO_OS_MCP_BIN;
  return override ? pathApi(platform).resolve(repoRoot, override) : resolveBuiltMcpBinaryPath(repoRoot, env, platform);
}

/** 🛡️ Resolves and verifies the real executable so a missing black-box subject cannot skip green. */
export function requireMcpBinary(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const binary = resolveMcpBinaryPath(repoRoot, env, platform);
  try {
    if (!statSync(binary).isFile()) throw new Error("path is not a file");
    if (platform !== "win32") accessSync(binary, fsConstants.X_OK);
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    throw new Error(`semio-os-mcp binary gate failed at ${binary}: ${detail}`);
  }
  return binary;
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

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("resolveMcpBinaryPath", () => {
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🧱️binary-gate.json", import.meta.url), "utf8")) as {
      pathCases: Array<{ name: string; platform: NodeJS.Platform; repoRoot: string; environment: NodeJS.ProcessEnv; expected: string }>;
    };

    for (const testCase of fixture.pathCases) {
      it(testCase.name, () => {
        expect(resolveMcpBinaryPath(testCase.repoRoot, testCase.environment, testCase.platform)).toBe(testCase.expected);
      });
    }

    it("accepts an independently executable process artifact", () => {
      expect(requireMcpBinary("/", { SEMIO_OS_MCP_BIN: process.execPath })).toBe(process.execPath);
    });

    it("rejects a missing explicit artifact instead of permitting a skipped suite", () => {
      expect(() => requireMcpBinary("/workspace/semio", { SEMIO_OS_MCP_BIN: "missing/semio-os-mcp" }, "linux")).toThrow("binary gate failed");
    });

    it("defaults to the shared workspace target/debug, platform-named", () => {
      const path = resolveMcpBinaryPath("/repo", {});
      const expected = process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp";
      expect(path).toBe(`/repo/${TARGET_DEBUG_REL}/${expected}`);
    });

    it("prefers SEMIO_OS_MCP_BIN when set", () => {
      const path = resolveMcpBinaryPath("/repo", { SEMIO_OS_MCP_BIN: "/custom/semio-os-mcp" });
      expect(path).toBe("/custom/semio-os-mcp");
    });

    it("honours CARGO_TARGET_DIR so a scratch-target build is still found", () => {
      const path = resolveMcpBinaryPath("/repo", { CARGO_TARGET_DIR: "/scratch/target" });
      const expected = process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp";
      expect(path).toBe(`/scratch/target/debug/${expected}`);
    });
  });

}
//#endregion 🧪️Tests
