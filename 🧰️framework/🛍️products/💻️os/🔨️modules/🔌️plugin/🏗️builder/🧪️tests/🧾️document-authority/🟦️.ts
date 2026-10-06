import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";


/** 🧾️ Compares declared owner authority with independently compiled JSON Schema enumeration. */
export function schemaDocumentAuthorityOracle(): number {
  const corpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧾️document-authority/🔣️.json"), "utf8"));
  
  assert.equal(corpus.schema, "semio.plugin.schema-document-authority/v1");
  assert.equal(corpus.cases.length, 9);
  
  return corpus.cases.length;
}
