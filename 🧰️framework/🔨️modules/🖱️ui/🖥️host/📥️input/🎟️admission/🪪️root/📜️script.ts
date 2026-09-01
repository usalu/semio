/** 🪪️ Checks input-root arithmetic and exact u64 bytes independently of native minting. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

//#region 🪪️RootIdentityOracle
export function testInputRootFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const parent = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).addSchema(parent).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const maximum = (1n << 64n) - 1n;
  assert.equal(new Set(fixture.cases.map((row: { name: string }) => row.name)).size, fixture.cases.length);
  for (const row of fixture.cases) {
    const before = BigInt(row.before), current = BigInt(row.atCas);
    const outcome = row.grant !== "admitted" ? "blocked" : before === maximum ? "exhausted" : current !== before ? "busy" : "admitted";
    const root = outcome === "admitted" ? before + 1n : null;
    const after = root ?? current;
    assert.equal(outcome, row.outcome, row.name);
    assert.equal(after.toString(), row.after, row.name);
    assert.equal(root?.toString() ?? null, row.root, row.name);
    if (root !== null) {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64LE(root);
      assert.equal(bytes.toString("hex"), row.rootLeHex, row.name);
      assert.equal(bytes.readBigUInt64LE(), root);
    } else assert.equal(row.rootLeHex, null);
  }
  const reuse = fixture.reuse;
  assert.equal(BigInt(reuse.initial) + 1n, BigInt(reuse.firstRoot));
  assert.equal(BigInt(reuse.firstRoot) + 1n, BigInt(reuse.secondRoot));
  assert.equal(reuse.firstEpoch, reuse.secondEpoch);
  assert.equal(reuse.firstRoot === reuse.secondRoot && reuse.firstEpoch === reuse.secondEpoch, reuse.oldKeyAccepted);
  assert.equal(BigInt(fixture.failureAfterInstall.retainedRoot) + 1n, BigInt(fixture.failureAfterInstall.nextRoot));
  assert.equal(fixture.concurrent.workers * fixture.concurrent.attemptsPerWorker, fixture.concurrent.attempts);
  assert.equal(fixture.concurrent.minimumSuccesses, 1);
  assert.equal(fixture.concurrent.maximumSuccesses, fixture.concurrent.attempts);
  assert.equal(fixture.concurrent.successfulRootsUnique, true);
  for (const invalid of ["-1", "01", "18446744073709551616"]) {
    const hostile = structuredClone(fixture);
    hostile.cases[0].before = invalid;
    assert.equal(validate(hostile), false, invalid);
  }
  assert.equal(validate({ ...fixture, storage: { ...fixture.storage, maximumCasAttempts: 2 } }), false);
  assert.equal(validate({ ...fixture, scope: "distributed-wire-identity" }), false);
  assert.equal(validate({ ...fixture, extra: true }), false);
  console.log(`[DEBUG] input root oracle: ${fixture.cases.length} arithmetic vectors, 6 schema hostiles; native CAS, concurrency, queue ownership and allocation remain separate`);
}
//#endregion 🪪️RootIdentityOracle
