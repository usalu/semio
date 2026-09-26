/** 🛡️ The semio MCP's security boundary against a live hub, measured with the official `@modelcontextprotocol/sdk`
 * client as the third-party oracle.
 *
 * A human signs in, owns a fresh space holding one note (hub directory command + server-owned creation) and delegates
 * agents; every agent is a real `semio-os-mcp stdio --hub` process bound to that space by a `0600` delegated credential.
 *   S1 scopes: the same delegation under a read-only grant and under the full grant — every tool the read-only grant does
 *      not cover is refused `PERMISSION_DENIED`, and the full grant is never refused for a scope.
 *   S2 audience: a `read` delegation whose gateway claims `artifact.write` still cannot edit (the hub caps the audience).
 *   S3 rate limit: a burst of agent-session exchanges for one delegation is refused `429 rate-limited` with `Retry-After`.
 *   S4 revocation: a connected agent edits, the human revokes, the agent's very next request is refused, and the hub no
 *      longer lists a sync session of the agent's on the note (`/admin/api/connections`, the hub's own connection census).
 * Every row is required.
 *
 * Configuration by environment: `OS_MCP_HUB_ORIGIN` (default `http://127.0.0.1:8787`), `OS_MCP_HUB_EMAIL` /
 * `OS_MCP_HUB_PASSWORD` (default the second local user), `OS_HUB_ADMIN_CAPABILITY_FILE` (the hub launcher's `0600`
 * `admin-capability.json`, required for S4's connection census), `S_OS_MCP_SECURITY_OUT` (captures, default
 * `🌉️mcp/🤖️generated/🛡️security`). The gateway binary is `requireMcpBinary` (`SEMIO_OS_MCP_BIN` overrides).
 * Promoted from the ticket harness `wp-g11/g11-security-probe.ts` (ticket 26/09/23, G11 item 4).
 */
import { chmodSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { sealSpaceArtifactCreateV1 } from "../../../📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "../../../📇️directory/🏘️spaces/🟦️.ts";
import { agentDelegationRevokePathV1 } from "../../../📇️directory/🤖️delegations/🟦️.ts";
import { requireMcpBinary } from "../../🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const here = dirname(fileURLToPath(new URL(import.meta.url)));
function findRepoRoot(start: string): string {
  for (let current = start, depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`the security gate could not locate the repository root above ${start}`);
}
const repoRoot = findRepoRoot(here);
const HUB = (process.env.OS_MCP_HUB_ORIGIN ?? "http://127.0.0.1:8787").replace(/\/$/u, "");
const EMAIL = process.env.OS_MCP_HUB_EMAIL ?? "user2@semio.dev";
const PASSWORD = process.env.OS_MCP_HUB_PASSWORD ?? "gm1-local-dev-pass-2";
const ADMIN_FILE = process.env.OS_HUB_ADMIN_CAPABILITY_FILE ?? "";
const OUT = process.env.S_OS_MCP_SECURITY_OUT ?? join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️generated/🛡️security");
const BINARY = requireMcpBinary(repoRoot);
const CREDENTIALS = join(OUT, "credentials");
mkdirSync(CREDENTIALS, { recursive: true, mode: 0o700 });
const startedAt = new Date();
const seconds = (): string => ((Date.now() - startedAt.getTime()) / 1000).toFixed(1);
const rows: { step: string; ok: boolean; detail: string; at: string }[] = [];
const row = (step: string, ok: boolean, detail: string): void => {
  rows.push({ step, ok, detail, at: seconds() });
  console.log(`${ok ? "PASS" : "FAIL"}  [${seconds()} s] ${step} — ${detail}`);
};
const pause = (ms: number): Promise<void> => new Promise((resolveDelay) => setTimeout(resolveDelay, ms));

async function hub(method: string, path: string, token?: string, body?: string): Promise<{ status: number; json: any; retryAfter: string | null }> {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any = null;
  try {
    json = JSON.parse(text);
  } catch {
    json = null;
  }
  return { status: response.status, json, retryAfter: response.headers.get("retry-after") };
}

/** 🎫️ One delegation turned into the `0600` `semio.hub.agent-credential/v1` file the shell's "Set up MCP client" writes. */
async function delegate(human: string, spaceId: string, label: string, audience: "read" | "edit"): Promise<{ id: string; token: string; path: string }> {
  const created = await hub("POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: label, audience, ttlSecs: 900 }));
  const path = join(CREDENTIALS, `${label.replace(/[^a-z0-9]+/giu, "-").toLowerCase()}-${randomBytes(3).toString("hex")}.json`);
  writeFileSync(path, JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience, token: created.json?.token }), { mode: 0o600 });
  chmodSync(path, 0o600);
  return { id: String(created.json?.delegationId ?? ""), token: String(created.json?.token ?? ""), path };
}

/** 🔌️ The official SDK client on a real gateway process bound to the hub space under `scopes`. */
async function connect(name: string, spaceId: string, credential: string, scopes: string): Promise<Client> {
  const client = new Client({ name, version: "1" }, { capabilities: {} });
  await client.connect(new StdioClientTransport({ command: BINARY, args: ["stdio", "--hub", HUB, "--space", spaceId, "--credential-file", credential, "--scopes", scopes, "--no-bridge", "--auto-approve", "all"], env: { ...process.env } as Record<string, string>, stderr: "pipe" }), { timeout: 600_000 });
  return client;
}

async function call(client: Client, name: string, args: Record<string, unknown>): Promise<{ isError: boolean; code: string; content: any; message: string }> {
  const answer: any = await client.callTool({ name, arguments: args }, undefined, { timeout: 600_000 }).catch((error: unknown) => ({ isError: true, structuredContent: { code: "CLIENT", message: String(error) } }));
  return { isError: answer.isError === true, code: String(answer.structuredContent?.code ?? ""), content: answer.structuredContent ?? {}, message: String(answer.structuredContent?.message ?? "").slice(0, 200) };
}

/** 🔭️ How many active sync sessions the hub's own connection census lists on one note of this run's fresh space (only
 * this gate's agents ever open it; the human never does). Pages through the whole census. */
async function noteSyncSessions(spaceId: string, noteId: string): Promise<number> {
  const admin = JSON.parse(readFileSync(ADMIN_FILE, "utf8")) as { capability: string };
  let count = 0;
  let cursor = "";
  for (let page = 0; page < 64; page += 1) {
    const listed = await hub("GET", `/admin/api/connections${cursor ? `?cursor=${encodeURIComponent(cursor)}` : ""}`, admin.capability);
    if (listed.status !== 200) throw new Error(`the hub's connection census answered ${listed.status}`);
    count += ((listed.json?.rows ?? []) as any[]).filter((entry) => entry?.scope?.spaceId === spaceId && entry?.scope?.documentId === noteId).length;
    cursor = String(listed.json?.nextCursor ?? "");
    if (!cursor) break;
  }
  return count;
}

const clients: Client[] = [];
try {
  if (!existsSync(ADMIN_FILE)) throw new Error("OS_HUB_ADMIN_CAPABILITY_FILE must name the hub launcher's admin-capability.json");
  const signIn = await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `security${randomBytes(12).toString("hex")}`, clientClass: "browser" }));
  const human = String(signIn.json?.token ?? "");
  const spaceName = `Security ${randomBytes(3).toString("hex")}`;
  await hub("POST", "/directory/commands", human, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))));
  const spaceId = String((await hub("GET", "/directory/spaces", human)).json?.find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
  const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const catalog = await hub("GET", creations, human);
  const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
  const requestId = randomBytes(16).toString("hex");
  let creation = (await hub("POST", creations, human, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Security" })))).json;
  for (const deadline = Date.now() + 1_800_000; ["accepted", "preparing"].includes(creation?.phase) && Date.now() < deadline; ) {
    await pause(1_000);
    creation = (await hub("GET", `${creations}/${requestId}`, human)).json;
  }
  const noteId = String(creation?.ready?.artifactId ?? "");
  row("0 setup: the human owns a fresh space with one note (hub authorities)", signIn.status < 300 && spaceId.length > 0 && noteId.length > 0, `sign-in=${signIn.status} space=${spaceId} note=${noteId}`);

  const editor = await delegate(human, spaceId, "Security editor", "edit");
  const reader = await connect("security-read-only", spaceId, editor.path, "workspace.read");
  const full = await connect("security-full", spaceId, editor.path, "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write");
  clients.push(reader, full);
  const search: any = await full.callTool({ name: "capabilities_search", arguments: { query: "add a block", kind: ["mutation"] } }, undefined, { timeout: 120_000 });
  const addBlock = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId)).find((id) => id.endsWith(".addBlock")) ?? "");
  const probes: readonly { tool: string; args: Record<string, unknown>; grantedBy: string }[] = [
    { tool: "context_resolve", args: {}, grantedBy: "workspace.read" },
    { tool: "capabilities_search", args: { query: "add a block" }, grantedBy: "workspace.read" },
    { tool: "artifact_open", args: { artifactId: noteId }, grantedBy: "workspace.read" },
    { tool: "artifact_snapshot", args: { artifactId: noteId }, grantedBy: "workspace.read" },
    { tool: "inference_list", args: {}, grantedBy: "workspace.read" },
    { tool: "action_prepare", args: { capabilityId: addBlock, input: { kind: "text" } }, grantedBy: "artifact.write" },
    { tool: "action_invoke", args: { capabilityId: addBlock, input: { kind: "text" } }, grantedBy: "artifact.write" },
    { tool: "ui_reveal", args: { artifactId: noteId }, grantedBy: "ui.control" },
    { tool: "ui_focus", args: { artifactId: noteId }, grantedBy: "ui.control" },
    { tool: "conversation_reply", args: { text: "security scope probe" }, grantedBy: "conversation.write" },
  ];
  const table = ["| tool | granted by | read-only grant | full grant |", "|---|---|---|---|"];
  let scopesHold = addBlock.length > 0;
  for (const probe of probes) {
    const narrow = await call(reader, probe.tool, probe.args);
    const wide = await call(full, probe.tool, probe.args);
    const covered = probe.grantedBy === "workspace.read";
    scopesHold &&= (covered ? narrow.code !== "PERMISSION_DENIED" : narrow.isError && narrow.code === "PERMISSION_DENIED") && wide.code !== "PERMISSION_DENIED";
    table.push(`| \`${probe.tool}\` | \`${probe.grantedBy}\` | ${narrow.isError ? `refused ${narrow.code}` : "ok"} | ${wide.isError ? `refused ${wide.code}: ${wide.message.slice(0, 80)}` : "ok"} |`);
  }
  writeFileSync(join(OUT, "scopes.md"), `${table.join("\n")}\n`);
  row("S1 every tool outside a read-only grant is refused PERMISSION_DENIED and the full grant is never refused for a scope", scopesHold, `${probes.length} tools × 2 grants (scopes.md)`);
  await reader.close().catch(() => undefined);

  const readOnly = await delegate(human, spaceId, "Security reader", "read");
  const readClient = await connect("security-read-audience", spaceId, readOnly.path, "workspace.read,artifact.write").catch((error: unknown) => error);
  if (readClient instanceof Client) {
    const edit = await call(readClient, "action_invoke", { capabilityId: addBlock, input: { kind: "text" } });
    await readClient.close().catch(() => undefined);
    row("S2 a read delegation cannot edit even when its gateway claims artifact.write", edit.isError, edit.isError ? `refused ${edit.code}: ${edit.message}` : `edited: ${JSON.stringify(edit.content).slice(0, 160)}`);
  } else row("S2 a read delegation cannot edit even when its gateway claims artifact.write", true, `the gateway refused to bind a read delegation with a write grant: ${String(readClient).slice(0, 200)}`);

  const burst = await delegate(human, spaceId, "Security burst", "edit");
  const answers: { status: number; retryAfter: string | null }[] = [];
  for (let attempt = 0; attempt < 14; attempt += 1) {
    const answer = await hub("POST", "/auth/agent-sessions", burst.token, JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: "edit", agentInstanceId: `securityburst${attempt}` }));
    answers.push({ status: answer.status, retryAfter: answer.retryAfter });
  }
  const limited = answers.filter((answer) => answer.status === 429);
  row("S3 a burst of agent-session exchanges for one delegation is rate-limited with Retry-After", limited.length > 0 && limited.every((answer) => Number(answer.retryAfter ?? 0) > 0), `statuses ${answers.map((answer) => answer.status).join(",")}`);

  const before = await call(full, "action_invoke", { capabilityId: addBlock, input: { kind: "text" } });
  const listedBefore = await noteSyncSessions(spaceId, noteId);
  const revokeStarted = Date.now();
  const revoke = await hub("POST", agentDelegationRevokePathV1(editor.id), human);
  const revokeMs = Date.now() - revokeStarted;
  const after = await call(full, "action_invoke", { capabilityId: addBlock, input: { kind: "text" } });
  const refusedAtMs = Date.now() - revokeStarted;
  row("S4a a connected agent edits, the human revokes, and the agent's very next request is refused", before.content?.status === "SUCCEEDED" && revoke.status < 300 && after.isError, `before=${before.content?.status ?? before.code} revoke=${revoke.status} in ${revokeMs} ms; next=${after.isError ? `refused ${after.code} ${after.message}` : `edited ${after.content?.status}`} at +${refusedAtMs} ms`);
  let listedAfter = listedBefore;
  for (let tick = 0; tick < 20 && listedAfter > 0; tick += 1) {
    await pause(500);
    listedAfter = await noteSyncSessions(spaceId, noteId);
  }
  row("S4b the hub closes the revoked agent's document sockets (hub connection census)", listedBefore > 0 && listedAfter === 0, `agent sync sessions on the note: ${listedBefore} before the revoke, ${listedAfter} after`);
} catch (error) {
  row("run", false, error instanceof Error ? (error.stack ?? error.message) : String(error));
} finally {
  for (const client of clients) await client.close().catch(() => undefined);
  writeFileSync(join(OUT, "rows.json"), JSON.stringify(rows, null, 1));
}
const green = rows.filter((entry) => entry.ok).length;
const red = rows.filter((entry) => !entry.ok);
console.log(`security: ${green}/${rows.length} rows green in ${seconds()} s`);
publishAcceptanceCheckResult(
  repoRoot,
  acceptanceCheckResult({
    check: "mcp-security",
    status: red.length === 0 && rows.length >= 6 ? "pass" : "fail",
    startedAt,
    measured: { rows: rows.length, green, hub: HUB },
    summary: {
      en: `${green}/${rows.length} security rows green${red.length ? `; red: ${red.map((entry) => entry.step.split(" ")[0]).join(",")}` : ""}`,
      de: `${green}/${rows.length} Sicherheitszeilen grün${red.length ? `; rot: ${red.map((entry) => entry.step.split(" ")[0]).join(",")}` : ""}`,
    },
    evidence: [join(OUT, "rows.json"), join(OUT, "scopes.md")],
  }),
);
process.exit(red.length === 0 ? 0 : 1);
