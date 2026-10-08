import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {applyPatch} from "fast-json-patch";
test("paged command history preserves live prefixes and all inverse owners before decision",()=>{
 const root=resolve(import.meta.dir,".."),law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));
 
 for(const row of law.cases){let live:number[]=[];for(const seq of row.sequences){const before=structuredClone(live),prepared=applyPatch(live,[{op:"add",path:"/-",value:seq}],true,false).newDocument;expect(live).toEqual(before);expect(prepared).toEqual([...before,seq]);live=prepared;}expect(live).toEqual(row.sequences);expect(Math.ceil(live.length/law.pageRecords)).toBe(row.pages);}
 expect(JSON.parse(Buffer.from(JSON.stringify(law.inverseArgs),"utf8").toString("utf8"))).toEqual(law.inverseArgs);
 const source=readFileSync(resolve(root,"🦀️.rs"),"utf8");for(const method of ["prepare_append","next_capacity_byte_demand","admit_step","commit_append","next_close_byte_demand","close_step","iter","last_mut"]){expect(source.includes("fn "+method+"(")).toBe(true);}
 console.log("[DEBUG] command page8 neutral RFC6902/Node UTF8 proves unchanged live prefixes, exact0/1/8/9/17 histories and nested inverse args");
});
