import { serviceMcpTestArtifactRoot } from "../📁️artifact-root/🟦️.ts";
/** 🧩️ The plugin-coverage lane-parity sweep: every installed plugin package over the semio MCP.
 *
 * One fresh `semio-os-mcp stdio --folder <temporary>` gateway (the staged binary, auto-approving, no bridge). Per plugin
 * package: its capabilities (`capabilities_search` by owner, paginated) described one by one, how many are mutations,
 * destructive, carry an empty description or declare no arguments, its declared inferences (`inference_list`), and for
 * every installed artifact kind of the package `artifact_create` plus ONE mutation through `action_prepare` →
 * `action_invoke` whose required inputs the declared input schema can fill (default, first enum value, or a typed
 * neutral value; in the hub lane an id-like input takes an id the document's snapshot holds, as an agent reading the
 * document first would) — non-destructive verbs first, then destructive ones (the gateway binds a throwaway folder and
 * auto-approves, and the row says `invokeDestructive`). A kind passes when it is created and one mutation SUCCEEDS; a
 * kind whose package declares no mutation for it at all passes on creation and is counted apart
 * (`no-mutation-declared`), never rounded into the mutated count. The sweep passes when every kind does. A gateway that
 * stops answering is respawned and the call recorded as `CLIENT_TIMEOUT`.
 *
 * Configuration by environment: `S_OS_MCP_COVERAGE_PLUGINS` (comma list, default every installed plugin),
 * `S_OS_MCP_COVERAGE_OUT` (default `🌉️mcp/🤖️generated/🧩️plugin-coverage`). Rows: `coverage-rows.jsonl`, table:
 * `coverage-table.md`. Promoted from the session-12 ticket harness `wp-g10/g10-plugin-coverage.ts` (ticket 26/09/23, G10 S3).
 *
 * 🌐️ The hub lane (`OS_MCP_HUB_ORIGIN`, the verb's `--hub <url>`; `OS_MCP_HUB_EMAIL` / `OS_MCP_HUB_PASSWORD` required):
 * every kind the hub's own creation catalog offers is created by the hub's server-owned creation transaction in one
 * fresh space (at most `CREATE_WINDOW` in flight; a refused creation keeps the hub's status and answer in its row), then — over ONE delegated `semio-os-mcp --hub` gateway — opened, mutated once (the hub's head advances),
 * undone and redone (each relayed: the head advances again) and exported. A kind passes when all of that holds, or when
 * its package declares no mutation for it (then create + open + export). Rows `coverage-hub-rows.jsonl`, table
 * `coverage-hub-table.md`, the gateway's stderr `coverage-hub-gateway-stderr.txt`, record `mcp-plugin-coverage-hub`.
 */
import { appendFileSync, chmodSync, existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { AcceptancePreconditionMissing, hubCredentialFromEnv, isAcceptancePreconditionMissing, McpClientSession, requireMcpBinary, spawnRawMcp } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const here = dirname(fileURLToPath(new URL(import.meta.url)));
function findWorkspaceRoot(start: string): string {
  for (let current = start, depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`the plugin-coverage sweep could not locate the repository root above ${start}`);
}
const repoRoot = findWorkspaceRoot(here);
const outDir = process.env.S_OS_MCP_COVERAGE_OUT ?? join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️generated/🧩️plugin-coverage");
const only = new Set((process.env.S_OS_MCP_COVERAGE_PLUGINS ?? "").split(",").filter(Boolean));
const CALL_MS = 240_000;
const CREATE_MS = 900_000;
/** 🪟️ How many hub creations the hub lane has in flight at once — a bounded window, so one run never floods the hub's
 * creation workers with every kind of a large catalog at the same moment. */
const CREATE_WINDOW = 4;
/** 🪟️ The typed refusal of a verb whose whole effect is shell view state (`interactive-job.agent-lane-uncarried`): correct for
 * an agent, so the battery marks it — and a kind whose every candidate verb is one — "n/a for agents", never a failure. */
const AGENT_LANE_UNCARRIED = "interactive-job.agent-lane-uncarried";
const NOT_FOR_AGENTS = "n/a for agents";
const startedAt = new Date();
mkdirSync(outDir, { recursive: true });
const rowsPath = join(outDir, "coverage-rows.jsonl");
writeFileSync(rowsPath, "");
const log = (line: string): void => console.log(`${new Date().toISOString()} ${line}`);
const gatewayArgs = (): string[] => ["stdio", "--folder", mkdtempSync(join(serviceMcpTestArtifactRoot(), "semio-mcp-coverage-")), "--scopes", "workspace.read,artifact.write,inference.execute", "--no-bridge", "--auto-approve", "all"];

type CallAnswer = { ok: boolean; value: any; ms: number };

/** 🧮️ A required input the declared schema can fill: its default, its first enum value, or a neutral value of its type. */
function neutralInput(schema: any): Record<string, unknown> | null {
  const byType: Record<string, unknown> = { string: "Coverage", number: 1, integer: 1, boolean: false, array: [], object: {} };
  const input: Record<string, unknown> = {};
  for (const name of schema?.required ?? []) {
    const property = schema.properties?.[name] ?? {};
    const type = Array.isArray(property.type) ? property.type[0] : property.type;
    if (property.default !== undefined) input[name] = property.default;
    else if (Array.isArray(property.enum) && property.enum.length > 0) input[name] = property.enum[0];
    else if (type in byType) input[name] = byType[type];
    else return null;
  }
  return input;
}

/** 🔎️ An id-like input or snapshot key: `id`, `key`, `…Id`, `…_id`, `…Ids`, `…_ids`. */
const ID_KEY = /^(id|key)$|Id$|_id$|Ids$|_ids$/u;

/** 🔎️ The entity ids an agent reads from a document before acting on it: every string an id-like key of the artifact
 * snapshot holds, first seen first, at most 32. */
function harvestIds(snapshot: unknown): string[] {
  const ids: string[] = [];
  const walk = (value: unknown, key: string, depth: number): void => {
    if (ids.length >= 32 || depth > 16) return;
    if (typeof value === "string") {
      if (value && ID_KEY.test(key) && !ids.includes(value)) ids.push(value);
    } else if (Array.isArray(value)) {
      for (const item of value) walk(item, key, depth + 1);
    } else if (value && typeof value === "object") {
      for (const [child, item] of Object.entries(value)) walk(item, child, depth + 1);
    }
  };
  walk(snapshot, "", 0);
  return ids;
}

/** 🧮️ What an agent fills a verb's required inputs with after reading the document: an id-like input without a
 * default or enum takes the first id the snapshot holds (`harvestIds`), every other input its `neutralInput` value. */
function agentInput(schema: any, ids: string[]): Record<string, unknown> | null {
  const input = neutralInput(schema);
  if (input === null || ids.length === 0) return input;
  for (const name of schema?.required ?? []) {
    const property = schema.properties?.[name] ?? {};
    if (property.default !== undefined || Array.isArray(property.enum) || !ID_KEY.test(name)) continue;
    const type = Array.isArray(property.type) ? property.type[0] : property.type;
    if (type === "string") input[name] = ids[0];
    else if (type === "array" && property.items?.type === "string") input[name] = [ids[0]];
  }
  return input;
}

/** 🎯️ How many of a kind's candidate verbs the hub lane tries before the kind counts as not mutated. */
const HUB_CANDIDATES = 16;

/** 🎯️ A kind's mutation candidates: non-destructive verbs first, then those declaring arguments. */
const candidateOrder = (left: any, right: any): number => Number(left?.effects?.destructive === true) - Number(right?.effects?.destructive === true) || Number((right?.presentation?.args ?? []).length > 0) - Number((left?.presentation?.args ?? []).length > 0);

const HUB_ORIGIN = process.env.OS_MCP_HUB_ORIGIN?.replace(/\/$/u, "");
if (HUB_ORIGIN) await withAcceptanceRecord(repoRoot, "mcp-plugin-coverage-hub", () => hubLane(HUB_ORIGIN), isAcceptancePreconditionMissing);
else await withAcceptanceRecord(repoRoot, "mcp-plugin-coverage", async () => {
  let aborted = false;
  const cancel = (): void => void (aborted = true);
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);

  let gateway = spawnRawMcp(requireMcpBinary(repoRoot), gatewayArgs());
  const discover = async (): Promise<void> => {
    gateway.writeRaw(JSON.stringify({ jsonrpc: "2.0", id: 9001, method: "server/discover", params: {} }));
    await gateway.nextLine(120_000);
  };
  await discover();
  async function call(name: string, args: Record<string, unknown>, budgetMs = CALL_MS): Promise<CallAnswer> {
    const started = Date.now();
    try {
      const response: any = await gateway.request("tools/call", { name, arguments: args }, budgetMs);
      const result = response.result;
      return { ok: !response.error && result?.isError !== true, value: response.error ?? result?.structuredContent ?? result, ms: Date.now() - started };
    } catch (error) {
      log(`call ${name} threw ${String(error).slice(0, 200)}; respawning the gateway`);
      await gateway.close().catch(() => undefined);
      gateway = spawnRawMcp(requireMcpBinary(repoRoot), gatewayArgs());
      await discover();
      return { ok: false, value: { code: "CLIENT_TIMEOUT", message: String(error).slice(0, 300) }, ms: Date.now() - started };
    }
  }

  const probe = await call("artifact_create", { artifactId: "coverage-probe-kinds", kind: "no.such.kind" });
  const installedKinds: string[] = probe.value?.details?.installedKinds ?? [];
  const inferences = await call("inference_list", {});
  const declaredInferences: any[] = inferences.value?.declared ?? inferences.value?.services ?? inferences.value?.inferences ?? [];
  log(`installed kinds=${installedKinds.length} declared inferences=${declaredInferences.length}`);
  const pluginIds = [...new Set(installedKinds.filter((kind) => kind.startsWith("s.")).map((kind) => kind.split(".")[1]!))].filter((id) => only.size === 0 || only.has(id)).sort();
  const table: string[] = [];
  let kindsTotal = 0;
  let kindsCreated = 0;
  let kindsInvoked = 0;
  const failing: string[] = [];
  const undeclared: string[] = [];
  for (const [index, pluginId] of pluginIds.entries()) {
    if (aborted) break;
    const capabilities: any[] = [];
    let cursor: string | undefined;
    do {
      const page = await call("capabilities_search", { query: pluginId, owner: pluginId, limit: 100, ...(cursor ? { cursor } : {}) });
      capabilities.push(...(page.value?.results ?? []));
      cursor = page.value?.nextCursor ?? undefined;
    } while (cursor);
    const described: any[] = [];
    for (const capability of capabilities) {
      const answer = await call("capabilities_describe", { capabilityId: capability.capabilityId });
      if (answer.ok) described.push(answer.value);
    }
    const mutations = described.filter((capability) => capability?.kind === "mutation");
    const emptyDescriptions = described.filter((capability) => !String(capability?.description ?? "").trim()).length;
    const noArgs = mutations.filter((capability) => (capability?.presentation?.args ?? []).length === 0).length;
    const destructive = mutations.filter((capability) => capability?.effects?.destructive === true).length;
    const pluginInferences = declaredInferences.filter((row) => JSON.stringify(row).includes(`"${pluginId}"`) || String(row?.artifactKind ?? row?.kind ?? "").startsWith(`s.${pluginId}.`)).length;
    const kinds = installedKinds.filter((kind) => kind.startsWith(`s.${pluginId}.`) || kind === `s.${pluginId}`);
    let created = 0;
    let invoked = 0;
    for (const kind of kinds) {
      if (aborted) break;
      const create = await call("artifact_create", { artifactId: `coverage-${kind.replaceAll(".", "-")}`, kind }, CREATE_MS);
      const row: Record<string, unknown> = { pluginId, kind, create: create.ok ? "ok" : String(create.value?.code ?? "error"), createMs: create.ms, createDetail: create.ok ? `${create.value?.sizeBytes ?? "?"} B` : String(create.value?.message ?? "").slice(0, 220) };
      if (create.ok) {
        created += 1;
        const candidates = mutations
          .filter((capability) => capability?.artifactKind === kind)
          .sort(candidateOrder);
        const attempts: string[] = [];
        for (const capability of candidates.slice(0, 8)) {
          const input = neutralInput(capability.inputSchema ?? capability.input_schema ?? {});
          if (input === null) {
            attempts.push(`${capability.id}: needs required args`);
            continue;
          }
          const prepared = await call("action_prepare", { capabilityId: capability.id, input });
          if (!prepared.ok) {
            attempts.push(`${capability.id}: prepare ${prepared.value?.code} ${String(prepared.value?.message ?? "").slice(0, 120)}`);
            continue;
          }
          const done = await call("action_invoke", { preparedActionHandle: prepared.value.preparedHandle });
          if (done.ok && done.value?.status === "SUCCEEDED") {
            invoked += 1;
            Object.assign(row, { invoke: "ok", invokeVerb: capability.id, invokeMs: done.ms, invokeDestructive: capability?.effects?.destructive === true });
            break;
          }
          attempts.push(`${capability.id}: invoke ${done.value?.status ?? done.value?.code} ${String(done.value?.message ?? "").slice(0, 120)}`);
        }
        if (row.invoke !== "ok") Object.assign(row, { invoke: candidates.length === 0 ? "no-mutation-declared" : "failed", invokeAttempts: attempts });
        if (row.invoke === "no-mutation-declared") undeclared.push(kind);
      }
      if (!(row.create === "ok" && (row.invoke === "ok" || row.invoke === "no-mutation-declared"))) failing.push(`${kind}:${String(row.create)}/${String(row.invoke ?? "-")}`);
      appendFileSync(rowsPath, `${JSON.stringify(row)}\n`);
      log(`${pluginId} ${kind} create=${String(row.create)} (${create.ms} ms) invoke=${String(row.invoke ?? "-")} ${String(row.invokeVerb ?? "")}`);
    }
    kindsTotal += kinds.length;
    kindsCreated += created;
    kindsInvoked += invoked;
    const summary = { pluginId, capabilities: described.length, mutations: mutations.length, destructive, emptyDescriptions, mutationsWithoutArgs: noArgs, inferences: pluginInferences, kinds: kinds.length, created, invoked };
    appendFileSync(rowsPath, `${JSON.stringify({ summary })}\n`);
    table.push(`| ${pluginId} | ${described.length} | ${mutations.length} | ${destructive} | ${emptyDescriptions} | ${noArgs} | ${pluginInferences} | ${created}/${kinds.length} | ${invoked}/${created} |`);
    log(`${index + 1}/${pluginIds.length} SUMMARY ${JSON.stringify(summary)}`);
  }
  await gateway.close().catch(() => undefined);
  const header = "| package | capabilities | mutations | destructive | empty description | mutations declaring no args | inferences | artifact_create ok/kinds | action_invoke ok/created |\n|---|---|---|---|---|---|---|---|---|";
  writeFileSync(join(outDir, "coverage-table.md"), `${header}\n${table.join("\n")}\n`);
  console.log(`${header}\n${table.join("\n")}`);
  const status = !aborted && kindsTotal > 0 && failing.length === 0 ? "pass" : "fail";
  publishAcceptanceCheckResult(
    repoRoot,
    acceptanceCheckResult({
      check: "mcp-plugin-coverage",
      status,
      startedAt,
      measured: { plugins: pluginIds.length, kinds: kindsTotal, created: kindsCreated, invoked: kindsInvoked, noMutationDeclared: undeclared.length, failing: failing.length, cancelled: aborted },
      summary: {
        en: `${kindsInvoked}/${kindsTotal - undeclared.length} installed kinds with a declared mutation created and mutated over the semio MCP (${kindsCreated}/${kindsTotal} created; ${undeclared.length} declare no mutation${undeclared.length ? `: ${undeclared.join(", ")}` : ""})${failing.length ? `; failing: ${failing.slice(0, 8).join(", ")}` : ""}`,
        de: `${kindsInvoked}/${kindsTotal - undeclared.length} installierte Arten mit erklärter Änderung über das semio-MCP angelegt und verändert (${kindsCreated}/${kindsTotal} angelegt; ${undeclared.length} erklären keine Änderung${undeclared.length ? `: ${undeclared.join(", ")}` : ""})${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 8).join(", ")}` : ""}`,
      },
      evidence: [rowsPath, join(outDir, "coverage-table.md")],
    }),
  );
  if (status !== "pass") process.exitCode = 1;
});

/** 🌐️ The hub lane — see the module doc. One delegated gateway for the whole space, so a kind's verbs run on the guest
 * the HUB authorized for it, against the document the hub created. */
async function hubLane(hub: string): Promise<void> {
  const credential = hubCredentialFromEnv();
  const rowsHubPath = join(outDir, "coverage-hub-rows.jsonl");
  writeFileSync(rowsHubPath, "");
  const http = async (method: string, path: string, token?: string, body?: string): Promise<{ status: number; json: any }> => {
    const response = await fetch(`${hub}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
    const text = await response.text();
    try {
      return { status: response.status, json: JSON.parse(text) };
    } catch {
      return { status: response.status, json: null };
    }
  };
  const signIn = await http("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: credential.email, password: credential.password, deviceInstanceId: `coverage${randomBytes(12).toString("hex")}`, clientClass: "browser" }));
  const token = String(signIn.json?.token ?? "");
  if (!token) throw new AcceptancePreconditionMissing(`the human cannot sign in at ${hub} (HTTP ${signIn.status})`);
  const spaceName = `MCP coverage ${randomBytes(4).toString("hex")}`;
  await http("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))));
  const spaceId = String(((await http("GET", "/directory/spaces", token)).json ?? []).find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
  if (!spaceId) throw new Error(`create-space ${spaceName} is not listed at ${hub}`);
  const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const catalog = (await http("GET", creations, token)).json;
  const pluginOf = (kind: any): string => String(kind?.dialect?.artifactKind ?? "").split(".")[1] ?? "";
  const offered: any[] = (catalog?.kinds ?? []).filter((kind: any) => only.size === 0 || only.has(pluginOf(kind)));
  log(`hub ${hub} space ${spaceId}: ${offered.length} creatable kind(s)`);
  const pending: Array<{ kind: any; requestId: string; startedMs: number; creation: any; refusal: string }> = [];
  const settle = async (entry: (typeof pending)[number]) => {
    while (["accepted", "preparing"].includes(entry.creation?.phase) && Date.now() - entry.startedMs < CREATE_MS) {
      await new Promise((resolve) => setTimeout(resolve, 1_000));
      entry.creation = (await http("GET", `${creations}/${entry.requestId}`, token)).json;
    }
  };
  for (let start = 0; start < offered.length; start += CREATE_WINDOW) {
    const window = offered.slice(start, start + CREATE_WINDOW).map((kind) => ({ kind, requestId: randomBytes(16).toString("hex"), startedMs: Date.now(), creation: undefined as any, refusal: "" }));
    for (const entry of window) {
      const created = await http("POST", creations, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId: entry.requestId, expectedCatalogGenerationId: String(catalog?.catalogGenerationId ?? ""), kindId: String(entry.kind.kindId), name: `Coverage ${entry.kind.kindId}` })));
      entry.creation = created.json;
      if (!created.json?.phase) entry.refusal = `HTTP ${created.status} ${JSON.stringify(created.json).slice(0, 200)}`;
    }
    await Promise.all(window.map(settle));
    pending.push(...window);
  }
  const delegation = await http("POST", "/auth/agent-delegations", token, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "Coverage agent", audience: "edit", ttlSecs: 3_600 }));
  if (delegation.status !== 201) throw new Error(`the agent delegation was refused: HTTP ${delegation.status}`);
  const credentialPath = join(mkdtempSync(join(serviceMcpTestArtifactRoot(), "semio-mcp-coverage-hub-")), "agent-credential.json");
  writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: hub, spaceId, audience: "edit", token: delegation.json.token })}\n`, { mode: 0o600 });
  chmodSync(credentialPath, 0o600);
  const session = new McpClientSession({ command: requireMcpBinary(repoRoot), args: ["stdio", "--scopes", "workspace.read,artifact.write,inference.execute"] }, ["--hub", hub, "--space", spaceId, "--credential-file", credentialPath, "--no-bridge", "--auto-approve", "all"], repoRoot);
  const head = async (documentId: string): Promise<number> => Number((await http("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, token)).json?.head_seq ?? -1);
  const headAbove = async (documentId: string, floor: number): Promise<number> => {
    for (const deadline = Date.now() + 10_000; Date.now() < deadline; await new Promise((resolve) => setTimeout(resolve, 250))) {
      const current = await head(documentId);
      if (current > floor) return current;
    }
    return head(documentId);
  };
  const failing: string[] = [];
  const undeclared: string[] = [];
  const shellOnly: string[] = [];
  const table: string[] = [];
  try {
    const initialized = await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "semio-plugin-coverage-hub", title: "plugin coverage, hub lane", version: "1" } });
    if (initialized.error) throw new Error(`the delegated gateway refused to initialize: ${JSON.stringify(initialized.error).slice(0, 300)}`);
    session.notify("notifications/initialized", {});
    for (const entry of pending) {
      const artifactKind = String(entry.kind?.dialect?.artifactKind ?? "");
      const documentId = String(entry.creation?.ready?.artifactId ?? "");
      const row: Record<string, unknown> = { kindId: entry.kind.kindId, artifactKind, create: documentId ? "ok" : String(entry.creation?.phase ?? `refused ${entry.refusal}`), createMs: Date.now() - entry.startedMs };
      if (documentId) {
        const openStarted = Date.now();
        const opened = await session.call("artifact_open", { artifactId: documentId }, CALL_MS);
        row.openMs = Date.now() - openStarted;
        row.open = opened.isError ? `${opened.structuredContent?.code}` : "ok";
        if (opened.isError) row.openDetail = String(opened.structuredContent?.message ?? "").slice(0, 240);
        const plugin = pluginOf(entry.kind);
        const found = await session.call("capabilities_search", { query: plugin, owner: plugin, artifactKind, kind: ["mutation"], limit: 100 });
        const described: any[] = [];
        for (const hit of found.structuredContent?.results ?? []) {
          const answer = await session.call("capabilities_describe", { capabilityId: hit.capabilityId });
          if (!answer.isError && answer.structuredContent) described.push(answer.structuredContent);
        }
        const candidates = described.filter((capability: any) => capability?.artifactKind === artifactKind).sort(candidateOrder);
        const snapshot = opened.isError ? null : await session.call("artifact_snapshot", { artifactId: documentId }, CALL_MS);
        const ids = harvestIds(snapshot?.isError ? null : snapshot?.structuredContent);
        const attempts: string[] = [];
        let undoToken = "";
        for (const capability of candidates.slice(0, HUB_CANDIDATES)) {
          const input = agentInput(capability.inputSchema ?? {}, ids);
          if (input === null) {
            attempts.push(`${capability.id}: needs required args`);
            continue;
          }
          const before = await head(documentId);
          const prepared = await session.call("action_prepare", { capabilityId: capability.id, input }, CALL_MS);
          if (prepared.isError) {
            attempts.push(prepared.structuredContent?.details?.faultCode === AGENT_LANE_UNCARRIED ? `${capability.id}: ${NOT_FOR_AGENTS} (runs only from the shell)` : `${capability.id}: prepare ${prepared.structuredContent?.code} ${String(prepared.structuredContent?.message ?? "").slice(0, 120)}`);
            continue;
          }
          const invoked = await session.call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle }, CALL_MS);
          const succeeded = !invoked.isError && invoked.structuredContent?.status === "SUCCEEDED";
          const after = succeeded ? await headAbove(documentId, before) : await head(documentId);
          if (succeeded && after > before) {
            Object.assign(row, { mutate: "ok", verb: capability.id, input, destructive: capability?.effects?.destructive === true, head: `${before}→${after}` });
            undoToken = String(invoked.structuredContent?.undoToken ?? "");
            break;
          }
          const noChange = ((invoked.structuredContent?.warnings ?? []) as string[]).some((warning) => warning.startsWith("no-change")) ? "no-change (the verb emitted no operation) " : "";
          attempts.push(`${capability.id}: invoke ${invoked.structuredContent?.status ?? invoked.structuredContent?.code} head ${before}→${after} ${noChange}${String(invoked.structuredContent?.message ?? "").slice(0, 120)}`);
        }
        const viewOnly = attempts.length > 0 && attempts.every((attempt) => attempt.includes(`: ${NOT_FOR_AGENTS} `));
        if (row.mutate !== "ok") Object.assign(row, { mutate: row.open !== "ok" ? "not-reached" : candidates.length === 0 ? "no-mutation-declared" : viewOnly ? NOT_FOR_AGENTS : "failed", attempts });
        if (undoToken) {
          for (const tool of ["history_undo", "history_redo"] as const) {
            const before = await head(documentId);
            const answer = await session.call(tool, { undoToken }, CALL_MS);
            const after = await headAbove(documentId, before);
            row[tool === "history_undo" ? "undo" : "redo"] = !answer.isError && after > before ? `ok ${before}→${after}` : `${answer.structuredContent?.code ?? "no-relay"} ${before}→${after} ${String(answer.structuredContent?.message ?? "").slice(0, 120)}`;
          }
        }
        const exported = await session.call("artifact_export", { artifactId: documentId }, CALL_MS);
        row.export = exported.isError ? `${exported.structuredContent?.code} ${String(exported.structuredContent?.message ?? "").slice(0, 160)}` : `ok ${exported.structuredContent?.format} ${exported.structuredContent?.contentBytes} B`;
      }
      const passed = row.create === "ok" && row.open === "ok" && String(row.export).startsWith("ok") && (row.mutate === "no-mutation-declared" || row.mutate === NOT_FOR_AGENTS || (row.mutate === "ok" && String(row.undo).startsWith("ok") && String(row.redo).startsWith("ok")));
      if (row.mutate === "no-mutation-declared") undeclared.push(artifactKind);
      if (row.mutate === NOT_FOR_AGENTS) shellOnly.push(artifactKind);
      if (!passed) failing.push(`${artifactKind}(${String(row.kindId)})`);
      appendFileSync(rowsHubPath, `${JSON.stringify(row)}\n`);
      table.push(`| ${row.kindId} | ${artifactKind} | ${row.create} | ${row.open ?? "-"} | ${row.openMs ?? "-"} | ${row.mutate ?? "-"} ${row.verb ? String(row.verb).split(".").at(-1) : ""} | ${row.undo ?? "-"} | ${row.redo ?? "-"} | ${String(row.export ?? "-").slice(0, 60)} |`);
      log(`hub ${row.kindId} create=${row.create} open=${row.open} (${row.openMs ?? "-"} ms) mutate=${row.mutate} undo=${row.undo ?? "-"} redo=${row.redo ?? "-"} export=${String(row.export ?? "-").slice(0, 60)}`);
    }
  } finally {
    session.stop();
    writeFileSync(join(outDir, "coverage-hub-gateway-stderr.txt"), `${session.stderrLines().join("\n")}\n`);
    await http("POST", `/auth/agent-delegations/${encodeURIComponent(String(delegation.json.delegationId))}/revoke`, token).catch(() => undefined);
  }
  const header = "| kind id | artifact kind | create | open | open ms | mutate | undo | redo | export |\n|---|---|---|---|---|---|---|---|---|";
  writeFileSync(join(outDir, "coverage-hub-table.md"), `${header}\n${table.join("\n")}\n`);
  console.log(`${header}\n${table.join("\n")}`);
  const passedCount = pending.length - failing.length;
  publishAcceptanceCheckResult(
    repoRoot,
    acceptanceCheckResult({
      check: "mcp-plugin-coverage-hub",
      status: pending.length > 0 && failing.length === 0 ? "pass" : "fail",
      startedAt,
      measured: { hub, kinds: pending.length, passed: passedCount, noMutationDeclared: undeclared.length, notForAgents: shellOnly.length },
      summary: {
        en: `${passedCount}/${pending.length} hub-creatable kinds created, opened, mutated, undone, redone and exported over the semio MCP (${undeclared.length} declare no mutation, ${shellOnly.length} n/a for agents: every verb is shell-only view state)${failing.length ? `; failing: ${failing.slice(0, 10).join(", ")}` : ""}`,
        de: `${passedCount}/${pending.length} am Hub anlegbare Arten über das semio-MCP angelegt, geöffnet, verändert, rückgängig gemacht, wiederhergestellt und exportiert (${undeclared.length} erklären keine Änderung, ${shellOnly.length} für Agenten nicht anwendbar: jedes Verb ist reiner Ansichtszustand der Oberfläche)${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 10).join(", ")}` : ""}`,
      },
      evidence: [rowsHubPath, join(outDir, "coverage-hub-table.md")],
    }),
  );
  if (failing.length > 0) process.exitCode = 1;
}
