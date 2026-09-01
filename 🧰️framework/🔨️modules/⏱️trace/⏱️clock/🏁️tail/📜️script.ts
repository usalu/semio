/** 🏁️ Checks one-window admission/terminal expectations with independent BigInt arithmetic. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

//#region 🏁️TailOracle
export function testWatchdogTailFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const elapsed = (start: number | null, end: number | null): bigint | null => start === null || end === null || end < start ? null : BigInt(end) - BigInt(start);
  for (const row of fixture.cases) {
    const first = elapsed(row.clock[0], row.clock[1]);
    const monotonic = row.clock.every((value: number | null, index: number, clock: Array<number | null>) => value !== null && (index === 0 || clock[index - 1] !== null && value >= clock[index - 1]!));
    const last = monotonic ? elapsed(row.clock[0], row.clock[3]) : null;
    const refused = first === null || first >= BigInt(fixture.exclusiveCeilingUs);
    const fault = last === null || last >= BigInt(fixture.exclusiveCeilingUs);
    assert.equal(refused, row.admissionFault);
    assert.equal(fault, row.terminalFault);
    assert.equal(last === null ? null : Number(last), row.terminalElapsed);
    assert.equal(row.publication, refused ? "refused" : fault ? "already-committed-fault" : "allowed");
  }
  for (const hostile of [
    { ...fixture, exclusiveCeilingUs: 8001 },
    { ...fixture, scope: { ...fixture.scope, guards: 2 } },
    { ...fixture, scope: { ...fixture.scope, rollbackClaim: true } },
    { ...fixture, scope: { ...fixture.scope, terminalAfterTelemetry: false } },
    { ...fixture, scope: { ...fixture.scope, globalTelemetryAuthority: true } },
  ]) assert.equal(validate(hostile), false);
  console.log(`[DEBUG] watchdog tail oracle: ${fixture.cases.length} same-window vectors, 5 schema hostiles; actual WGPU publication and native guard execution remain separate`);
}
//#endregion 🏁️TailOracle
