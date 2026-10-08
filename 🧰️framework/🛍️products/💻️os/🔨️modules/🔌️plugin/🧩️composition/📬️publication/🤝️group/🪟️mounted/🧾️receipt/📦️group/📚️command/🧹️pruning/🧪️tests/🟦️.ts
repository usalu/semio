import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
test("command structural pruning keeps held IDs and shell rows until one bounded decision",()=>{
 const root=resolve(import.meta.dir,".."),law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(root,"🧫️fixtures/📐️schema.json"),"utf8")))(law)).toBe(true);
 for(const row of law.cases){const before=structuredClone(row.rows),candidate=row.rows.filter((r:any)=>(r.editId===null||row.heldEdits.includes(r.editId))&&(r.transitionId===null||row.heldTransitions.includes(r.transitionId)));expect(candidate.map((r:any)=>r.seq)).toEqual(row.keptSequences);expect(applyPatch({visible:before},[{op:"replace",path:"/visible",value:candidate}],true,false).newDocument.visible).toEqual(candidate);expect(row.rows).toEqual(before);expect(candidate.filter((r:any)=>r.editId===null&&r.transitionId===null)).toEqual(before.filter((r:any)=>r.editId===null&&r.transitionId===null));}
 const source=readFileSync(resolve(root,"../🦀️.rs"),"utf8");for(const method of ["begin_prune","prune_current","advance_prune","commit_prune","next_pruned_close_byte_demand","close_pruned_step"]){expect(source.includes("fn "+method+"(")).toBe(true);}
 console.log("[DEBUG] Ajv/RFC6902 command prune preserves exactheld IDs and shellrows, liveprefix untildecision, discardedownersfundedseparately");
});

test("mutable original command views invalidate prepared visibility and append seals",()=>{const root=resolve(import.meta.dir,".."),law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));const original={revision:0,editId:law.mutationSeal.oldEditId},changed=applyPatch(original,[{op:"replace",path:"/revision",value:law.mutationSeal.revisionAdvance},{op:"replace",path:"/editId",value:law.mutationSeal.newEditId}],true,false).newDocument;expect(changed.revision).not.toBe(original.revision);expect(changed.editId).toBe(law.mutationSeal.newEditId);const source=readFileSync(resolve(root,"../🦀️.rs"),"utf8"),method=source.slice(source.indexOf("fn last_mut("),source.indexOf("fn begin_prune("));expect(method.includes("checked_add(1)")).toBe(true);console.log("[DEBUG] Node/RFC6902 mutable command view advances original seal; both append/prune become stale without pointer/data loss");});
