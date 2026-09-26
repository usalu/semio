/** 🧩️ The plugin-coverage lane-parity sweep: every installed plugin package over the semio MCP.
 *
 * One fresh `semio-os-mcp stdio --folder <temporary>` gateway (the staged binary, auto-approving, no bridge). Per plugin
 * package: its capabilities (`capabilities_search` by owner, paginated) described one by one, how many are mutations,
 * destructive, carry an empty description or declare no arguments, its declared inferences (`inference_list`), and for
 * every installed artifact kind of the package `artifact_create` plus ONE non-destructive mutation through
 * `action_prepare` → `action_invoke` whose required inputs the declared input schema can fill (default, first enum
 * value, or a typed neutral value). A kind passes when it is created and one mutation SUCCEEDS; the sweep passes when
 * every kind does. A gateway that stops answering is respawned and the call recorded as `CLIENT_TIMEOUT`.
 *
 * Configuration by environment: `S_OS_MCP_COVERAGE_PLUGINS` (comma list, default every installed plugin),
 * `S_OS_MCP_COVERAGE_OUT` (default `🌉️mcp/🤖️generated/🧩️plugin-coverage`). Rows: `coverage-rows.jsonl`, table:
 * `coverage-table.md`. Promoted from the session-12 ticket harness `wp-g10/g10-plugin-coverage.ts` (ticket 26/09/23, G10 S3).
 */
import { appendFileSync, existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { requireMcpBinary, spawnRawMcp } from "../../🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const here = dirname(fileURLToPath(new URL(import.meta.url)));
function findRepoRoot(start: string): string {
  for (let current = start, depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error(`the plugin-coverage sweep could not locate the repository root above ${start}`);
}
const repoRoot = findRepoRoot(here);
const outDir = process.env.S_OS_MCP_COVERAGE_OUT ?? join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️generated/🧩️plugin-coverage");
const only = new Set((process.env.S_OS_MCP_COVERAGE_PLUGINS ?? "").split(",").filter(Boolean));
const CALL_MS = 240_000;
const CREATE_MS = 900_000;
const startedAt = new Date();
mkdirSync(outDir, { recursive: true });
const rowsPath = join(outDir, "coverage-rows.jsonl");
writeFileSync(rowsPath, "");
const log = (line: string): void => console.log(`${new Date().toISOString()} ${line}`);
const gatewayArgs = (): string[] => ["stdio", "--folder", mkdtempSync(join(tmpdir(), "semio-mcp-coverage-")), "--scopes", "workspace.read,artifact.write,inference.execute", "--no-bridge", "--auto-approve", "all"];

type CallAnswer = { ok: boolean; value: any; ms: number };

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
  for (const capability of capabilities) described.push((await call("capabilities_describe", { capabilityId: capability.capabilityId })).value?.capability);
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
        .filter((capability) => capability?.artifactKind === kind && capability?.effects?.destructive !== true)
        .sort((left, right) => Number((right?.presentation?.args ?? []).length > 0) - Number((left?.presentation?.args ?? []).length > 0));
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
          Object.assign(row, { invoke: "ok", invokeVerb: capability.id, invokeMs: done.ms });
          break;
        }
        attempts.push(`${capability.id}: invoke ${done.value?.status ?? done.value?.code} ${String(done.value?.message ?? "").slice(0, 120)}`);
      }
      if (row.invoke !== "ok") Object.assign(row, { invoke: candidates.length === 0 ? "no-candidate" : "failed", invokeAttempts: attempts });
    }
    if (!(row.create === "ok" && row.invoke === "ok")) failing.push(`${kind}:${String(row.create)}/${String(row.invoke ?? "-")}`);
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
const status = !aborted && kindsTotal > 0 && kindsInvoked === kindsTotal ? "pass" : "fail";
publishAcceptanceCheckResult(
  repoRoot,
  acceptanceCheckResult({
    check: "mcp-plugin-coverage",
    status,
    startedAt,
    measured: { plugins: pluginIds.length, kinds: kindsTotal, created: kindsCreated, invoked: kindsInvoked, cancelled: aborted },
    summary: {
      en: `${kindsInvoked}/${kindsTotal} installed kinds created and mutated over the semio MCP (${kindsCreated} created)${failing.length ? `; failing: ${failing.slice(0, 8).join(", ")}` : ""}`,
      de: `${kindsInvoked}/${kindsTotal} installierte Arten über das semio-MCP angelegt und verändert (${kindsCreated} angelegt)${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 8).join(", ")}` : ""}`,
    },
    evidence: [rowsPath, join(outDir, "coverage-table.md")],
  }),
);
process.exit(status === "pass" ? 0 : 1);
