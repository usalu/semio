import { existsSync as originalCorpusAuthorityExists } from "node:fs";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import corpus from "../../🧫️fixtures/⏱️finite.json";
import policySchema from "../../🧬️schema/🔣️.json";
import envelopeSchema from "../../🏃️process/🧬️schema/🔣️.json";
import { checkScriptInvocation, type ScriptProgress } from "../../🟦️.ts";
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
