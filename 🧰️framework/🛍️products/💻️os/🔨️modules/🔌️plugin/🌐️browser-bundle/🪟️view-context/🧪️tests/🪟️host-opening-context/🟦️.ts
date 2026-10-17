/** 🧪️ Verifies host-only context envelopes with a neutral schema and wire round trip. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { parseBrowserActorViewStateRequest } from "../../../../../../../../🔨️modules/🌐️browser-bundle/🪟️view-context/🟦️.ts";
import { encodeBackboneWorkerRequest, decodeBackboneWorkerRequest } from "../../../../../../../../🔨️modules/🛰️backbone/🧱️worker-wire/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import { manifestViewContextSchema as viewSchema } from "../../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🟦️.ts";
import fixture from "../🧫️fixtures/🪟️host-opening-context/🔣️.json";

export function testBrowserActorHostContext(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).addSchema(viewSchema).compile(schema);
  assert(validate(fixture.valid), JSON.stringify(validate.errors));
  const request = parseBrowserActorViewStateRequest(fixture.valid);
  assert.deepEqual(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)), fixture.valid);
  for (const row of fixture.invalid) {
    const value: Record<string, unknown> = structuredClone(fixture.valid);
    if (row.remove !== undefined) for (const key of row.remove) delete value[key];
    if ("set" in row) Object.assign(value, row.set);
    assert.equal(validate(value), false, row.name);
    assert.throws(() => parseBrowserActorViewStateRequest(value), row.name);
  }
}
