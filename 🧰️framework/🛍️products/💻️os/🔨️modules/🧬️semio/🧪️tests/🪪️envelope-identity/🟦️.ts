/** 🧪️ Exact OS envelope admission agrees with an independent JSON Schema oracle. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { matchesSemioEnvelope, type SemioEnvelope } from "../../🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️envelope-identity/🔣️.json" with { type: "json" };

export function testSemioEnvelopeIdentity(): void {
  const expected = vectors.expected;
  const [plugin, artifact] = expected.id.split(".");
  const validate = new Ajv({ strict: true }).compile({
    type: "object", additionalProperties: false, required: ["plugin", "artifact", "component", "version"],
    properties: { plugin: { const: plugin }, artifact: { const: artifact }, component: { const: expected.component }, version: { const: expected.version } },
  });
  for (const item of vectors.cases) {
    assert.equal(validate(item.envelope), item.matches, item.name);
    assert.equal(matchesSemioEnvelope(item.envelope as SemioEnvelope, expected.id, expected.component as SemioEnvelope["component"], expected.version), item.matches, item.name);
  }
  console.log(`[DEBUG] Semio exact owner/component/version admission agrees with ${vectors.cases.length} independent Ajv vectors`);
}
