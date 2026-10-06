import { readFileSync } from "node:fs";
import { strict as assert } from "node:assert";


/** 📥️ Independent node discriminator admission for every authored host variant. */
export function runHostKindAdmissionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../../🧫️fixtures/📥️host-kind-admission/🔣️.json");
  assert.equal(corpus.schema, "semio.dag.host-kind-admission/v1");
  assert.equal(new Set(corpus.cases.map((row: { id: string }) => row.id)).size, 15);
  
  return corpus.cases.length + 3;
}
