/** 🧪️ Verifies host-only context envelopes with a neutral schema and wire round trip. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { parseBrowserActorViewStateRequest } from "../../🟦️.ts";
import { encodeBackboneWorkerRequest, decodeBackboneWorkerRequest } from "../../../../../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import viewSchema from "../../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

export function testBrowserActorHostContext(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).addSchema(viewSchema).compile(schema);
  assert(validate(fixture.valid), JSON.stringify(validate.errors));
  const request = parseBrowserActorViewStateRequest(fixture.valid);
  assert.deepEqual(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)), fixture.valid);
  for (const row of fixture.invalid) {
    const value: Record<string, unknown> = structuredClone(fixture.valid);
    if ("remove" in row) for (const key of row.remove) delete value[key];
    if ("set" in row) Object.assign(value, row.set);
    assert.equal(validate(value), false, row.name);
    assert.throws(() => parseBrowserActorViewStateRequest(value), undefined, row.name);
  }
  console.log(`[DEBUG] browser-actor-host-context cases=${fixture.invalid.length + 1} schema=valid wire=round-trip`);
}
