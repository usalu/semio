import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export function testFemAssemblyPhysicalOwners(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...structuredClone(fixture), foreignOwner: true }));
  for (const [owner, maximum] of Object.entries(fixture.mountedModel.logicalLimits)) {
    const admits = new Ajv2020({ strict: true }).compile({ type: "integer", minimum: 0, maximum });
    assert(!admits(maximum + 1));
    for (const target of fixture.mountedModel.targets) {
      const count = target === "empty" ? 0 : target === "one" ? 1 : maximum;
      const before = { owner, admitted: 0, initialized: 0 };
      const patch: Operation[] = [{ op: "replace", path: "/admitted", value: count }];
      const after = { owner, admitted: count, initialized: 0 };
      assert.deepEqual(applyPatch(structuredClone(before), patch, true, false).newDocument, after);
      assert(admits(count));
      assert.deepEqual(applyPatch(structuredClone(after), [], true, false).newDocument, after, "maximum-plus-one rejection preserves logical ownership");
    }
  }
  for (const owner of fixture.inlineOwners) {
    const before = { owner, live: true, releasedBytes: 0 };
    const patch: Operation[] = [{ op: "replace", path: "/live", value: false }];
    assert.deepEqual(applyPatch(structuredClone(before), patch, true, false).newDocument, { owner, live: false, releasedBytes: fixture.release.inlineOwnerBytes });
  }
  for (const row of fixture.cases) {
    const capacities = [32, 64, 128].slice(0, row.backingCount);
    for (const grant of fixture.grants) {
      const nextBytes = capacities[0] ?? 0;
      const bytes = grant.relativeBytes === "zero" ? 0 : grant.relativeBytes === "one-less" ? Math.max(nextBytes - 1, 0) : nextBytes;
      const released = capacities.length > 0 && bytes >= nextBytes ? 1 : 0;
      const before = { backings: capacities, releasedBytes: 0 };
      const after = { backings: capacities.slice(released), releasedBytes: released === 1 ? nextBytes : 0 };
      const patch: Operation[] = released === 1 ? [{ op: "remove", path: "/backings/0" }, { op: "replace", path: "/releasedBytes", value: nextBytes }] : [];
      assert.deepEqual(applyPatch(structuredClone(before), patch, true, false).newDocument, after);
      assert.equal(released, capacities.length > 0 ? grant.releasedBackings : 0);
      assert.equal(after.releasedBytes + after.backings.reduce((sum, value) => sum + value, 0), capacities.reduce((sum, value) => sum + value, 0));
    }
  }
  console.log(`[DEBUG] FEM assembly physical owners: ${fixture.inlineOwners.length} inline owners, ${fixture.cases.length} backing cases and ${fixture.grants.length} grants match fast-json-patch ${fixture.oracle.version}`);
}

if (import.meta.main) testFemAssemblyPhysicalOwners();
