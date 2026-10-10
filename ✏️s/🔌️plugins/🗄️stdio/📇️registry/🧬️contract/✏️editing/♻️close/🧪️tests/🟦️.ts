import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {RetainedCopyClose} from "../🟦️.ts";
import decodeCorpus from "../../../📐️part21/🚦️controlled/🧫️fixtures/🔣️.json";
import decodeSchema from "../../../📐️part21/🚦️controlled/🧬️schema/🔣️.json";
/** 🧪️ Replays genuine backing quotes with independent Buffer and JSON Patch custody witnesses. */
export function runOriginalCopyCloseChecks(){
 assert.equal(new Ajv({strict:false}).compile(schema)(corpus),true);assert.equal(new Ajv({strict:false}).compile(decodeSchema)(decodeCorpus),true);let assertions=2;
 for(const row of decodeCorpus.cases){const independent=JSON.parse(JSON.stringify(row.value));assert.deepEqual(independent,row.value);const recipient={owner:{source:independent}};const cleared=jsonPatch.applyPatch(recipient,[{op:"remove",path:"/owner"}]).newDocument;assert.deepEqual(cleared,{});assertions+=2;}
 for(const row of corpus.cases){const backing=Buffer.alloc(row.capacity);backing.write(row.content);const cursor=new RetainedCopyClose(backing.byteLength,row.reserved,row.complete);let oracle={capacity:backing.byteLength,reserved:row.reserved,complete:row.complete};const grant={items:1,copy:0,capacity:0,release:backing.byteLength,depth:1};if(row.capacity!==0){for(const denied of [{...grant,items:0},{...grant,release:grant.release-1},{...grant,depth:0}]){assert.deepEqual(cursor.step(denied),{terminal:false,released:0});assert.equal(cursor.capacity,oracle.capacity);assertions+=2;}assert.equal(cursor.step(grant).released,backing.byteLength);oracle=jsonPatch.applyPatch(oracle,[{op:"replace",path:"/capacity",value:0}]).newDocument;assertions++;}if(cursor.demand.depth!==0){assert.equal(cursor.step({...grant,release:0}).released,0);oracle=jsonPatch.applyPatch(oracle,[{op:"replace",path:"/reserved",value:false},{op:"replace",path:"/complete",value:false}]).newDocument;assertions++;}assert.deepEqual({capacity:cursor.capacity,reserved:cursor.reserved,complete:cursor.complete},oracle);assert.equal(cursor.step({...grant,release:0}).terminal,true);assertions+=2;}
 const native=readFileSync(join(import.meta.dir,"../../🦀️.rs"),"utf8"),media=readFileSync(join(import.meta.dir,"../../../🎬️media-export/🦀️.rs"),"utf8"),partial=readFileSync(join(import.meta.dir,"../../../📐️part21/🚦️controlled/🛬️input/🦀️.rs"),"utf8");assert(!native.includes("self.bytes.truncate(self.bytes.len() - released_bytes)"));assert(native.includes("factory.begin(request,grant)"));assert(native.includes("factory.begin_demand(mutation,lane)"));assert(media.includes("cursor.is_some()||self.completion.is_some()"));assert(!media.includes("self.cursor.take()"));assert(!media.includes("snapshot.take()"));assert(partial.includes("control.with_retirement_owner"));assert(partial.includes("self.text.extend_from_slice(chunk);control.step()?"));assert(!partial.includes("DecodedValue::new"));assertions+=9;
 console.log(`[DEBUG] Stdio original ownership ${corpus.cases.length} cases ${assertions} assertions`);return assertions;
}
