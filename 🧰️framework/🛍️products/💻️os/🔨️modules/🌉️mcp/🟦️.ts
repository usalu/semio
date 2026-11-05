import {MCP_MANIFEST_REL} from "./📡️stdio/🏗️staging/🟦️.ts";
import {readNativeCatalogV1} from "../🔌️plugin/📇️registry/📥️runtime/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../📦️packages/🟦️typescript/🧬️schema/🟦️.ts";
import {parseActionPrepareInput,parseInferenceRunInput,parseInferenceSubmitInput} from "./🧬️schema/🟦️.ts";
export {buildMcpArtifactV1,MCP_MANIFEST_REL} from "./📡️stdio/🏗️staging/🟦️.ts";
import{mcpJsonPathV1,mcpJsonArrayV1}from"./📡️stdio/🎫️session/📋️data/🟦️.ts";
import{withMcpClientSessionV1,type McpClientSessionV1}from"./📡️stdio/🎫️session/🟦️.ts";
import { runToolSyncV1, startToolV1 } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
export{withMcpClientSessionV1}from"./📡️stdio/🎫️session/🟦️.ts";
export type{McpClientSessionV1}from"./📡️stdio/🎫️session/🟦️.ts";
import{createRequestedIssuedResourceControlV1,type ScriptInvocation}from"../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{type ResourceLeaseControl}from"../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
/** 🌉️ TypeScript surface of the `🌉️mcp` module (packet `P5-conformance-tests`) — the pieces the
 * conformance test suite under `📦️packages/🟦️typescript` shares: where the real `semio-os-mcp`
 * stdio binary lives, a minimal raw newline-delimited JSON-RPC client for the modern era the
 * installed `@modelcontextprotocol/sdk` (1.30.0, legacy-only — `📓️design-decisions.md` D1) cannot
 * speak, and a JSON Schema 2020-12 validator wrapper. Never touches the Rust crate directly — this
 * module only ever crosses the process boundary over real stdio, exactly like a real IDE client.
 */

import { type ChildProcessByStdio, spawn } from "node:child_process";
import { createHash,randomUUID } from "node:crypto";
import { accessSync, constants as fsConstants, readdirSync, readFileSync, statSync } from "node:fs";
import { join, posix, resolve, win32 } from "node:path";
import { createInterface } from "node:readline";
import { serveWarmMcpGateway } from "./📡️stdio/🚪️gateway/🟦️.ts";
import type { Readable, Writable } from "node:stream";
import { cargoTargetDirectory } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { CARGO_BUILD_OWNER_PID_ENV, cargoBinarySourcesFreshnessV1, parseCargoBinarySourcesV1 } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";

//#region 🔖️BinaryPath
/** 📦️ The build target owns the executable separately from mutable compiler state. */
export const MCP_ARTIFACT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build";
/** 📦️ Where the optimized distribution binary is staged. Separate from {@link MCP_ARTIFACT_REL} so a
 * release build never clobbers the dev-loop artifact `.mcp.json` and every black-box gate exec. */
export const MCP_RELEASE_ARTIFACT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release";
export const MCP_CARGO_PACKAGE = "semio-os-mcp";
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

/** 📍️ Selects the executable from its owning admitted compiler publication. */
export function resolvePublishedMcpBinaryPathV1(publication: string, platform: NodeJS.Platform = process.platform): string {
 return pathApi(platform).join(publication, platform === "win32" ? MCP_BINARY_NAME + ".exe" : MCP_BINARY_NAME);
}

/** 📁️ Resolves the staged executable or an explicit independent binary override. */
export function resolveMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const override = env.SEMIO_OS_MCP_BIN;
  return override ? pathApi(platform).resolve(repoRoot, override) : pathApi(platform).resolve(repoRoot, MCP_ARTIFACT_REL, platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME);
}

/** 🎯 The neutral target that stages the debug `semio-os-mcp` every `.mcp.json` / `dev mcp` route reads. */
/** 🌱️ The authored interactive MCP compiler intent, independent of shell configuration. */
export const MCP_STDIO_BUILD_MODE_V1 = "prototype" as const;

export const MCP_BINARY_TARGET = "@semio-tech/framework-os-mcp-rs:build";

/** 🧾️ The sources record the build stages beside the binary (`buildCargoArtifacts`' `sourcesRecord`):
 * every file Cargo compiled it from and when that build started. */
export const MCP_BINARY_SOURCES_FILE = "semio-os-mcp.sources.json";

/** 🧾️ Where the sources record of `binary` lives — beside it, in the same atomic publication. */
export function resolveMcpBinarySourcesPath(binary: string, platform: NodeJS.Platform = process.platform): string {
  return pathApi(platform).join(pathApi(platform).dirname(binary), MCP_BINARY_SOURCES_FILE);
}

/** 🏗️ Injectable staging boundary: one attempt returning the neutral build target's exit status. */
export type McpBinaryStaging = Readonly<{ stage: (progress: (line: string) => void) => number }>;

const mcpStagingEnvironment = (environment: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv => ({ ...environment, CARGO_INCREMENTAL: environment.CARGO_INCREMENTAL ?? "0", [CARGO_BUILD_OWNER_PID_ENV]: String(process.pid) });

/** 🏗️ Non-blocking staging: the neutral build target runs as a child while the caller keeps serving; resolves with its exit status, `-1` when it cannot start. */
export function stageMcpBinaryAsync(repoRoot: string, progress: (line: string) => void, signal?: AbortSignal, target: string = MCP_BINARY_TARGET, environment: NodeJS.ProcessEnv = process.env): Promise<number> {
  return new Promise((resolveStatus) => {
    progress(`[semio-os-mcp] staging via repo run ${target}`);
    const child = startToolV1("repo", ["run", target, "--output=stream"], { cwd: repoRoot, shell: false, env: mcpStagingEnvironment(environment), stdio: ["ignore", "pipe", "pipe"], signal });
    for (const stream of [child.stdout, child.stderr]) createInterface({ input: stream }).on("line", (line) => void (line.trim() && progress(line)));
    child.on("error", (error) => {
      progress(`[semio-os-mcp] ${error.message}`);
      resolveStatus(-1);
    });
    child.on("close", (code) => resolveStatus(code ?? -1));
  });
}

const nativeMcpBinaryStagingFor = (repoRoot: string): McpBinaryStaging => ({
  stage: (progress) => {
    progress(`[semio-os-mcp] staging via repo run ${MCP_BINARY_TARGET}`);
    const result = runToolSyncV1("repo", ["run", MCP_BINARY_TARGET, "--output=stream"], {
      cwd: repoRoot,
      encoding: "utf8",
      maxBuffer: 16 * 1024 * 1024,
      shell: false,
      env: mcpStagingEnvironment(),
    });
    const combined = `${result.stdout ?? ""}${result.stderr ?? ""}`;
    for (const line of combined.split(/\r?\n/)) {
      if (line.trim()) progress(line);
    }
    return result.status ?? -1;
  },
});

/** 🕰️ Whether the staged binary is still what its sources build, and if not, the first source that
 * says so: a missing binary or record, a vanished source, or one modified after the build started. */
export function mcpBinaryFreshness(binary: string, platform: NodeJS.Platform = process.platform, modifiedAtMs: (path: string) => number | null = (path) => {
  try {
    return statSync(path).mtimeMs;
  } catch {
    return null;
  }
}): { readonly fresh: boolean; readonly reason: string } {
  if (modifiedAtMs(binary) === null) return { fresh: false, reason: "no staged binary" };
  let text: string;
  try {
    text = readFileSync(resolveMcpBinarySourcesPath(binary, platform), "utf8");
  } catch {
    return { fresh: false, reason: "the staged binary carries no sources record" };
  }
  const record = parseCargoBinarySourcesV1(text);
  if (record === null) return { fresh: false, reason: "the staged binary's sources record is malformed" };
  const verdict = cargoBinarySourcesFreshnessV1(record, modifiedAtMs);
  return verdict.fresh ? { fresh: true, reason: `${record.sources.length} sources unchanged since ${new Date(record.builtAtMs).toISOString()}` } : { fresh: false, reason: `${verdict.changed} changed after the staged build` };
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

/** 📦️ Stages `semio-os-mcp` when it is missing or stale, reports progress on stderr, returns the verified path.
 *
 * Explicit `SEMIO_OS_MCP_BIN` overrides skip staging (tests inject a subject). Freshness is the
 * staged build's own sources record ({@link mcpBinaryFreshness}): Cargo's dep-info for the binary,
 * i.e. the whole dependency closure — a kernel or plugin-host change the gateway links is as stale as
 * an edit in this module — stat-checked, so a warm tree answers in milliseconds. Any build of the neutral
 * target (a gate's `dependsOn`, `repo ./📜️script.ts build`) publishes the record with the binary, so
 * `.mcp.json`'s `initialize` never re-stages a binary a build just produced. `workspace:setup` /
 * `repo ./📜️script.ts setup` calls this to stage ahead of time; a client that starts first is served by
 * {@link serveWarmMcpStdio}, which never blocks `initialize` on a build. */
export function ensureMcpBinary(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform, staging: McpBinaryStaging = nativeMcpBinaryStagingFor(repoRoot)): string {
  if (env.SEMIO_OS_MCP_BIN) return requireMcpBinary(repoRoot, env, platform);
  const binary = resolveMcpBinaryPath(repoRoot, env, platform);
  const freshness = mcpBinaryFreshness(binary, platform);
  if (freshness.fresh) {
    console.error(`[semio-os-mcp] staged binary is fresh (${freshness.reason})`);
    return requireMcpBinary(repoRoot, env, platform);
  }
  console.error(`[semio-os-mcp] staging binary (${freshness.reason}) — progress on stderr`);
  const status = staging.stage((line) => console.error(line));
  if (status !== 0) throw new Error(`semio-os-mcp staging via ${MCP_BINARY_TARGET} exited with status ${status}`);
  console.error(`[semio-os-mcp] staged ${binary}`);
  return requireMcpBinary(repoRoot, env, platform);
}

/** 🏛️ The authored embedding policy of the host driver (finite wake, drive and operation currency plus the root turn limits) that `semio-os-mcp` requires through `--host-driver-policy`; schema: `🔌️plugin/🖥️host/🔔️wake/📃️policy/🧬️schema`. */
export const MCP_HOST_DRIVER_POLICY_PATH = join(import.meta.dirname, "🎚️config", "🏛️host-driver-policy.json");

/** 💡️ Declares cleanup currency for each separately authorized inference operation. */
export const MCP_HOST_OPERATION_POLICY_PATH = join(import.meta.dirname, "🎚️config", "💡️host-operation-policy.json");

/** 🧷️ Applies declared host and inference policies once, preserving caller paths. */
export function withMcpHostDriverPolicy(args: readonly string[]): string[] {
  const configured = args.includes("--host-driver-policy") ? [...args] : [...args, "--host-driver-policy", MCP_HOST_DRIVER_POLICY_PATH];
  return configured.includes("--operation-policy") ? configured : [...configured, "--operation-policy", MCP_HOST_OPERATION_POLICY_PATH];
}

/** 📇️ Takes the one explicit `--catalog <root>` off a launch argument list: the plugin catalog publication (`🦀️catalog.json` + `🔌️plugins.json`) the gateway compiles its capabilities from. The binary infers none from the repository (without it the catalog holds only the gateway's own tools), so the composition names it; `SEMIO_PLUGIN_CATALOG` in the caller's environment is the same input and `--catalog` wins. */
export function takeMcpPluginCatalog(args: readonly string[], repoRoot: string): { readonly args: string[]; readonly env: NodeJS.ProcessEnv; readonly target?: string } {
  const buildAt = args.indexOf("--catalog-target");
  const target = buildAt < 0 ? undefined : args[buildAt + 1];
  if (buildAt >= 0 && (!target || !/^@[a-z0-9-]+\/[a-z0-9-]+:[a-z0-9-]+$/u.test(target) || args.indexOf("--catalog-target", buildAt + 1) >= 0)) throw Error("--catalog-target takes exactly one owning build target");
  args = buildAt < 0 ? args : [...args.slice(0, buildAt), ...args.slice(buildAt + 2)];
  const at = args.indexOf("--catalog");
  if (at < 0) { if (target) throw Error("--catalog-target requires --catalog"); return { args: [...args], env: {} }; }
  const value = args[at + 1];
  if (!value || value.startsWith("-") || args.indexOf("--catalog", at + 1) >= 0) throw new Error("--catalog takes exactly one plugin catalog root");
  return { args: [...args.slice(0, at), ...args.slice(at + 2)], env: { SEMIO_PLUGIN_CATALOG: resolve(repoRoot, value) }, ...(target ? {target} : {}) };
}

/** 🔥️ Whether `initialize` can be answered by the real server immediately: an explicit override, or a staged binary whose sources are unchanged. */
export function mcpBinaryIsStaged(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): boolean {
  return Boolean(env.SEMIO_OS_MCP_BIN) || mcpBinaryFreshness(resolveMcpBinaryPath(repoRoot, env, platform), platform).fresh;
}

/** ⏹️ Stops publication preparation on cancellation without retaining a pending waiter. */
export function waitForMcpPublicationV1(binary: string | Promise<string>, signal: AbortSignal): Promise<string> {
 return new Promise((resolve,reject) => {
  const abort = () => {signal.removeEventListener("abort",abort);reject(signal.reason);};
  if (signal.aborted) return abort();
  signal.addEventListener("abort",abort,{once:true});
  void Promise.resolve(binary).then(value => {signal.removeEventListener("abort",abort);resolve(value);},error => {signal.removeEventListener("abort",abort);reject(error);});
 });
}

/** 🚪️ Serves `semio-os-mcp stdio …` for a client whose binary is missing or stale: the handshake is answered at once, staging runs in the background, and the real server takes over through `list_changed` notifications — so a cold build never lands inside the client's connect timeout. */
export function serveWarmMcpStdio(repoRoot: string, extra: readonly string[], env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform, binary: string | Promise<string> = resolveMcpBinaryPath(repoRoot, env, platform), catalogTarget?: string, stage: (target: string, progress: (line: string) => void, signal: AbortSignal) => Promise<number> = (target, progress, signal) => stageMcpBinaryAsync(repoRoot, progress, signal, target), onStop?: () => void): Promise<number> {
  let selected: string;
  return serveWarmMcpGateway({ stage: async (progress, signal) => { selected = await waitForMcpPublicationV1(binary, signal); signal.throwIfAborted(); const status = env.SEMIO_OS_MCP_BIN || mcpBinaryFreshness(selected, platform).fresh ? 0 : await stage(MCP_BINARY_TARGET, progress, signal); return status || (catalogTarget ? await stage(catalogTarget, progress, signal) : 0); }, command: () => { if (!statSync(selected).isFile()) throw Error("Published MCP executable required"); if (platform !== "win32") accessSync(selected, fsConstants.X_OK); return selected; }, args: withMcpHostDriverPolicy(["stdio", ...extra]), cwd: repoRoot, env, input: process.stdin, output: process.stdout, onStop });
}

/** 🗄️ Keeps the original compiler publication admitted throughout the stdio server lifetime. */
export async function servePublishedMcpStdioV1(repoRoot:string,extra:readonly string[],invocation:ScriptInvocation,control:ResourceLeaseControl,env:NodeJS.ProcessEnv=process.env,catalogTarget?:string,stage:(target:string,progress:(line:string)=>void,signal:AbortSignal)=>Promise<number>=(target,progress,signal)=>stageMcpBinaryAsync(repoRoot,progress,signal,target),onStop?:(reason:unknown)=>void):Promise<number>{
 const {withAdmittedPublishedCargoExecutableV1}=await import("../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts");
 const {readBuildMode}=await import("../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts");
 const mode=readBuildMode(env.SEMIO_BUILD_MODE??MCP_STDIO_BUILD_MODE_V1);
 if(env.SEMIO_OS_MCP_BIN)return serveWarmMcpStdio(repoRoot,extra,env,process.platform,resolveMcpBinaryPath(repoRoot,env),catalogTarget,stage);
 const stopped=new AbortController(),phase=createRequestedIssuedResourceControlV1(control,0,stopped.signal);
 let selected:string,acquired=false,release!:()=>void,publication:Promise<number>|undefined,preparing:Promise<number>|undefined,stageSignal:AbortSignal|undefined;
 const closed=new Promise<void>(resolve=>{release=resolve;});
 const prepare=async(progress:(line:string)=>void,signal:AbortSignal):Promise<number>=>{
  for(const target of[MCP_BINARY_TARGET,...(catalogTarget?[catalogTarget]:[])]){signal.throwIfAborted();const status=await stage(target,progress,signal);if(status!==0)return status;}
  signal.throwIfAborted();
  return new Promise<number>((ready,refuse)=>{
   publication=withAdmittedPublishedCargoExecutableV1(MCP_MANIFEST_REL,MCP_BINARY_NAME+(process.platform==="win32"?".exe":""),repoRoot,mode,invocation,phase,async artifact=>{
    phase.signal.throwIfAborted();selected=artifact.path;acquired=true;ready(0);await closed;return 0;
   }).catch(error=>{refuse(error);if(error!==stopped.signal.reason)throw error;return 0;});
   void publication.catch(()=>{});
  });
 };
 try{return await serveWarmMcpGateway({stage:(progress,signal)=>{stageSignal=signal;preparing=prepare(progress,signal);return preparing;},command:()=>selected,args:withMcpHostDriverPolicy(["stdio",...extra]),cwd:repoRoot,env,input:process.stdin,output:process.stdout,onStop:()=>{if(!acquired)stopped.abort(stageSignal?.reason);onStop?.(stageSignal?.reason);}});}
 finally{if(!acquired)stopped.abort();release();try{await preparing;}catch(error){if(error!==stageSignal?.reason&&error!==stopped.signal.reason)throw error;}await publication;}
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
export function spawnRawMcp(bin: string, args: readonly string[] = ["stdio"], environment: NodeJS.ProcessEnv = process.env): RawMcpProcess {
  const child = spawn(bin, withMcpHostDriverPolicy(args), { env: environment, stdio: ["pipe", "pipe", "pipe"] }) as ChildProcessByStdio<Writable, Readable, Readable>;
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

/** 🛠️ The tool-call reply shape every `tools/call` answers with — `isError` is the TOOL's own verdict
 * and is independent of the JSON-RPC `error` channel, which only carries protocol faults. */
type McpToolReply = { readonly isError?: boolean; readonly structuredContent?: Record<string, any>; readonly content?: Array<{ readonly text?: string }> };

const MCP_CLIENT_PROTOCOL_VERSION = "2025-06-18";

/** 🔬️ Harness variables a real Claude Code parent exports; kept here because the `semio` server's
 * process-entry credential seal must admit them (ticket 26/09/18 M1 R1) — an e2e client that spawned
 * a pristine environment would never re-prove that. */
const MCP_CLIENT_HARNESS_ENVIRONMENT: Readonly<Record<string, string>> = { CLAUDE_CODE_SESSION_ID: "mcp-client-e2e", CLAUDE_CODE_MESSAGING_TOKEN: "mcp-client-e2e" };

/** 🚧️ A precondition an acceptance harness cannot run without — a hub origin, a credential, an admin capability file.
 * The harness records `blocked` (never `fail`) and names the environment variable that supplies it. */
export class AcceptancePreconditionMissing extends Error {}

/** 🔐️ The environment variable `name`, or the {@link AcceptancePreconditionMissing} naming it: credentials and hub
 * coordinates reach a harness only through its environment — never argv, never a default written in source. */
export function requiredHarnessEnv(name: string): string {
  const value = process.env[name];
  if (!value) throw new AcceptancePreconditionMissing(`${name} is not set`);
  return value;
}

/** 🔐️ The human a hub harness signs in as, from `OS_MCP_HUB_EMAIL` / `OS_MCP_HUB_PASSWORD`. */
export function hubCredentialFromEnv(): { readonly email: string; readonly password: string } {
  return { email: requiredHarnessEnv("OS_MCP_HUB_EMAIL"), password: requiredHarnessEnv("OS_MCP_HUB_PASSWORD") };
}

/** 🚦️ The `blockedWhen` of every os-mcp harness's acceptance record. */
export const isAcceptancePreconditionMissing = (error: unknown): boolean => error instanceof AcceptancePreconditionMissing;

/** 📖️ The `.mcp.json` server table at `repoRoot` — the single source this driver spawns from, so a
 * change to how clients launch a server is a change this gate immediately exercises. */
export function mcpServerEntries(repoRoot: string): Record<string, McpServerEntry> {
  const config = JSON.parse(readFileSync(posix.join(repoRoot, ".mcp.json"), "utf8")) as { mcpServers?: Record<string, McpServerEntry> };
  const servers = config.mcpServers;
  if (!servers || Object.keys(servers).length === 0) throw new Error(".mcp.json declares no mcpServers");
  return servers;
}

/** 🎯️ The ONE plugin, artifact kind and verb, plus the one inference service, the os journey dispatches
 * against. The caller injects them: a gate that picked its target from a fuzzy `capabilities_search`
 * measured whichever plugin BM25 happened to rank first that day, so the pin is explicit data owned by
 * the specific layer that knows its plugins, never a constant of this neutral driver.
 * `capabilities_search` keeps its own step — see `capabilitySearchRankingVerdict`, which asserts ranking
 * PROPERTIES over the compiled catalog and needs no guest at all. */
export type McpClientE2ePinsV1 = {
  readonly pluginId: string;
  readonly artifactKind: string;
  readonly capabilityId: string;
  readonly caller:Readonly<{appId:string;modeId:string;windowKindId:string;locale:"en"|"de";terminology:"native"|"reuse"}>;
  /** 🔎️ Natural-language query the ranking step searches the pinned kind's mutations with. */
  readonly searchQuery: string;
  /** 📭️ A verb of the pinned kind that changes nothing on a document with no selection: its preview
   * produces no operation, which the gateway must answer as a committed no-change, never INTERNAL. */
  readonly noChangeCapabilityId: string;
  /** 💡️ The plugin whose declared inference the journey runs, pinned against the same kind of drift:
   * `inference_list` returns the UNION of every installed plugin's declared roster. */
  readonly inferencePluginId: string;
  readonly inferenceArtifactKind: string;
  readonly inferenceSchema: string;
  readonly inferenceInitial:Readonly<Record<string,unknown>>;
  readonly inferenceCancellationInitial:Readonly<Record<string,unknown>>;
  readonly inferenceExpectedOutput:Readonly<Record<string,unknown>>;
  readonly inferenceLimits:Readonly<{workUnits:number;maximumElapsedMilliseconds:number;retainedPolicy:Readonly<{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}>}>;
};

/** 🧾️ What the preflight found for one pinned plugin: whether its component is staged AND is the
 * exact build its committed descriptor describes. */
export type StagedComponentVerdict = { readonly ok: boolean; readonly detail: string };

/** 🪟️ Establishes and retains a programmatic caller surface; it asserts no mounted human UI. */
export function createHeadlessCallerSurfaceV1(app:Readonly<{id:string;modes:readonly Readonly<{id:string}>[];windowKinds:readonly Readonly<{id:string}>[]}>,binding:Readonly<{sessionId:string;principal:string;artifactId:string;pluginId:string;windowId:string}>,preferences:Readonly<{appId:string;modeId:string;windowKindId:string;locale:string;terminology:string}>){
 if(Object.values(binding).some(value=>typeof value!=="string"||!value)||app.id!==preferences.appId||!app.modes.some(mode=>mode.id===preferences.modeId)||!app.windowKinds.some(kind=>kind.id===preferences.windowKindId))throw Error("Actual session, artifact and published caller surface declaration required");
 const target=Object.freeze({pluginId:binding.pluginId,artifactRef:binding.artifactId,appId:app.id});
 const context=Object.freeze({locale:preferences.locale,terminology:preferences.terminology,activeModeId:preferences.modeId,windowId:binding.windowId,windowInstances:Object.freeze([Object.freeze({id:binding.windowId,windowKindId:preferences.windowKindId})]),sessionIdentity:Object.freeze({userId:binding.principal,displayName:binding.principal})});
 parseActionPrepareInput({capabilityId:"caller-surface-validation",target,context});
 return Object.freeze({sessionId:binding.sessionId,artifactId:binding.artifactId,target,context});
}

/** 💡️ Requires a completed typed result from the exact requested document and service. */
export function inferenceClientVerdictV1(reply: {readonly isError?:boolean;readonly structuredContent?:Readonly<Record<string,unknown>>},binding:Readonly<Record<string,string>>,outputSchema:unknown):StagedComponentVerdict{
 const result=reply.structuredContent;
 const faults:string[]=[];
 if(reply.isError===true||!result)return {ok:false,detail:"Inference returned an error or no structured result"};
 if(result.status!=="SUCCEEDED"||result.complete!==true)faults.push("Inference did not complete successfully");
 if(typeof result.jobId!=="string"||!result.jobId)faults.push("Inference returned no job identity");
 for(const [name,value]of Object.entries(binding))if(result[name]!==value)faults.push(`Inference ${name} differs from the requested binding`);
 if(!Number.isSafeInteger(result.payloadBytes)||Number(result.payloadBytes)<1)faults.push("Inference returned no payload bytes");
 faults.push(...validateJsonSchemaSubset(outputSchema,result.payload));
 return {ok:faults.length===0,detail:faults.length?faults.join("; "):"Completed typed inference output matches the requested artifact and service"};
}

/** 🛑️ A terminal cancellation counts only after observable active work and without committed effects. */
export function inferenceCancellationVerdictV1(active:Readonly<Record<string,unknown>>|undefined,terminal:Readonly<Record<string,unknown>>|undefined,unchanged:boolean):boolean{
 return ["ACCEPTED","RUNNING"].includes(String(active?.state??"").toUpperCase())&&Array.isArray(active?.progress)&&active.progress.length>0&&String(terminal?.state??"").toUpperCase()==="CANCELLED"&&terminal?.cancelRequested===true&&terminal?.commit==null&&unchanged;
}

/** 🧱️ Checks the selected publication's exact component mode against its actual descriptor digest. */
export function verifyStagedPluginComponent(repoRoot: string, pluginId: string, catalogRoot: string, mode: "prototype"|"release"): StagedComponentVerdict {
  const registryPath = join(catalogRoot, "🦀️catalog.json");
  let rows: Array<{ pluginId?: string; cratePath?: string; wasmOut?: string }>;
  try {
    rows = [...readNativeCatalogV1(JSON.parse(readFileSync(registryPath, "utf8"))).plugins];
  } catch (error) {
    return { ok: false, detail: `the explicit native catalog ${registryPath} is unavailable: ${String(error)}` };
  }
  const row = rows.find((entry) => entry.pluginId === pluginId);
  if (!row?.cratePath || !row.wasmOut) return { ok: false, detail: `\`${pluginId}\` is absent from the explicit native catalog ${registryPath}` };
  const ownerRoot = posix.dirname(posix.dirname(posix.join(repoRoot, row.cratePath)));
  const descriptorPath = posix.join(ownerRoot, "🔣️.json");
  let declaredWasmSha256: string | undefined;
  try {
    declaredWasmSha256 = (JSON.parse(readFileSync(descriptorPath, "utf8")) as { hashes?: { wasmSha256?: string } }).hashes?.wasmSha256;
  } catch (error) {
    return { ok: false, detail: `\`${pluginId}\` has no readable descriptor at ${descriptorPath}: ${String(error)}` };
  }
  if (!declaredWasmSha256) return { ok: false, detail: `\`${pluginId}\`'s committed descriptor declares no \`hashes.wasmSha256\`, so a staged component cannot be checked against it` };
  const tried: string[] = [];
  for (const profile of [`dist/components/${mode}`]) {
    const candidate = posix.join(repoRoot, row.cratePath, profile, row.wasmOut);
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
      detail: `${pluginId}: ${profile}/${row.wasmOut} digest ${staged} differs from its descriptor ${declaredWasmSha256}; run its authored plugin producer with the explicit catalog`,
    };
  }
  return { ok: false, detail: `${pluginId}: no compiled component is staged (tried ${tried.join(", ")}) — build its authored plugin target with the explicit catalog` };
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
  const pack = String(untrustedContent(reply).packBase64 ?? "");
  const spr = String(untrustedContent(reply).sprBase64 ?? "");
  return pack.length === 0 && spr.length === 0 ? "" : `${pack}.${spr}`;
}

/** 🧷️ The document-authored content a reply carries — only ever inside its `untrusted` envelope
 * (`semio.mcp.untrusted-content/v1`); an empty record when the reply carries none. */
function untrustedContent(reply: McpToolReply): Record<string, unknown> {
  const envelope = reply.structuredContent?.untrusted as Record<string, any> | undefined;
  return envelope?.schema === "semio.mcp.untrusted-content/v1" ? ((envelope.content ?? {}) as Record<string, unknown>) : {};
}

/** 📄️ Walks one cursor-paginated list method exactly as a spec-strict client does: page one, then
 * every `nextCursor` until there is none. Bounded by a page cap so a server that ever returned a
 * non-advancing cursor fails this gate instead of hanging it. */
async function paginationWalk(session: McpClientSessionV1, method: string, key: string, identify: (entry: any) => string): Promise<{ ok: boolean; detail: string; pages: number; visited: string[]; firstCursor?: string }> {
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
async function headRevision(session: McpClientSessionV1, capabilityId: string, input: Record<string, unknown>,target:Readonly<Record<string,string>>,context:Readonly<Record<string,unknown>>): Promise<RevisionStampWire | undefined> {
  const probe = await session.call("action_prepare", { capabilityId, input,target,context });
  if (probe.isError === true) return undefined;
  const handle = probe.structuredContent?.preparedHandle as string | undefined;
  if (handle) await session.call("action_cancel", { preparedActionHandle: handle });
  return revisionOf(probe);
}

/** 🤝️ Drives the `repo` server exactly as `.mcp.json` launches it: handshake, resource listing and a
 * real read of `repo://goals` — the resource family the repo client exists to serve. */
export async function runRepoMcpClientJourney(repoRoot: string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl): Promise<readonly McpClientStep[]> {
  const entry = mcpServerEntries(repoRoot).repo;
  if (!entry) return [mcpStepFailure("repo: .mcp.json entry", "`.mcp.json` declares no `repo` server")];
  return withMcpClientSessionV1(invocation,phaseControl,{command:entry.command,args:[...entry.args,...[]],cwd:repoRoot,environment:{...process.env,...MCP_CLIENT_HARNESS_ENVIRONMENT}},async session=>{
  const steps: McpClientStep[] = [];
    const initialized = await session.request("initialize", { protocolVersion: MCP_CLIENT_PROTOCOL_VERSION, capabilities: { roots: { listChanged: false } }, clientInfo: { name: "semio-mcp-client-e2e", title: "semio MCP client e2e", version: "1" } });
    if (initialized.error) return [...steps, mcpStepFailure("repo: initialize", JSON.stringify(initialized.error))];
    await session.notify("notifications/initialized", {});
    announce(steps, { step: "repo: initialize", ok: true, detail: `server=${mcpJsonPathV1(initialized.result,["serverInfo","name"])}@${mcpJsonPathV1(initialized.result,["serverInfo","version"])} protocol=${initialized.result?.protocolVersion}`, wire: initialized.result });

    const resources = await session.request("resources/list", {});
    const uris = ((resources.result?.resources ?? []) as Array<{ uri: string }>).map((resource) => resource.uri).sort();
    announce(steps, { step: "repo: resources/list", ok: !resources.error && uris.includes("repo://goals"), detail: `${uris.length} resources: ${uris.join(", ")}`, wire: resources.result });

    const goals = await session.request("resources/read", { uri: "repo://goals" });
    const body = String(mcpJsonPathV1(goals.result,["contents",0,"text"]) ?? "");
    announce(steps, { step: "repo: resources/read repo://goals", ok: !goals.error && body.length > 0, detail: goals.error ? JSON.stringify(goals.error) : `${body.length} byte(s): ${body.slice(0, 120).replace(/\s+/g, " ")}`, wire: goals.result });
    return steps;
  });
}

/** 🚀️ Drives the `semio` (os gateway) server end to end as a real MCP client would — every step a
 * live JSON-RPC exchange with the process `.mcp.json` names, never an in-process call:
 * `initialize` → `tools/list` → the compiled catalog's own health → `capabilities_search` →
 * `artifact_create` → `action_prepare`/`action_invoke` of a discovered mutation →
 * `artifact_snapshot` (the mutation must be visible) → `history_undo` (it must be gone again) →
 * `inference_run` with its job progress and a cancel. `folder` binds the headless workspace; the
 * capability catalog comes from the explicit caller-selected application publication. */
export async function runOsMcpClientJourney(repoRoot: string, folder: string, pins: McpClientE2ePinsV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl): Promise<readonly McpClientStep[]> {
  const entry = mcpServerEntries(repoRoot).semio;
  if (!entry) return [mcpStepFailure("os: .mcp.json entry", "`.mcp.json` declares no `semio` server")];
  return withMcpClientSessionV1(invocation,phaseControl,{command:entry.command,args:[...entry.args,...["--folder", folder, "--no-bridge"]],cwd:repoRoot,environment:{...process.env,...MCP_CLIENT_HARNESS_ENVIRONMENT}},async session=>{
  const steps: McpClientStep[] = [];
    const initialized = await session.request("initialize", { protocolVersion: MCP_CLIENT_PROTOCOL_VERSION, capabilities: { roots: { listChanged: false } }, clientInfo: { name: "semio-mcp-client-e2e", title: "semio MCP client e2e", version: "1" } });
    if (initialized.error) return [...steps, mcpStepFailure("os: initialize", JSON.stringify(initialized.error))];
    await session.notify("notifications/initialized", {});
    announce(steps, { step: "os: initialize", ok: true, detail: `server=${mcpJsonPathV1(initialized.result,["serverInfo","name"])}@${mcpJsonPathV1(initialized.result,["serverInfo","version"])} protocol=${initialized.result?.protocolVersion}`, wire: initialized.result });

    const context = await session.call("context_resolve", { locale: pins.caller.locale });
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

    const catalogIndex=entry.args.indexOf("--catalog");
    const configuredCatalog=catalogIndex<0?undefined:entry.args[catalogIndex+1];
    const staged = configuredCatalog ? [pins.pluginId, pins.inferencePluginId].map((pluginId) => verifyStagedPluginComponent(repoRoot, pluginId,resolve(repoRoot,configuredCatalog),"prototype")) : [{ok:false,detail:"The configured journey requires an explicit --catalog publication"}];
    announce(steps, { step: "os: every pinned component is staged and current", ok: staged.every((verdict) => verdict.ok), detail: staged.map((verdict) => verdict.detail).join(" | "), wire: staged });
    if (!staged.every((verdict) => verdict.ok)) return steps;

    const tools = await session.request("tools/list", {});
    const toolNames = ((tools.result?.tools ?? []) as Array<{ name: string }>).map((tool) => tool.name).sort();
    const requiredTools = ["action_invoke", "action_prepare", "artifact_create", "artifact_open", "artifact_snapshot", "capabilities_search", "history_undo", "inference_run", "job_cancel"];
    const missingTools = requiredTools.filter((name) => !toolNames.includes(name));
    announce(steps, { step: "os: tools/list", ok: missingTools.length === 0 && toolNames.length >= 9, detail: missingTools.length === 0 ? `${toolNames.length} tools, all ${requiredTools.length} required present` : `missing: ${missingTools.join(", ")}`, wire: toolNames });

    const walk = await paginationWalk(session, "tools/list", "tools", (tool: any) => String(tool.name));
    announce(steps, { step: "os: tools/list pagination walk", ok: walk.ok && walk.visited.join("\0") === toolNames.join("\0"), detail: walk.ok ? `${walk.pages} page(s), ${walk.visited.length} tool(s), order identical to the unpaged list` : walk.detail, wire: { pages: walk.pages, firstCursor: walk.firstCursor } });
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
    announce(steps, { step: "os: artifact_create", ok: created.isError !== true, detail: created.isError === true ? JSON.stringify(created.structuredContent) : `artifactId=${created.structuredContent?.artifactId} revision=${mcpJsonPathV1(created.structuredContent,["revision","cursor"]) ?? "<none>"}`, wire: created.structuredContent });
    if (created.isError === true) return steps;

    const opened = await session.call("artifact_open", { artifactId });
    announce(steps, { step: "os: artifact_open", ok: opened.isError !== true, detail: opened.isError === true ? JSON.stringify(opened.structuredContent) : `kind=${opened.structuredContent?.kind} sizeBytes=${opened.structuredContent?.sizeBytes}`, wire: opened.structuredContent });

    const mutations = await session.call("capabilities_search", { query: pins.searchQuery, kind: ["mutation"], artifactKind: pins.artifactKind });
    const mutationHits = (mutations.structuredContent?.results ?? []) as Array<Record<string, any>>;
    const ranking = capabilitySearchRankingVerdict(mutationHits, pins.capabilityId, pins.artifactKind);
    announce(steps, { step: "os: capabilities_search ranking properties", ok: mutations.isError !== true && ranking.ok, detail: mutations.isError === true ? JSON.stringify(mutations.structuredContent).slice(0, 240) : ranking.detail, wire: mutationHits.slice(0, 5) });

    const described = await session.call("capabilities_describe", { capabilityId: pins.capabilityId });
    const inputSchema = described.structuredContent?.inputSchema ?? mcpJsonPathV1(described.structuredContent,["capability","inputSchema"]);
    const input = minimalInputForSchema(inputSchema);
    const describedKind = String(described.structuredContent?.artifactKind ?? mcpJsonPathV1(described.structuredContent,["capability","artifactKind"]) ?? "");
    announce(steps, {
      step: "os: capabilities_describe (the pinned verb)",
      ok: described.isError !== true && describedKind === pins.artifactKind,
      detail:
        described.isError === true
          ? `${pins.capabilityId}: ${JSON.stringify(described.structuredContent).slice(0, 240)}`
          : describedKind === pins.artifactKind
            ? `${pins.capabilityId} artifactKind=${describedKind} input=${JSON.stringify(input).slice(0, 120)}`
            : `${pins.capabilityId} is typed against \`${describedKind || "<none>"}\`, not the pinned \`${pins.artifactKind}\``,
      wire: described.structuredContent,
    });

    const typedKind = pins.artifactKind;
    const typedArtifactId = `${artifactId}-typed`;
    const typedCreated = await session.call("artifact_create", { artifactId: typedArtifactId, kind: typedKind });
    announce(steps, { step: "os: artifact_create (a real plugin artifact kind)", ok: typedCreated.isError !== true && typedCreated.structuredContent?.kind === typedKind, detail: typedCreated.isError === true ? `kind=${typedKind}: ${JSON.stringify(typedCreated.structuredContent).slice(0, 260)}` : `artifactId=${typedArtifactId} kind=${typedCreated.structuredContent?.kind} pluginId=${typedCreated.structuredContent?.pluginId} sizeBytes=${typedCreated.structuredContent?.sizeBytes}`, wire: typedCreated.structuredContent });

    if (typedCreated.isError === true || typedCreated.structuredContent?.kind !== typedKind) return steps;

    const publication=resolve(repoRoot,configuredCatalog!);
    const nativeCatalog=readNativeCatalogV1(JSON.parse(readFileSync(join(publication,"🦀️catalog.json"),"utf8")));
    const owner=nativeCatalog.plugins.find(row=>row.pluginId===pins.pluginId);
    if(!owner)throw Error("Selected native catalog owner disappeared");
    const descriptor=JSON.parse(readFileSync(join(repoRoot,owner.cratePath,"../..","🔣️.json"),"utf8"));
    const app=descriptor.manifest?.apps?.find((row:{id:string})=>row.id===pins.caller.appId);
    if(!app||descriptor.manifest.pluginId!==pins.pluginId)throw Error("Published app and plugin declaration required");
    const surface=createHeadlessCallerSurfaceV1(app,{sessionId:String(context.structuredContent?.sessionId??""),principal:String(context.structuredContent?.principal??""),artifactId:typedArtifactId,pluginId:pins.pluginId,windowId:`mcp-caller-${randomUUID()}`},pins.caller);
    const target=surface.target;
    announce(steps,{step:"os: establish programmatic headless caller surface",ok:true,detail:`session=${surface.sessionId} artifact=${surface.artifactId} app=${app.id}; caller-owned virtual window ${surface.context.windowId}; no mounted UI asserted`,wire:surface});

    const exportTarget = typedArtifactId;
    const exported = await session.call("artifact_export", { artifactId: exportTarget });
    const exportedBase64 = String(untrustedContent(exported).contentBase64 ?? "");
    announce(steps, { step: "os: artifact_export", ok: exported.isError !== true && exportedBase64.length > 0, detail: exported.isError === true ? `${exportTarget}: ${JSON.stringify(exported.structuredContent).slice(0, 300)}` : `artifactId=${exportTarget} port=${exported.structuredContent?.format} base64Bytes=${exportedBase64.length} (inside untrusted) declaredFormats=${JSON.stringify(exported.structuredContent?.declaredExportFormats ?? [])}`, wire: { ...exported.structuredContent, untrusted: { provenance: (exported.structuredContent?.untrusted as any)?.provenance, contentBase64: `${exportedBase64.slice(0, 32)}…` } } });

    const beforeSnapshot = await session.call("artifact_snapshot", { artifactId: typedArtifactId });
    const packBefore = String(untrustedContent(beforeSnapshot).packBase64 ?? "");
    const documentBefore = snapshotDocumentBase64(beforeSnapshot);

    const prepared = await session.call("action_prepare", { capabilityId: pins.capabilityId, input,target,context:surface.context });
    const baselineRevision = revisionOf(prepared);
    announce(steps, { step: "os: the prepared revision names the ARTIFACT, not the plugin", ok: baselineRevision?.artifactId === typedArtifactId, detail: `revision.artifactId=${baselineRevision?.artifactId ?? "<none>"} artifact_create used ${typedArtifactId} (pinned plugin is ${pins.pluginId})`, wire: baselineRevision });
    announce(steps, { step: "os: action_prepare", ok: prepared.isError !== true, detail: prepared.isError === true ? `${pins.capabilityId} input=${JSON.stringify(input)}: ${JSON.stringify(prepared.structuredContent).slice(0, 260)}` : `handle=${prepared.structuredContent?.preparedHandle} baseline=${stampText(baselineRevision)}`, wire: prepared.structuredContent });
    if (prepared.isError === true) return steps;

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

    const updates = session.serverNotifications().slice(notificationsBefore).filter((envelope) => envelope.method === "notifications/resources/updated");
    const updatedUris = updates.map((envelope) => String((envelope.params as any)?.uri ?? ""));
    announce(steps, { step: "os: notifications/resources/updated after the commit", ok: updatedUris.includes(watchedUri), detail: updatedUris.includes(watchedUri) ? `${updates.length} update(s), including ${watchedUri}` : `${updates.length} update(s), none naming ${watchedUri}: ${updatedUris.join(", ") || "<none>"}`, wire: updatedUris });
    const unsubscribed = await session.request("resources/unsubscribe", { uri: watchedUri });
    announce(steps, { step: "os: resources/unsubscribe", ok: !unsubscribed.error, detail: unsubscribed.error ? JSON.stringify(unsubscribed.error) : `unsubscribed from ${watchedUri}`, wire: unsubscribed.result });

    const afterSnapshot = await session.call("artifact_snapshot", { artifactId: typedArtifactId });
    const packAfter = String(untrustedContent(afterSnapshot).packBase64 ?? "");
    const afterEnvelope = afterSnapshot.structuredContent?.untrusted as Record<string, any> | undefined;
    announce(steps, { step: "os: artifact_snapshot (the created artifact)", ok: afterSnapshot.isError !== true && Number(afterSnapshot.structuredContent?.packBytes ?? 0) > 0 && packAfter.length > 0, detail: afterSnapshot.isError === true ? `${typedArtifactId}: ${JSON.stringify(afterSnapshot.structuredContent).slice(0, 300)}` : `artifactId=${typedArtifactId} packBytes=${afterSnapshot.structuredContent?.packBytes} sprBytes=${afterSnapshot.structuredContent?.sprBytes} untrusted.source=${afterEnvelope?.provenance?.source} authors=${afterEnvelope?.provenance?.authors?.kind}`, wire: { ...afterSnapshot.structuredContent, untrusted: { provenance: afterEnvelope?.provenance, packBase64: `${packAfter.slice(0, 32)}…` } } });
    const documentAfter = snapshotDocumentBase64(afterSnapshot);
    announce(steps, { step: "os: the snapshot shows the mutation", ok: documentBefore.length > 0 && documentAfter.length > 0 && documentAfter !== documentBefore, detail: documentBefore.length === 0 ? `the pre-mutation snapshot answered no bytes: ${JSON.stringify(beforeSnapshot.structuredContent).slice(0, 200)}` : documentAfter === documentBefore ? `${typedArtifactId} is byte-identical across the commit (pack ${packAfter.length}, spr ${String(untrustedContent(afterSnapshot).sprBase64 ?? "").length} base64 chars) — the snapshot is reading a frozen row, not the document the mutation went to` : `${documentBefore.length} → ${documentAfter.length} base64 chars across the commit (pack ${packBefore.length} → ${packAfter.length}, spr ${String(untrustedContent(beforeSnapshot).sprBase64 ?? "").length} → ${String(untrustedContent(afterSnapshot).sprBase64 ?? "").length})`, wire: { before: documentBefore.slice(0, 24), after: documentAfter.slice(0, 24) } });

    const headAfterInvoke = await headRevision(session, pins.capabilityId, input,target,surface.context);
    announce(steps, { step: "os: live head advanced", ok: headAfterInvoke !== undefined && headAfterInvoke.headEditId === revisionAfter?.headEditId && headAfterInvoke.headEditId !== baselineRevision?.headEditId, detail: `re-read head ${stampText(headAfterInvoke)} (baseline ${stampText(baselineRevision)}, invoke reported ${stampText(revisionAfter)})`, wire: headAfterInvoke });

    if (!undoToken) {
      announce(steps, mcpStepFailure("os: history_undo (mutation reverted)", "action_invoke minted no undoToken — nothing to undo", invoked.structuredContent));
      return steps;
    }
    const undone = await session.call("history_undo", { undoToken });
    const headAfterUndo = await headRevision(session, pins.capabilityId, input,target,surface.context);
    const undoWarnings = (undone.structuredContent?.warnings ?? []) as string[];
    announce(steps, { step: "os: history_undo (mutation reverted)", ok: undone.isError !== true && Number(undone.structuredContent?.members ?? 0) > 0 && undoWarnings.length === 0 && headAfterUndo?.headEditId === baselineRevision?.headEditId, detail: undone.isError === true ? JSON.stringify(undone.structuredContent).slice(0, 300) : `members=${undone.structuredContent?.members} warnings=${undoWarnings.length === 0 ? "none" : undoWarnings.join(" / ").slice(0, 200)} head ${stampText(headAfterUndo)} (baseline ${stampText(baselineRevision)})`, wire: undone.structuredContent });

    const redone = await session.call("history_redo", { undoToken });
    const headAfterRedo = await headRevision(session, pins.capabilityId, input,target,surface.context);
    const redoWarnings = (redone.structuredContent?.warnings ?? []) as string[];
    announce(steps, { step: "os: history_redo (mutation restored)", ok: redone.isError !== true && Number(redone.structuredContent?.members ?? 0) > 0 && redoWarnings.length === 0 && headAfterRedo?.headEditId !== undefined && headAfterRedo.headEditId !== baselineRevision?.headEditId, detail: redone.isError === true ? JSON.stringify(redone.structuredContent).slice(0, 300) : `members=${redone.structuredContent?.members} warnings=${redoWarnings.length === 0 ? "none" : redoWarnings.join(" / ").slice(0, 200)} head ${stampText(headAfterRedo)} (undone head was ${stampText(headAfterUndo)})`, wire: redone.structuredContent });

    const noChangePrepared = await session.call("action_prepare", { capabilityId: pins.noChangeCapabilityId, input: {},target,context:surface.context });
    const noChange = noChangePrepared.isError === true ? noChangePrepared : await session.call("action_invoke", { preparedActionHandle: noChangePrepared.structuredContent?.preparedHandle });
    const noChangeWarnings = (noChange.structuredContent?.warnings as string[] | undefined) ?? [];
    announce(steps, {
      step: "os: an action that changes nothing answers no-change, never INTERNAL",
      ok: noChange.isError !== true && noChange.structuredContent?.status === "SUCCEEDED" && noChangeWarnings.some((warning) => warning.startsWith("no-change:")) && JSON.stringify(noChange.structuredContent?.revisionAfter) === JSON.stringify(noChange.structuredContent?.revisionBefore) && noChange.structuredContent?.undoToken == null,
      detail: noChange.isError === true ? `${pins.noChangeCapabilityId}: ${JSON.stringify(noChange.structuredContent).slice(0, 300)}` : `status=${noChange.structuredContent?.status} opsCount=${JSON.stringify(mcpJsonPathV1(noChangePrepared.structuredContent,["preview","opsCount"]))} warnings=${noChangeWarnings.join(" / ").slice(0, 120)} undoToken=${noChange.structuredContent?.undoToken ?? "none"}`,
      wire: noChange.structuredContent,
    });

    const sagaMember = await session.call("action_prepare", { capabilityId: pins.capabilityId, input,target,context:surface.context });
    const began = await session.call("transaction_begin", { preparedHandles: [sagaMember.structuredContent?.preparedHandle] });
    const transactionHandle = began.structuredContent?.transactionHandle as string | undefined;
    announce(steps, { step: "os: transaction_begin", ok: began.isError !== true && typeof transactionHandle === "string", detail: began.isError === true ? JSON.stringify(began.structuredContent).slice(0, 300) : `transactionHandle=${transactionHandle} members=1`, wire: began.structuredContent });
    const rolledBack = await session.call("transaction_rollback", { transactionHandle });
    const headAfterRollback = await headRevision(session, pins.capabilityId, input,target,surface.context);
    announce(steps, { step: "os: transaction_rollback (no change)", ok: rolledBack.isError !== true && headAfterRollback?.headEditId === headAfterRedo?.headEditId, detail: rolledBack.isError === true ? JSON.stringify(rolledBack.structuredContent).slice(0, 300) : `rolledBack=${rolledBack.structuredContent?.rolledBack} head ${stampText(headAfterRollback)} unchanged from ${stampText(headAfterRedo)}`, wire: rolledBack.structuredContent });

    const listed = await session.call("inference_list", {});
    const declared = (listed.structuredContent?.declared ?? []) as Array<Record<string, any>>;
    const service = declared.find((row) => row.artifactKind === pins.inferenceArtifactKind && row.inferenceSchema === pins.inferenceSchema);
    const contract = service?.payload as Record<string, any> | undefined;
    const contractOk = typeof contract?.payloadSchemaId === "string" && typeof contract?.inputSchema === "string" && contract.inputSchema.length > 0;
    announce(steps, { step: "os: inference_list declares the pinned service", ok: listed.isError !== true && service !== undefined && contractOk, detail: listed.isError === true ? JSON.stringify(listed.structuredContent).slice(0, 200) : service ? `${declared.length} declared inference(s), including ${pins.inferenceArtifactKind}/${pins.inferenceSchema} by ${service.contributor || service.owner}${contractOk ? `, contract ${contract?.payloadSchemaId} (binds \`${contract?.artifactBinding?.field ?? "<none>"}\`, ${String(contract?.inputSchema ?? "").length} B input schema)` : ", publishing NO payload contract — a client cannot know what to send"}` : `${declared.length} declared inference(s), none of them ${pins.inferenceArtifactKind}/${pins.inferenceSchema}: ${declared.map((row) => `${row.artifactKind}/${row.inferenceSchema}`).join(", ")}`, wire: declared.slice(0, 8) });
    if (!service) return steps;
    const inferenceArtifactId = `${artifactId}-inference`;
    const inferenceArtifact = await session.call("artifact_create", { artifactId: inferenceArtifactId, kind: pins.inferenceArtifactKind,initial:pins.inferenceInitial });
    announce(steps,{step:"os: create declared inference problem",ok:inferenceArtifact.isError!==true,detail:inferenceArtifactId,wire:inferenceArtifact.structuredContent});
    if(inferenceArtifact.isError===true)return steps;
    const binding={artifactKind:pins.inferenceArtifactKind,inferenceSchema:pins.inferenceSchema,pluginId:pins.inferencePluginId,artifactId:inferenceArtifactId};
    const request=parseInferenceRunInput({...binding,...pins.inferenceLimits,revision:0,documentGeneration:1,cancellationId:`${artifactId}-solve`});
    const progressToken = `m5b-${Date.now().toString(36)}`;
    const progressBefore = session.serverNotifications().length;
    const inferenceEnvelope = await session
      .request("tools/call", { name: "inference_run", arguments: request, _meta: { progressToken } })
;
    const inference = (inferenceEnvelope.error ? { isError: true, structuredContent: { code: `JSONRPC_${inferenceEnvelope.error.code}`, message: inferenceEnvelope.error.message } } : (inferenceEnvelope.result ?? {})) as McpToolReply;
    const progressRows = session.serverNotifications().slice(progressBefore).filter((envelope) => envelope.method === "notifications/progress" && (envelope.params as any)?.progressToken === progressToken);
    announce(steps, { step: "os: notifications/progress for _meta.progressToken", ok: progressRows.length > 0, detail: progressRows.length > 0 ? `${progressRows.length} progress row(s), last progress=${(progressRows[progressRows.length - 1]?.params as any)?.progress}` : `no progress notification carried token ${progressToken} — the job reported none, or the push is not wired`, wire: progressRows.map((envelope) => envelope.params) });
    const jobId = (inference.structuredContent?.jobId ?? (inference.structuredContent?.details as Record<string, unknown> | undefined)?.jobId) as string | undefined;
    let outputSchema:unknown;
    try{outputSchema=JSON.parse(String(contract?.outputSchema??""));}catch{announce(steps,mcpStepFailure("os: typed inference output","Declared service has no readable output schema"));return steps;}
    const verdict=inferenceClientVerdictV1(inference,binding,outputSchema);
    const semanticSchema={type:"object",required:Object.keys(pins.inferenceExpectedOutput),properties:Object.fromEntries(Object.entries(pins.inferenceExpectedOutput).map(([name,value])=>[name,{const:value}]))};
    const semanticFaults=validateJsonSchemaSubset(semanticSchema,inference.structuredContent?.payload);
    announce(steps, { step: "os: inference_run produces the declared solution", ok: verdict.ok&&semanticFaults.length===0, detail: `${verdict.detail}${semanticFaults.length?`; ${semanticFaults.join("; ")}`:""}`, wire: inference.structuredContent });

    if (jobId) {
      const progress = await session.call("job_get", { jobId });
      announce(steps, { step: "os: inference job terminal success", ok: progress.isError !== true && progress.structuredContent?.status === "SUCCEEDED", detail: `jobId=${jobId} status=${progress.structuredContent?.status}`, wire: progress.structuredContent });
    } else {
      announce(steps, mcpStepFailure("os: inference job progress", "inference_run minted no jobId", inference.structuredContent));
    }

    const cancelArtifactId=`${artifactId}-active-cancel`;
    const candidate=await session.call("artifact_create",{artifactId:cancelArtifactId,kind:pins.inferenceArtifactKind,initial:pins.inferenceCancellationInitial});
    if(candidate.isError===true){announce(steps,mcpStepFailure("os: active inference cancellation problem",JSON.stringify(candidate.structuredContent)));return steps;}
    const beforeCancel=await session.call("artifact_snapshot",{artifactId:cancelArtifactId});
    const submit=parseInferenceSubmitInput({documentId:cancelArtifactId,inferenceSchema:pins.inferenceSchema,pluginId:pins.inferencePluginId,...pins.inferenceLimits,revision:0,documentGeneration:1,cancellationId:`${artifactId}-active-cancel`});
    const submitted=await session.call("inference_submit",submit);
    const jobHandle=submitted.structuredContent?.jobHandle;
    if(submitted.isError===true||typeof jobHandle!=="string"){announce(steps,mcpStepFailure("os: active inference cancellation submit",JSON.stringify(submitted.structuredContent)));return steps;}
    let page=submitted;
    for(let poll=0;poll<64;poll++){
      const job=page.structuredContent?.job as Record<string,unknown>|undefined;
      const state=String(job?.state??"").toUpperCase();
      if(!["ACCEPTED","RUNNING"].includes(state)||Array.isArray(job?.progress)&&job.progress.length>0)break;
      page=await session.call("inference_events",{jobHandle});
    }
    const active=page.structuredContent?.job as Record<string,unknown>|undefined;
    const activeState=String(active?.state??"").toUpperCase();
    const observedActive=page.isError!==true&&["ACCEPTED","RUNNING"].includes(activeState)&&Array.isArray(active?.progress)&&active.progress.length>0;
    announce(steps,{step:"os: observe active progressing inference before cancel",ok:observedActive,detail:`state=${activeState} progressRows=${Array.isArray(active?.progress)?active.progress.length:0}`,wire:page.structuredContent});
    const cancelled=await session.call("inference_cancel",{jobHandle});
    let terminal=cancelled;
    for(let poll=0;poll<64;poll++){
      const job=terminal.structuredContent?.job as Record<string,unknown>|undefined;
      if(!["ACCEPTED","RUNNING"].includes(String(job?.state??"").toUpperCase()))break;
      terminal=await session.call("inference_events",{jobHandle});
    }
    const cancelledJob=terminal.structuredContent?.job as Record<string,unknown>|undefined;
    const afterCancel=await session.call("artifact_snapshot",{artifactId:cancelArtifactId});
    const unchanged=beforeCancel.isError!==true&&afterCancel.isError!==true&&snapshotDocumentBase64(beforeCancel).length>0&&snapshotDocumentBase64(beforeCancel)===snapshotDocumentBase64(afterCancel);
    announce(steps,{step:"os: active inference cancellation settles without committed effects",ok:observedActive&&cancelled.isError!==true&&terminal.isError!==true&&inferenceCancellationVerdictV1(active,cancelledJob,unchanged),detail:`state=${cancelledJob?.state} cancelRequested=${cancelledJob?.cancelRequested} documentUnchanged=${unchanged}`,wire:terminal.structuredContent});
    return steps;
  });
}

/** 🏁️ Both `.mcp.json` servers, one report. Returns every step so a caller can print the table and
 * exit non-zero on the first red — the gate never summarises a failure away. */
export async function runMcpClientEndToEnd(repoRoot: string, folder: string, pins: McpClientE2ePinsV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl): Promise<readonly McpClientStep[]> {
  const os = await runOsMcpClientJourney(repoRoot, folder, pins,invocation,phaseControl);
  const repo = await runRepoMcpClientJourney(repoRoot,invocation,phaseControl);
  return [...os, ...repo];
}
//#endregion 🔖️ClientEndToEnd

//#region 🔖️GatewaySourceTree
/** 🌳️ The library roots of the producer crates the gateway splits into (`semio-os-mcp-*` plus the `semio-os-mcp` facade); each one mounts the module files it owns, so the gateway's whole Rust text is the union of their module trees. */
export const MCP_GATEWAY_CRATE_ROOTS: readonly string[] = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧱️foundation/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️wire/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📐️schema-contract/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📚️catalog-compiler/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧰️capabilities/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔌️channel-port/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧶️bridge-codec/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🪢️bridge-session/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🙋️consent/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔬️probe-document/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/⚙️plugin-runtime/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌐️inference-hub/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔗️hub-pair/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌍️hub-remote/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎛️shell-channel/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧲️channel-routing/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🪝️action-dispatch/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🪟️workspace-host/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔭️inference-tools/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧳️gateway-tools/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎯️server/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📮️http-transport/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs",
];

/** 📦️ Every cargo package the gateway splits into: the `semio-os-mcp` facade (library, binary and integration laws) and its producer crates. Native test routes run all of them. */
export const MCP_GATEWAY_PACKAGES: readonly string[] = [
  "semio-os-mcp",
  "semio-os-mcp-foundation",
  "semio-os-mcp-wire",
  "semio-os-mcp-schema",
  "semio-os-mcp-catalog",
  "semio-os-mcp-capabilities",
  "semio-os-mcp-port",
  "semio-os-mcp-bridge-codec",
  "semio-os-mcp-bridge",
  "semio-os-mcp-consent",
  "semio-os-mcp-probe-document",
  "semio-os-mcp-plugin-runtime",
  "semio-os-mcp-inference-hub",
  "semio-os-mcp-hub-pair",
  "semio-os-mcp-hub-remote",
  "semio-os-mcp-shell-channel",
  "semio-os-mcp-routing",
  "semio-os-mcp-dispatch",
  "semio-os-mcp-workspace",
  "semio-os-mcp-tools-inference",
  "semio-os-mcp-tools-gateway",
  "semio-os-mcp-server",
  "semio-os-mcp-http-transport",
];

/** 📖️ Reads the gateway's whole production Rust text as the one text the unsplit crate held: every module file below `🌉️mcp` outside the test, generated and package directories, in path order. A crate root mounts its sibling module files from outside its own directory, so the tree reader cannot follow them; the directory walk is the single enumeration instead. */
export function readMcpGatewaySource(repoRoot: string, readFile: (path: string) => string = (path) => readFileSync(path, "utf8")): string {
  const skipped = new Set(["🧪️tests", "🤖️generated", "📦️packages", "dist", "node_modules"]);
  const files: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true }).sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0))) {
      if (entry.isDirectory()) {
        if (!skipped.has(entry.name)) walk(join(directory, entry.name));
      } else if (entry.name.endsWith(".rs")) files.push(join(directory, entry.name));
    }
  };
  walk(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"));
  return files.map((file) => readFile(file)).join("\n");
}
//#endregion 🔖️GatewaySourceTree
