import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import stableStringify from "fast-json-stable-stringify";

/** 🏠️ Original host ownership agrees with its original fields and independent JSONPatch and UTF8 oracles. */
export function testHostSourceCustodyContract():void{
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️host-source/🔣️.json",import.meta.url),"utf8"));
 const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");const definition=source.slice(source.indexOf("pub struct FlowHost {"),source.indexOf("impl Default for FlowHost"));
 const sourceFields=Array.from(definition.matchAll(/^\s*(?:pub(?:\([^)]*\))?\s+)?([a-z_]+)\s*:/gm),(match)=>match[1]);assert.deepEqual(sourceFields,fixture.sourceFields);
 const retirement=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");assert.ok(retirement.includes("pending_evaluation:Option<FlowEvaluationWork>"));
 for(const field of ["pending_evaluation","evaluation_revision","evaluation_progress","pending_remaining"]){assert.ok(retirement.includes("   "+field+","));assert.ok(retirement.includes(field+":"+field));}
 const fields=Object.fromEntries(fixture.sourceFields.map((field:string)=>[field,field]));
 assert.equal(stableStringify(applyPatch(fields,[],true,false).newDocument),stableStringify(fields));
 assert.equal(new Set(Object.keys(fields)).size,fixture.sourceFields.length);assert.equal(Buffer.byteLength(fixture.text.repeat(fixture.repeat)),fixture.utf8Bytes);assert(fixture.capacityBytes>fixture.utf8Bytes);
 console.log("[DEBUG] original Host source examples originalFields="+fixture.sourceFields.length+" wholeCorpusSchema=false pendingEvaluationCustody=true JSONPatch=true UTF8=true");
}
