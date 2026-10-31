import { runToolV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import Ajv from "ajv";
import corpus from "./🧫️fixtures/🔣️.json";
import { compileJsonSchemaSubset, jsonSchemaSubsetValueEquals, validateJsonSchemaSubset } from "../🟦️.ts";

/** 🧬️ Proves the owned schema subset against its portable corpus and independent AJV. */
export async function proveJsonSchemaSubsetContractV1(): Promise<number> {
  for (const row of corpus.cases) {
    const documents: unknown[] = ("documents" in row.input ? row.input.documents : undefined) ?? [];
    assert.equal(new Ajv({ strict: false, schemas: documents as object[] }).compile(row.input.schema)(row.value), row.valid, row.id + " AJV");
    assert.equal(validateJsonSchemaSubset(row.input.schema, row.value, row.input.schema, documents).length === 0, row.valid, row.id + " Bun");
  }
  for (const row of corpus.compilationRefusals) {
    assert.throws(() => new Ajv({ strict: false }).compile(row.input.schema), row.id + " AJV");
    assert.throws(() => compileJsonSchemaSubset(row.input.schema), row.id + " Bun");
  }
  for (const row of corpus.comparisons) {
    assert.equal(new Ajv({ strict: false }).compile({ const: row.left })(row.right), row.equal, row.id + " AJV");
    assert.equal(jsonSchemaSubsetValueEquals(row.left, row.right), row.equal, row.id + " Bun");
  }
  const { build } = await import("esbuild");
  const validator = resolve(import.meta.dirname, "../🟦️.ts");
  const program = `import { compileJsonSchemaSubset, jsonSchemaSubsetValueEquals, validateJsonSchemaSubset } from ${JSON.stringify(validator)}; const corpus=${JSON.stringify(corpus)}; console.log(JSON.stringify({cases:corpus.cases.map(row=>validateJsonSchemaSubset(row.input.schema,row.value,row.input.schema,row.input.documents??[]).length===0),comparisons:corpus.comparisons.map(row=>jsonSchemaSubsetValueEquals(row.left,row.right)),refusals:corpus.compilationRefusals.map(row=>{try{compileJsonSchemaSubset(row.input.schema);return false;}catch{return true;}})}));`;
  const result = await build({ stdin: { contents: program, resolveDir: import.meta.dirname, loader: "ts" }, bundle: true, platform: "node", format: "esm", write: false });
  const node = await runToolV1("node", ["--input-type=module"], { input: result.outputFiles![0]!.text, capture: true, caller: import.meta.url, cwd: import.meta.dirname });
  assert.equal(node.status, 0, node.stderr);
  assert.deepEqual(JSON.parse(node.stdout), { cases: corpus.cases.map(row => row.valid), comparisons: corpus.comparisons.map(row => row.equal), refusals: corpus.compilationRefusals.map(() => true) });
  return corpus.cases.length + corpus.comparisons.length + corpus.compilationRefusals.length;
}
