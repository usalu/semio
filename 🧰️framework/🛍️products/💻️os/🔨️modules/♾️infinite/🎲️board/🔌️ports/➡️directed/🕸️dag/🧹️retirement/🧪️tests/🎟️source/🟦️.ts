import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import assert from "node:assert/strict";
import { applyPatch } from "fast-json-patch";
import stable from "fast-json-stable-stringify";

export function testDagSourceCustodyContract(): void {
  const base=resolve(import.meta.dir,"../..");
  const law=JSON.parse(readFileSync(resolve(base,"🧫️fixtures/🎟️source/🔣️.json"),"utf8"));
  assert.equal(new Set(law.dagFields).size,law.dagFields.length);
  assert.equal(new Set(law.engineFields).size,law.engineFields.length);
  const text=law.text.repeat(law.repeat);
  assert.equal(Buffer.byteLength(text,"utf8"),law.utf8Bytes);
  const source={schema:text,nodes:[{id:"source",note:text}],edges:[]};
  const original=stable(source);
  const answer=applyPatch(structuredClone(source),[],true,false).newDocument;
  assert.equal(stable(answer),original);
  console.log(`[DEBUG] original DAG source examples dagFields=${law.dagFields.length} engineFields=${law.engineFields.length} JSONPatch=true UTF8=true`);
}
