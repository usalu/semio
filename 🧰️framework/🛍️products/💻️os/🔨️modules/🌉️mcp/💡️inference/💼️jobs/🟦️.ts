/** 💼️ Independent Bun/AJV oracle for the one inference service model (`💼️jobs/🦀️.rs`). It reads the
 * language-agnostic law `🧫️fixtures/💼️inference-service-law.json` and the scope's own
 * `🧬️schema/🔣️.json`, never the Rust implementation:
 *
 * 1. AJV compiles `InferenceServiceLawV1` and the fixture must satisfy it;
 * 2. every expected proposal digest is recomputed here — SHA-256 over key-sorted canonical JSON of
 *    `{domain, serviceId, documentId, capabilityId, input}` — and every expected `input` is re-derived
 *    as exactly the result fields the action declares;
 * 3. every selection row is re-decided from the declared roster and the published hub services;
 * 4. every lifecycle's expected pages are held against `InferenceJobPageV1`'s state vocabulary. */
import Ajv from "ajv";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const INFERENCE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference";
const PROPOSAL_DOMAIN = "semio.mcp.inference-proposal/v1";

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type Declared = { artifactKind: string; inferenceSchema: string; contributor: string; owner: string; payload?: { commit?: { action: string } } };
type Law = {
  declared: Declared[];
  selection: { name: string; artifactKind: string; inferenceSchema?: string; pluginId?: string; hubServices: { serviceId: string; route: string }[]; expect?: Record<string, string>; refusal?: { code: string; field?: string } }[];
  proposals: { name: string; serviceId: string; documentId: string; action: string; capabilityId: string; declaredArgs: string[]; result: Json; expect?: { input: Json; hash: string }; refusal?: { code: string } }[];
  lifecycles: { name: string; steps: { op: string; expect: { state: string; proposalState: string } }[] }[];
};

/** 🧾️ Key-sorted, whitespace-free JSON — the byte form a proposal digest is taken over. */
export const canonicalJson = (value: Json): string => {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(",")}]`;
  if (value !== null && typeof value === "object") return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalJson(value[key] as Json)}`).join(",")}}`;
  return JSON.stringify(value);
};

const fail = (message: string): never => {
  throw new Error(`inference-service-law: ${message}`);
};

/** 🧭️ The selection rule, re-stated from the law's own words. */
const select = (law: Law, row: Law["selection"][number]): { expect?: Record<string, string>; refusal?: { code: string; field?: string } } => {
  const onKind = law.declared.filter((item) => item.artifactKind === row.artifactKind);
  const matches = onKind.filter((item) => (row.inferenceSchema === undefined || item.inferenceSchema === row.inferenceSchema) && (row.pluginId === undefined || (item.contributor || item.owner) === row.pluginId));
  if (matches.length === 0) return { refusal: { code: "NOT_FOUND" } };
  if (matches.length > 1) return { refusal: { code: "INPUT_INVALID", field: new Set(matches.map((item) => item.inferenceSchema)).size > 1 ? "inferenceSchema" : "pluginId" } };
  const item = matches[0]!;
  const hub = row.hubServices.find((service) => service.serviceId === item.inferenceSchema);
  const expect: Record<string, string> = { serviceId: item.inferenceSchema, pluginId: item.contributor || item.owner, site: hub ? "hub" : "guest" };
  if (hub) expect.route = hub.route;
  if (item.payload?.commit) expect.commitAction = item.payload.commit.action;
  return { expect };
};

export function proveInferenceServiceLaw(repoRoot: string): { ajv: number; selection: number; proposals: number; lifecycles: number } {
  const schema = JSON.parse(readFileSync(resolve(repoRoot, INFERENCE_ROOT, "🧬️schema/🔣️.json"), "utf8")) as { $defs: Record<string, unknown> };
  const law = JSON.parse(readFileSync(resolve(repoRoot, INFERENCE_ROOT, "🧫️fixtures/💼️inference-service-law.json"), "utf8")) as Law;
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema({ $id: "inference-component", $defs: Object.fromEntries(Object.entries(schema.$defs).filter(([name]) => name !== "InferenceBindingCancellationLawV1")) });
  const validateLaw = ajv.compile({ $ref: "inference-component#/$defs/InferenceServiceLawV1" });
  if (!validateLaw(law)) fail(`fixture violates InferenceServiceLawV1: ${ajv.errorsText(validateLaw.errors)}`);
  const jobState = ajv.compile({ $ref: "inference-component#/$defs/InferenceJobStateV1" });
  const proposalState = ajv.compile({ $ref: "inference-component#/$defs/InferenceProposalStateV1" });

  for (const row of law.selection) {
    const decided = select(law, row);
    if (JSON.stringify(decided.expect ?? null) !== JSON.stringify(row.expect ?? null) || JSON.stringify(decided.refusal ?? null) !== JSON.stringify(row.refusal ?? null))
      fail(`selection "${row.name}" re-decides to ${JSON.stringify(decided)}`);
  }
  for (const row of law.proposals) {
    if (row.result === null || typeof row.result !== "object" || Array.isArray(row.result)) {
      if (row.refusal?.code !== "PRECONDITION_FAILED") fail(`proposal "${row.name}" offers a non-object result`);
      continue;
    }
    const result = row.result;
    const input: Record<string, Json> = Object.fromEntries(Object.entries(result).filter(([key]) => row.declaredArgs.includes(key)));
    const hash = createHash("sha256").update(canonicalJson({ domain: PROPOSAL_DOMAIN, serviceId: row.serviceId, documentId: row.documentId, capabilityId: row.capabilityId, input })).digest("hex");
    if (canonicalJson(input) !== canonicalJson(row.expect!.input)) fail(`proposal "${row.name}" input re-derives to ${canonicalJson(input)}`);
    if (hash !== row.expect!.hash) fail(`proposal "${row.name}" digest re-derives to ${hash}`);
  }
  for (const lifecycle of law.lifecycles) {
    for (const step of lifecycle.steps) {
      if (!jobState(step.expect.state) || !proposalState(step.expect.proposalState)) fail(`lifecycle "${lifecycle.name}" step ${step.op} leaves the page vocabulary`);
    }
  }
  return { ajv: 3, selection: law.selection.length, proposals: law.proposals.length, lifecycles: law.lifecycles.length };
}
