import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import stableStringify from "fast-json-stable-stringify";

/** 🎟️ Independent JSON Patch and Map references preserve original ordered insertion output. */
export function testHistoryIndexInsertion():void{
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️insertion/🔣️.json",import.meta.url),"utf8"));
 const map=new Map<string,string>(fixture.sourceRows);const source=Object.fromEntries(map);
 const patched=applyPatch(source,fixture.insertions.map(([key,value]:[string,string])=>({op:"add" as const,path:"/"+key,value})),true,false).newDocument;
 for(const [key,value]of fixture.insertions)map.set(key,value);
 assert.deepEqual(Object.fromEntries(map),fixture.expected.map);assert.deepEqual(patched,fixture.expected.map);
 assert.equal(stableStringify(patched),stableStringify(fixture.expected.map));
 assert(fixture.sourceCapacityBytes>Buffer.byteLength(fixture.sourceRows[0][0]));
 const membership=new Set<string>(fixture.sourceRows.map((row:[string,string])=>row[0]));for(const [key]of fixture.insertions)membership.add(key);assert.equal(stableStringify([...membership].sort()),stableStringify(fixture.setExpectedKeys));
 const reuse=new Map<number,string>(fixture.reuse.sourceRows);
 for(let turn=0;turn<fixture.reuse.rounds;turn++){
  const keys=[...reuse.keys()].sort((a,b)=>a-b);const mode=fixture.reuse.modes[turn%fixture.reuse.modes.length];
  const key=mode==="pop-first"?keys[0]:mode==="pop-last"?keys.at(-1)!:keys[turn%keys.length];const value=reuse.get(key)!;
  const reference=applyPatch(Object.fromEntries(reuse),[{op:"remove",path:"/"+key},{op:"add",path:"/"+key,value}],true,false).newDocument;
  reuse.delete(key);reuse.set(key,value);assert.deepEqual(Object.fromEntries(reuse),reference);assert.deepEqual(reference,fixture.reuse.expected.map);
 }
 assert.equal(stableStringify(Object.fromEntries(reuse)),stableStringify(fixture.reuse.expected.map));
 console.log("[DEBUG] original History index admitted insertion neutral rows="+map.size+" JSONPatch=true Map=true JSSet=true canonicalUTF8=true");
}
