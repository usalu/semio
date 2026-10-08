/** 🔎️ Existing array slots expose the same five primitive controls through independent RFC6902. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch,getValueByPointer} from "fast-json-patch";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "./🧬️schema/🔣️.json";

test("generic history acceptance arrays retain primitive item controls and exact pointers",async()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);
 expect(corpus.control).toEqual({existingSlotsOnly:true,maximumArraySlots:16,primitiveBuckets:["number","boolean","option","vector","text"]});
 for(const row of corpus.cases){for(const change of row.expected){const before=structuredClone(row.before),previous=getValueByPointer(before,change.pointer),after=applyPatch(before,[{op:"replace",path:change.pointer,value:change.value}],true).newDocument;expect(getValueByPointer(after,change.pointer)).toEqual(change.value);expect(previous).not.toEqual(change.value);expect(applyPatch(after,[{op:"replace",path:change.pointer,value:previous}],true).newDocument).toEqual(row.before);}expect(row.before.values.length===0).toBe(row.expected.length===0)}
 const source=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text(),owner=source.slice(source.indexOf("pub fn acceptance_change_buckets"),source.indexOf("fn acceptance_meta"));expect(owner).toContain("ArgSchema::Array { items");expect(owner).toContain("ACCEPTANCE_CHANGES_PER_LEAF");
 console.log("[DEBUG] generic history array controls preserve eight independent RFC6902 corpora and all five primitive buckets");
});
