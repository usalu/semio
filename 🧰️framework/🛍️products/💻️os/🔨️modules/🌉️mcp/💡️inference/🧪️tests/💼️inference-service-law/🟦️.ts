/** 🧪️ Independent SHA-256, selection and state-vocabulary oracle for plain inference test cases. */
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
  const law = JSON.parse(readFileSync(resolve(repoRoot, INFERENCE_ROOT, "🧪️tests/🧫️fixtures/💼️inference-service-law.json"), "utf8")) as Law;
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema({ $id: "inference-component", $defs: schema.$defs });
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
  return { ajv: 2, selection: law.selection.length, proposals: law.proposals.length, lifecycles: law.lifecycles.length };
}

import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";

/** 🎟️ Independent schemas and SQLite compare a caller's full source against its distinct loan. */
test("original MCP inference caller keeps finite work and a bounded physical policy", async () => {
 const source = await Bun.file(new URL("../🧫️fixtures/original-caller.json", import.meta.url)).json();
 const component = await Bun.file(new URL("../../../🧬️schema/🔣️.json", import.meta.url)).json();
 const ajv = new Ajv({strict:false,allErrors:true});
 ajv.addSchema(component,"original-mcp");
 const turn = ajv.compile({$ref:"original-mcp#/$defs/InferenceRetainedTurnInput"});
 const policy = ajv.compile({$ref:"original-mcp#/$defs/InferenceRetainedAuthority"});
 expect(turn(source.turn)).toBe(true);expect(policy(source.retainedPolicy)).toBe(true);
 for(const key of ["operation","generation","epoch","grant","work"]) {
  const missing = {...source.turn};delete missing[key];expect(turn(missing)).toBe(false);
 }
 for(const key of ["clockBinding","absoluteDeadlineUs","fuel"]) {
  const work = {...source.turn.work};delete work[key];expect(turn({...source.turn,work})).toBe(false);
 }
 expect(turn({...source.turn,work:{...source.turn.work,extra:0}})).toBe(false);
 const db = new Database(":memory:");
 try {
  db.exec("CREATE TABLE source(axis TEXT PRIMARY KEY,total INTEGER CHECK(total>=0),loan INTEGER CHECK(loan>=0))");
  for(const key of Object.keys(source.retainedPolicy))db.query("INSERT INTO source VALUES(?,?,?)").run(key,source.turn.grant[key],source.retainedPolicy[key]);
  expect(db.query("SELECT count(*) AS count FROM source WHERE loan<=total").get()).toEqual({count:5});
  expect(db.query("SELECT total,loan FROM source WHERE axis='maximumItems'").get()).toEqual({total:64,loan:1});
  expect(db.query("SELECT total,loan FROM source WHERE axis='maximumCopyBytes'").get()).toEqual({total:4096,loan:3});
  expect(db.query("SELECT (?1<=?2) AS permitted,?2-?1 AS unused").get(source.workUnits,source.turn.work.fuel)).toEqual({permitted:1,unused:53});
  expect(db.query("SELECT (?1<=?2) AS permitted").get(source.turn.work.fuel+1,source.turn.work.fuel)).toEqual({permitted:0});
 }finally{db.close()}
});
