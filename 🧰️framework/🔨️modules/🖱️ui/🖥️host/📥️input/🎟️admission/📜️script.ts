/** 🧪️ Verifies neutral input admission expectations with strict Ajv, Node Buffer and BigInt. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import { testInputRootFixture } from "./🪪️root/📜️script.ts";
import { testInputWriterFixture } from "./✍️writer/📜️script.ts";
import { testInputCommitObserverFixture } from "./🔗️commit/📜️script.ts";
import { testWatchdogTailFixture } from "../../../../⏱️trace/⏱️clock/🏁️tail/📜️script.ts";

//#region 🧪️InputAdmission
export function testInputAdmissionFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(new Set(fixture.cases.map((row: { name: string }) => row.name)).size, fixture.cases.length);
  assert.equal(Buffer.from(fixture.ownedEvent.text, "utf8").toString("hex"), fixture.ownedEvent.utf8Hex);
  const maximum = (1n << 64n) - 1n;
  const backing = Buffer.alloc(fixture.physicalRetirement.payloadMinimumCapacity);
  Buffer.from(fixture.ownedEvent.text).copy(backing);
  let logical = backing.subarray(0, fixture.physicalRetirement.logicalBytes);
  assert(backing.byteLength > logical.byteLength);
  for (const [index, grant] of fixture.physicalRetirement.logicalGrants.entries()) {
    logical = logical.subarray(Math.min(grant, logical.byteLength));
    assert.equal(logical.byteLength, fixture.physicalRetirement.remaining[index]);
    assert.equal(logical.buffer, backing.buffer);
    assert.equal(backing.byteLength, fixture.physicalRetirement.payloadMinimumCapacity);
  }
  assert.equal(validate({ ...fixture, physicalRetirement: { ...fixture.physicalRetirement, terminalRequiresEmptyBacking: false } }), false);
  for (const row of fixture.cases) {
    const missing = row.startUs === null || row.finishUs === null;
    const elapsed = missing ? null : BigInt(row.finishUs) - BigInt(row.startUs);
    const fault = elapsed === null || elapsed < 0n || elapsed >= BigInt(fixture.limits.exclusiveCallbackCeilingUs) || row.receiver === "mailboxPoisoned";
    const exhausted = BigInt(row.frameGeneration) === maximum || BigInt(row.inputGeneration) === maximum;
    const outcome = fault ? "fault" : row.cancelled ? "cancelled" : exhausted ? "exhausted" : row.grant === "zero" || row.receiver !== "available" ? "blocked" : "accepted";
    const accepted = outcome === "accepted";
    assert.deepEqual(row.expected, { outcome, sourcePreserved: !accepted, eventCommits: Number(accepted), surfaceCommits: Number(accepted && row.kind === "metrics"), mailboxCommits: Number(accepted && row.kind === "metrics"), generationDelta: Number(accepted) }, row.name);
    const before = Buffer.from(row.frameGeneration);
    const after = accepted ? Buffer.from((BigInt(row.frameGeneration) + 1n).toString()) : before;
    assert.equal(BigInt(after.toString()) - BigInt(before.toString()), BigInt(row.expected.generationDelta));
    if (!accepted) assert.equal(after, before);
  }
  for (const invalid of ["-1", "01", "18446744073709551616", "99999999999999999999"]) {
    const hostile = structuredClone(fixture);
    hostile.cases[0].inputGeneration = invalid;
    assert.equal(validate(hostile), false, invalid);
  }
  assert.equal(validate({ ...fixture, extra: true }), false);
  assert.equal(validate({ ...fixture, limits: { ...fixture.limits, exclusiveCallbackCeilingUs: 8001 } }), false);
  console.log(`[DEBUG] input admission oracle: ${fixture.cases.length} neutral cases, 7 schema hostiles, 3 logical-close frontiers over retained 64-byte backing; native ownership and actual Watchdog execution remain separate`);
  testInputRootFixture();
  testInputWriterFixture();
  testInputCommitObserverFixture();
  testWatchdogTailFixture();
}
//#endregion 🧪️InputAdmission
