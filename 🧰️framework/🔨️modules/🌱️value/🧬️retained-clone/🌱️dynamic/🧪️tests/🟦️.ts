import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const owner=resolve(import.meta.dir,"..");
const law=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8"));

test("original dynamic value clone preserves independent arbitrary variants",()=>{
  for(const row of law.cases){
    const original=structuredClone(row.value);
    const actual=JSON.parse(JSON.stringify(row.value));
    const independent=applyPatch({},[{op:"add",path:"/value",value:row.value}],true,false).newDocument.value;
    expect(actual).toEqual(independent);
    expect(row.value).toEqual(original);
  }
  expect(law.cases.at(-1).value.value.map((entry:[string,unknown])=>entry[0])).toEqual(["same","same","ä/日~🌱"]);
});

test("original UTF-8 bytes and numeric payload bits agree with independent native codecs",()=>{
  for(const row of law.cases){
    if(row.value.kind==="text")expect([...new TextEncoder().encode(row.value.value)]).toEqual([...Buffer.from(row.value.value,"utf8")]);
    if(row.value.kind==="float"){
      const bits=BigInt(`0x${row.value.bits}`);
      const native=new BigUint64Array([bits]);
      const copy=structuredClone(native);
      expect(copy[0]).toBe(bits);
      expect(copy.buffer).not.toBe(native.buffer);
    }
  }
  for(const depth of law.depths){
    let original:unknown={kind:"text",value:"ä日🌱"};
    for(let i=0;i<depth;i++)original={kind:"array",value:[original]};
    const actual=JSON.parse(JSON.stringify(original));
    expect(actual).toEqual(applyPatch({},[{op:"add",path:"/root",value:original}],true,false).newDocument.root);
  }
  console.log("[DEBUG] Original dynamic tree, duplicate keys, UTF-8 and numeric bits agree with independent JSON Patch");
});

test("original dynamic examples do not own a whole-trial schema",()=>{expect(existsSync(resolve(owner,"🧬️schema/🔣️.json"))).toBe(false);});
