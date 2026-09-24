/** 🌉️ TypeScript surface of the `🌉️mcp` module (packet `P5-conformance-tests`) — the pieces the
 * conformance test suite under `📦️packages/🟦️typescript` shares: where the real `semio-os-mcp`
 * stdio binary lives, a minimal raw newline-delimited JSON-RPC client for the modern era the
 * installed `@modelcontextprotocol/sdk` (1.30.0, legacy-only — `📓️design-decisions.md` D1) cannot
 * speak, and a JSON Schema 2020-12 validator wrapper. Never touches the Rust crate directly — this
 * module only ever crosses the process boundary over real stdio, exactly like a real IDE client.
 */

import { type ChildProcessByStdio, spawn, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { accessSync, chmodSync, constants as fsConstants, copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, posix, relative, win32 } from "node:path";
import { createInterface } from "node:readline";
import type { Readable, Writable } from "node:stream";
import { cargoTargetDirectory } from "../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";

//#region 🔖️BinaryPath
/** 📦️ Nx owns the executable separately from mutable compiler state. */
export const MCP_ARTIFACT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build";
/** 📦️ Where the optimized distribution binary is staged. Separate from {@link MCP_ARTIFACT_REL} so a
 * release build never clobbers the dev-loop artifact `.mcp.json` and every black-box gate exec. */
export const MCP_RELEASE_ARTIFACT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release";
export const MCP_CARGO_PACKAGE = "semio-framework-os-mcp";
export const MCP_BINARY_NAME = "semio-os-mcp";
/** 🏗️ The cargo profiles this crate's build targets produce. */
export type McpBuildProfile = "debug" | "release";

function pathApi(platform: NodeJS.Platform): typeof posix {
  return platform === "win32" ? win32 : posix;
}

/** 📁️ Absolute Cargo target root shared by the build and black-box test gates. */
export function resolveMcpTargetDirectory(repoRoot: string, env: NodeJS.ProcessEnv = process.env, _platform: NodeJS.Platform = process.platform): string {
  return cargoTargetDirectory(repoRoot, env);
}

/** 📦️ The exact artifact Cargo's MCP build command must produce for `profile`, ignoring test overrides. */
export function resolveBuiltMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform, profile: McpBuildProfile = "debug"): string {
  const filename = platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME;
  return pathApi(platform).join(resolveMcpTargetDirectory(repoRoot, env, platform), profile, filename);
}

/** 📁️ The staged release executable — the one a tarball ships and an end user installs. */
export function resolveStagedReleaseMcpBinaryPath(repoRoot: string, platform: NodeJS.Platform = process.platform): string {
  return pathApi(platform).resolve(repoRoot, MCP_RELEASE_ARTIFACT_REL, platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME);
}

/** 📁️ Resolves the staged executable or an explicit independent binary override. */
export function resolveMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const override = env.SEMIO_OS_MCP_BIN;
  return override ? pathApi(platform).resolve(repoRoot, override) : pathApi(platform).resolve(repoRoot, MCP_ARTIFACT_REL, platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME);
}

/** 🎯 The single Nx target that stages the debug `semio-os-mcp` every `.mcp.json` / `dev mcp` route reads. */
export const MCP_BINARY_TARGET = "@semio-tech/framework-os-mcp-rs:build";

/** 🏷️ Stamp written next to the staged binary so a content-hash match skips a cold rebuild. */
export function resolveMcpBinaryContentHashPath(binary: string): string {
  return `${binary}.content-hash`;
}

/** 🏗️ Injectable staging boundary: one attempt that returns the exit status of the Nx build target. */
export type McpBinaryStaging = Readonly<{ stage: (progress: (line: string) => void) => number }>;

const nativeMcpBinaryStagingFor = (repoRoot: string): McpBinaryStaging => ({
  stage: (progress) => {
    // Prefer the package verb directly: under fleet load the Nx daemon HASH_TASKS / socket path
    // stalls for minutes, while `bun ./📜️script.ts build` is the same deliverable.
    const packageRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust");
    progress(`[semio-os-mcp] staging via bun ./📜️script.ts build (cwd=${packageRoot})`);
    const child = spawnSync("bun", ["./📜️script.ts", "build"], {
      cwd: packageRoot,
      encoding: "utf8",
      maxBuffer: 16 * 1024 * 1024,
      shell: false,
      env: { ...process.env, CARGO_INCREMENTAL: process.env.CARGO_INCREMENTAL ?? "0" },
    });
    const combined = `${child.stdout ?? ""}${child.stderr ?? ""}`;
    for (const line of combined.split(/\r?\n/)) {
      if (line.trim()) progress(line);
    }
    if ((child.status ?? -1) === 0) return 0;
    progress(`[semio-os-mcp] package verb failed (status ${child.status}); falling back to bun nx run ${MCP_BINARY_TARGET}`);
    const nx = spawnSync("bun", ["nx", "run", MCP_BINARY_TARGET, "--output-style=stream"], {
      cwd: repoRoot,
      encoding: "utf8",
      maxBuffer: 16 * 1024 * 1024,
      shell: false,
      env: { ...process.env, CARGO_INCREMENTAL: process.env.CARGO_INCREMENTAL ?? "0" },
    });
    const nxOut = `${nx.stdout ?? ""}${nx.stderr ?? ""}`;
    for (const line of nxOut.split(/\r?\n/)) {
      if (line.trim()) progress(line);
    }
    return nx.status ?? -1;
  },
});

/** 📁 Relative source roots whose bytes decide whether the staged binary is still fresh. */
const MCP_SOURCE_REL_ROOTS = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp",
] as const;

/** 🔏️ Walks one tree for content-hash inputs (Rust sources + cargo manifests + binary gate). */
function collectMcpSourceFiles(repoRoot: string, relativeRoot: string): string[] {
  const absolute = join(repoRoot, relativeRoot);
  if (!existsSync(absolute)) return [];
  const out: string[] = [];
  const stack = [absolute];
  while (stack.length) {
    const current = stack.pop()!;
    let entries;
    try {
      entries = readdirSync(current, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const entry of entries) {
      const path = join(current, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "node_modules" || entry.name === "target" || entry.name === "dist" || entry.name === "generated") continue;
        stack.push(path);
        continue;
      }
      if (!entry.isFile()) continue;
      const isSchemaJson = entry.name.endsWith(".json") && (entry.name.includes("binary-gate") || path.includes("🧬️schema"));
      if (entry.name.endsWith(".rs") || entry.name === "Cargo.toml" || isSchemaJson) {
        out.push(path);
      }
    }
  }
  return out.sort();
}

/** 🔐 Content hash of the MCP package sources the staged binary is built from (blake2 not required — sha256 of sorted path+bytes). */
export function mcpSourceContentHash(repoRoot: string): string {
  const hash = createHash("sha256");
  hash.update("semio-os-mcp-source-v1\n");
  for (const relativeRoot of MCP_SOURCE_REL_ROOTS) {
    for (const file of collectMcpSourceFiles(repoRoot, relativeRoot)) {
      hash.update(relative(repoRoot, file).split("\\").join("/"));
      hash.update("\0");
      hash.update(readFileSync(file));
      hash.update("\0");
    }
  }
  return hash.digest("hex");
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

/** 📦️ Stages `semio-os-mcp` when missing or content-hash-stale, reports progress on stderr, returns the verified path.
 *
 * Explicit `SEMIO_OS_MCP_BIN` overrides skip staging (tests inject a subject). Freshness is a
 * content hash of the MCP module's Rust sources + cargo manifests + binary-gate / schema JSON —
 * never mtime — so a warm tree answers in milliseconds and a cold tree rebuilds exactly once.
 * `workspace:setup` / `bun ./📜️script.ts setup` calls this so `.mcp.json`'s handshake never waits
 * on a first-ever compile past the client's initialize timeout. */
export function ensureMcpBinary(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform, staging: McpBinaryStaging = nativeMcpBinaryStagingFor(repoRoot)): string {
  if (env.SEMIO_OS_MCP_BIN) return requireMcpBinary(repoRoot, env, platform);
  const binary = resolveMcpBinaryPath(repoRoot, env, platform);
  const stamp = resolveMcpBinaryContentHashPath(binary);
  const hash = mcpSourceContentHash(repoRoot);
  const progress = (line: string): void => {
    console.error(line);
  };
  let fresh = false;
  try {
    if (statSync(binary).isFile() && existsSync(stamp) && readFileSync(stamp, "utf8").trim() === hash) {
      if (platform === "win32" || (() => { try { accessSync(binary, fsConstants.X_OK); return true; } catch { return false; } })()) fresh = true;
    }
  } catch {
    fresh = false;
  }
  if (fresh) {
    progress(`[semio-os-mcp] staged binary is fresh (content-hash ${hash.slice(0, 12)}…)`);
    return requireMcpBinary(repoRoot, env, platform);
  }
  progress(`[semio-os-mcp] staging binary (content-hash ${hash.slice(0, 12)}…) — progress on stderr; initialize waits for this stage so run \`bun ./📜️script.ts setup\` once for a zero-touch cold start`);
  const status = staging.stage(progress);
  if (status !== 0) throw new Error(`semio-os-mcp staging via ${MCP_BINARY_TARGET} exited with status ${status}`);
  mkdirSync(dirname(binary), { recursive: true });
  writeFileSync(stamp, `${hash}\n`);
  progress(`[semio-os-mcp] staged ${binary}`);
  return requireMcpBinary(repoRoot, env, platform);
}
//#endregion 🔖️BinaryPath

//#region 🔖️RawJsonRpc
export type RawJsonRpcRequest = { readonly jsonrpc: "2.0"; readonly id?: number | string | null; readonly method: string; readonly params?: unknown };
export type RawJsonRpcResponse = { readonly jsonrpc: "2.0"; readonly id: number | string | null; readonly result?: unknown; readonly error?: { readonly code: number; readonly message: string; readonly data?: unknown } };

export type RawMcpProcess = {
  readonly stdoutLines: () => readonly string[];
  readonly stderrText: () => string;
  readonly pid: number | undefined;
  request(method: string, params?: unknown, timeoutMs?: number): Promise<RawJsonRpcResponse>;
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
  const request = async (method: string, params?: unknown, timeoutMs = 120_000): Promise<RawJsonRpcResponse> => {
    const id = nextId++;
    writeRaw(JSON.stringify({ jsonrpc: "2.0", id, method, params }));
    const deadline = Date.now() + timeoutMs;
    while (true) {
      const remaining = Math.max(1, deadline - Date.now());
      const line = await nextLine(remaining);
      const parsed = JSON.parse(line) as RawJsonRpcResponse & { readonly method?: string };
      // Server→client notifications (progress, resources/updated, …) share stdout; skip until the matching id.
      if (parsed.id === id) return parsed;
      if (typeof parsed.method === "string" && (parsed.id === undefined || parsed.id === null)) continue;
      throw new Error(`spawnRawMcp: expected response id ${id} for ${method}, got ${line.slice(0, 240)}`);
    }
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

//#region 🔖️ClientEndToEnd
/** 🪪️ One `.mcp.json` server entry, read verbatim — the literal `command`/`args` a real MCP client
 * (Claude Code, Cursor, the VS Code MCP host) spawns, never a reconstruction of them. */
export type McpServerEntry = { readonly command: string; readonly args: readonly string[] };

/** 🧾️ One step of the client journey: what was attempted, whether it held, and the one-line
 * evidence the report table prints beside it. `wire` is the raw JSON-RPC result the step read. */
export type McpClientStep = { readonly step: string; readonly ok: boolean; readonly detail: string; readonly wire?: unknown };

/** 📨️ A JSON-RPC envelope as it arrives on a server's stdout — a response (`id` present) or a
 * server-initiated notification (`method` present, no `id`), which is how progress travels. */
type McpEnvelope = { readonly jsonrpc: "2.0"; readonly id?: number | string | null; readonly method?: string; readonly params?: unknown; readonly result?: any; readonly error?: { readonly code: number; readonly message: string; readonly data?: unknown } };

/** 🛠️ The tool-call reply shape every `tools/call` answers with — `isError` is the TOOL's own verdict
 * and is independent of the JSON-RPC `error` channel, which only carries protocol faults. */
type McpToolReply = { readonly isError?: boolean; readonly structuredContent?: Record<string, any>; readonly content?: Array<{ readonly text?: string }> };

const MCP_CLIENT_PROTOCOL_VERSION = "2025-06-18";

/** 🔬️ Harness variables a real Claude Code parent exports; kept here because the `semio` server's
 * process-entry credential seal must admit them (ticket 26/09/18 M1 R1) — an e2e client that spawned
 * a pristine environment would never re-prove that. */
const MCP_CLIENT_HARNESS_ENVIRONMENT: Readonly<Record<string, string>> = { CLAUDE_CODE_SESSION_ID: "mcp-client-e2e", CLAUDE_CODE_MESSAGING_TOKEN: "mcp-client-e2e" };

/** 📖️ The `.mcp.json` server table at `repoRoot` — the single source this driver spawns from, so a
 * change to how clients launch a server is a change this gate immediately exercises. */
export function mcpServerEntries(repoRoot: string): Record<string, McpServerEntry> {
  const config = JSON.parse(readFileSync(posix.join(repoRoot, ".mcp.json"), "utf8")) as { mcpServers?: Record<string, McpServerEntry> };
  const servers = config.mcpServers;
  if (!servers || Object.keys(servers).length === 0) throw new Error(".mcp.json declares no mcpServers");
  return servers;
}

/** 🎯️ The ONE plugin, artifact kind and verb the os journey dispatches against. A gate that picked
 * its target from a fuzzy `capabilities_search` measured whichever plugin BM25 happened to rank
 * first that day: the same journey read `animate` on 2026-09-20 18:00 and `energy` four hours later
 * purely because 20 more descriptors became decodable in between (`📓️a3b-descriptor-sweep.md` §6),
 * and `energy` has no compiled component at all — three red rows that say nothing about the
 * dispatch lane. `🗒️note` is the plugin whose guest the two-phase prepare/apply law is proven
 * against (`📓️wr4-typed-command-dispatch-and-gates.md` §5.1, the same `addBlock` verb), and its
 * component is the smallest of the rebuilt set (64 MB against `🌍️gis`'s 202 MB, which is what wedged
 * an earlier run behind a 240 s request wall). `capabilities_search` keeps its own step — see
 * `capabilitySearchRankingVerdict`, which asserts ranking PROPERTIES over the compiled catalog and
 * needs no guest at all. */
export const CLIENT_E2E_PINNED_PLUGIN_ID = "note";
export const CLIENT_E2E_PINNED_ARTIFACT_KIND = "s.note.note";
export const CLIENT_E2E_PINNED_CAPABILITY_ID = "note.s.note.note@1/*#editor.addBlock";

/** 💡️ The inference the journey runs, pinned for the same reason and against the same kind of
 * drift: `inference_list` returns the UNION of every installed plugin's declared roster, and taking
 * `declared[0]` handed the gate `🌍️gis` — whose 202 MB component is the one that wedges a journey
 * and whose guest answers `inference instantiate: wasmtime: failed to convert function to given
 * type` (measured 22:38). `🀄️wfc` declares five real inference services of its own, and its
 * component is staged and current. */
export const CLIENT_E2E_PINNED_INFERENCE_PLUGIN_ID = "wfc";
export const CLIENT_E2E_PINNED_INFERENCE_ARTIFACT_KIND = "s.wfc.bitmap";
export const CLIENT_E2E_PINNED_INFERENCE_SCHEMA = "s.wfc.bitmap.solve";

/** 📁️ Where every plugin component is staged, and in which order a profile wins — the TypeScript
 * twin of `PLUGIN_WASM_TARGET_DIR`/`PLUGIN_WASM_PROFILE_DIRS` in `🌉️mcp/🏠️workspace/🦀️.rs`, so the
 * preflight looks in exactly the places the gateway will look. */
const PLUGIN_WASM_TARGET_REL = ".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2";
const PLUGIN_WASM_PROFILE_DIRS = ["wasm-dev", "wasm-release"] as const;
const PLUGIN_REGISTRY_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json";

/** 🧾️ What the preflight found for one pinned plugin: whether its component is staged AND is the
 * exact build its committed descriptor describes. */
export type StagedComponentVerdict = { readonly ok: boolean; readonly detail: string };

/** 🧱️ Refuses, by name and up front, unless `pluginId`'s compiled component is staged where the
 * gateway resolves it AND is byte-identical to the build its committed descriptor was cut from
 * (`🔣️.json` → `hashes.wasmSha256`).
 *
 * 🐛️ Both failures are otherwise invisible until the journey is minutes deep: a MISSING component
 * surfaces as a `NOT_FOUND` on the third dispatch row, and a STALE one is worse — the catalog types
 * the verb from a descriptor that no longer describes the guest that will run it, so a refusal
 * reads as a dispatch bug. Neither is a statement about the lane this gate exists to measure, so
 * the gate stops here instead of spending a 240 s request wall to say so. */
export function verifyStagedPluginComponent(repoRoot: string, pluginId: string): StagedComponentVerdict {
  const registryPath = posix.join(repoRoot, PLUGIN_REGISTRY_REL);
  let rows: Array<{ pluginId?: string; cratePath?: string; wasmOut?: string }>;
  try {
    rows = JSON.parse(readFileSync(registryPath, "utf8"));
  } catch (error) {
    return { ok: false, detail: `the generated plugin registry ${PLUGIN_REGISTRY_REL} is unreadable (${String(error)}) — run \`bun nx run @semio-tech/plugin-registry:generate\`` };
  }
  const row = rows.find((entry) => entry.pluginId === pluginId);
  if (!row?.cratePath || !row.wasmOut) return { ok: false, detail: `\`${pluginId}\` is not a row of the generated plugin registry — this gate's pinned target names a plugin that is not installed` };
  const ownerRoot = posix.dirname(posix.dirname(posix.join(repoRoot, row.cratePath)));
  const descriptorPath = posix.join(ownerRoot, "🔣️.json");
  let declaredWasmSha256: string | undefined;
  try {
    declaredWasmSha256 = (JSON.parse(readFileSync(descriptorPath, "utf8")) as { hashes?: { wasmSha256?: string } }).hashes?.wasmSha256;
  } catch (error) {
    return { ok: false, detail: `\`${pluginId}\` has no readable committed descriptor at ${descriptorPath} (${String(error)}) — describe it: \`cd ${row.cratePath} && bun ./📜️script.ts describe\`` };
  }
  if (!declaredWasmSha256) return { ok: false, detail: `\`${pluginId}\`'s committed descriptor declares no \`hashes.wasmSha256\`, so a staged component cannot be checked against it` };
  const tried: string[] = [];
  for (const profile of PLUGIN_WASM_PROFILE_DIRS) {
    const candidate = posix.join(repoRoot, PLUGIN_WASM_TARGET_REL, profile, row.wasmOut);
    tried.push(candidate);
    let bytes: Buffer;
    try {
      bytes = readFileSync(candidate);
    } catch {
      continue;
    }
    const staged = createHash("sha256").update(bytes).digest("hex");
    if (staged === declaredWasmSha256) return { ok: true, detail: `${pluginId}: ${profile}/${row.wasmOut} ${bytes.length} B, sha256 ${staged.slice(0, 12)}… matches its committed descriptor` };
    return {
      ok: false,
      detail: `${pluginId}: the staged ${profile}/${row.wasmOut} (${bytes.length} B, sha256 ${staged.slice(0, 12)}…) is NOT the build its committed descriptor describes (${declaredWasmSha256.slice(0, 12)}…) — the catalog would type this verb from a descriptor that no longer describes the guest that runs it; re-describe it: \`cd ${row.cratePath} && bun ./📜️script.ts describe\``,
    };
  }
  return { ok: false, detail: `${pluginId}: no compiled component is staged (tried ${tried.join(", ")}) — build it: \`bun nx run @semio-tech/framework-os-dev:build -- ${pluginId}\`` };
}

/** 🔎️ The ranking properties a compiled capability catalog owes an agent, asserted over one
 * `capabilities_search` reply and NOTHING else — no guest, no component, no artifact. This is the
 * step that used to be implicit in "whatever hit 0 is": a search is a ranking, so what a permanent
 * gate can hold it to is that the ranking is ordered, filtered, unambiguous and reaches the verb a
 * client would be looking for. */
export function capabilitySearchRankingVerdict(hits: ReadonlyArray<Record<string, any>>, mustReach: string, filteredArtifactKind: string): StagedComponentVerdict {
  if (hits.length === 0) return { ok: false, detail: `the compiled catalog returned no mutation of \`${filteredArtifactKind}\` at all` };
  const scores = hits.map((hit) => Number(hit.score));
  const unordered = scores.findIndex((score, index) => index > 0 && score > (scores[index - 1] as number));
  const ids = hits.map((hit) => String(hit.capabilityId));
  const duplicate = ids.find((id, index) => ids.indexOf(id) !== index);
  const foreignAudience = hits.find((hit) => String(hit.audience) !== "agent");
  const foreignKind = hits.find((hit) => String(hit.artifactKind) !== filteredArtifactKind);
  const rank = ids.indexOf(mustReach);
  const faults = [
    unordered >= 0 ? `score rises at hit ${unordered} (${scores[unordered - 1]} → ${scores[unordered]}) — the ranking is not ordered` : "",
    duplicate ? `\`${duplicate}\` appears twice — two catalog rows carry one capability id` : "",
    foreignAudience ? `\`${foreignAudience.capabilityId}\` is audience=${foreignAudience.audience}, but every published hit must be an agent projection` : "",
    foreignKind ? `\`${foreignKind.capabilityId}\` acts on \`${foreignKind.artifactKind}\`, but the search filtered for \`${filteredArtifactKind}\` — the filter is not honored` : "",
    rank < 0 ? `\`${mustReach}\` is not among the ${hits.length} hit(s) — the pinned verb is unreachable by search` : "",
  ].filter((fault) => fault.length > 0);
  return faults.length === 0 ? { ok: true, detail: `${hits.length} mutation hit(s) of \`${filteredArtifactKind}\`, scores ${scores[0]}…${scores[scores.length - 1]} monotonically non-increasing, ids unique, all audience=agent, \`${mustReach}\` at rank ${rank}` } : { ok: false, detail: faults.join("; ") };
}

/** 🔌️ An id-correlated newline-delimited JSON-RPC client over a spawned server's stdio. Unlike
 * `spawnRawMcp`'s next-line reader it never assumes the next stdout line answers the last request:
 * responses are matched by id and everything else (server notifications, `notifications/progress`)
 * is retained, which is the only way a long tool call and its progress stream can be read from one
 * connection. */
export class McpClientSession {
  private readonly child: ChildProcessByStdio<Writable, Readable, Readable>;
  private readonly pending = new Map<number, { resolve: (envelope: McpEnvelope) => void; reject: (error: Error) => void }>();
  private readonly notifications: McpEnvelope[] = [];
  private readonly diagnostics: string[] = [];
  private nextId = 1;
  private exited: number | null = null;

  constructor(entry: McpServerEntry, extraArgs: readonly string[], repoRoot: string) {
    this.child = spawn(entry.command, [...entry.args, ...extraArgs], { cwd: repoRoot, stdio: ["pipe", "pipe", "pipe"], env: { ...process.env, ...MCP_CLIENT_HARNESS_ENVIRONMENT } }) as ChildProcessByStdio<Writable, Readable, Readable>;
    createInterface({ input: this.child.stdout }).on("line", (line) => this.deliver(line));
    this.child.stderr.on("data", (chunk: Buffer) => {
      for (const line of chunk.toString("utf8").split("\n")) if (line.trim()) this.diagnostics.push(line.trim());
    });
    this.child.once("exit", (code) => {
      this.exited = code ?? 0;
      for (const waiter of this.pending.values()) waiter.reject(new Error(`server exited (code ${code}) with pending requests; stderr tail: ${this.diagnostics.slice(-2).join(" | ")}`));
      this.pending.clear();
    });
  }

  private deliver(line: string): void {
    const trimmed = line.trim();
    if (!trimmed) return;
    let envelope: McpEnvelope;
    try {
      envelope = JSON.parse(trimmed) as McpEnvelope;
    } catch {
      this.diagnostics.push(`non-JSON stdout line: ${trimmed.slice(0, 200)}`);
      return;
    }
    if (typeof envelope.id === "number" && this.pending.has(envelope.id)) {
      const waiter = this.pending.get(envelope.id);
      this.pending.delete(envelope.id);
      waiter?.resolve(envelope);
      return;
    }
    this.notifications.push(envelope);
  }

  notify(method: string, params: unknown): void {
    this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method, params })}\n`);
  }

  async request(method: string, params: unknown, budgetMs = 240_000): Promise<McpEnvelope> {
    if (this.exited !== null) throw new Error(`server already exited (code ${this.exited})`);
    const id = this.nextId++;
    const answered = new Promise<McpEnvelope>((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      setTimeout(() => {
        if (!this.pending.delete(id)) return;
        reject(new Error(`${method} did not answer within ${budgetMs}ms; stderr tail: ${this.diagnostics.slice(-2).join(" | ")}`));
      }, budgetMs).unref?.();
    });
    this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
    return answered;
  }

  /** 🛠️ `tools/call`, with the protocol-level error folded into the tool-level reply so a caller
   * asserts one shape; a transport fault can never masquerade as a tool that answered. */
  async call(name: string, args: Record<string, unknown>, budgetMs?: number): Promise<McpToolReply> {
    const response = await this.request("tools/call", { name, arguments: args }, budgetMs);
    if (response.error) return { isError: true, structuredContent: { code: `JSONRPC_${response.error.code}`, message: response.error.message } };
    return (response.result ?? {}) as McpToolReply;
  }

  serverNotifications(): readonly McpEnvelope[] {
    return this.notifications;
  }

  stderrLines(): readonly string[] {
    return this.diagnostics;
  }

  stop(): void {
    this.child.stdin.end();
    this.child.kill();
  }
}

function mcpStepFailure(step: string, detail: string, wire?: unknown): McpClientStep {
  return { step, ok: false, detail, wire };
}

/** 📣️ Prints one finished step immediately. A journey step can take minutes (a cold plugin guest,
 * a real inference), so a driver that only printed at the end would leave a hung step invisible —
 * exactly the failure mode this gate exists to diagnose. */
function announce(steps: McpClientStep[], step: McpClientStep): void {
  steps.push(step);
  console.log(`${step.ok ? "PASS" : "FAIL"}  ${step.step} — ${step.detail}`);
}

/** 🧪️ Builds the smallest input a capability's own published JSON Schema admits — every required
 * property filled with a value of its declared type, recursively for a required object (a
 * `#[dsl(block)]` payload like `setFrame.frame` publishes its own `required` list, and an empty `{}`
 * would be refused by the guest that decodes it). The catalog types action inputs per capability,
 * so an e2e that hardcoded one plugin's argument names would only ever exercise that plugin. */
export function minimalInputForSchema(schema: any): Record<string, unknown> {
  const properties = (schema?.properties ?? {}) as Record<string, any>;
  const required = (schema?.required ?? []) as string[];
  const input: Record<string, unknown> = {};
  for (const name of required) {
    const property = properties[name] ?? {};
    const type = Array.isArray(property.type) ? property.type[0] : property.type;
    if (Array.isArray(property.enum) && property.enum.length > 0) input[name] = property.enum[0];
    else if (type === "number" || type === "integer") input[name] = 0;
    else if (type === "boolean") input[name] = false;
    else if (type === "array") input[name] = [];
    else if (type === "object") input[name] = minimalInputForSchema(property);
    else input[name] = "";
  }
  return input;
}

/** 🧾️ The `RevisionStamp` shape every `action.prepare`/`action.invoke` report carries. */
type RevisionStampWire = { readonly artifactId?: string; readonly headEditId?: string; readonly cursor?: string };

function stampText(stamp: RevisionStampWire | undefined): string {
  return stamp ? `${stamp.artifactId ?? "<no artifact>"}@${stamp.headEditId ?? "<no head>"}/${stamp.cursor ?? "<no cursor>"}` : "<none>";
}

function revisionOf(reply: McpToolReply): RevisionStampWire | undefined {
  return reply.structuredContent?.expectedRevision as RevisionStampWire | undefined;
}

/** 🧊️ The WHOLE document an `artifact_snapshot` reply carries, as one comparable string. A `.spk`
 * `pack` is the GENESIS snapshot (`envelope.vcs.initial_snapshot.encode_pack()`) and never moves
 * again; every committed mutation lands in the `.spr` event log. Comparing `packBase64` alone
 * therefore asserted something the format can never deliver — it read identical across a commit
 * that landed perfectly (measured 2026-09-22, slice CE3). */
function snapshotDocumentBase64(reply: McpToolReply): string {
  const pack = String(reply.structuredContent?.packBase64 ?? "");
  const spr = String(reply.structuredContent?.sprBase64 ?? "");
  return pack.length === 0 && spr.length === 0 ? "" : `${pack}.${spr}`;
}

/** 📄️ Walks one cursor-paginated list method exactly as a spec-strict client does: page one, then
 * every `nextCursor` until there is none. Bounded by a page cap so a server that ever returned a
 * non-advancing cursor fails this gate instead of hanging it. */
async function paginationWalk(session: McpClientSession, method: string, key: string, identify: (entry: any) => string): Promise<{ ok: boolean; detail: string; pages: number; visited: string[]; firstCursor?: string }> {
  const visited: string[] = [];
  const seenCursors = new Set<string>();
  let cursor: string | undefined;
  let firstCursor: string | undefined;
  let pages = 0;
  while (pages < 64) {
    const page = await session.request(method, cursor === undefined ? {} : { cursor });
    if (page.error) return { ok: false, detail: `${method} page ${pages + 1}: ${JSON.stringify(page.error)}`, pages, visited, firstCursor };
    const entries = (page.result?.[key] ?? []) as unknown[];
    visited.push(...entries.map(identify));
    pages += 1;
    const next = page.result?.nextCursor as string | undefined;
    if (next === undefined || next === null) return { ok: true, detail: `${pages} page(s)`, pages, visited, firstCursor };
    if (seenCursors.has(next)) return { ok: false, detail: `${method} repeated cursor ${next} — the walk would never terminate`, pages, visited, firstCursor };
    seenCursors.add(next);
    if (firstCursor === undefined) firstCursor = next;
    cursor = next;
  }
  return { ok: false, detail: `${method} did not finish within 64 pages`, pages, visited, firstCursor };
}

/** 🔭️ The artifact's CURRENT head, re-read live: `action.prepare` opens with a real `ReadHistory`
 * against the owning plugin and returns the head it saw as `expectedRevision`. That is the only
 * revision oracle a headless agent has for a PLUGIN-owned artifact (`artifact_open` knows the
 * gateway's own workspace documents, not a guest's), so undo/redo/rollback are verified against a
 * fresh live read rather than against the report that claimed the change. The preparation itself is
 * pure — it reads history and stages a handle, it never commits. */
async function headRevision(session: McpClientSession, capabilityId: string, input: Record<string, unknown>): Promise<RevisionStampWire | undefined> {
  const probe = await session.call("action_prepare", { capabilityId, input });
  if (probe.isError === true) return undefined;
  const handle = probe.structuredContent?.preparedHandle as string | undefined;
  if (handle) await session.call("action_cancel", { preparedActionHandle: handle });
  return revisionOf(probe);
}

/** 🤝️ Drives the `repo` server exactly as `.mcp.json` launches it: handshake, resource listing and a
 * real read of `repo://goals` — the resource family the repo client exists to serve. */
export async function runRepoMcpClientJourney(repoRoot: string): Promise<readonly McpClientStep[]> {
  const entry = mcpServerEntries(repoRoot).repo;
  if (!entry) return [mcpStepFailure("repo: .mcp.json entry", "`.mcp.json` declares no `repo` server")];
  const session = new McpClientSession(entry, [], repoRoot);
  const steps: McpClientStep[] = [];
  try {
    const initialized = await session.request("initialize", { protocolVersion: MCP_CLIENT_PROTOCOL_VERSION, capabilities: { roots: { listChanged: true }, sampling: {}, elicitation: {} }, clientInfo: { name: "semio-mcp-client-e2e", title: "semio MCP client e2e", version: "1" } });
    if (initialized.error) return [...steps, mcpStepFailure("repo: initialize", JSON.stringify(initialized.error))];
    session.notify("notifications/initialized", {});
    announce(steps, { step: "repo: initialize", ok: true, detail: `server=${initialized.result?.serverInfo?.name}@${initialized.result?.serverInfo?.version} protocol=${initialized.result?.protocolVersion}`, wire: initialized.result });

    const resources = await session.request("resources/list", {});
    const uris = ((resources.result?.resources ?? []) as Array<{ uri: string }>).map((resource) => resource.uri).sort();
    announce(steps, { step: "repo: resources/list", ok: !resources.error && uris.includes("repo://goals"), detail: `${uris.length} resources: ${uris.join(", ")}`, wire: resources.result });

    const goals = await session.request("resources/read", { uri: "repo://goals" });
    const body = String((goals.result?.contents ?? [])[0]?.text ?? "");
    announce(steps, { step: "repo: resources/read repo://goals", ok: !goals.error && body.length > 0, detail: goals.error ? JSON.stringify(goals.error) : `${body.length} byte(s): ${body.slice(0, 120).replace(/\s+/g, " ")}`, wire: goals.result });
    return steps;
  } finally {
    session.stop();
  }
}

/** 🚀️ Drives the `semio` (os gateway) server end to end as a real MCP client would — every step a
 * live JSON-RPC exchange with the process `.mcp.json` names, never an in-process call:
 * `initialize` → `tools/list` → the compiled catalog's own health → `capabilities_search` →
 * `artifact_create` → `action_prepare`/`action_invoke` of a discovered mutation →
 * `artifact_snapshot` (the mutation must be visible) → `history_undo` (it must be gone again) →
 * `inference_run` with its job progress and a cancel. `folder` binds the headless workspace; the
 * capability catalog itself is always compiled from the repo's installed plugin registry, so a
 * throwaway folder still sees every real plugin. */
export async function runOsMcpClientJourney(repoRoot: string, folder: string): Promise<readonly McpClientStep[]> {
  const entry = mcpServerEntries(repoRoot).semio;
  if (!entry) return [mcpStepFailure("os: .mcp.json entry", "`.mcp.json` declares no `semio` server")];
  // 🔒️ `--no-bridge` PINS this gate to the headless lane. Without it the gateway publishes a
  //    rendezvous offer, a developer's live `dev s` shell dials it, `SessionChannelBinding::resolve`
  //    answers `ChannelKind::Shell`, and this `--folder` gate silently measures — and DRIVES — that
  //    human's open document instead of the workspace it names (measured 2026-09-20, WR3 §5.0:
  //    "the shell reports 1 open instance(s)" in a run against a throwaway tmpdir). The shell route
  //    has its own permanent gate, `live-agent-loop-check`; this one owns the headless route.
  const session = new McpClientSession(entry, ["--folder", folder, "--no-bridge"], repoRoot);
  const steps: McpClientStep[] = [];
  try {
    const initialized = await session.request("initialize", { protocolVersion: MCP_CLIENT_PROTOCOL_VERSION, capabilities: { roots: { listChanged: true }, sampling: {}, elicitation: {} }, clientInfo: { name: "semio-mcp-client-e2e", title: "semio MCP client e2e", version: "1" } });
    if (initialized.error) return [...steps, mcpStepFailure("os: initialize", JSON.stringify(initialized.error))];
    session.notify("notifications/initialized", {});
    announce(steps, { step: "os: initialize", ok: true, detail: `server=${initialized.result?.serverInfo?.name}@${initialized.result?.serverInfo?.version} protocol=${initialized.result?.protocolVersion}`, wire: initialized.result });

    // 🧭️ Step 0 of the gate proper: WHICH lane is this run measuring? `context_resolve` takes the
    //    session's channel decision and is sticky, so the value read here is the value every later
    //    tool call uses. A `--folder --no-bridge` run that reports anything but `headless` is not a
    //    headless measurement and every number below it would be about somebody else's shell.
    const context = await session.call("context_resolve", {});
    const resolvedChannel = String(context.structuredContent?.channel ?? "");
    announce(steps, {
      step: "os: context_resolve pins the headless channel",
      ok: context.isError !== true && resolvedChannel === "headless",
      detail:
        context.isError === true
          ? JSON.stringify(context.structuredContent).slice(0, 240)
          : resolvedChannel === "headless"
            ? `channel=headless principal=${context.structuredContent?.principal} session=${context.structuredContent?.sessionId}`
            : `channel=${resolvedChannel || "<none>"} — this gate spawns with --no-bridge and must never resolve a live shell; a shell route means the offer was published anyway`,
      wire: context.structuredContent,
    });
    if (resolvedChannel !== "headless") return steps;

    // 🧱️ The second thing this gate establishes, before it spends a single request on a guest: is
    //    the ONE component every dispatch row below is pinned to staged, and is it the build its
    //    committed descriptor describes? Both answers are a file read; neither is worth a 240 s
    //    request wall to discover.
    const staged = [CLIENT_E2E_PINNED_PLUGIN_ID, CLIENT_E2E_PINNED_INFERENCE_PLUGIN_ID].map((pluginId) => verifyStagedPluginComponent(repoRoot, pluginId));
    announce(steps, { step: "os: every pinned component is staged and current", ok: staged.every((verdict) => verdict.ok), detail: staged.map((verdict) => verdict.detail).join(" | "), wire: staged });
    if (!staged.every((verdict) => verdict.ok)) return steps;

    const tools = await session.request("tools/list", {});
    const toolNames = ((tools.result?.tools ?? []) as Array<{ name: string }>).map((tool) => tool.name).sort();
    const requiredTools = ["action_invoke", "action_prepare", "artifact_create", "artifact_open", "artifact_snapshot", "capabilities_search", "history_undo", "inference_run", "job_cancel"];
    const missingTools = requiredTools.filter((name) => !toolNames.includes(name));
    announce(steps, { step: "os: tools/list", ok: missingTools.length === 0 && toolNames.length >= 9, detail: missingTools.length === 0 ? `${toolNames.length} tools, all ${requiredTools.length} required present` : `missing: ${missingTools.join(", ")}`, wire: toolNames });

    // 📄️ Cursor pagination, walked as a spec-strict client walks it: page one, then every
    // `nextCursor` until there is none, and the concatenation must equal the whole stable list.
    const walk = await paginationWalk(session, "tools/list", "tools", (tool: any) => String(tool.name));
    announce(steps, { step: "os: tools/list pagination walk", ok: walk.ok && walk.visited.join(" ") === toolNames.join(" "), detail: walk.ok ? `${walk.pages} page(s), ${walk.visited.length} tool(s), order identical to the unpaged list` : walk.detail, wire: { pages: walk.pages, firstCursor: walk.firstCursor } });
    const foreignCursor = await session.request("tools/list", { cursor: "someone-elses-cursor" });
    announce(steps, { step: "os: tools/list rejects a foreign cursor", ok: foreignCursor.error?.code === -32602, detail: foreignCursor.error ? `code=${foreignCursor.error.code} ${foreignCursor.error.message}` : "a cursor this server never minted was ACCEPTED", wire: foreignCursor.error ?? foreignCursor.result });
    const resourcesWalk = await paginationWalk(session, "resources/list", "resources", (resource: any) => String(resource.uri));
    const duplicateUris = resourcesWalk.visited.filter((uri, index) => resourcesWalk.visited.indexOf(uri) !== index);
    announce(steps, { step: "os: resources/list pagination + unique URIs", ok: resourcesWalk.ok && duplicateUris.length === 0, detail: resourcesWalk.ok ? `${resourcesWalk.pages} page(s), ${resourcesWalk.visited.length} resource(s), ${duplicateUris.length} duplicate URI(s)` : resourcesWalk.detail, wire: duplicateUris });

    const search = await session.call("capabilities_search", { query: "edit the document" });
    const results = (search.structuredContent?.results ?? []) as Array<Record<string, any>>;
    announce(steps, { step: "os: capabilities_search", ok: search.isError !== true && results.length > 0, detail: `isError=${search.isError === true} hits=${results.length}${results[0] ? ` first=${results[0].capabilityId}` : " — the compiled catalog exposes no plugin capability"}`, wire: results.slice(0, 5) });

    const catalogFaults = session.stderrLines().filter((line) => line.includes("catalog compile failed") || line.includes("skipping plugin"));
    announce(steps, { step: "os: capability catalog health", ok: catalogFaults.length === 0, detail: catalogFaults.length === 0 ? "catalog compiled with zero skips and zero duplicate ids" : `${catalogFaults.length} diagnostic(s), first: ${catalogFaults[0]?.slice(0, 160)}`, wire: catalogFaults });

    const artifactId = `mcp-client-e2e-${Date.now().toString(36)}`;
    const created = await session.call("artifact_create", { artifactId, kind: "os.agent.probe/v1", initial: { note: "created by the MCP client e2e" } });
    announce(steps, { step: "os: artifact_create", ok: created.isError !== true, detail: created.isError === true ? JSON.stringify(created.structuredContent) : `artifactId=${created.structuredContent?.artifactId} revision=${created.structuredContent?.revision?.cursor ?? "<none>"}`, wire: created.structuredContent });
    if (created.isError === true) return steps;

    const opened = await session.call("artifact_open", { artifactId });
    announce(steps, { step: "os: artifact_open", ok: opened.isError !== true, detail: opened.isError === true ? JSON.stringify(opened.structuredContent) : `kind=${opened.structuredContent?.kind} sizeBytes=${opened.structuredContent?.sizeBytes}`, wire: opened.structuredContent });

    // 🔎️ `capabilities_search` keeps a step of ITS OWN, and it is a ranking step: the reply is read
    //    for the properties a ranking owes an agent (ordered, filtered, unambiguous, reaching the
    //    verb a client would look for) over the COMPILED CATALOG alone — no guest, no component, no
    //    artifact. What it deliberately no longer does is hand the dispatch rows below their target.
    const mutations = await session.call("capabilities_search", { query: "add a block to the note", kind: ["mutation"], artifactKind: CLIENT_E2E_PINNED_ARTIFACT_KIND });
    const mutationHits = (mutations.structuredContent?.results ?? []) as Array<Record<string, any>>;
    const ranking = capabilitySearchRankingVerdict(mutationHits, CLIENT_E2E_PINNED_CAPABILITY_ID, CLIENT_E2E_PINNED_ARTIFACT_KIND);
    announce(steps, { step: "os: capabilities_search ranking properties", ok: mutations.isError !== true && ranking.ok, detail: mutations.isError === true ? JSON.stringify(mutations.structuredContent).slice(0, 240) : ranking.detail, wire: mutationHits.slice(0, 5) });

    const described = await session.call("capabilities_describe", { capabilityId: CLIENT_E2E_PINNED_CAPABILITY_ID });
    const inputSchema = described.structuredContent?.inputSchema ?? described.structuredContent?.capability?.inputSchema;
    const input = minimalInputForSchema(inputSchema);
    const describedKind = String(described.structuredContent?.artifactKind ?? described.structuredContent?.capability?.artifactKind ?? "");
    announce(steps, {
      step: "os: capabilities_describe (the pinned verb)",
      ok: described.isError !== true && describedKind === CLIENT_E2E_PINNED_ARTIFACT_KIND,
      detail:
        described.isError === true
          ? `${CLIENT_E2E_PINNED_CAPABILITY_ID}: ${JSON.stringify(described.structuredContent).slice(0, 240)}`
          : describedKind === CLIENT_E2E_PINNED_ARTIFACT_KIND
            ? `${CLIENT_E2E_PINNED_CAPABILITY_ID} artifactKind=${describedKind} input=${JSON.stringify(input).slice(0, 120)}`
            : `${CLIENT_E2E_PINNED_CAPABILITY_ID} is typed against \`${describedKind || "<none>"}\`, not the pinned \`${CLIENT_E2E_PINNED_ARTIFACT_KIND}\``,
      wire: described.structuredContent,
    });

    const typedKind = CLIENT_E2E_PINNED_ARTIFACT_KIND;
    const typedArtifactId = `${artifactId}-typed`;
    const typedCreated = await session.call("artifact_create", { artifactId: typedArtifactId, kind: typedKind });
    announce(steps, { step: "os: artifact_create (a real plugin artifact kind)", ok: typedCreated.isError !== true && typedCreated.structuredContent?.kind === typedKind, detail: typedCreated.isError === true ? `kind=${typedKind}: ${JSON.stringify(typedCreated.structuredContent).slice(0, 260)}` : `artifactId=${typedArtifactId} kind=${typedCreated.structuredContent?.kind} pluginId=${typedCreated.structuredContent?.pluginId} sizeBytes=${typedCreated.structuredContent?.sizeBytes}`, wire: typedCreated.structuredContent });

    if (typedCreated.isError === true || typedCreated.structuredContent?.kind !== typedKind) return steps;

    const exportTarget = typedArtifactId;
    const exported = await session.call("artifact_export", { artifactId: exportTarget });
    announce(steps, { step: "os: artifact_export", ok: exported.isError !== true && typeof exported.structuredContent?.contentBase64 === "string" && (exported.structuredContent?.contentBase64 as string).length > 0, detail: exported.isError === true ? `${exportTarget}: ${JSON.stringify(exported.structuredContent).slice(0, 300)}` : `artifactId=${exportTarget} port=${exported.structuredContent?.format} base64Bytes=${String(exported.structuredContent?.contentBase64 ?? "").length} declaredFormats=${JSON.stringify(exported.structuredContent?.declaredExportFormats ?? [])}`, wire: { ...exported.structuredContent, contentBase64: `${String(exported.structuredContent?.contentBase64 ?? "").slice(0, 32)}…` } });

    // 🎯️ Every dispatch row from here down is the PINNED verb on the PINNED artifact kind, whose
    //    component the step above proved staged and current. Walking on to another hit when one
    //    refuses was tried and REVERTED (WR4 §5.4): each hit belongs to a different plugin, stdio
    //    dispatches one request at a time, and abandoning a call client-side does not free the
    //    server — so one cold 202 MB component (`gis`) wedges every later step behind it. One
    //    plugin per journey is also one cold component compile per journey.
    const beforeSnapshot = await session.call("artifact_snapshot", { artifactId: typedArtifactId });
    const packBefore = String(beforeSnapshot.structuredContent?.packBase64 ?? "");
    const documentBefore = snapshotDocumentBase64(beforeSnapshot);

    const prepared = await session.call("action_prepare", { capabilityId: CLIENT_E2E_PINNED_CAPABILITY_ID, input });
    const baselineRevision = revisionOf(prepared);
    // 🗿️ ONE artifact identity across the whole journey. `RevisionStamp.artifactId` used to carry
    //    the PLUGIN id (`🌉️mcp/🏠️workspace/🦀️.rs` stamped `entry.plugin_id`), so `artifact_create`,
    //    `action_invoke` and `artifact_snapshot` named two different things for one document and
    //    `artifact_snapshot(revisionAfter.artifactId)` answered `no such artifact: note`
    //    (`📓️ce1-…` §8 gap 4). Ticket 26/09/18 slice M8 binds the channel's session document to the
    //    artifact this workspace created from the plugin's genesis; this row is what holds it there.
    announce(steps, { step: "os: the prepared revision names the ARTIFACT, not the plugin", ok: baselineRevision?.artifactId === typedArtifactId, detail: `revision.artifactId=${baselineRevision?.artifactId ?? "<none>"} artifact_create used ${typedArtifactId} (pinned plugin is ${CLIENT_E2E_PINNED_PLUGIN_ID})`, wire: baselineRevision });
    announce(steps, { step: "os: action_prepare", ok: prepared.isError !== true, detail: prepared.isError === true ? `${CLIENT_E2E_PINNED_CAPABILITY_ID} input=${JSON.stringify(input)}: ${JSON.stringify(prepared.structuredContent).slice(0, 260)}` : `handle=${prepared.structuredContent?.preparedHandle} baseline=${stampText(baselineRevision)}`, wire: prepared.structuredContent });
    if (prepared.isError === true) return steps;

    // 🔔️ `resources/subscribe` on the very artifact the prepared action will mutate — the one URI
    // whose `notifications/resources/updated` the commit below must push.
    const watchedArtifact = baselineRevision?.artifactId ?? artifactId;
    const watchedUri = `semio://artifact/${watchedArtifact}`;
    const subscribed = await session.request("resources/subscribe", { uri: watchedUri });
    announce(steps, { step: "os: resources/subscribe", ok: !subscribed.error, detail: subscribed.error ? `${watchedUri}: ${JSON.stringify(subscribed.error)}` : `subscribed to ${watchedUri}`, wire: subscribed.result });
    const unknownSubscribe = await session.request("resources/subscribe", { uri: "semio://there-is-no-such-resource" });
    announce(steps, { step: "os: resources/subscribe refuses an unknown URI", ok: Boolean(unknownSubscribe.error), detail: unknownSubscribe.error ? `code=${unknownSubscribe.error.code} ${unknownSubscribe.error.message.slice(0, 120)}` : "an unserveable URI was accepted into a silent forever-wait", wire: unknownSubscribe.error ?? unknownSubscribe.result });
    const notificationsBefore = session.serverNotifications().length;

    const invoked = await session.call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
    const undoToken = invoked.structuredContent?.undoToken as string | undefined;
    const revisionBefore = invoked.structuredContent?.revisionBefore as RevisionStampWire | undefined;
    const revisionAfter = invoked.structuredContent?.revisionAfter as RevisionStampWire | undefined;
    announce(steps, { step: "os: action_invoke (a real mutation)", ok: invoked.isError !== true && invoked.structuredContent?.status === "SUCCEEDED" && revisionAfter?.headEditId !== undefined && revisionAfter?.headEditId !== revisionBefore?.headEditId, detail: invoked.isError === true ? JSON.stringify(invoked.structuredContent).slice(0, 300) : `status=${invoked.structuredContent?.status} ${stampText(revisionBefore)} → ${stampText(revisionAfter)} undoToken=${undoToken ?? "<none>"}`, wire: invoked.structuredContent });

    // 🔔️ The push the subscription promised: a `notifications/resources/updated` naming the exact
    // URI subscribed to, arriving without a single poll.
    const updates = session.serverNotifications().slice(notificationsBefore).filter((envelope) => envelope.method === "notifications/resources/updated");
    const updatedUris = updates.map((envelope) => String((envelope.params as any)?.uri ?? ""));
    announce(steps, { step: "os: notifications/resources/updated after the commit", ok: updatedUris.includes(watchedUri), detail: updatedUris.includes(watchedUri) ? `${updates.length} update(s), including ${watchedUri}` : `${updates.length} update(s), none naming ${watchedUri}: ${updatedUris.join(", ") || "<none>"}`, wire: updatedUris });
    const unsubscribed = await session.request("resources/unsubscribe", { uri: watchedUri });
    announce(steps, { step: "os: resources/unsubscribe", ok: !unsubscribed.error, detail: unsubscribed.error ? JSON.stringify(unsubscribed.error) : `unsubscribed from ${watchedUri}`, wire: unsubscribed.result });

    // 📦️ The snapshot is taken of the artifact this journey created, by the SAME id
    //    `action_prepare` stamped and `action_invoke` mutated — one identity, asserted above. It is
    //    read from the live guest session the mutation went to, not from the row frozen at create
    //    time, so the DOCUMENT bytes MUST differ across the commit: a snapshot that cannot show the
    //    change the agent just made is worse than no snapshot, and the shipped `mutate-safely`
    //    prompt tells an agent to confirm exactly this way. The document is `pack` AND `spr`, never
    //    `pack` alone — see `snapshotDocumentBase64`.
    const afterSnapshot = await session.call("artifact_snapshot", { artifactId: typedArtifactId });
    const packAfter = String(afterSnapshot.structuredContent?.packBase64 ?? "");
    announce(steps, { step: "os: artifact_snapshot (the created artifact)", ok: afterSnapshot.isError !== true && Number(afterSnapshot.structuredContent?.packBytes ?? 0) > 0, detail: afterSnapshot.isError === true ? `${typedArtifactId}: ${JSON.stringify(afterSnapshot.structuredContent).slice(0, 300)}` : `artifactId=${typedArtifactId} packBytes=${afterSnapshot.structuredContent?.packBytes} sprBytes=${afterSnapshot.structuredContent?.sprBytes}`, wire: { ...afterSnapshot.structuredContent, packBase64: `${packAfter.slice(0, 32)}…` } });
    const documentAfter = snapshotDocumentBase64(afterSnapshot);
    announce(steps, { step: "os: the snapshot shows the mutation", ok: documentBefore.length > 0 && documentAfter.length > 0 && documentAfter !== documentBefore, detail: documentBefore.length === 0 ? `the pre-mutation snapshot answered no bytes: ${JSON.stringify(beforeSnapshot.structuredContent).slice(0, 200)}` : documentAfter === documentBefore ? `${typedArtifactId} is byte-identical across the commit (pack ${packAfter.length}, spr ${String(afterSnapshot.structuredContent?.sprBase64 ?? "").length} base64 chars) — the snapshot is reading a frozen row, not the document the mutation went to` : `${documentBefore.length} → ${documentAfter.length} base64 chars across the commit (pack ${packBefore.length} → ${packAfter.length}, spr ${String(beforeSnapshot.structuredContent?.sprBase64 ?? "").length} → ${String(afterSnapshot.structuredContent?.sprBase64 ?? "").length})`, wire: { before: documentBefore.slice(0, 24), after: documentAfter.slice(0, 24) } });

    const headAfterInvoke = await headRevision(session, CLIENT_E2E_PINNED_CAPABILITY_ID, input);
    announce(steps, { step: "os: live head advanced", ok: headAfterInvoke !== undefined && headAfterInvoke.headEditId === revisionAfter?.headEditId && headAfterInvoke.headEditId !== baselineRevision?.headEditId, detail: `re-read head ${stampText(headAfterInvoke)} (baseline ${stampText(baselineRevision)}, invoke reported ${stampText(revisionAfter)})`, wire: headAfterInvoke });

    if (!undoToken) {
      announce(steps, mcpStepFailure("os: history_undo (mutation reverted)", "action_invoke minted no undoToken — nothing to undo", invoked.structuredContent));
      return steps;
    }
    const undone = await session.call("history_undo", { undoToken });
    const headAfterUndo = await headRevision(session, CLIENT_E2E_PINNED_CAPABILITY_ID, input);
    // ⚠️ `members` is the COUNT OF MEMBERS ATTEMPTED (`ActionAdapter::fan_out` returns
    //    `undo.members.len()` whether or not a member failed) — a per-member failure travels in
    //    `warnings`, and only an all-members failure is a tool error. A gate that read `members > 0`
    //    alone therefore passed on a fan-out where every member warned, so `warnings` is asserted.
    const undoWarnings = (undone.structuredContent?.warnings ?? []) as string[];
    announce(steps, { step: "os: history_undo (mutation reverted)", ok: undone.isError !== true && Number(undone.structuredContent?.members ?? 0) > 0 && undoWarnings.length === 0 && headAfterUndo?.headEditId === baselineRevision?.headEditId, detail: undone.isError === true ? JSON.stringify(undone.structuredContent).slice(0, 300) : `members=${undone.structuredContent?.members} warnings=${undoWarnings.length === 0 ? "none" : undoWarnings.join(" / ").slice(0, 200)} head ${stampText(headAfterUndo)} (baseline ${stampText(baselineRevision)})`, wire: undone.structuredContent });

    const redone = await session.call("history_redo", { undoToken });
    const headAfterRedo = await headRevision(session, CLIENT_E2E_PINNED_CAPABILITY_ID, input);
    const redoWarnings = (redone.structuredContent?.warnings ?? []) as string[];
    announce(steps, { step: "os: history_redo (mutation restored)", ok: redone.isError !== true && Number(redone.structuredContent?.members ?? 0) > 0 && redoWarnings.length === 0 && headAfterRedo?.headEditId !== undefined && headAfterRedo.headEditId !== baselineRevision?.headEditId, detail: redone.isError === true ? JSON.stringify(redone.structuredContent).slice(0, 300) : `members=${redone.structuredContent?.members} warnings=${redoWarnings.length === 0 ? "none" : redoWarnings.join(" / ").slice(0, 200)} head ${stampText(headAfterRedo)} (undone head was ${stampText(headAfterUndo)})`, wire: redone.structuredContent });

    const sagaMember = await session.call("action_prepare", { capabilityId: CLIENT_E2E_PINNED_CAPABILITY_ID, input });
    const began = await session.call("transaction_begin", { preparedHandles: [sagaMember.structuredContent?.preparedHandle] });
    const transactionHandle = began.structuredContent?.transactionHandle as string | undefined;
    announce(steps, { step: "os: transaction_begin", ok: began.isError !== true && typeof transactionHandle === "string", detail: began.isError === true ? JSON.stringify(began.structuredContent).slice(0, 300) : `transactionHandle=${transactionHandle} members=1`, wire: began.structuredContent });
    const rolledBack = await session.call("transaction_rollback", { transactionHandle });
    const headAfterRollback = await headRevision(session, CLIENT_E2E_PINNED_CAPABILITY_ID, input);
    announce(steps, { step: "os: transaction_rollback (no change)", ok: rolledBack.isError !== true && headAfterRollback?.headEditId === headAfterRedo?.headEditId, detail: rolledBack.isError === true ? JSON.stringify(rolledBack.structuredContent).slice(0, 300) : `rolledBack=${rolledBack.structuredContent?.rolledBack} head ${stampText(headAfterRollback)} unchanged from ${stampText(headAfterRedo)}`, wire: rolledBack.structuredContent });

    const listed = await session.call("inference_list", {});
    const declared = (listed.structuredContent?.declared ?? []) as Array<Record<string, any>>;
    // 💡️ The roster must CONTAIN the pinned service — the list is a union over every installed
    //    plugin's declared roster, so which entry is first is another thing the gate must not read.
    const service = declared.find((row) => row.artifactKind === CLIENT_E2E_PINNED_INFERENCE_ARTIFACT_KIND && row.inferenceSchema === CLIENT_E2E_PINNED_INFERENCE_SCHEMA);
    // 📜️ A declared row is not enough: an agent also has to be able to READ what to send. The row
    //    must publish its payload contract (`payloadSchemaId` + an input schema), because without
    //    one `inference_run` can only ever dispatch `{}` and the guest answers a decode fault after
    //    the whole component has been compiled (`📓️pz2-…md` §5.2 measured 240 s of exactly that).
    const contract = service?.payload as Record<string, any> | undefined;
    const contractOk = typeof contract?.payloadSchemaId === "string" && typeof contract?.inputSchema === "string" && contract.inputSchema.length > 0;
    announce(steps, { step: "os: inference_list declares the pinned service", ok: listed.isError !== true && service !== undefined && contractOk, detail: listed.isError === true ? JSON.stringify(listed.structuredContent).slice(0, 200) : service ? `${declared.length} declared inference(s), including ${CLIENT_E2E_PINNED_INFERENCE_ARTIFACT_KIND}/${CLIENT_E2E_PINNED_INFERENCE_SCHEMA} by ${service.contributor || service.owner}${contractOk ? `, contract ${contract?.payloadSchemaId} (binds \`${contract?.artifactBinding?.field ?? "<none>"}\`, ${String(contract?.inputSchema ?? "").length} B input schema)` : ", publishing NO payload contract — a client cannot know what to send"}` : `${declared.length} declared inference(s), none of them ${CLIENT_E2E_PINNED_INFERENCE_ARTIFACT_KIND}/${CLIENT_E2E_PINNED_INFERENCE_SCHEMA}: ${declared.map((row) => `${row.artifactKind}/${row.inferenceSchema}`).join(", ")}`, wire: declared.slice(0, 8) });
    if (!service) return steps;
    // 🗿️ The artifact the inference is RUN ON — its own kind, created through the same
    //    `artifact_create` every other artifact in this journey goes through. It carries no row of
    //    its own on purpose: the denominator stays comparable across slices, and a create that
    //    fails is visible in the `inference_run` row it serves (the gateway then refuses by name,
    //    `INPUT_INVALID … artifactId`, instead of dispatching a body no guest can decode).
    const inferenceArtifactId = `${artifactId}-inference`;
    const inferenceArtifact = await session.call("artifact_create", { artifactId: inferenceArtifactId, kind: CLIENT_E2E_PINNED_INFERENCE_ARTIFACT_KIND });
    if (inferenceArtifact.isError === true) console.log(`[client-e2e] artifact_create ${CLIENT_E2E_PINNED_INFERENCE_ARTIFACT_KIND}: ${JSON.stringify(inferenceArtifact.structuredContent).slice(0, 200)}`);
    // 📈️ The spec's own progress mechanism, on the one tool that mints a job: the call carries
    // `_meta.progressToken`, and every `JobRegistry` row of that job must arrive as a
    // `notifications/progress` carrying that exact token — no `job_get` poll in this step.
    const progressToken = `m5b-${Date.now().toString(36)}`;
    const progressBefore = session.serverNotifications().length;
    // ⏳️ A request that never answers must become a NAMED RED, not an exception: an uncaught
    //    rejection here aborts the journey before it prints a tally or a single `FAIL` line, which
    //    is strictly worse than a red row (WR4 §5.4 recorded the same trap for a cold `gis`).
    //    Measured 2026-09-20: `s.wfc.bitmap.solve` did not answer within **900 000 ms** — that is
    //    not a cold-compile budget, it is a solve that does not return, so the budget stays at the
    //    journey's own 240 s and the row says so.
    const inferenceEnvelope = await session
      .request("tools/call", { name: "inference_run", arguments: { artifactKind: service.artifactKind, inferenceSchema: service.inferenceSchema, pluginId: CLIENT_E2E_PINNED_INFERENCE_PLUGIN_ID, artifactId: inferenceArtifactId, cancellationId: `${artifactId}-cancel` }, _meta: { progressToken } })
      .catch((error: Error): McpEnvelope => ({ jsonrpc: "2.0", id: null, error: { code: -32000, message: error.message } }));
    const inference = (inferenceEnvelope.error ? { isError: true, structuredContent: { code: `JSONRPC_${inferenceEnvelope.error.code}`, message: inferenceEnvelope.error.message } } : (inferenceEnvelope.result ?? {})) as McpToolReply;
    const progressRows = session.serverNotifications().slice(progressBefore).filter((envelope) => envelope.method === "notifications/progress" && (envelope.params as any)?.progressToken === progressToken);
    announce(steps, { step: "os: notifications/progress for _meta.progressToken", ok: progressRows.length > 0, detail: progressRows.length > 0 ? `${progressRows.length} progress row(s), last progress=${(progressRows[progressRows.length - 1]?.params as any)?.progress}` : `no progress notification carried token ${progressToken} — the job reported none, or the push is not wired`, wire: progressRows.map((envelope) => envelope.params) });
    // 🛑️ MCP's own request-scoped cancellation. stdio dispatches one request at a time, so this
    // arrives after the call it names returned: what it proves here is that the server accepts the
    // notification, answers nothing (JSON-RPC's rule for notifications) and stays responsive.
    session.notify("notifications/cancelled", { requestId: inferenceEnvelope.id ?? 0, reason: "client-e2e cancellation probe" });
    // ⏳️ Same law as the `inference_run` call above, and for the same reason: this `ping` follows a
    //    call that can still be busy in the guest, and an uncaught rejection here abandons the
    //    journey before it prints a tally or a single `FAIL` line. Measured 2026-09-22 (slice CE3):
    //    a `s.wfc.bitmap.solve` that did not return inside its own 240 s budget left this `ping`
    //    unanswered too, and the whole gate died with a stack trace at 27 printed rows.
    const stillAlive = await session.request("ping", {}).catch((error: Error): McpEnvelope => ({ jsonrpc: "2.0", id: null, error: { code: -32000, message: error.message } }));
    announce(steps, { step: "os: notifications/cancelled is accepted and unanswered", ok: !stillAlive.error, detail: stillAlive.error ? `the server stopped answering after notifications/cancelled: ${JSON.stringify(stillAlive.error)}` : "cancelled accepted with no response; ping still answers", wire: stillAlive.result });
    // 🏃️ A REFUSED inference still minted a job — the gateway registers the row before it dispatches
    //    and reports it back inside the error's own `details` — so the job rows below are read from
    //    whichever of the two places the reply carries it. Reading only the success shape made
    //    `job_get`/`job_cancel` unreachable the moment the plugin refused, which is exactly when a
    //    client most needs them to work.
    const jobId = (inference.structuredContent?.jobId ?? (inference.structuredContent?.details as Record<string, unknown> | undefined)?.jobId) as string | undefined;
    announce(steps, { step: "os: inference_run", ok: inference.isError !== true, detail: inference.isError === true ? JSON.stringify(inference.structuredContent).slice(0, 300) : `jobId=${jobId} artifactId=${inference.structuredContent?.artifactId || "<none>"} status=${inference.structuredContent?.status} complete=${inference.structuredContent?.complete} bytes=${inference.structuredContent?.payloadBytes ?? 0}`, wire: inference.structuredContent });

    if (jobId) {
      const progress = await session.call("job_get", { jobId });
      announce(steps, { step: "os: inference job progress", ok: progress.isError !== true && typeof progress.structuredContent?.status === "string", detail: progress.isError === true ? JSON.stringify(progress.structuredContent).slice(0, 200) : `status=${progress.structuredContent?.status} progress=${progress.structuredContent?.progress ?? "<none>"} message=${progress.structuredContent?.message ?? "<none>"}`, wire: progress.structuredContent });
      const cancelled = await session.call("job_cancel", { jobId });
      const cancelCode = cancelled.structuredContent?.code;
      announce(steps, { step: "os: inference cancel", ok: cancelled.isError !== true || cancelCode === "PRECONDITION_FAILED", detail: cancelled.isError === true ? `${cancelCode}: ${String(cancelled.structuredContent?.message).slice(0, 160)}` : `status=${cancelled.structuredContent?.status} cancelRequested=${cancelled.structuredContent?.cancelRequested}`, wire: cancelled.structuredContent });
    } else {
      announce(steps, mcpStepFailure("os: inference job progress", "inference_run minted no jobId", inference.structuredContent));
    }
    return steps;
  } finally {
    session.stop();
  }
}

/** 🏁️ Both `.mcp.json` servers, one report. Returns every step so a caller can print the table and
 * exit non-zero on the first red — the gate never summarises a failure away. */
export async function runMcpClientEndToEnd(repoRoot: string, folder: string): Promise<readonly McpClientStep[]> {
  const os = await runOsMcpClientJourney(repoRoot, folder);
  const repo = await runRepoMcpClientJourney(repoRoot);
  return [...os, ...repo];
}
//#endregion 🔖️ClientEndToEnd

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts");
  await registerTests1(
    import.meta.vitest,
    {
      chmodSync,
      copyFileSync,
      ensureMcpBinary,
      join,
      mcpSourceContentHash,
      mkdirSync,
      mkdtempSync,
      readFileSync,
      requireMcpBinary,
      resolveBuiltMcpBinaryPath,
      resolveMcpBinaryContentHashPath,
      resolveMcpBinaryPath,
      rmSync,
      tmpdir,
      writeFileSync,
    },
    { directory: import.meta.dir, url: import.meta.url },
  );
}
//#endregion 🧪️Tests
