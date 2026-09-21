/** 🤖️ The hub-agent-participant gate: is an MCP agent a REAL third participant in a hub space?
 *
 * `live-agent-loop-check` owns the shell route (an agent driving a human's open `dev s` session).
 * This gate owns the other one — the route a Claude Code / Cursor user actually configures for a
 * remote space: `semio-os-mcp stdio --hub <origin> --space <id> --credential-file <0600 file>`,
 * authenticated by a delegation its human minted, with no launcher, no fd 3 and no local folder.
 *
 * It boots nothing. An `os-hub` with a published trusted catalog and at least one document costs
 * minutes to bring up and every collaboration run already has one; when none answers, the gate says
 * which origin it looked for instead of pretending. Configuration is entirely by environment:
 *
 *   OS_MCP_HUB_ORIGIN    default `http://127.0.0.1:7631`
 *   OS_MCP_HUB_EMAIL     default `user1@semio.dev`
 *   OS_MCP_HUB_PASSWORD  default `gm1-local-dev-pass-1`
 *   OS_MCP_HUB_SPACE     default: the first space the human authors that holds a document
 *
 * The rows are the chain, in order. Every one is required: a red row exits non-zero and prints the
 * refusal verbatim, because the whole value of this gate is that it cannot round a missing
 * participant up to a passing one. Ticket 26/09/18 slice M8.
 */
import { chmodSync, existsSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { McpClientSession, mcpServerEntries, requireMcpBinary } from "../../🟦️.ts";

// 📁️ `new URL(x, import.meta.url).pathname` percent-encodes emoji path segments, and a counted
// `..` chain silently walks past the root when a file moves — so the root is LOCATED, not counted.
const here = dirname(fileURLToPath(new URL(import.meta.url)));
function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`the hub-agent-participant gate could not locate the repository root above ${start}`);
}
const repoRoot = findRepoRoot(here);

const ORIGIN = process.env.OS_MCP_HUB_ORIGIN ?? "http://127.0.0.1:7631";
const EMAIL = process.env.OS_MCP_HUB_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.OS_MCP_HUB_PASSWORD ?? "gm1-local-dev-pass-1";

type Row = { readonly step: string; readonly ok: boolean; readonly detail: string };
const rows: Row[] = [];
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

function finish(): never {
  const red = rows.filter((entry) => !entry.ok);
  console.log(`hub-agent-participant-check: ${rows.length - red.length}/${rows.length} rows green against ${ORIGIN}`);
  if (red.length > 0) throw new Error(`hub-agent-participant-check: ${red.length} red row(s): ${red.map((entry) => entry.step).join(", ")}`);
  process.exit(0);
}

const readiness = await hub("GET", "/readyz").catch((error: Error) => ({ status: 0, text: error.message, json: undefined }));
row("0 a hub answers /readyz", readiness.status === 200 || readiness.status === 503, `HTTP ${readiness.status} at ${ORIGIN} — start one with 📜️ds1-hub-hold.ts or set OS_MCP_HUB_ORIGIN`);
if (readiness.status === 0) finish();
row("0b the hub declares features.mcpWorkspace", typeof readiness.json?.features?.mcpWorkspace === "boolean", `features=${JSON.stringify(readiness.json?.features)}`);
row("0c features.mcpWorkspace is true — this hub can serve a delegated MCP workspace", readiness.json?.features?.mcpWorkspace === true, `mcpWorkspace=${readiness.json?.features?.mcpWorkspace} openPlan=${readiness.json?.features?.openPlan}`);

const signIn = await hub("POST", "/auth/sessions", { body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: "mcphubparticipantgate0000000000".slice(0, 32), clientClass: "browser" }) });
row("1 the human signs in", signIn.status === 200 && typeof signIn.json?.token === "string", `HTTP ${signIn.status} ${signIn.status === 200 ? "session capability minted" : signIn.text.slice(0, 200)}`);
if (signIn.status !== 200) finish();
const token: string = signIn.json.token;

const spaces = await hub("GET", "/directory/spaces", { token });
const authored = (Array.isArray(spaces.json) ? spaces.json : []).map((entry: any) => String(entry?.space?.id ?? "")).filter((id: string) => id.length > 0);
const spaceId = process.env.OS_MCP_HUB_SPACE ?? authored[authored.length - 1] ?? "";
row("2 the human authors a space", spaceId.length > 0, `spaceId=${spaceId || "<none>"} of ${authored.length} space(s)`);
if (!spaceId) finish();

const delegation = await hub("POST", "/auth/agent-delegations", { token, body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "Participant gate agent", audience: "edit", ttlSecs: 900 }) });
row("3 POST /auth/agent-delegations mints a scoped credential", delegation.status === 201 && typeof delegation.json?.token === "string", `HTTP ${delegation.status} ${delegation.status === 201 ? `agentPrincipalId=${delegation.json?.agentPrincipalId}` : delegation.text.slice(0, 240)}`);
if (delegation.status !== 201) finish();
const delegationId: string = delegation.json.delegationId;
row("3b the agent principal is NOT the delegating human", delegation.json?.agentPrincipalId === `agent:${delegationId}`, `agentPrincipalId=${delegation.json?.agentPrincipalId}`);

const credentialPath = join(mkdtempSync(join(tmpdir(), "semio-mcp-hub-agent-")), "agent-credential.json");
writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: ORIGIN, spaceId, audience: "edit", token: delegation.json.token })}\n`, { mode: 0o600 });
chmodSync(credentialPath, 0o600);
row("4 the credential file is written at mode 0600", true, credentialPath);

const declared = mcpServerEntries(repoRoot).semio;
const scopes = declared?.args.includes("--scopes") ? String(declared.args[declared.args.indexOf("--scopes") + 1]) : "workspace.read,artifact.write,inference.execute,ui.observe,ui.control";
// 🚧️ `.mcp.json`'s `semio` entry hard-codes `--folder .`, and `--folder`/`--hub` are mutually
// exclusive, so a hub-bound agent is a DIFFERENT argv, not an extension of that one. This gate
// spawns the identical binary the wrapper execs, with the hub selector in its place.
const entry = { command: requireMcpBinary(repoRoot), args: ["stdio", "--scopes", scopes] };
const session = new McpClientSession(entry, ["--hub", ORIGIN, "--space", spaceId, "--credential-file", credentialPath, "--no-bridge"], repoRoot);
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
  row("8 artifact_open of a HUB document answers", opened.isError !== true, opened.isError === true ? JSON.stringify(opened.structuredContent).slice(0, 300) : `kind=${opened.structuredContent?.kind} sizeBytes=${opened.structuredContent?.sizeBytes}`);

  const snapshot = await session.call("artifact_snapshot", { artifactId: documentId });
  row("9 artifact_snapshot of a HUB document answers real bytes", snapshot.isError !== true && Number(snapshot.structuredContent?.packBytes ?? 0) > 0, snapshot.isError === true ? JSON.stringify(snapshot.structuredContent).slice(0, 300) : `packBytes=${snapshot.structuredContent?.packBytes} sprBytes=${snapshot.structuredContent?.sprBytes}`);

  const kind = String(opened.structuredContent?.kind ?? "");
  const search = await session.call("capabilities_search", { query: "add", kind: "mutation" });
  // 🔎️ `results`, the key `capabilities_search_output_shape` declares (`🌉️mcp/🧬️schema/🦀️.rs:340`).
  const hits = (search.structuredContent?.results ?? []) as Array<Record<string, any>>;
  row("10 the catalog the agent searches is the HUB's own", search.isError !== true && hits.length > 0, `${hits.length} hit(s) over the hub-selected roster; opened kind ${kind || "<none>"}`);
  const capabilityId = String(hits[0]?.id ?? hits[0]?.capabilityId ?? "");

  if (capabilityId) {
    const prepared = await session.call("action_prepare", { capabilityId, input: {} });
    row("11 action_prepare reaches a guest the HUB authorized", prepared.isError !== true, prepared.isError === true ? `${capabilityId}: ${JSON.stringify(prepared.structuredContent).slice(0, 300)}` : `handle=${prepared.structuredContent?.preparedHandle}`);
    const handle = prepared.structuredContent?.preparedHandle as string | undefined;
    if (handle) {
      const invoked = await session.call("action_invoke", { preparedActionHandle: handle });
      row("12 action_invoke commits the agent's edit", invoked.isError !== true && invoked.structuredContent?.status === "SUCCEEDED", invoked.isError === true ? JSON.stringify(invoked.structuredContent).slice(0, 300) : `status=${invoked.structuredContent?.status}`);
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

const revoked = await hub("DELETE", `/auth/agent-delegations/${encodeURIComponent(delegationId)}`, { token });
row("13 the human revokes the delegation", revoked.status === 204, `HTTP ${revoked.status}`);
finish();
