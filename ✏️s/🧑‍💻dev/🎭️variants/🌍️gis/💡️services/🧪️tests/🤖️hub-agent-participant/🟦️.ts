import { serviceMcpTestArtifactRoot } from "../📁️artifact-root/🟦️.ts";
/** 🤖️ The hub-agent-participant gate: is an MCP agent a REAL third participant in a hub space?
 *
 * `live-agent-loop-check` owns the shell route (an agent driving a human's open `dev s` session).
 * This gate owns the other one — the route a Claude Code / Cursor user actually configures for a
 * remote space: `semio-os-mcp stdio --hub <origin> --space <id> --credential-file <0600 file>`,
 * authenticated by a delegation its human minted, with no launcher, no fd 3 and no local folder.
 *
 * It boots no hub. An `os-hub` with a published trusted catalog costs minutes to bring up and every
 * collaboration run already has one (`dev s` keeps one at `http://127.0.0.1:8787`); when none
 * answers, the gate says which origin it looked for instead of pretending. It needs nothing inside
 * that hub but the human's credential: unless told otherwise it creates its own space through the
 * directory's `create-space` command and its own note through the hub's server-owned creation
 * transaction. Configuration is entirely by environment:
 *
 *   OS_MCP_HUB_ORIGIN    default `http://127.0.0.1:8787` (the `dev s` local hub)
 *   OS_MCP_HUB_EMAIL     the human's sign-in (required; `blocked` without it)
 *   OS_MCP_HUB_PASSWORD  the human's password (required; `blocked` without it)
 *   OS_MCP_HUB_SPACE     an existing space holding a document, instead of a fresh one
 *   S_OS_MCP_PARTICIPANT_OUT  where each gateway session's stderr lands (default `🌉️mcp/🤖️generated/🤖️hub-agent-participant`)
 *
 * It writes its acceptance record (`mcp-hub-agent-participant`, en + de) through `publishAcceptanceCheckResult`.
 *
 * The rows are the chain, in order. Every one is required: a red row exits non-zero and prints the
 * refusal verbatim, because the whole value of this gate is that it cannot round a missing
 * participant up to a passing one. Ticket 26/09/18 slice M8.
 */
import { chmodSync, existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { AcceptancePreconditionMissing, hubCredentialFromEnv, McpClientSession, mcpServerEntries, minimalInputForSchema, requireMcpBinary } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";

// 📁️ `new URL(x, import.meta.url).pathname` percent-encodes emoji path segments, and a counted
// `..` chain silently walks past the root when a file moves — so the root is LOCATED, not counted.
const here = dirname(fileURLToPath(new URL(import.meta.url)));
function findWorkspaceRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`the hub-agent-participant gate could not locate the repository root above ${start}`);
}
const repoRoot = findWorkspaceRoot(here);

const ORIGIN = process.env.OS_MCP_HUB_ORIGIN ?? "http://127.0.0.1:8787";
/** ⏳️ How long the hub's server-owned creation transaction may take to make the gate's note ready. */
const CREATION_BUDGET_MS = 1_800_000;
const startedAt = new Date();

type Row = { readonly step: string; readonly ok: boolean; readonly detail: string };
const rows: Row[] = [];
const outDir = process.env.S_OS_MCP_PARTICIPANT_OUT ?? join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️generated/🤖️hub-agent-participant");
const gateways: Array<{ readonly name: string; readonly session: McpClientSession }> = [];
function row(step: string, ok: boolean, detail: string): void {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);
}

async function hub(method: string, path: string, options: { token?: string; body?: string } = {}): Promise<{ status: number; text: string; json: any }> {
  const headers: Record<string, string> = {};
  if (options.body !== undefined) headers["content-type"] = "application/json";
  if (options.token !== undefined) headers.authorization = `Bearer ${options.token}`;
  const response = await fetch(`${ORIGIN}${path}`, { method, headers, ...(options.body === undefined ? {} : { body: options.body }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {
    json = undefined;
  }
  return { status: response.status, text, json };
}

function finish(blocked?: string): never {
  const red = rows.filter((entry) => !entry.ok);
  mkdirSync(outDir, { recursive: true });
  const evidence = gateways.map(({ name, session }) => {
    const path = join(outDir, `gateway-stderr-${name}.txt`);
    writeFileSync(path, `${session.stderrLines().join("\n")}\n`);
    return path;
  });
  console.log(`hub-agent-participant-check: ${rows.length - red.length}/${rows.length} rows green against ${ORIGIN}`);
  publishAcceptanceCheckResult(
    repoRoot,
    acceptanceCheckResult({
      check: "mcp-hub-agent-participant",
      status: blocked ? "blocked" : red.length === 0 && rows.length > 0 ? "pass" : "fail",
      startedAt,
      measured: { rows: rows.length, green: rows.length - red.length },
      evidence,
      summary: blocked
        ? { en: `precondition missing: ${blocked}`, de: `Voraussetzung fehlt: ${blocked}` }
        : { en: `${rows.length - red.length}/${rows.length} agent-participant rows green against ${ORIGIN}${red.length ? `; red: ${red.map((entry) => entry.step.split(" ")[0]).join(",")}` : ""}`, de: `${rows.length - red.length}/${rows.length} Zeilen des Agenten als Teilnehmer grün gegen ${ORIGIN}${red.length ? `; rot: ${red.map((entry) => entry.step.split(" ")[0]).join(",")}` : ""}` },
    }),
  );
  if (blocked) process.exit(1);
  if (red.length > 0) throw new Error(`hub-agent-participant-check: ${red.length} red row(s): ${red.map((entry) => entry.step).join(", ")}`);
  process.exit(0);
}

const readiness = await hub("GET", "/readyz").catch((error: Error) => ({ status: 0, text: error.message, json: undefined }));
row("0 a hub answers /readyz", readiness.status === 200 || readiness.status === 503, `HTTP ${readiness.status} at ${ORIGIN} — start one with 📜️ds1-hub-hold.ts or set OS_MCP_HUB_ORIGIN`);
if (readiness.status === 0) finish(`no hub answers at ${ORIGIN} (OS_MCP_HUB_ORIGIN)`);
row("0b the hub declares features.mcpWorkspace", typeof readiness.json?.features?.mcpWorkspace === "boolean", `features=${JSON.stringify(readiness.json?.features)}`);
row("0c features.mcpWorkspace is true — this hub can serve a delegated MCP workspace", readiness.json?.features?.mcpWorkspace === true, `mcpWorkspace=${readiness.json?.features?.mcpWorkspace} openPlan=${readiness.json?.features?.openPlan}`);

const credential = (() => {
  try {
    return hubCredentialFromEnv();
  } catch (error) {
    if (error instanceof AcceptancePreconditionMissing) finish(error.message);
    throw error;
  }
})();
const signIn = await hub("POST", "/auth/sessions", { body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: credential.email, password: credential.password, deviceInstanceId: "mcphubparticipantgate0000000000".slice(0, 32), clientClass: "browser" }) });
row("1 the human signs in", signIn.status === 200 && typeof signIn.json?.token === "string", `HTTP ${signIn.status} ${signIn.status === 200 ? "session capability minted" : signIn.text.slice(0, 200)}`);
if (signIn.status !== 200) finish();
const token: string = signIn.json.token;

/** 🏘️ A fresh space and a fresh note in it, both through the hub's own authorities, so the gate
 * measures the agent path against a document that matches the catalog the hub serves right now. */
async function freshSpaceWithNote(): Promise<{ spaceId: string; detail: string }> {
  const name = `Hub agent participant ${randomBytes(4).toString("hex")}`;
  const created = await hub("POST", "/directory/commands", { token, body: directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "atelier", "private"))) });
  const listed = await hub("GET", "/directory/spaces", { token });
  const spaceId = String((Array.isArray(listed.json) ? listed.json : []).find((entry: any) => entry?.space?.name === name)?.space?.id ?? "");
  if (!spaceId) return { spaceId, detail: `create-space HTTP ${created.status}: ${created.text.slice(0, 200)}` };
  const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const catalog = await hub("GET", creations, { token });
  const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
  const requestId = randomBytes(16).toString("hex");
  let creation = (await hub("POST", creations, { token, body: JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: "Agent participant note" })) })).json;
  for (const deadline = Date.now() + CREATION_BUDGET_MS; ["accepted", "preparing"].includes(creation?.phase) && Date.now() < deadline; ) {
    await new Promise((resolve) => setTimeout(resolve, 1_000));
    creation = (await hub("GET", `${creations}/${requestId}`, { token })).json;
  }
  return { spaceId, detail: `fresh space ${spaceId}, note kind=${kind?.kindId ?? "<none>"} phase=${creation?.phase} document=${creation?.ready?.artifactId ?? "<none>"}` };
}

const chosen = process.env.OS_MCP_HUB_SPACE ? { spaceId: process.env.OS_MCP_HUB_SPACE, detail: `OS_MCP_HUB_SPACE=${process.env.OS_MCP_HUB_SPACE}` } : await freshSpaceWithNote();
const spaceId = chosen.spaceId;
row("2 the human authors a space holding a note", spaceId.length > 0, chosen.detail);
if (!spaceId) finish();

const delegation = await hub("POST", "/auth/agent-delegations", { token, body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "Participant gate agent", audience: "edit", ttlSecs: 900 }) });
row("3 POST /auth/agent-delegations mints a scoped credential", delegation.status === 201 && typeof delegation.json?.token === "string", `HTTP ${delegation.status} ${delegation.status === 201 ? `agentPrincipalId=${delegation.json?.agentPrincipalId}` : delegation.text.slice(0, 240)}`);
if (delegation.status !== 201) finish();
const delegationId: string = delegation.json.delegationId;
row("3b the agent principal is NOT the delegating human", delegation.json?.agentPrincipalId === `agent:${delegationId}`, `agentPrincipalId=${delegation.json?.agentPrincipalId}`);

/** 🔁️ A SECOND agent session — a fresh gateway process on the same delegation — opens the note after the first
 * one edited it: it must start from the hub's head (its first prepare names a revision at or past the first
 * session's commit) and its own edit must land as a NEW edit on the hub. A gateway that seeded its guest from the
 * hub's last checkpoint alone re-derived the first session's state and ids and the hub took a colliding op (WG9,
 * 26/09/27). */
async function freshSessionContinuesFromTheHead(documentId: string, capabilityId: string, input: unknown, firstCursor: number): Promise<void> {
  const head = async (): Promise<number> => Number((await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, { token })).json?.head_seq ?? -1);
  const second = new McpClientSession(entry, ["--hub", ORIGIN, "--space", spaceId, "--credential-file", credentialPath, "--no-bridge"], repoRoot);
  gateways.push({ name: "second", session: second });
  try {
    const initialized = await second.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "semio-hub-agent-participant-second", title: "hub agent participant gate, second session", version: "1" } });
    if (initialized.error) {
      row("12b a fresh agent session starts from the hub's head, not its last checkpoint", false, JSON.stringify(initialized.error).slice(0, 300));
      return;
    }
    second.notify("notifications/initialized", {});
    await second.call("artifact_open", { artifactId: documentId });
    const prepared = await second.call("action_prepare", { capabilityId, input });
    const cursor = Number(prepared.structuredContent?.expectedRevision?.cursor ?? -1);
    row("12b a fresh agent session starts from the hub's head, not its last checkpoint", prepared.isError !== true && cursor >= firstCursor && firstCursor > 0, prepared.isError === true ? JSON.stringify(prepared.structuredContent).slice(0, 300) : `second session's first revision cursor ${cursor}, first session committed at cursor ${firstCursor}`);
    const handle = prepared.structuredContent?.preparedHandle as string | undefined;
    const before = await head();
    const invoked = handle ? await second.call("action_invoke", { preparedActionHandle: handle }) : undefined;
    const after = await head();
    row("12c the fresh session's edit lands on the hub as a new edit", invoked?.structuredContent?.status === "SUCCEEDED" && after === before + 1, `status=${invoked?.structuredContent?.status ?? "<no handle>"} hub head ${before}→${after}`);
  } finally {
    second.stop();
  }
}

const credentialPath = join(mkdtempSync(join(serviceMcpTestArtifactRoot(), "semio-mcp-hub-agent-")), "agent-credential.json");
writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: ORIGIN, spaceId, audience: "edit", token: delegation.json.token })}\n`, { mode: 0o600 });
chmodSync(credentialPath, 0o600);
row("4 the credential file is written at mode 0600", true, credentialPath);

const declared = mcpServerEntries(repoRoot).semio;
const scopes = declared?.args.includes("--scopes") ? String(declared.args[declared.args.indexOf("--scopes") + 1]) : "workspace.read,artifact.write,inference.execute,ui.observe,ui.control";
// 🚧️ `.mcp.json`'s `semio` entry binds a `--folder`, and `--folder`/`--hub` are mutually
// exclusive, so a hub-bound agent is a DIFFERENT argv, not an extension of that one. This gate
// spawns the identical binary the wrapper execs, with the hub selector in its place.
const entry = { command: requireMcpBinary(repoRoot), args: ["stdio", "--scopes", scopes] };
const session = new McpClientSession(entry, ["--hub", ORIGIN, "--space", spaceId, "--credential-file", credentialPath, "--no-bridge"], repoRoot);
gateways.push({ name: "first", session });
try {
  const initialized = await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: { roots: { listChanged: true }, sampling: {}, elicitation: {} }, clientInfo: { name: "semio-hub-agent-participant", title: "hub agent participant gate", version: "1" } });
  row("5 the gateway serves over the delegated hub session", !initialized.error, initialized.error ? JSON.stringify(initialized.error).slice(0, 300) : `server=${initialized.result?.serverInfo?.name}@${initialized.result?.serverInfo?.version}`);
  if (initialized.error) finish();
  session.notify("notifications/initialized", {});

  const context = await session.call("context_resolve", {});
  const principal = String(context.structuredContent?.principal ?? "");
  row("6 context_resolve names the agent's own principal", principal === `agent:${delegationId}`, `principal=${principal} channel=${context.structuredContent?.channel}`);

  const listed = await session.request("resources/read", { uri: "semio://workspace/artifacts" });
  const listedText = String((listed.result?.contents ?? [])[0]?.text ?? "");
  let parsed: any;
  try {
    parsed = JSON.parse(listedText);
  } catch {
    parsed = undefined;
  }
  const documentId = String((parsed?.artifacts ?? [])[0]?.scope?.documentId ?? "");
  row("7 the agent sees the space's own documents", documentId.length > 0, documentId.length > 0 ? `documentId=${documentId}` : `no document in ${listedText.slice(0, 240)} — this space holds none, or the binding is unauthorized`);
  if (!documentId) finish();

  const opened = await session.call("artifact_open", { artifactId: documentId });
  row("8 artifact_open of a HUB document answers", opened.isError !== true, opened.isError === true ? JSON.stringify(opened.structuredContent).slice(0, 300) : `kind=${opened.structuredContent?.kind} sizeBytes=${opened.structuredContent?.sizeBytes} link=${JSON.stringify(opened.structuredContent?.sessionDocument?.sync ?? null)}`);

  const snapshot = await session.call("artifact_snapshot", { artifactId: documentId });
  row("9 artifact_snapshot of a HUB document answers real bytes", snapshot.isError !== true && Number(snapshot.structuredContent?.packBytes ?? 0) > 0, snapshot.isError === true ? JSON.stringify(snapshot.structuredContent).slice(0, 300) : `packBytes=${snapshot.structuredContent?.packBytes} sprBytes=${snapshot.structuredContent?.sprBytes}`);

  const kind = String(opened.structuredContent?.kind ?? "");
  const search = await session.call("capabilities_search", { query: "add", kind: "mutation" });
  // 🔎️ `results`, the key `capabilities_search_output_shape` declares (`🌉️mcp/🧬️schema/🦀️.rs:340`).
  const hits = (search.structuredContent?.results ?? []) as Array<Record<string, any>>;
  row("10 the catalog the agent searches is the HUB's own", search.isError !== true && hits.length > 0, `${hits.length} hit(s) over the hub-selected roster; opened kind ${kind || "<none>"}`);
  const capabilityId = String(hits[0]?.id ?? hits[0]?.capabilityId ?? "");

  if (capabilityId) {
    const described = await session.call("capabilities_describe", { capabilityId });
    const input = minimalInputForSchema(described.structuredContent?.inputSchema ?? described.structuredContent?.capability?.inputSchema);
    const prepared = await session.call("action_prepare", { capabilityId, input });
    row("11 action_prepare reaches a guest the HUB authorized", prepared.isError !== true, prepared.isError === true ? `${capabilityId} input=${JSON.stringify(input)}: ${JSON.stringify(prepared.structuredContent).slice(0, 300)}` : `${capabilityId} input=${JSON.stringify(input)} handle=${prepared.structuredContent?.preparedHandle}`);
    const handle = prepared.structuredContent?.preparedHandle as string | undefined;
    if (handle) {
      const invoked = await session.call("action_invoke", { preparedActionHandle: handle });
      row("12 action_invoke commits the agent's edit", invoked.isError !== true && invoked.structuredContent?.status === "SUCCEEDED", invoked.isError === true ? JSON.stringify(invoked.structuredContent).slice(0, 300) : `status=${invoked.structuredContent?.status}`);
      await freshSessionContinuesFromTheHead(documentId, capabilityId, input, Number(invoked.structuredContent?.revisionAfter?.cursor ?? 0));
    } else {
      row("12 action_invoke commits the agent's edit", false, "action_prepare minted no handle");
    }
  } else {
    row("11 action_prepare reaches a guest the HUB authorized", false, "capabilities_search returned no mutation to dispatch");
    row("12 action_invoke commits the agent's edit", false, "no capability to prepare");
  }
} finally {
  session.stop();
}

const revoked = await hub("POST", `/auth/agent-delegations/${encodeURIComponent(delegationId)}/revoke`, { token });
row("13 the human revokes the delegation", revoked.status === 204, `HTTP ${revoked.status}`);
finish();
