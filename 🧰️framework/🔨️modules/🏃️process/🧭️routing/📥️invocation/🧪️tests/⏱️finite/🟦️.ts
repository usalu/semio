import { existsSync as originalCorpusAuthorityExists } from "node:fs";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/⏱️finite.json";
import policySchema from "../../🧬️schema/🔣️.json";
import envelopeSchema from "../../🏃️process/🧬️schema/🔣️.json";
import { advanceScriptInvocation, checkScriptInvocation, readScriptPolicy, type ScriptProgress } from "../../🟦️.ts";
import { compileJsonSchemaSubset, validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { withScriptProcessEnvelope, type ScriptProcessEnvelope } from "../../🏃️process/🟦️.ts";

test("actual finite parent admission refuses null before effect and preserves explicit ordinary hosts", async () => {
  expect(originalCorpusAuthorityExists(new URL("../../\ud83e\uddec\ufe0fschema/\u23f1\ufe0ffinite.json", import.meta.url))).toBe(false);

  const admits = new Ajv({ strict: true }).addSchema(policySchema).compile(envelopeSchema);
  const oracle = Bun.spawnSync(["node", "--eval", 'const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(row=>row.maximumElapsedMilliseconds===0?row.deadlineOffsetMilliseconds===null:Number.isSafeInteger(row.maximumElapsedMilliseconds)&&row.maximumElapsedMilliseconds>0&&Number.isSafeInteger(row.deadlineOffsetMilliseconds)&&row.deadlineOffsetMilliseconds>0&&row.deadlineOffsetMilliseconds<=row.maximumElapsedMilliseconds)))', JSON.stringify(corpus.cases)]);
  expect(oracle.exitCode).toBe(0); expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(corpus.cases.map(row => row.accepted));
  for (const row of corpus.cases) {
    const started = Date.now(), policy = { version: 1 as const, owner: row.id, maximumElapsedMilliseconds: row.maximumElapsedMilliseconds }, capabilities = { owner: row.id, command: "original-parent" };
    const envelope: ScriptProcessEnvelope = { version: 1, policy, deadlineEpochMilliseconds: row.deadlineOffsetMilliseconds === null ? null : started + row.deadlineOffsetMilliseconds, capabilities };
    expect(admits(envelope), row.id).toBe(row.accepted);
    let effects = 0;
    const receive = () => withScriptProcessEnvelope(envelope, async original => {
      effects++; expect(original.policy).toBe(policy); expect(original.capabilities).toBe(capabilities);
      expect(original.control.signal.aborted).toBe(false);
      const remaining = original.control.remainingMilliseconds();
      if (row.maximumElapsedMilliseconds > 0) { expect(remaining).not.toBeNull(); expect(remaining).toBeGreaterThan(0); expect(remaining!).toBeLessThanOrEqual(row.deadlineOffsetMilliseconds!); }
      else expect(remaining).toBeNull();
      await original.control.publish({ owner: policy.owner, command: "original-parent", stage: "running" });
      await original.control.yieldContinuation();
    });
    if (row.accepted) await receive(); else await expect(receive()).rejects.toThrow();
    expect(effects, row.id).toBe(row.accepted ? 1 : 0);
  }
  const signal = new AbortController(), progress: ScriptProgress[] = [], remainingMilliseconds = () => 512.5;
  const original = { policy: { version: 1 as const, owner: "original-fractional-finite-caller", maximumElapsedMilliseconds: 1000 }, control: { signal: signal.signal, remainingMilliseconds, publish: async (event: ScriptProgress) => { progress.push(event); }, yieldContinuation: () => new Promise<void>(done => setImmediate(done)) }, capabilities: { events: progress } };
  checkScriptInvocation(original); expect(original.control.remainingMilliseconds).toBe(remainingMilliseconds); expect(original.control.remainingMilliseconds()).toBe(512.5);
  console.log("[DEBUG] Canonical actual finite-policy/null handoff refused before effect; exact policy/capability identities, explicit ordinary host and fractional incoming clock retained; independent Ajv/Node agree");
}, 15000);

test("prepared original policy validates every mutable value with independent Ajv and real compiler", () => {
  const fixture = corpus.preparedPolicy, oracle = new Ajv({ strict: true }).compile(policySchema), policy: Record<string, unknown> = { ...fixture.initial };
  expect(readScriptPolicy(policy)).toBe(policy);
  for (const row of fixture.mutations) {
    if ("remove" in row) delete policy[row.key]; else policy[row.key] = row.value;
    expect(oracle(policy)).toBe(row.accepted);
    if (row.accepted) expect(readScriptPolicy(policy)).toBe(policy); else expect(() => readScriptPolicy(policy)).toThrow("Complete Script policy required");
  }
  let schemaReads = 0;
  const observed = new Proxy(policySchema, { get(target, key, receiver) { schemaReads++; return Reflect.get(target, key, receiver); }, ownKeys(target) { schemaReads++; return Reflect.ownKeys(target); } });
  const prepared = compileJsonSchemaSubset(observed), initialReads = schemaReads;
  expect(initialReads).toBeGreaterThan(0);
  for (const row of fixture.controls) expect(prepared.validate({ ...fixture.initial, maximumElapsedMilliseconds: row.maximum }).length).toBe(0);
  expect(schemaReads).toBe(initialReads);
  for (const row of fixture.controls) validateJsonSchemaSubset(observed, { ...fixture.initial, maximumElapsedMilliseconds: row.maximum });
  expect(schemaReads).toBeGreaterThan(initialReads);
  const mutable = { type: "string" }, snapshot = compileJsonSchemaSubset(mutable);
  mutable.type = "number";
  expect(validateJsonSchemaSubset(mutable, 7)).toEqual([]);
  expect(validateJsonSchemaSubset(mutable, "value").length).toBeGreaterThan(0);
  expect(snapshot.validate("value")).toEqual([]);
  expect(snapshot.validate(7).length).toBeGreaterThan(0);
});

test("prepared original policy preserves every awaited control boundary", async () => {
  const fixture = corpus.preparedPolicy;
  const oracle = Bun.spawnSync(["node", "--eval", 'const r=JSON.parse(process.argv[1]);console.log(JSON.stringify(r.map(v=>!v.abort&&(v.maximum===0?v.remaining===null:typeof v.remaining==="number"&&Number.isFinite(v.remaining)&&v.remaining>0&&v.remaining<=v.maximum))))', JSON.stringify(fixture.controls)]);
  expect(oracle.exitCode).toBe(0); expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(fixture.controls.map(row => row.accepted));
  for (const row of fixture.controls) {
    const controller = new AbortController(); if ("abort" in row) controller.abort(Error(row.id));
    const invocation = { policy: { ...fixture.initial, version: 1 as const, maximumElapsedMilliseconds: row.maximum }, control: { signal: controller.signal, remainingMilliseconds: () => row.remaining, publish: async () => {}, yieldContinuation: async () => {} }, capabilities: {} };
    if (row.accepted) expect(() => checkScriptInvocation(invocation)).not.toThrow(); else expect(() => checkScriptInvocation(invocation)).toThrow();
  }
  for (const row of fixture.advances) {
    const controller = new AbortController(), policy = { ...fixture.initial, version: 1 as const }, events: string[] = [], capabilities = { id: row.id }; let remaining = 750, checks = 0;
    const control = { signal: controller.signal, remainingMilliseconds: () => { checks++; return remaining; }, publish: async (progress: ScriptProgress) => { expect(progress.owner).toBe(policy.owner); events.push("publish:start"); await new Promise<void>(done => setImmediate(done)); if (row.change === "publish-policy") policy.owner = ""; if (row.change === "publish-cancel") controller.abort(Error(row.id)); events.push("publish:end"); }, yieldContinuation: async () => { events.push("yield:start"); await new Promise<void>(done => setImmediate(done)); if (row.change === "yield-policy") policy.owner = ""; if (row.change === "yield-expiry") remaining = 0; events.push("yield:end"); } };
    const invocation = { policy, control, capabilities }, advance = advanceScriptInvocation(invocation, row.id, "running");
    if (row.accepted) await advance; else await expect(advance).rejects.toThrow();
    expect(events).toEqual(row.events); expect(checks).toBe(row.checks); expect(invocation.policy).toBe(policy); expect(invocation.control).toBe(control); expect(invocation.capabilities).toBe(capabilities);
  }
});

test("prepared original policy owns one module schema preparation without a generic read compiler", () => {
  const typescript = createRequire(new URL("../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json", import.meta.url))("typescript"), source = typescript.createSourceFile("invocation.ts", readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8"), typescript.ScriptTarget.Latest, true), preparations: any[] = [];
  for (const statement of source.statements) if (typescript.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (declaration.initializer && typescript.isCallExpression(declaration.initializer) && declaration.initializer.expression.getText(source) === "compileJsonSchemaSubset") preparations.push(declaration);
  expect(preparations.length).toBe(corpus.preparedPolicy.topLevelPreparations);
  const preparation = preparations[0], read = source.statements.find((node: any) => typescript.isFunctionDeclaration(node) && node.name?.text === "readScriptPolicy"), calls: any[] = [];
  expect(preparation.initializer.arguments.map((node: any) => node.getText(source))).toEqual(["schema"]);
  const visit = (node: any) => { if (typescript.isCallExpression(node)) calls.push(node); typescript.forEachChild(node, visit); }; visit(read);
  expect(calls.some(node => node.expression.getText(source) === `${preparation.name.getText(source)}.validate` && node.arguments[0]?.getText(source) === "value")).toBe(true);
  expect(calls.some(node => ["compileJsonSchemaSubset", "validateJsonSchemaSubset"].includes(node.expression.getText(source)))).toBe(false);
});
