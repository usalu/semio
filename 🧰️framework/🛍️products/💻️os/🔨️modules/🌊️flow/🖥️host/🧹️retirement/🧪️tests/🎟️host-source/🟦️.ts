import assert from "node:assert/strict";
import {existsSync,readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import stableStringify from "fast-json-stable-stringify";

/** 🏠️ Original host ownership agrees with its original fields and independent JSONPatch and UTF8 oracles. */
export function testHostSourceCustodyContract():void{
 assert.equal(existsSync(new URL("../../🧬️schema/🎟️host-source/🔣️.json",import.meta.url)),false,"Examples cannot own a host-source corpus schema");
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️host-source/🔣️.json",import.meta.url),"utf8"));
 const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");const definition=source.slice(source.indexOf("pub struct FlowHost {"),source.indexOf("impl Default for FlowHost"));
 const sourceFields=Array.from(definition.matchAll(/^\s*(?:pub(?:\([^)]*\))?\s+)?([a-z_]+)\s*:/gm),(match)=>match[1]);assert.deepEqual(sourceFields,fixture.sourceFields);
 const fields=Object.fromEntries(fixture.sourceFields.map((field:string)=>[field,field]));
 assert.equal(stableStringify(applyPatch(fields,[],true,false).newDocument),stableStringify(fields));
 assert.equal(new Set(Object.keys(fields)).size,fixture.sourceFields.length);assert.equal(Buffer.byteLength(fixture.text.repeat(fixture.repeat)),fixture.utf8Bytes);assert(fixture.capacityBytes>fixture.utf8Bytes);
 console.log("[DEBUG] original Host source examples originalFields="+fixture.sourceFields.length+" wholeCorpusSchema=false JSONPatch=true UTF8=true");
}
