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
import { cargoTargetDirectory } from "../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";

//#region 🔖️BinaryPath
/** 📦️ Nx owns the executable separately from mutable compiler state. */
export const MCP_ARTIFACT_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build";
export const MCP_CARGO_PACKAGE = "semio-framework-os-mcp";
export const MCP_BINARY_NAME = "semio-os-mcp";

function pathApi(platform: NodeJS.Platform): typeof posix {
  return platform === "win32" ? win32 : posix;
}

/** 📁️ Absolute Cargo target root shared by the build and black-box test gates. */
export function resolveMcpTargetDirectory(repoRoot: string, env: NodeJS.ProcessEnv = process.env, _platform: NodeJS.Platform = process.platform): string {
  return cargoTargetDirectory(repoRoot, env);
}

/** 📦️ The exact debug artifact Cargo's MCP build command must produce, ignoring test overrides. */
export function resolveBuiltMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const filename = platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME;
  return pathApi(platform).join(resolveMcpTargetDirectory(repoRoot, env, platform), "debug", filename);
}

/** 📁️ Resolves the staged executable or an explicit independent binary override. */
export function resolveMcpBinaryPath(repoRoot: string, env: NodeJS.ProcessEnv = process.env, platform: NodeJS.Platform = process.platform): string {
  const override = env.SEMIO_OS_MCP_BIN;
  return override ? pathApi(platform).resolve(repoRoot, override) : pathApi(platform).resolve(repoRoot, MCP_ARTIFACT_REL, platform === "win32" ? `${MCP_BINARY_NAME}.exe` : MCP_BINARY_NAME);
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
 * property filled with a value of its declared type. The catalog types action inputs per capability,
 * so an e2e that hardcoded one plugin's argument names would only ever exercise that plugin. */
function minimalInputForSchema(schema: any): Record<string, unknown> {
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
    else if (type === "object") input[name] = {};
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
  const session = new McpClientSession(entry, ["--folder", folder], repoRoot);
  const steps: McpClientStep[] = [];
  try {
    const initialized = await session.request("initialize", { protocolVersion: MCP_CLIENT_PROTOCOL_VERSION, capabilities: { roots: { listChanged: true }, sampling: {}, elicitation: {} }, clientInfo: { name: "semio-mcp-client-e2e", title: "semio MCP client e2e", version: "1" } });
    if (initialized.error) return [...steps, mcpStepFailure("os: initialize", JSON.stringify(initialized.error))];
    session.notify("notifications/initialized", {});
    announce(steps, { step: "os: initialize", ok: true, detail: `server=${initialized.result?.serverInfo?.name}@${initialized.result?.serverInfo?.version} protocol=${initialized.result?.protocolVersion}`, wire: initialized.result });

    const tools = await session.request("tools/list", {});
    const toolNames = ((tools.result?.tools ?? []) as Array<{ name: string }>).map((tool) => tool.name).sort();
    const requiredTools = ["action_invoke", "action_prepare", "artifact_create", "artifact_open", "artifact_snapshot", "capabilities_search", "history_undo", "inference_run", "job_cancel"];
    const missingTools = requiredTools.filter((name) => !toolNames.includes(name));
    announce(steps, { step: "os: tools/list", ok: missingTools.length === 0 && toolNames.length >= 9, detail: missingTools.length === 0 ? `${toolNames.length} tools, all ${requiredTools.length} required present` : `missing: ${missingTools.join(", ")}`, wire: toolNames });

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

    const mutations = await session.call("capabilities_search", { query: "set", kind: ["mutation"] });
    const mutationHits = (mutations.structuredContent?.results ?? []) as Array<Record<string, any>>;
    const target = mutationHits[0];
    if (!target) {
      announce(steps, mcpStepFailure("os: pick a mutation capability", `capabilities_search(kind=[mutation]) returned ${mutationHits.length} hit(s) — no plugin mutation is reachable`, mutations.structuredContent));
      return steps;
    }
    const described = await session.call("capabilities_describe", { capabilityId: target.capabilityId });
    const inputSchema = described.structuredContent?.inputSchema ?? described.structuredContent?.capability?.inputSchema;
    const input = minimalInputForSchema(inputSchema);
    announce(steps, { step: "os: capabilities_describe", ok: described.isError !== true, detail: `${target.capabilityId} input=${JSON.stringify(input).slice(0, 120)}`, wire: described.structuredContent });

    const typedKind = String(described.structuredContent?.artifactKind ?? described.structuredContent?.capability?.artifactKind ?? target.artifactKind ?? "");
    const typedArtifactId = `${artifactId}-typed`;
    const typedCreated = await session.call("artifact_create", { artifactId: typedArtifactId, kind: typedKind });
    announce(steps, { step: "os: artifact_create (a real plugin artifact kind)", ok: typedCreated.isError !== true && typedCreated.structuredContent?.kind === typedKind, detail: typedCreated.isError === true ? `kind=${typedKind}: ${JSON.stringify(typedCreated.structuredContent).slice(0, 260)}` : `artifactId=${typedArtifactId} kind=${typedCreated.structuredContent?.kind} pluginId=${typedCreated.structuredContent?.pluginId} sizeBytes=${typedCreated.structuredContent?.sizeBytes}`, wire: typedCreated.structuredContent });

    const exportTarget = typedCreated.isError === true ? artifactId : typedArtifactId;
    const exported = await session.call("artifact_export", { artifactId: exportTarget });
    announce(steps, { step: "os: artifact_export", ok: exported.isError !== true && typeof exported.structuredContent?.contentBase64 === "string" && (exported.structuredContent?.contentBase64 as string).length > 0, detail: exported.isError === true ? `${exportTarget}: ${JSON.stringify(exported.structuredContent).slice(0, 300)}` : `artifactId=${exportTarget} port=${exported.structuredContent?.format} base64Bytes=${String(exported.structuredContent?.contentBase64 ?? "").length} declaredFormats=${JSON.stringify(exported.structuredContent?.declaredExportFormats ?? [])}`, wire: { ...exported.structuredContent, contentBase64: `${String(exported.structuredContent?.contentBase64 ?? "").slice(0, 32)}…` } });

    const prepared = await session.call("action_prepare", { capabilityId: target.capabilityId, input });
    const baselineRevision = revisionOf(prepared);
    announce(steps, { step: "os: action_prepare", ok: prepared.isError !== true, detail: prepared.isError === true ? JSON.stringify(prepared.structuredContent).slice(0, 300) : `handle=${prepared.structuredContent?.preparedHandle} baseline=${stampText(baselineRevision)}`, wire: prepared.structuredContent });
    if (prepared.isError === true) return steps;

    const invoked = await session.call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
    const undoToken = invoked.structuredContent?.undoToken as string | undefined;
    const revisionBefore = invoked.structuredContent?.revisionBefore as RevisionStampWire | undefined;
    const revisionAfter = invoked.structuredContent?.revisionAfter as RevisionStampWire | undefined;
    announce(steps, { step: "os: action_invoke (a real mutation)", ok: invoked.isError !== true && invoked.structuredContent?.status === "SUCCEEDED" && revisionAfter?.headEditId !== undefined && revisionAfter?.headEditId !== revisionBefore?.headEditId, detail: invoked.isError === true ? JSON.stringify(invoked.structuredContent).slice(0, 300) : `status=${invoked.structuredContent?.status} ${stampText(revisionBefore)} → ${stampText(revisionAfter)} undoToken=${undoToken ?? "<none>"}`, wire: invoked.structuredContent });

    const mutatedArtifact = revisionAfter?.artifactId ?? ((invoked.structuredContent?.affectedResources ?? []) as string[]).find((resource) => typeof resource === "string") ?? artifactId;
    const afterSnapshot = await session.call("artifact_snapshot", { artifactId: mutatedArtifact });
    announce(steps, { step: "os: artifact_snapshot (mutation visible)", ok: afterSnapshot.isError !== true && Number(afterSnapshot.structuredContent?.packBytes ?? 0) > 0, detail: afterSnapshot.isError === true ? JSON.stringify(afterSnapshot.structuredContent).slice(0, 300) : `artifactId=${mutatedArtifact} packBytes=${afterSnapshot.structuredContent?.packBytes} sprBytes=${afterSnapshot.structuredContent?.sprBytes}`, wire: { ...afterSnapshot.structuredContent, packBase64: `${String(afterSnapshot.structuredContent?.packBase64 ?? "").slice(0, 32)}…` } });

    const headAfterInvoke = await headRevision(session, target.capabilityId, input);
    announce(steps, { step: "os: live head advanced", ok: headAfterInvoke !== undefined && headAfterInvoke.headEditId === revisionAfter?.headEditId && headAfterInvoke.headEditId !== baselineRevision?.headEditId, detail: `re-read head ${stampText(headAfterInvoke)} (baseline ${stampText(baselineRevision)}, invoke reported ${stampText(revisionAfter)})`, wire: headAfterInvoke });

    if (!undoToken) {
      announce(steps, mcpStepFailure("os: history_undo (mutation reverted)", "action_invoke minted no undoToken — nothing to undo", invoked.structuredContent));
      return steps;
    }
    const undone = await session.call("history_undo", { undoToken });
    const headAfterUndo = await headRevision(session, target.capabilityId, input);
    announce(steps, { step: "os: history_undo (mutation reverted)", ok: undone.isError !== true && Number(undone.structuredContent?.members ?? 0) > 0 && headAfterUndo?.headEditId === baselineRevision?.headEditId, detail: undone.isError === true ? JSON.stringify(undone.structuredContent).slice(0, 300) : `members=${undone.structuredContent?.members} head ${stampText(headAfterUndo)} (baseline ${stampText(baselineRevision)})`, wire: undone.structuredContent });

    const redone = await session.call("history_redo", { undoToken });
    const headAfterRedo = await headRevision(session, target.capabilityId, input);
    announce(steps, { step: "os: history_redo (mutation restored)", ok: redone.isError !== true && Number(redone.structuredContent?.members ?? 0) > 0 && headAfterRedo?.headEditId !== undefined && headAfterRedo.headEditId !== baselineRevision?.headEditId, detail: redone.isError === true ? JSON.stringify(redone.structuredContent).slice(0, 300) : `members=${redone.structuredContent?.members} head ${stampText(headAfterRedo)} (undone head was ${stampText(headAfterUndo)})`, wire: redone.structuredContent });

    const sagaMember = await session.call("action_prepare", { capabilityId: target.capabilityId, input });
    const began = await session.call("transaction_begin", { preparedHandles: [sagaMember.structuredContent?.preparedHandle] });
    const transactionHandle = began.structuredContent?.transactionHandle as string | undefined;
    announce(steps, { step: "os: transaction_begin", ok: began.isError !== true && typeof transactionHandle === "string", detail: began.isError === true ? JSON.stringify(began.structuredContent).slice(0, 300) : `transactionHandle=${transactionHandle} members=1`, wire: began.structuredContent });
    const rolledBack = await session.call("transaction_rollback", { transactionHandle });
    const headAfterRollback = await headRevision(session, target.capabilityId, input);
    announce(steps, { step: "os: transaction_rollback (no change)", ok: rolledBack.isError !== true && headAfterRollback?.headEditId === headAfterRedo?.headEditId, detail: rolledBack.isError === true ? JSON.stringify(rolledBack.structuredContent).slice(0, 300) : `rolledBack=${rolledBack.structuredContent?.rolledBack} head ${stampText(headAfterRollback)} unchanged from ${stampText(headAfterRedo)}`, wire: rolledBack.structuredContent });

    const listed = await session.call("inference_list", {});
    const declared = (listed.structuredContent?.declared ?? []) as Array<Record<string, any>>;
    announce(steps, { step: "os: inference_list", ok: listed.isError !== true && declared.length > 0, detail: listed.isError === true ? JSON.stringify(listed.structuredContent).slice(0, 200) : `${declared.length} declared inference(s)${declared[0] ? `, first=${declared[0].artifactKind}/${declared[0].inferenceSchema} by ${declared[0].contributor || declared[0].owner}` : " — no installed plugin declares one"}`, wire: declared.slice(0, 5) });
    const service = declared[0];
    if (!service) return steps;
    const inference = await session.call("inference_run", { artifactKind: service.artifactKind, inferenceSchema: service.inferenceSchema, pluginId: service.contributor || service.owner, cancellationId: `${artifactId}-cancel` });
    const jobId = inference.structuredContent?.jobId as string | undefined;
    announce(steps, { step: "os: inference_run", ok: inference.isError !== true, detail: inference.isError === true ? JSON.stringify(inference.structuredContent).slice(0, 300) : `jobId=${jobId} status=${inference.structuredContent?.status} complete=${inference.structuredContent?.complete} bytes=${inference.structuredContent?.payloadBytes ?? 0}`, wire: inference.structuredContent });

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
  await registerTests1(import.meta.vitest, { readFileSync, requireMcpBinary, resolveBuiltMcpBinaryPath, resolveMcpBinaryPath }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
