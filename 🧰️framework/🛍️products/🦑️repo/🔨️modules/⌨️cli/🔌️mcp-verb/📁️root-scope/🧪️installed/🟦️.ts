import "../📄️resources/🧪️tests/🟦️.ts";
import assert from "node:assert/strict";
import { SqliteDatabaseV1 as Database } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗃️sqlite/🟦️.ts";
import { test } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧪️test/🟦️.ts";
import { mkdir, mkdtemp, readdir, readFile, lstat, writeFile } from "node:fs/promises";
import { join, resolve, dirname } from "node:path";
import { pathToFileURL } from "node:url";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import ResourceAjv from "ajv";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { JSONRPCMessageSchema, ListRootsRequestSchema, type JSONRPCMessage } from "@modelcontextprotocol/sdk/types.js";
import { findWorkspaceRoot, scriptInvocationEnvironment, withRequestedIssuedResourceControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { withOwnedCommandChannelV1, readOwnedCommandChannelLineV1, writeOwnedCommandChannelV1, closeOwnedCommandChannelV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/📥️capture/🟦️.ts";
import { registerBuildTestLawV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { captureInstalledFrontendProofV1, assertSameInstalledFrontendProofV1 } from "../../../../📚️library/📦️packages/🟦️typescript/🧪️testing/📦️installed-frontend/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../📚️library/📦️packages/🟦️typescript/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "../🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import resources from "../📄️resources/🧬️schema/🔣️.json" with { type: "json" };
import resourceCases from "../📄️resources/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };

const policy = Object.freeze(schema.$defs.InstalledProtocol.const), root = findWorkspaceRoot(import.meta.dirname);
const ajv = new Ajv({ strict: false, allErrors: true });
const resourceAjv = new ResourceAjv({ strict: false, allErrors: true }); resourceAjv.addSchema(resources);
const resourceValidators = { files: resourceAjv.getSchema(resources.$id + "#/$defs/FilesResource")!, folders: resourceAjv.getSchema(resources.$id + "#/$defs/FoldersResource")! };
function physical(value: string): string { assert.equal(resolve(value), value); assert.ok(value.length <= policy.maximumPhysicalPathUnits && !value.includes("\0")); return value; }
function key(value: string | number | undefined): string { return typeof value + ":" + value; }
function content(field: "files" | "folders", text: string): Record<string, string>[] {
  const value = JSON.parse(text), own = field === "files" ? resources.$defs.FilesResource : resources.$defs.FoldersResource;
  assert.deepEqual(validateJsonSchemaSubset(own, value, resources), []);
  assert.equal(resourceValidators[field](value), true, JSON.stringify(resourceValidators[field].errors));
  return value.repo[field];
}
test("canonical files and folders JSON agrees with independent schema and SQLite oracles", () => {
  assert.equal(ajv.validate(schema, corpus), true, JSON.stringify(ajv.errors));
  assert.equal(resourceAjv.getSchema(resources.$id + "#/$defs/Fixture")!(resourceCases), true, JSON.stringify(resourceAjv.errors));
  const db = new Database(":memory:");
  try {
    for (const row of resourceCases.cases) {
      const field = row.uri === "repo://files" ? "files" : "folders", rows = content(field, JSON.stringify(row.document));
      const oracle = db.query("SELECT json_extract(value,'$.path') AS path FROM json_each($document,$pointer) ORDER BY path").all({ $document: JSON.stringify(row.document), $pointer: "$.repo." + field }) as { path: string }[];
      assert.deepEqual(rows.map(value => value.path).sort(), oracle.map(value => value.path), row.name);
    }
  } finally { db.close(); }
});
async function inventory(directory: string, control: ResourceLeaseControl): Promise<Readonly<{ files: readonly string[]; folders: readonly string[]; facts: readonly string[] }>> {
  const files: string[] = [], folders: string[] = [], facts: string[] = [];
  const visit = async (current: string, prefix: string): Promise<void> => {
    checkResourceLeaseControl(control);
    for (const name of (await readdir(current)).sort()) {
      checkResourceLeaseControl(control); assert.ok(files.length + folders.length < policy.maximumFixtureEntries);
      const file = physical(join(current, name)), relative = prefix + name, stat = await lstat(file);
      assert.ok(!stat.isSymbolicLink());
      if (stat.isDirectory()) { folders.push(relative); facts.push(relative + ":" + stat.dev + ":" + stat.ino + ":directory"); await visit(file, relative + "/"); }
      else { assert.ok(stat.isFile() && stat.nlink === 1 && stat.size <= 256); files.push(relative); facts.push(relative + ":" + stat.dev + ":" + stat.ino + ":" + createHash("sha256").update(await readFile(file)).digest("hex")); }
    }
  };
  await visit(directory, ""); return Object.freeze({ files: files.sort(), folders: folders.sort(), facts: facts.sort() });
}
type ScopePorts = Readonly<{ check(): void; read(): Promise<string | null>; write(text: string): Promise<void>; close(): Promise<{ status: number | null; signal: string | null }> }>;
async function conversation(ports: ScopePorts, directory: string, expected: Readonly<{ files: readonly string[]; folders: readonly string[] }>, profile: string, name: string) {
  let pump: Promise<void> | undefined, failure: unknown, failed = false, messages = 0, notifications = 0, bytes = 0, closed = false;
  let terminal: Awaited<ReturnType<typeof closeOwnedCommandChannelV1>> | undefined;
  const refuse = (error: unknown): void => { if (!failed) { failed = true; failure = error; } };
  const pending = new Map<string, string>(), completed = new Set<string>(), serverIds = new Set<string>();
  const transport: { onmessage?: (message: JSONRPCMessage) => void; onerror?: (error: Error) => void; onclose?: () => void; start(): Promise<void>; send(message: JSONRPCMessage): Promise<void>; close(): Promise<void> } = {
    async start() {
      pump = (async () => {
        while (true) {
          if (closed) break;
          ports.check(); const line = await ports.read(); if (line === null) break;
          bytes += Buffer.byteLength(line) + 1; assert.ok(++messages <= policy.maximumMessages && bytes <= policy.output.maximumBytes);
          const message = JSONRPCMessageSchema.parse(JSON.parse(line));
          if ("method" in message) {
            if ("id" in message) { const id = key(message.id); assert.ok(serverIds.size < policy.maximumServerRequests && !serverIds.has(id)); serverIds.add(id); }
            else assert.ok(++notifications <= policy.maximumNotifications);
          } else { const id = key(message.id); assert.ok(pending.has(id) && !completed.has(id), "Unsolicited or duplicate native response refused"); pending.delete(id); completed.add(id); }
          transport.onmessage?.(message);
        }
        if (!closed) transport.onclose?.();
      })().catch(error => { refuse(error); transport.onerror?.(error instanceof Error ? error : Error("Native protocol reader refused")); transport.onclose?.(); });
    },
    async send(message) {
      ports.check(); JSONRPCMessageSchema.parse(message);
      if ("method" in message && "id" in message) { const id = key(message.id); assert.ok(!pending.has(id) && !completed.has(id)); pending.set(id, message.method); }
      else if (!("method" in message)) assert.ok(serverIds.has(key(message.id)), "Unsolicited client response refused");
      await ports.write(JSON.stringify(message) + "\n");
    },
    async close() { if (!closed) { closed = true; try { terminal = await ports.close(); } catch (error) { refuse(error); } } await pump; }
  };
  const client = new Client(policy.client, { capabilities: { roots: { listChanged: false } } });
  client.setRequestHandler(ListRootsRequestSchema, async () => ({ roots: [{ uri: pathToFileURL(directory).href, name: name }] }));
  try {
    await client.connect(transport);
    const advertised = await client.listResources();
    for (const read of corpus.reads) {
      assert.equal(advertised.resources.find(value => value.uri === read.uri)?.mimeType, corpus.mimeType);
      const response = await client.readResource({ uri: read.uri }); assert.equal(response.contents.length, 1);
      const resource = response.contents[0]!; assert.equal(resource.uri, read.uri); assert.equal(resource.mimeType, corpus.mimeType); assert.ok("text" in resource);
      const field = read.field as "files" | "folders", actual = content(field, resource.text as string);
      assert.deepEqual(actual.map(value => value.path).sort(), expected[field], profile + ":" + name + ":" + field);
    }
    assert.equal(pending.size, 0); assert.equal(completed.size, 4); if (failed) throw failure;
  } catch (error) { refuse(error); }
  try { await client.close(); await transport.close(); } catch (error) { refuse(error); }
  if (failed) throw failure;
  assert.ok(terminal && (terminal.status !== null || terminal.signal !== null));
  return { profile, root: name, protocolBody: "passed", messages, notifications, serverRequests: serverIds.size, terminal };
}

test("SDK bridge preserves its first parser or protocol refusal through connection and close settlement", async () => {
  for (const row of corpus.bridgeCases) {
    let waiting: { accept(value: string | null): void; refuse(error: unknown): void } | undefined, sent = 0, closes = 0, observed = false, actual: unknown;
    const cleanup = Error("Independent cleanup refusal");
    const ports: ScopePorts = {
      check() {},
      read() { assert.equal(waiting, undefined); return new Promise((accept, refuse) => { waiting = { accept, refuse }; }); },
      async write(text) { const request = JSONRPCMessageSchema.parse(JSON.parse(text)); assert.ok("method" in request && request.method === "initialize"); assert.equal(++sent, 1); const read = waiting!; waiting = undefined; if (row.mode === "reader-undefined") read.refuse(undefined); else read.accept(row.mode === "malformed" ? "{" : JSON.stringify({ jsonrpc: "2.0", id: 987654321, result: {} })); },
      async close() { closes++; if (waiting) { waiting.accept(null); waiting = undefined; } throw cleanup; }
    };
    try { await conversation(ports, root, { files: [], folders: [] }, "generic", row.name); } catch (error) { observed = true; actual = error; }
    assert.equal(observed, true, row.name); assert.equal(closes, 1, row.name); assert.equal(waiting, undefined, row.name); assert.notEqual(actual, cleanup);
    if (row.expected === "syntax") assert.ok(actual instanceof SyntaxError);
    else if (row.expected === "undefined") assert.equal(actual, undefined);
    else assert.ok(actual instanceof Error && actual.message.includes("Unsolicited or duplicate native response refused"));
  }
}, 2000);

registerBuildTestLawV1("official installed repo MCP SDK bounds explicit sibling roots and preserves canonical JSON", "sdk-runtime", async original => {
  const admission = scriptCompilerAdmissionControlV1(original, "test:installed-repo-mcp-scope", policy.maximumMilliseconds);
  await withRequestedIssuedResourceControlV1(admission, policy.maximumMilliseconds, async control => {
    const environment = repoTestArtifactEnvironment(root, "installed-repo-mcp-scope", { ...process.env });
    const artifacts = physical(resolve(environment.SEMIO_TEST_ARTIFACT_DIR!));
    physical(join(artifacts, "rp-XXXXXX")); await mkdir(artifacts, { recursive: true });
    const fixtures = physical(await mkdtemp(join(artifacts, "rp-"))), beforeCwd = process.cwd(), paths = new Map<string, string>();
    for (const row of corpus.cases) {
      const directory = physical(join(fixtures, row.name)); await mkdir(directory); paths.set(row.name, directory);
      for (const folder of row.folders) await mkdir(physical(join(directory, folder)), { recursive: true });
      for (const file of row.files) { const target = physical(join(directory, file.path)); physical(dirname(target)); await mkdir(dirname(target), { recursive: true }); await writeFile(target, file.text, { flag: "wx" }); }
    }
    const before = await captureInstalledFrontendProofV1(root, control, environment), results: unknown[] = [];
    for (const profile of corpus.profiles) for (const row of corpus.cases) {
      checkResourceLeaseControl(control);
      const directory = paths.get(row.name)!, expected = await inventory(directory, control);
      assert.deepEqual(expected.files, row.files.map(value => value.path).sort()); assert.deepEqual(expected.folders, [...row.folders].sort());
      const run = await withOwnedCommandChannelV1(before.record.executable, [...policy.args, directory], root, { control, env: scriptInvocationEnvironment(original, { ...environment, SEMIO_REPO_MCP_CLIENT: profile }), output: policy.output }, async channel => {
        return conversation({ check: () => checkResourceLeaseControl(control), read: () => readOwnedCommandChannelLineV1(channel), write: text => writeOwnedCommandChannelV1(channel, { version: 1, text }), close: () => closeOwnedCommandChannelV1(channel) }, directory, expected, profile, row.name);
      });
      assert.equal(run.capture.status, run.value.terminal!.status); assert.equal(run.capture.signal, run.value.terminal!.signal);
      const captured = run.capture.stdout.split("\n"); if (captured.at(-1) === "") captured.pop();
      assert.ok(captured.length >= run.value.messages && captured.length <= policy.maximumMessages);
      for (let index = 0; index < captured.length; index++) { const message = JSONRPCMessageSchema.parse(JSON.parse(captured[index]!)); if (index >= run.value.messages) assert.ok("method" in message && !("id" in message), "Unanswered trailing native message refused"); }
      assert.ok(run.value.notifications + captured.length - run.value.messages <= policy.maximumNotifications);
      assert.deepEqual(await inventory(directory, control), expected); assert.equal(process.cwd(), beforeCwd);
      results.push({ ...run.value, stdoutBytes: Buffer.byteLength(run.capture.stdout), stderrBytes: Buffer.byteLength(run.capture.stderr), stdoutSha256: createHash("sha256").update(run.capture.stdout).digest("hex"), stderrSha256: createHash("sha256").update(run.capture.stderr).digest("hex") });
    }
    assertSameInstalledFrontendProofV1(before, await captureInstalledFrontendProofV1(root, control, environment));
    await writeFile(physical(join(artifacts, "installed-repo-mcp-scope.json")), JSON.stringify({ version: 1, protocolBody: "passed", installed: before.record, results, outerWhole: "recorded-by-owning-target" }, null, 2) + "\n", { flag: "wx" });
  });
});
