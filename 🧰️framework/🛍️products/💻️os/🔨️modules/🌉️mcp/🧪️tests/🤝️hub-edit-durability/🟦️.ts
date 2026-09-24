/** 🤝️ The hub-edit-durability gate: is every accepted hub edit durable collaboration state?
 *
 * Fixture `🌉️mcp/🧫️fixtures/🤝️hub-edit-durability/🔣️.json` names the lanes. Each lane commits one
 * edit on the same hub note while a human observer holds a live document socket: agent lanes run
 * the fixture gesture through a FRESH `semio-os-mcp stdio --hub` process (same gesture, same first
 * sequence, so two replicas that shared an identity would mint the same id), the human lane sends
 * a real note operation over its own document socket. Every lane must be relayed to the observer,
 * advance the ledger head by one, and reach a late joiner's catch-up — before and after the gate
 * restarts the hub on the same data directory.
 *
 * The gate owns its hub (a restart is part of the law) and its document: every run creates a fresh
 * note through the hub's own server-owned creation transaction, so the document always matches the
 * catalog generation the hub serves. Configuration by environment:
 *
 *   OS_MCP_HUB_BINARY    the `os-hub` executable to boot
 *   OS_MCP_HUB_DATA_DIR  a published catalog root (note in its generation) where the human authors a space (copied, never mutated)
 *   OS_MCP_HUB_PORT      loopback port (default 7852)
 *   OS_MCP_HUB_EMAIL / OS_MCP_HUB_PASSWORD  the human (defaults `user1@semio.dev`)
 *   SEMIO_TEST_ARTIFACT_DIR  where the run copy of the data directory lives (default: tmpdir)
 */
import { chmodSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { McpClientSession, mcpServerEntries, requireMcpBinary } from "../../🟦️.ts";
import { decodeServerFrame, encodeClientFrame } from "../../../../../../🔨️modules/📡️replication/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "../../../📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const here = dirname(fileURLToPath(new URL(import.meta.url)));
function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`the hub-edit-durability gate could not locate the repository root above ${start}`);
}
const repoRoot = findRepoRoot(here);
const fixture = JSON.parse(readFileSync(join(here, "../../🧫️fixtures/🤝️hub-edit-durability/🔣️.json"), "utf8"));
const hubRoot = join(repoRoot, readdirSync(repoRoot).find((name) => existsSync(join(repoRoot, name, "🚀️local-bootstrap", "🏃️execution", "🟦️.ts")))!);
const { startLocalHub, waitForReadiness, finishLocalHub } = await import(join(hubRoot, "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const BINARY = process.env.OS_MCP_HUB_BINARY ?? "";
const SOURCE = process.env.OS_MCP_HUB_DATA_DIR ?? "";
const PORT = Number(process.env.OS_MCP_HUB_PORT ?? 7852);
const ORIGIN = `http://127.0.0.1:${PORT}`;
const EMAIL = process.env.OS_MCP_HUB_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.OS_MCP_HUB_PASSWORD ?? "gm1-local-dev-pass-1";
if (!existsSync(BINARY) || !existsSync(SOURCE)) throw new Error(`hub-edit-durability-check needs OS_MCP_HUB_BINARY and OS_MCP_HUB_DATA_DIR (got ${BINARY || "<unset>"}, ${SOURCE || "<unset>"})`);

type Row = { readonly step: string; readonly ok: boolean; readonly detail: string };
const rows: Row[] = [];
const row = (step: string, ok: boolean, detail: string) => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);
};
const pause = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const hub = async (method: string, path: string, token?: string, body?: unknown) => {
  const response = await fetch(`${ORIGIN}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }), signal: AbortSignal.timeout(60_000) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};

const artifactBase = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
mkdirSync(artifactBase, { recursive: true });
const dataDir = join(mkdtempSync(join(artifactBase, "hub-edit-durability-")), "hub");
cpSync(SOURCE, dataDir, { recursive: true, filter: (path) => !path.includes(".semio-wal-writer") });
process.env.OS_HUB_CREDENTIAL_SIGN_IN = "true";
const bootHub = async () => {
  const run = await startLocalHub(repoRoot, join(hubRoot, "📦️packages", "🦀️rust"), [{ profileId: "developer", subject: "local-developer-hub-edit-durability", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], { port: PORT, dataDir, binaryPath: BINARY, capture: true });
  const readiness = await waitForReadiness(run, false, fixture.budgets.readinessMs);
  return { run, readiness };
};

/** 🔌️ One human document socket: open-plan → socket-grant → `semio.session.v1` hello; frames are collected decoded. */
async function documentSocket(token: string, spaceId: string, documentId: string, surfaceId: string) {
  const base = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const plan = await hub("POST", `${base}/open-plan`, token, { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: `hub-edit-durability-${randomBytes(4).toString("hex")}` });
  const grant = await hub("POST", `${base}/socket-grants`, token, { schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json?.receipt });
  const frames: any[] = [];
  const socket = new WebSocket(`${ORIGIN.replace(/^http/u, "ws")}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json?.surface?.surfaceId ?? surfaceId)}`, ["semio.session.v1", token]);
  socket.binaryType = "arraybuffer";
  socket.addEventListener("message", (event) => {
    if (event.data instanceof ArrayBuffer) frames.push(decodeServerFrame(new Uint8Array(event.data)).frame);
  });
  for (let tick = 0; tick < 200 && socket.readyState === WebSocket.CONNECTING; tick += 1) await pause(50);
  const hash = [...(String(plan.json?.artifact?.packSchemaHash ?? "").match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
  socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json?.artifact?.schema, pack_schema_hash: hash, resume_token: null, frontier: null } }, "command"));
  const waitFor = async (predicate: (frame: any, index: number) => boolean, ms: number) => {
    const deadline = Date.now() + ms;
    while (Date.now() < deadline) {
      const hit = frames.find(predicate);
      if (hit) return hit;
      await pause(50);
    }
    return undefined;
  };
  const session = await waitFor((frame) => "Session" in frame, fixture.budgets.catchUpMs);
  const sessionAt = frames.findIndex((frame) => "Session" in frame);
  const catchUp = new Set<string>(frames.slice(0, Math.max(sessionAt, 0)).filter((frame) => "Commands" in frame).flatMap((frame) => frame.Commands.envelopes.map((envelope: any) => String(envelope.mutation_id))));
  return { socket, frames, waitFor, session, catchUp, planStatus: plan.status, grantStatus: grant.status };
}

/** 🤖️ One agent lane: a fresh delegation and a fresh MCP process open the space's note (resolving
 * it from the workspace listing when the gate has not yet), let `beforeInvoke` observe, then commit
 * the fixture gesture once. */
async function agentLane(token: string, spaceId: string, known: string, label: string, beforeInvoke: (documentId: string, surfaceId: string) => Promise<void>) {
  const delegation = await hub("POST", "/auth/agent-delegations", token, { schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: label, audience: "edit", ttlSecs: 900 });
  const credentialPath = join(mkdtempSync(join(tmpdir(), "semio-hub-edit-durability-")), "agent-credential.json");
  writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: ORIGIN, spaceId, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
  chmodSync(credentialPath, 0o600);
  const declared = mcpServerEntries(repoRoot).semio;
  const scopes = declared?.args.includes("--scopes") ? String(declared.args[declared.args.indexOf("--scopes") + 1]) : "workspace.read,artifact.write";
  const session = new McpClientSession({ command: requireMcpBinary(repoRoot), args: ["stdio", "--scopes", scopes] }, ["--hub", ORIGIN, "--space", spaceId, "--credential-file", credentialPath, "--no-bridge"], repoRoot);
  try {
    await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "semio-hub-edit-durability", version: "1" } });
    session.notify("notifications/initialized", {});
    const listed = await session.request("resources/read", { uri: "semio://workspace/artifacts" });
    const artifacts = (JSON.parse(String(listed.result?.contents?.[0]?.text ?? "{}")).artifacts ?? []) as any[];
    const documentId = known || String((artifacts.find((entry) => String(entry?.schema ?? entry?.artifactSchema ?? entry?.descriptor?.artifactSchema ?? "").startsWith(fixture.document.schemaPrefix)) ?? artifacts[0])?.scope?.documentId ?? "");
    const opened = await session.call("artifact_open", { artifactId: documentId });
    const surfaceId = String(opened.structuredContent?.sessionDocument?.surfaceId ?? "");
    await beforeInvoke(documentId, surfaceId);
    const search = await session.call("capabilities_search", { query: "add a block", kind: ["mutation"] });
    const capabilityId = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId ?? hit.id)).find((id) => id.endsWith(fixture.gesture.capabilitySuffix)) ?? "");
    const prepared = await session.call("action_prepare", { capabilityId, input: fixture.gesture.input });
    const invoked = await session.call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
    return { ok: opened.isError !== true && invoked.structuredContent?.status === "SUCCEEDED", detail: `${capabilityId} open=${opened.isError !== true} status=${invoked.structuredContent?.status ?? JSON.stringify(invoked.structuredContent ?? prepared.structuredContent).slice(0, 200)}`, documentId, surfaceId };
  } finally {
    session.stop();
    await hub("DELETE", `/auth/agent-delegations/${encodeURIComponent(String(delegation.json?.delegationId ?? ""))}`, token).catch(() => undefined);
  }
}

let current = await bootHub();
row("0 the gate's own hub is ready", current.readiness?.status === "ready" && current.readiness?.features?.mcpWorkspace === true, `${ORIGIN} status=${current.readiness?.status} data=${dataDir}`);
const sockets: WebSocket[] = [];
try {
  const signIn = await hub("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `hubeditdurability${randomBytes(8).toString("hex")}`, clientClass: "browser" });
  const token = String(signIn.json?.token ?? "");
  row("1 the human signs in", signIn.status === 200 && token.length > 0, `HTTP ${signIn.status}`);
  const spaceId = String((await hub("GET", "/directory/spaces", token)).json?.filter((entry: any) => entry?.access === "author").map((entry: any) => entry?.space?.id).filter(Boolean).at(-1) ?? "");
  const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const catalog = await hub("GET", creations, token);
  const kind = (catalog.json?.kinds ?? []).find((row: any) => String(row?.schema ?? "").startsWith(fixture.document.schemaPrefix));
  const requestId = randomBytes(16).toString("hex");
  let creation = (await hub("POST", creations, token, sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Hub edit durability" }))).json;
  for (const deadline = Date.now() + fixture.budgets.creationMs; ["accepted", "preparing"].includes(creation?.phase) && Date.now() < deadline; ) {
    await pause(1_000);
    creation = (await hub("GET", `${creations}/${requestId}`, token)).json;
  }
  const created = String(creation?.ready?.artifactId ?? "");
  row("2a the hub creates a fresh note in the author's space", created.length > 0, `space=${spaceId} kind=${kind?.kindId} generation=${catalog.json?.catalogGenerationId} phase=${creation?.phase} document=${created}`);
  let documentId = "";
  let surfaceId = "";
  let observer: Awaited<ReturnType<typeof documentSocket>> | undefined;
  const status = async () => Number((await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, token)).json?.head_seq ?? -1);
  const observe = async (resolved: string, surface: string) => {
    if (observer) return;
    documentId = resolved;
    surfaceId = surface;
    row("2 the agent resolves the space's note document", spaceId.length > 0 && documentId.length > 0 && surfaceId.length > 0, `space=${spaceId} document=${documentId} surface=${surfaceId}`);
    observer = await documentSocket(token, spaceId, documentId, surfaceId);
    sockets.push(observer.socket);
    row("3 the human observer holds a live document socket", Boolean(observer.session), `plan=${observer.planStatus} grant=${observer.grantStatus}`);
  };
  const committed: string[] = [];
  let relayTemplate: any;
  for (const lane of fixture.lanes as Array<{ id: string; kind: string; label?: string }>) {
    let before = -1;
    let framesBefore = 0;
    const mark = async (resolved: string, surface: string) => {
      await observe(resolved, surface);
      before = await status();
      framesBefore = observer!.frames.length;
    };
    if (lane.kind === "agent") {
      const result = await agentLane(token, spaceId, documentId || created, lane.label ?? lane.id, mark);
      row(`4 ${lane.id} commits the gesture through a fresh MCP process`, result.ok, result.detail);
    } else {
      await mark(documentId, surfaceId);
      const writer = await documentSocket(token, spaceId, documentId, surfaceId);
      sockets.push(writer.socket);
      const actor = String(writer.session?.Session?.actor ?? "");
      const template = relayTemplate?.Commands?.envelopes?.[0];
      const mutationId = `edit-${randomBytes(8).toString("hex")}`;
      if (template) writer.socket.send(encodeClientFrame({ Commands: { batch_id: 1, envelopes: [{ ...template, mutation_id: mutationId, actor, dependencies: [], timestamp: { actor: Number.parseInt(randomBytes(4).toString("hex"), 16), physical_ms: Date.now(), logical: 0 } }] } }, "command"));
      const ack = template ? await writer.waitFor((frame) => "Ack" in frame, fixture.budgets.relayMs) : undefined;
      const accepted = JSON.stringify(ack?.Ack?.stages ?? []).includes("Accepted");
      row(`4 ${lane.id} commits a note operation over its own document socket`, Boolean(template) && accepted, template ? `mutation=${mutationId} ack=${JSON.stringify(ack?.Ack?.stages ?? null).slice(0, 200)}` : "no agent operation was relayed to take a note operation shape from");
    }
    const relayed = await observer!.waitFor((frame, index) => index >= framesBefore && "Commands" in frame, fixture.budgets.relayMs);
    relayTemplate ??= relayed;
    const ids = (relayed?.Commands?.envelopes ?? []).map((envelope: any) => String(envelope.mutation_id));
    committed.push(...ids);
    row(`5 ${lane.id}'s edit is relayed to the observer`, ids.length > 0, `ids=${JSON.stringify(ids)}`);
    let after = await status();
    for (const deadline = Date.now() + fixture.budgets.headMs; after < before + fixture.expectations.headAdvancePerLane && Date.now() < deadline; after = await status()) await pause(500);
    row(`6 ${lane.id}'s edit advances the ledger head`, after === before + fixture.expectations.headAdvancePerLane, `head_seq ${before}→${after}`);
  }
  row("7 every lane minted a distinct mutation id", new Set(committed).size === fixture.expectations.distinctMutationIds, JSON.stringify(committed));
  const lateJoiner = await documentSocket(token, spaceId, documentId, surfaceId);
  sockets.push(lateJoiner.socket);
  row("8 a late joiner's catch-up carries every lane", committed.length === fixture.expectations.distinctMutationIds && committed.every((id) => lateJoiner.catchUp.has(id)), `catchUp=${lateJoiner.catchUp.size} missing=${JSON.stringify(committed.filter((id) => !lateJoiner.catchUp.has(id)))}`);

  const headBeforeRestart = await status();
  for (const socket of sockets.splice(0)) socket.close();
  await finishLocalHub(current.run);
  current = await bootHub();
  const reToken = String((await hub("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `hubeditdurability${randomBytes(8).toString("hex")}`, clientClass: "browser" })).json?.token ?? "");
  const headAfterRestart = Number((await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, reToken)).json?.head_seq ?? -1);
  row("9 a hub restart preserves the ledger head", current.readiness?.status === "ready" && headAfterRestart === headBeforeRestart && headAfterRestart >= fixture.lanes.length, `head_seq ${headBeforeRestart}→${headAfterRestart}`);
  const rejoin = await documentSocket(reToken, spaceId, documentId, surfaceId);
  sockets.push(rejoin.socket);
  row("10 after the restart a fresh joiner's catch-up carries every lane", committed.length === fixture.expectations.distinctMutationIds && committed.every((id) => rejoin.catchUp.has(id)), `catchUp=${rejoin.catchUp.size} missing=${JSON.stringify(committed.filter((id) => !rejoin.catchUp.has(id)))}`);
} catch (error) {
  row("gate", false, error instanceof Error ? error.stack ?? error.message : String(error));
} finally {
  for (const socket of sockets) socket.close();
  await finishLocalHub(current.run).catch(() => undefined);
}
const red = rows.filter((entry) => !entry.ok);
console.log(`hub-edit-durability-check: ${rows.length - red.length}/${rows.length} rows green (${ORIGIN}, data ${dataDir})`);
process.exit(red.length === 0 ? 0 : 1);
