import assert from "node:assert/strict";
import { constants } from "node:fs";
import { lstat, open, mkdtemp, writeFile } from "node:fs/promises";
import { join, resolve, dirname } from "node:path";
import { homedir } from "node:os";
import { createHash } from "node:crypto";
import { performance } from "node:perf_hooks";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import { JSONRPCMessageSchema, InitializeResultSchema, ListToolsResultSchema } from "@modelcontextprotocol/sdk/types.js";
import { requireIssuedResourceControlInvocationV1, withRequestedIssuedResourceControlV1, scriptInvocationEnvironment, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { withOwnedCommandChannelV1, readOwnedCommandChannelLineV1, writeOwnedCommandChannelV1, closeOwnedCommandChannelV1, type OwnedCommandChannelV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/📥️capture/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { resolveMcpBin } from "../../../../../🦑️repo/🔨️modules/📚️library/🚨️policy/🖥️cli/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { readMcpWireMessageV1, writeMcpWireMessageV1, type McpWireMessageV1, type McpJsonRecordV1 } from "../../📡️stdio/📋️protocol/🟦️.ts";
import installationSchema from "../../../../../🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🧬️schema/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🧫️fixtures/🔣️.json" with { type: "json" };

const policy = Object.freeze(schema.$defs.Protocol.const), installation = Object.freeze(installationSchema.$defs.Protocol.const);
/** ⏱️ Supplies the schema-owned finite phase bound to the exact test router. */
export const canonicalOsMcpProbeMaximumMillisecondsV1 = policy.maximumMilliseconds;
type Action = "response" | "roots" | "method-not-found" | "notification" | "reject";
type Exchange = Readonly<{ pendingId: number | null; completedIds: readonly number[]; serverIds: readonly string[]; messages: number; notifications: number; nowMs: number; deadlineMs: number; message: McpWireMessageV1 }>;
type Receipt = Readonly<{ version: 1; platform: "darwin" | "linux" | "win32"; digest: string; executable: string; directory: string; source: string; mode: "prototype" | "release" }>;
type FileProof = Readonly<{ path: string; ancestry: readonly string[]; stamp: string; digest: string; bytes: number; header: string; text?: string }>;
type Provenance = Readonly<{ receipt: FileProof; pin: FileProof; executable: FileProof; profile: FileProof; record: Receipt }>;

function requestKey(id: number | string): string { return typeof id + ":" + id; }
function classify(row: Exchange): Action {
  if (row.nowMs >= row.deadlineMs || row.messages >= policy.maximumMessages) return "reject";
  const message = row.message;
  if ("method" in message) {
    if (!("id" in message)) return row.notifications < policy.maximumNotifications ? "notification" : "reject";
    if (row.serverIds.length >= policy.maximumServerRequests || row.serverIds.includes(requestKey(message.id))) return "reject";
    return message.method === "roots/list" ? "roots" : "method-not-found";
  }
  return typeof message.id === "number" && message.id === row.pendingId && !row.completedIds.includes(message.id) ? "response" : "reject";
}
function nativeReady(value: unknown): boolean { return validateJsonSchemaSubset(schema.$defs.NativeReady, value).length === 0; }
function wire(line: string): McpWireMessageV1 {
  const message = readMcpWireMessageV1(line);
  assert.equal(JSONRPCMessageSchema.safeParse(message).success, true, "SDK rejects captured JSON-RPC envelope");
  return message;
}
function neutralLaws(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  assert.equal(ajv.validate(schema, corpus), true, JSON.stringify(ajv.errors));
  const readyOracle = ajv.compile(schema.$defs.NativeReady), db = new Database(":memory:");
  try {
    for (const row of corpus.wireCases) {
      let own = false, oracle = false;
      try { readMcpWireMessageV1(row.line); own = true; } catch {}
      try { oracle = JSONRPCMessageSchema.safeParse(JSON.parse(row.line)).success; } catch {}
      assert.equal(own, row.accepted, row.id); assert.equal(oracle, row.accepted, row.id);
    }
    const readySql = db.query("SELECT CASE WHEN json_extract($v,'$.protocolVersion')=$protocol AND json_type($v,'$.capabilities.tools')='object' AND json_extract($v,'$.serverInfo.name')=$server AND json_type($v,'$.serverInfo.version')='text' AND length(json_extract($v,'$.serverInfo.version')) BETWEEN 1 AND 256 AND json_extract($v,'$.serverInfo.version')<>'warming' THEN 1 ELSE 0 END AS accepted");
    for (const row of corpus.readyCases) {
      const result = readySql.get({ $v: JSON.stringify(row.value), $protocol: policy.nativeProtocolVersion, $server: policy.serverName }) as { accepted: number };
      assert.equal(nativeReady(row.value), row.accepted, row.id); assert.equal(readyOracle(row.value), row.accepted, row.id); assert.equal(Boolean(result.accepted), row.accepted, row.id);
      if (row.accepted) assert.equal(InitializeResultSchema.safeParse(row.value).success, true, row.id);
    }
    const exchangeSql = db.query("SELECT CASE WHEN json_extract($r,'$.nowMs')>=json_extract($r,'$.deadlineMs') OR json_extract($r,'$.messages')>=$messages THEN 'reject' WHEN json_type($r,'$.message.method')='text' THEN CASE WHEN json_type($r,'$.message.id') IS NULL THEN CASE WHEN json_extract($r,'$.notifications')<$notifications THEN 'notification' ELSE 'reject' END WHEN json_array_length(json_extract($r,'$.serverIds'))>=$requests OR EXISTS(SELECT 1 FROM json_each($r,'$.serverIds') WHERE value=(CASE json_type($r,'$.message.id') WHEN 'integer' THEN 'number:' ELSE 'string:' END)||json_extract($r,'$.message.id')) THEN 'reject' WHEN json_extract($r,'$.message.method')='roots/list' THEN 'roots' ELSE 'method-not-found' END WHEN json_type($r,'$.message.id')='integer' AND json_extract($r,'$.message.id')=json_extract($r,'$.pendingId') AND NOT EXISTS(SELECT 1 FROM json_each($r,'$.completedIds') WHERE value=json_extract($r,'$.message.id')) THEN 'response' ELSE 'reject' END AS action");
    for (const row of corpus.exchangeCases) {
      const input = { ...row, message: wire(JSON.stringify(row.message)) };
      const result = exchangeSql.get({ $r: JSON.stringify(row), $messages: policy.maximumMessages, $notifications: policy.maximumNotifications, $requests: policy.maximumServerRequests }) as { action: Action };
      assert.equal(classify(input), row.action, row.id); assert.equal(result.action, row.action, row.id);
    }
  } finally { db.close(); }
}
function physicalPath(value: string): string {
  assert.equal(resolve(value), value, "Absolute canonical path required");
  assert.ok(value.length <= installation.maximumPathLength && [...value].length <= installation.maximumPathLength && !value.includes("\0"), "Bounded canonical path required");
  return value;
}
function stamp(facts: Readonly<{ dev: number; ino: number; mode: number; nlink: number; size: number; mtimeMs: number; ctimeMs: number }>): string {
  return [facts.dev, facts.ino, facts.mode, facts.nlink, facts.size, facts.mtimeMs, facts.ctimeMs].join(":");
}
async function parents(file: string, control: ResourceLeaseControl): Promise<readonly string[]> {
  const identities: string[] = [];
  for (let directory = dirname(file);; directory = dirname(directory)) {
    checkResourceLeaseControl(control); const facts = await lstat(physicalPath(directory));
    assert.ok(facts.isDirectory() && !facts.isSymbolicLink(), "Physical installation ancestry required");
    identities.push([directory, facts.dev, facts.ino, facts.mode].join(":"));
    if (dirname(directory) === directory) return Object.freeze(identities);
  }
}
async function fileProof(file: string, maximum: number, control: ResourceLeaseControl, text = false): Promise<FileProof> {
  physicalPath(file); checkResourceLeaseControl(control); const ancestry = await parents(file, control), proofStarted = performance.now();
  const before = await lstat(file); assert.ok(before.isFile() && !before.isSymbolicLink() && before.nlink === 1 && before.size > 0 && before.size <= maximum, "Bounded physical publication required");
  const handle = await open(file, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    assert.equal(stamp(await handle.stat()), stamp(before), "Captured publication descriptor changed");
    const hash = createHash("sha256"), buffer = Buffer.alloc(65536), chunks: Buffer[] = [];
    let offset = 0, header = "";
    while (offset < before.size) {
      checkResourceLeaseControl(control);
      const result = await handle.read(buffer, 0, Math.min(buffer.length, before.size - offset), offset);
      assert.ok(result.bytesRead > 0, "Complete publication bytes required");
      const chunk = buffer.subarray(0, result.bytesRead); if (!offset) header = chunk.subarray(0, 4).toString("hex");
      hash.update(chunk); if (text) chunks.push(Buffer.from(chunk)); offset += result.bytesRead;
      if (offset % 1048576 === 0) { await control.publish({ resource: "test:canonical-os-mcp:provenance", mode: "shared", elapsedMs: Math.floor(performance.now() - proofStarted) }); await control.yieldContinuation(); }
    }
    assert.equal(stamp(await handle.stat()), stamp(before), "Publication descriptor changed during read");
    assert.equal(stamp(await lstat(file)), stamp(before), "Publication path changed during read");
    assert.deepEqual(await parents(file, control), ancestry, "Installation ancestry changed during read");
    checkResourceLeaseControl(control);
    return Object.freeze({ path: file, ancestry, stamp: stamp(before), digest: hash.digest("hex"), bytes: offset, header, ...(text ? { text: Buffer.concat(chunks).toString("utf8") } : {}) });
  } finally { await handle.close(); }
}
function nativeHeader(header: string, platform: Receipt["platform"]): boolean {
  return platform === "win32" ? header.startsWith("4d5a") : platform === "linux" ? header === "7f454c46" : ["cffaedfe", "cefaedfe", "feedfacf", "feedface", "cafebabe", "bebafeca", "cafebabf", "bfbafeca"].includes(header);
}
async function provenance(root: string, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>>): Promise<Provenance> {
  assert.ok(["darwin", "linux", "win32"].includes(process.platform), "Qualified native frontend platform required");
  assert.ok(!environment.REPO_MCP_BIN && !environment.REPO_CLI_BIN && !process.env.REPO_MCP_BIN && !process.env.REPO_CLI_BIN, "Canonical frontend profile required without binary overrides");
  const platform = process.platform as Receipt["platform"], home = homedir(), local = environment.LOCALAPPDATA;
  if (platform === "win32") assert.ok(local, "Official LOCALAPPDATA installation root required");
  const installationRoot = platform === "win32" ? join(local!, ...installation.windowsDirectory) : join(home, ".local", "share", "semio", "repo");
  const receipt = await fileProof(join(installationRoot, "installed.json"), 4096, control, true), record = JSON.parse(receipt.text!) as Receipt;
  assert.deepEqual(Object.keys(record).sort(), ["version", "platform", "digest", "executable", "directory", "source", "mode"].sort());
  assert.equal(record.version, 1); assert.equal(record.platform, platform); assert.match(record.digest, /^[a-f0-9]{64}$/u); assert.ok(record.mode === "prototype" || record.mode === "release");
  assert.equal(record.mode, environment.SEMIO_BUILD_MODE, "Requested canonical frontend profile required");
  const directory = platform === "win32" ? join(installationRoot, record.digest) : join(home, ...installation.unixDirectory);
  assert.equal(record.directory, directory); assert.equal(record.executable, join(directory, platform === "win32" ? "repo.exe" : "repo"));
  assert.equal(record.source, resolveMcpBin(root, record.mode));
  const pin = await fileProof(join(root, ".🧬semio", "🦑️repo", "native-repo-path"), 4096, control, true);
  assert.equal(pin.text, record.executable.replaceAll("\\", "/") + "\n");
  const profile = await fileProof(record.source, installation.maximumBinaryBytes, control), executable = await fileProof(record.executable, installation.maximumBinaryBytes, control);
  assert.equal(profile.digest, record.digest); assert.equal(executable.digest, record.digest);
  assert.ok(nativeHeader(profile.header, platform) && nativeHeader(executable.header, platform), "Native profile and installed executable required");
  return Object.freeze({ receipt, pin, profile, executable, record: Object.freeze(record) });
}
function assertSameProvenance(before: Provenance, after: Provenance): void {
  assert.deepEqual(after.record, before.record);
  for (const key of ["receipt", "pin", "profile", "executable"] as const) assert.deepEqual(after[key], before[key], "Canonical publication changed during protocol probe");
}

async function send(channel: OwnedCommandChannelV1, message: unknown): Promise<void> { await writeOwnedCommandChannelV1(channel, { version: 1, text: writeMcpWireMessageV1(message) + "\n" }); }
function transcript(stdout: string, consumed: number): Readonly<{ messages: number; notifications: number; serverRequests: number }> {
  const lines = stdout.split("\n"); if (lines.at(-1) === "") lines.pop();
  const completed: number[] = [], serverIds: string[] = []; let notifications = 0;
  for (let index = 0; index < lines.length; index++) {
    const message = wire(lines[index]!), action = classify({ pendingId: completed.length < 3 ? completed.length + 1 : null, completedIds: completed, serverIds, messages: index, notifications, nowMs: 0, deadlineMs: policy.maximumMilliseconds, message });
    assert.notEqual(action, "reject", "Unexpected or duplicate captured protocol message");
    if (index >= consumed) assert.equal(action, "notification", "Unanswered trailing server message refused");
    if (action === "notification") notifications++;
    else if (action === "response") { assert.ok("result" in message, "Captured method failure refused"); completed.push(message.id as number); }
    else { assert.ok("id" in message); serverIds.push(requestKey(message.id)); }
  }
  assert.deepEqual(completed, [1, 2, 3]); return Object.freeze({ messages: lines.length, notifications, serverRequests: serverIds.length });
}
/** 🧪️ Probes the actual officially installed native frontend and its forwarded OS server under the received Original. */
export async function runCanonicalOsMcpProbeV1(root: string, invocation: ScriptInvocation, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>>): Promise<void> {
  requireIssuedResourceControlInvocationV1(control, invocation);
  await withRequestedIssuedResourceControlV1(control, policy.maximumMilliseconds, phase => runProbe(root, invocation, phase, environment));
}
async function runProbe(root: string, invocation: ScriptInvocation, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>>): Promise<void> {
  requireIssuedResourceControlInvocationV1(control, invocation); checkResourceLeaseControl(control); neutralLaws();
  const artifacts = repoTestArtifactEnvironment(root, "canonical-os-mcp", { ...environment }), artifactRoot = physicalPath(resolve(artifacts.SEMIO_TEST_ARTIFACT_DIR!));
  const bridge = physicalPath(await mkdtemp(join(artifactRoot, "os-"))), before = await provenance(root, control, artifacts), started = performance.now();
  const run = await withOwnedCommandChannelV1(before.record.executable, policy.args, root, { control, env: scriptInvocationEnvironment(invocation, { ...artifacts, S_AGENT_BRIDGE_DIR: bridge }), output: policy.output }, async channel => {
    const completed: number[] = [], serverIds: string[] = []; let messages = 0, notifications = 0;
    const request = async (id: number, method: string, params: McpJsonRecordV1): Promise<McpJsonRecordV1> => {
      checkResourceLeaseControl(control); await send(channel, { jsonrpc: "2.0", id, method, params });
      while (true) {
        checkResourceLeaseControl(control); const line = await readOwnedCommandChannelLineV1(channel); assert.notEqual(line, null, "Canonical frontend closed before correlated response");
        const message = wire(line!), action = classify({ pendingId: id, completedIds: completed, serverIds, messages, notifications, nowMs: Math.floor(performance.now() - started), deadlineMs: policy.maximumMilliseconds, message });
        assert.notEqual(action, "reject", "Unexpected, duplicate or unbounded protocol message"); messages++;
        if (action === "notification") { notifications++; continue; }
        if (action === "roots" || action === "method-not-found") {
          assert.ok("id" in message); serverIds.push(requestKey(message.id));
          await send(channel, action === "roots" ? { jsonrpc: "2.0", id: message.id, result: { roots: [] } } : { jsonrpc: "2.0", id: message.id, error: { code: -32601, message: "Method not found" } }); continue;
        }
        assert.ok("result" in message, "Correlated native method returned a protocol error"); completed.push(id); return message.result;
      }
    };
    const initialized = await request(1, "initialize", { protocolVersion: policy.protocolVersion, capabilities: { roots: { listChanged: false } }, clientInfo: policy.client });
    InitializeResultSchema.parse(initialized); assert.equal(initialized.protocolVersion, policy.protocolVersion);
    await send(channel, { jsonrpc: "2.0", method: "notifications/initialized", params: {} });
    const discovered = await request(2, policy.nativeMethod, {});
    assert.equal(nativeReady(discovered), true, "Real forwarded native readiness required; warming is insufficient"); InitializeResultSchema.parse(discovered);
    const tools = ListToolsResultSchema.parse(await request(3, "tools/list", { _meta: { [policy.metaKey]: policy.nativeProtocolVersion } }));
    const names = tools.tools.map(tool => tool.name); assert.equal(new Set(names).size, names.length); assert.ok(names.length > 0);
    for (const name of policy.requiredTools) assert.ok(names.includes(name), "Actual native gateway tool required: " + name);
    const terminal = await closeOwnedCommandChannelV1(channel);
    assert.ok(terminal.status !== null || terminal.signal !== null, "Actual owned shutdown terminal values required");
    return Object.freeze({ protocolBody: "passed", consumed: messages, notifications, serverRequests: serverIds.length, nativeVersion: (discovered.serverInfo as McpJsonRecordV1).version, tools: names.length, terminal });
  });
  assert.equal(run.capture.status, run.value.terminal.status); assert.equal(run.capture.signal, run.value.terminal.signal);
  const observed = transcript(run.capture.stdout, run.value.consumed), after = await provenance(root, control, artifacts); assertSameProvenance(before, after);
  checkResourceLeaseControl(control);
  const result = { version: 1, protocolBody: run.value.protocolBody, installed: { platform: before.record.platform, mode: before.record.mode, digest: before.record.digest, executable: before.record.executable, source: before.record.source }, nativeVersion: run.value.nativeVersion, tools: run.value.tools, messages: observed.messages, notifications: observed.notifications, serverRequests: observed.serverRequests, ownedShutdown: run.value.terminal, stdoutBytes: Buffer.byteLength(run.capture.stdout), stderrBytes: Buffer.byteLength(run.capture.stderr), stdoutSha256: createHash("sha256").update(run.capture.stdout).digest("hex"), stderrSha256: createHash("sha256").update(run.capture.stderr).digest("hex"), outerWhole: "recorded-by-owning-target" };
  await writeFile(physicalPath(join(artifactRoot, "canonical-os-mcp.json")), JSON.stringify(result, null, 2) + "\n", { flag: "wx", mode: 0o600 });
  console.log("[test] Canonical OS MCP protocol passed; actual owned shutdown recorded separately from target whole status");
}
