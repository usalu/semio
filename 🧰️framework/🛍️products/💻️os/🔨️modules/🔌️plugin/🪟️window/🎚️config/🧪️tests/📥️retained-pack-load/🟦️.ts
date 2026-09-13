import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020";
import schema from "../../🧬️schema/📥️retained-pack-load/🔣️.json";
import fixture from "../../🧫️fixtures/📥️retained-pack-load/🔣️.json";

/** 📥️ The retained-load native gate consumes one closed, schema-first lifecycle fixture. */
export function testWindowConfigRetainedPackLoadFixture(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  assert.deepEqual(fixture.owners.map((owner) => owner.windowIds.length), [2, 1]);
  assert.deepEqual(fixture.candidateProtocol.phases, [
    "ingress",
    "decoding",
    "validating-identity",
    "preparing-store",
    "ready",
    "cancelling",
    "retiring-rejected-candidate",
    "retiring-displaced-store",
    "complete",
  ]);
  assert.equal(fixture.candidateProtocol.typedReplayInput, "retained-value-tokens");
  assert.equal(fixture.candidateProtocol.identityChecks.length, 8);
  assert.deepEqual(fixture.candidateProtocol.closeOrder, ["pending-event", "logical-owners", "allocation-demands", "terminal-empty"]);
  assert.equal(new Set(fixture.requiredScenarios.map((scenario) => scenario.id)).size, fixture.requiredScenarios.length);
  assert.deepEqual(
    new Set(fixture.requiredScenarios.map((scenario) => scenario.id)),
    new Set([
      "missing-envelope",
      "wrong-component",
      "wrong-state-schema",
      "wrong-version",
      "cancel-during-decode",
      "cancel-ready-candidate",
      "publication-load-race",
      "same-kind-and-cross-kind-reopen",
      "app-close",
    ]),
  );
  console.log(`[DEBUG] Window retained-load fixture: owners=${fixture.owners.length} packs=${fixture.owners.flatMap((owner) => owner.windowIds).length} requiredScenarios=${fixture.requiredScenarios.length}`);
}
