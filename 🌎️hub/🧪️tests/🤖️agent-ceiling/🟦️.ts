import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/📝️text/🟦️.ts";
/** 🤖️ The hub's agent-audience ceiling, live: an AI agent acting under a delegation holds at most its delegation's audience,
 * only in the delegation's one space, never a space's administration and never the admin console — whatever its delegating
 * human may do (`HubDirectory::principal_ceiling`, policy roles `agent-reader` / `agent-editor`).
 *
 * One human (an admin subject of the hub under test) creates two private spaces and one document, delegates a `read` and an
 * `edit` agent to the first space, exchanges each delegation for an agent session exactly as the semio MCP gateway does
 * (`POST /auth/agent-sessions`), and drives the hub directly — no gateway in between, so the hub's own boundary is what is
 * measured: a `read` agent's document edit is refused and the document head does not move, an `edit` agent's edit is
 * accepted (control), neither agent sees the second space, neither may rename the space, add a member or mint an invite,
 * neither may read the admin console, and the human still may (controls; the admin one only when the human is an admin
 * subject of the hub).
 *
 * Acceptance ledger 2.13 / outcome 2 security (P0 of 2026-09-27: a `read` delegation's agent edited a hub note).
 */
import { randomBytes } from "node:crypto";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenPlan, hubProbeSignIn } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️testkit/📡️client-probe/🟦️.ts";
import { decodeServerFrame, encodeClientFrame } from "../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { parseDocumentSocketGrantReceiptV1 } from "../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { type DirectoryCommand } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

/** 🧾️ One observed boundary: what the hub answered and whether that is what the ceiling demands. */
export type AgentCeilingRowV1 = { readonly check: string; readonly expected: string; readonly observed: string; readonly pass: boolean };

/** 📊️ The whole check: every row, the document and spaces it ran in, and whether it was cancelled. */
export type AgentCeilingReportV1 = { readonly rows: AgentCeilingRowV1[]; readonly spaceId: string; readonly foreignSpaceId: string; readonly documentId: string; readonly kindId: string; readonly cancelled: boolean };

/** ⚙️ Where and as whom the check runs. */
export type AgentCeilingOptionsV1 = { readonly hub: string; readonly email: string; readonly password: string; readonly memberEmail: string; readonly kind: string; readonly signal: AbortSignal; readonly onProgress: (line: string) => void };

const AGENT_DELEGATION_TTL_SECS = 900;
const FRAME_BUDGET_MS = 30_000;

async function delegate(hub: string, human: string, spaceId: string, audience: "read" | "edit"): Promise<string> {
  const created = await hubProbeCall(hub, "POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: `ceiling ${audience}`, audience, ttlSecs: AGENT_DELEGATION_TTL_SECS }));
  if (created.status !== 201 || typeof created.json?.token !== "string") throw new Error(`agent delegation (${audience}) ${created.status}`);
  const session = await hubProbeCall(hub, "POST", "/auth/agent-sessions", created.json.token, JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience, agentInstanceId: `ceiling.${randomBytes(6).toString("hex")}` }));
  if (session.status !== 200 || typeof session.json?.token !== "string") throw new Error(`agent session (${audience}) ${session.status}`);
  return session.json.token;
}

async function documentHead(hub: string, human: string, spaceId: string, documentId: string): Promise<number> {
  const answer = await hubProbeCall(hub, "GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, human);
  return Number(answer.json?.head_seq ?? -1);
}

/** 📡️ Opens the document as `token` and submits one opaque edit on the document socket; answers how far it got. */
async function submitEdit(hub: string, token: string, spaceId: string, documentId: string, signal: AbortSignal): Promise<string> {
  const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const plan = await hubProbeOpenPlan(hub, token, spaceId, documentId, "agent-ceiling");
  if (plan.status !== 200) return `open-plan ${plan.status}`;
  const grant = await hubProbeCall(hub, "POST", `${scope}/socket-grants`, token, JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json.receipt }));
  if (grant.status !== 200) return `socket-grant ${grant.status}`;
  const granted = parseDocumentSocketGrantReceiptV1(grant.json);
  const socket = new WebSocket(`${hub.replace(/^http/u, "ws")}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json.surface.surfaceId)}`, ["semio.session.v1", token]);
  socket.binaryType = "arraybuffer";
  return new Promise<string>((resolve) => {
    let settled = false;
    const finish = (verdict: string): void => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      signal.removeEventListener("abort", aborted);
      resolve(verdict);
      socket.close(1000, "agent-ceiling");
    };
    const aborted = (): void => finish("cancelled");
    const timer = setTimeout(() => finish("no answer"), FRAME_BUDGET_MS);
    signal.addEventListener("abort", aborted);
    socket.addEventListener("close", (event) => finish(`socket closed ${event.code}`));
    socket.addEventListener("open", () => {
      const packSchemaHash = [...(String(plan.json.artifact.packSchemaHash).match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
      socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
    });
    socket.addEventListener("message", (event) => {
      if (!(event.data instanceof ArrayBuffer)) return;
      const frame: any = decodeServerFrame(new Uint8Array(event.data)).frame;
      if ("Error" in frame) return finish(`error ${JSON.stringify(frame.Error).slice(0, 160)}`);
      if ("Welcome" in frame) {
        socket.send(encodeClientFrame({ Commands: { batch_id: 1, envelopes: [{ mutation_id: `agent-ceiling-${randomBytes(8).toString("hex")}`, document_id: documentId, actor: granted.actorId, dependencies: [], observed: null, target: [], diff: { schema: plan.json.artifact.schema, payload: Array.from(new TextEncoder().encode("agent-ceiling")) }, inverse: { schema: plan.json.artifact.schema, payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 }, transaction: null, verb: null, line: null }] } }, "command"));
        return;
      }
      if ("Ack" in frame && frame.Ack.batch_id === 1) finish(JSON.stringify(frame.Ack.stages).includes("Accepted") ? "accepted" : `ack ${JSON.stringify(frame.Ack.stages).slice(0, 160)}`);
    });
  });
}

async function command(hub: string, token: string, body: DirectoryCommand): Promise<number> {
  return (await hubProbeCall(hub, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), body)))).status;
}

/** 🤖️ Runs every boundary once and answers the rows; a failed precondition (sign-in, creation, delegation) throws. */
export async function runAgentCeilingCheck(options: AgentCeilingOptionsV1): Promise<AgentCeilingReportV1> {
  const { hub, signal, onProgress } = options;
  const rows: AgentCeilingRowV1[] = [];
  const row = (check: string, expected: string, observed: string, pass: boolean): void => {
    rows.push({ check, expected, observed, pass });
    onProgress(`${pass ? "PASS" : "FAIL"} ${check}: expected ${expected}, observed ${observed}`);
  };
  const human = await hubProbeSignIn(hub, options.email, options.password, "agent-ceiling");
  const stamp = new Date().toISOString();
  const spaceId = await hubProbeCreateSpace(hub, human, `Agent ceiling ${stamp}`);
  const foreignSpaceId = await hubProbeCreateSpace(hub, human, `Agent ceiling foreign ${stamp}`);
  const catalog = await hubProbeCreationCatalog(hub, human, spaceId);
  const kind = catalog.kinds.find((entry) => entry.kindId === options.kind || entry.schema.startsWith(options.kind));
  if (!kind) throw new Error(`kind ${options.kind} does not exist in the seeded catalog`);
  const created = await hubProbeCreateArtifact(hub, human, spaceId, catalog.generationId, kind.kindId, "Agent ceiling");
  onProgress(`created ${kind.kindId} ${created.artifactId} in ${created.ms} ms`);
  const reader = await delegate(hub, human, spaceId, "read");
  const editor = await delegate(hub, human, spaceId, "edit");
  const report = (): AgentCeilingReportV1 => ({ rows, spaceId, foreignSpaceId, documentId: created.artifactId, kindId: kind.kindId, cancelled: signal.aborted });
  for (const [agent, token] of [["read", reader], ["edit", editor]] as const) {
    const listed = await hubProbeCall(hub, "GET", "/directory/spaces", token);
    const ids = ((listed.json ?? []) as any[]).map((entry) => String(entry?.space?.id ?? ""));
    row(`${agent}-agent-lists-only-its-space`, "its delegation's space, not the other", `${listed.status} [${ids.includes(spaceId) ? "own" : "-"}${ids.includes(foreignSpaceId) ? ",foreign" : ""}]`, listed.status === 200 && ids.includes(spaceId) && !ids.includes(foreignSpaceId));
    const foreign = await hubProbeCall(hub, "GET", `/directory/spaces/${encodeURIComponent(foreignSpaceId)}`, token);
    row(`${agent}-agent-cannot-read-a-foreign-space`, "not 200", String(foreign.status), foreign.status !== 200);
    if (signal.aborted) return report();
  }
  const before = await documentHead(hub, human, spaceId, created.artifactId);
  const readEdit = await submitEdit(hub, reader, spaceId, created.artifactId, signal);
  const afterRead = await documentHead(hub, human, spaceId, created.artifactId);
  row("read-agent-edit-is-refused", "refused, head unchanged", `${readEdit}, head ${before}→${afterRead}`, readEdit !== "accepted" && afterRead === before);
  const editEdit = await submitEdit(hub, editor, spaceId, created.artifactId, signal);
  const afterEdit = await documentHead(hub, human, spaceId, created.artifactId);
  row("edit-agent-edit-is-accepted", "accepted, head +1", `${editEdit}, head ${afterRead}→${afterEdit}`, editEdit === "accepted" && afterEdit === afterRead + 1);
  if (signal.aborted) return report();
  for (const [agent, token] of [["read", reader], ["edit", editor]] as const) {
    const rename = await command(hub, token, { kind: "rename-space", spaceId, name: `renamed by ${agent} agent` });
    row(`${agent}-agent-cannot-rename-the-space`, "403", String(rename), rename === 403);
    const member = await command(hub, token, { kind: "upsert-member", spaceId, email: options.memberEmail, role: "author" });
    row(`${agent}-agent-cannot-add-a-member`, "403", String(member), member === 403);
    const invite = await command(hub, token, { kind: "create-invite", spaceId, role: "author", ttlSecs: 600 });
    row(`${agent}-agent-cannot-mint-an-invite`, "403", String(invite), invite === 403);
    const admin = await hubProbeCall(hub, "GET", "/admin/api/documents", token);
    row(`${agent}-agent-is-refused-by-the-admin-console`, "401 or 403", String(admin.status), admin.status === 401 || admin.status === 403);
    if (signal.aborted) return report();
  }
  const humanRename = await command(hub, human, { kind: "rename-space", spaceId, name: "Agent ceiling (renamed by its author)" });
  row("the-delegating-author-still-renames-the-space", "202", String(humanRename), humanRename === 202);
  const humanAdmin = await hubProbeCall(hub, "GET", "/admin/api/documents", human);
  if (humanAdmin.status === 200) row("the-delegating-admin-still-reads-the-admin-console", "200", "200", true);
  else onProgress(`the probe human is no admin subject of this hub (${humanAdmin.status}): the agents' admin refusals are not contrasted`);
  return report();
}
