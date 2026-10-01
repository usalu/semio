import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

/** 🧾️ Compares declared owner authority with independently compiled JSON Schema enumeration. */
export function schemaDocumentAuthorityOracle(): number {
  const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧾️document-authority/🔣️.json"), "utf8"));
  const corpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧾️document-authority/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true });
  assert.equal(corpus.schema, "semio.plugin.schema-document-authority/v1");
  assert.equal(corpus.cases.length, 9);
  for (const row of corpus.cases) {
    const authority = { ...schema, properties: { ...schema.properties, ownerPluginId: { ...schema.properties.ownerPluginId, enum: [row.input.pluginId, ...row.input.dependencies] } } };
    assert.equal(ajv.compile(authority)(row.input), row.accepted, row.id);
  }
  return corpus.cases.length;
}
