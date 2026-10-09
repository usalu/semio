import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import stableStringify from "fast-json-stable-stringify";

/** 🧯️ Closed neutral causal and conflict sources match an independent JSON Patch oracle. */
export function testOriginalConflictSource():void{
 const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const original={envelopes:fixture.envelopes,conflicts:fixture.conflicts};assert.equal(stableStringify(applyPatch(original,[],true,false).newDocument),stableStringify(original));
 for(const row of fixture.envelopes){const text=row.text.repeat(row.repeat);assert.equal(Buffer.byteLength(text),new TextEncoder().encode(text).length);const backing=Buffer.alloc(Math.max(row.stringCapacity,Buffer.byteLength(text)));const written=backing.write(text);assert.equal(written,Buffer.byteLength(text));assert.equal(backing.subarray(0,written).toString(),text);}
 console.log("[DEBUG] original causal/conflict JSONPatch=true stableJson=true UTF8=true originalEnvelopes="+fixture.envelopes.length+" originalConflicts="+fixture.conflicts.length);
}
