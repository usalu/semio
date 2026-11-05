/** 🧪️ Neutral IO descriptors agree with owned parsers and the independent Ajv schema oracle. */
import { artifactReferenceSchema } from "@semio-tech/framework/schema/artifact-reference";
import assert from "node:assert/strict";
import Ajv from "ajv";
import { ioSchema as schema } from "@semio-tech/framework/io/schema";
import { parseIoEntryDescriptor } from "@semio-tech/framework/io/schema";
import fixture from "../🧫️fixtures/📇️descriptor-parity.json" with { type: "json" };
import { ioEntries } from "../../🟦️.ts";

export function testArtifactIoDescriptorParity(): number {
  const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(artifactReferenceSchema);
  ajv.addKeyword({ keyword: "x-semio-ui", valid: true });
  ajv.addSchema(schema);
  const descriptor = ajv.compile({ $ref: `${schema.$id}#/$defs/IoEntryDescriptor` });
  let checks = 0;
  assert.deepEqual(ioEntries, fixture.entries);
  checks++;
  for (const entry of fixture.entries) {
    assert.equal(descriptor(entry), true, JSON.stringify(descriptor.errors));
    assert.deepEqual(parseIoEntryDescriptor(entry), entry);
    checks += 2;
  }
  for (const entry of fixture.invalidEntries) {
    assert.equal(descriptor(entry), false, JSON.stringify(entry));
    assert.throws(() => parseIoEntryDescriptor(entry));
    checks += 2;
  }
  for (const pair of fixture.entries) {
    assert.equal(ioEntries.filter(entry => JSON.stringify(entry.from) === JSON.stringify(pair.from) && JSON.stringify(entry.into) === JSON.stringify(pair.into)).length, 1);
    checks++;
  }
  for (const pair of fixture.unsupportedPairs) {
    assert.equal(ioEntries.some(entry => JSON.stringify(entry.from) === JSON.stringify(pair.from) && JSON.stringify(entry.into) === JSON.stringify(pair.into)), false);
    checks++;
  }
  for (const mismatch of fixture.mismatches) {
    const stale = { ...ioEntries[mismatch.index], [mismatch.field]: mismatch.value };
    assert.equal(descriptor(stale), true);
    assert.notDeepEqual(stale, fixture.entries[mismatch.index]);
    checks += 2;
  }
  console.log(`Artifact IO descriptor parity checks=${checks}, entries=${ioEntries.length}`);
  return checks;
}

if (import.meta.main) testArtifactIoDescriptorParity();
