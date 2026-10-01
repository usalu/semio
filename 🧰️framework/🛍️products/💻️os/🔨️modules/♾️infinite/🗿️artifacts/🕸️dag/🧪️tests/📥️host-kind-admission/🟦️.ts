import { readFileSync } from "node:fs";
import { strict as assert } from "node:assert";
import Ajv from "ajv";

/** 📥️ Independent node discriminator admission for every authored host variant. */
export function runHostKindAdmissionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../../🧫️fixtures/📥️host-kind-admission/🔣️.json");
  const schema = read("../../🧬️schema/📥️host-kind-admission/🔣️.json");
  const validate = new Ajv({ strict: true }).compile(schema);
  assert.equal(corpus.schema, "semio.dag.host-kind-admission/v1");
  assert.equal(new Set(corpus.cases.map((row: { id: string }) => row.id)).size, 15);
  for (const row of corpus.cases) assert.equal(validate(row.snapshot), row.accepted, row.id);
  assert.deepEqual(corpus.cases.filter((row: { accepted: boolean }) => row.accepted).map((row: { id: string }) => row.id), schema.properties.nodes.items.properties.kind.enum);
  return corpus.cases.length + 3;
}
