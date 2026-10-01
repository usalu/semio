import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🎼️ Independent schema and JSON oracle for all owned typed sample representations. */
export function runWavTypedChunkChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../../🧫️fixtures/🎼️typed-chunks/🔣️.json");
  const valid = new Ajv({ strict: true }).addKeyword({ keyword: "x-semio-state", schemaType: "string", valid: true }).compile(read("../../🔣️.json"));
  for (const vector of corpus.cases) {
    assert.equal(valid(vector.snapshot), true, JSON.stringify(valid.errors));
    assert.deepEqual(JSON.parse(JSON.stringify(vector.snapshot)), vector.snapshot);
    assert.deepEqual(vector.snapshot.chunkOrder.map((entry: { kind: string }) => entry.kind), ["other", "format", "other", "samples"]);
    assert.deepEqual(vector.snapshot.otherChunks[0], { fourcc: "JUNK", data: [9, 8, 7], padByte: 165 });
  }
  for (const data of [{kind:"pcm16",value:[32768]},{kind:"pcm8",value:[-1]},{kind:"raw",value:[256]},{kind:"unknown",value:[]}]) {
    assert.equal(valid({...corpus.cases[0].snapshot,data}), false);
  }
  assert.equal(valid({...corpus.cases[0].snapshot,chunkOrder:[{kind:"format",value:0}]}), false);
  return corpus.cases.length + 5;
}
