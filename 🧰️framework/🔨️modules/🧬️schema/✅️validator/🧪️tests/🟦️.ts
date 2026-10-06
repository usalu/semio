import assert from "node:assert/strict";
import { resolve } from "node:path";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/🔣️.json";
import { jsonSchemaSubsetValueEquals, validateJsonSchemaSubset } from "../🟦️.ts";

/** 🧬️ Proves the owned schema subset against its portable corpus and independent AJV. */
export async function proveJsonSchemaSubsetContractV1(): Promise<number> {
  const ajv = new Ajv({ strict: false });
  for (const row of corpus.cases) {
    assert.equal(ajv.compile(row.schema)(row.value), row.valid, row.id + " AJV");
    assert.equal(validateJsonSchemaSubset(row.schema, row.value).length === 0, row.valid, row.id + " Bun");
  }
  for (const row of corpus.comparisons) {
    assert.equal(ajv.compile({ const: row.left })(row.right), row.equal, row.id + " AJV");
    assert.equal(jsonSchemaSubsetValueEquals(row.left, row.right), row.equal, row.id + " Bun");
  }
  const { build } = await import("esbuild");
  const validator = resolve(import.meta.dirname, "../🟦️.ts");
  const program = `import { jsonSchemaSubsetValueEquals, validateJsonSchemaSubset } from ${JSON.stringify(validator)}; const corpus=${JSON.stringify(corpus)}; console.log(JSON.stringify({cases:corpus.cases.map(row=>validateJsonSchemaSubset(row.schema,row.value).length===0),comparisons:corpus.comparisons.map(row=>jsonSchemaSubsetValueEquals(row.left,row.right))}));`;
  const result = await build({ stdin: { contents: program, resolveDir: import.meta.dirname, loader: "ts" }, bundle: true, platform: "node", format: "esm", write: false });
  const node = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(result.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
  assert.equal(node.exitCode, 0, Buffer.from(node.stderr).toString());
  assert.deepEqual(JSON.parse(Buffer.from(node.stdout).toString()), { cases: corpus.cases.map(row => row.valid), comparisons: corpus.comparisons.map(row => row.equal) });
  return corpus.cases.length + corpus.comparisons.length;
}
