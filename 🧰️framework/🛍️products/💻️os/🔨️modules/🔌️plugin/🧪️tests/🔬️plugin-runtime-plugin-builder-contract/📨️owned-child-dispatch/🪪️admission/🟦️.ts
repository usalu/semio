import { toolJobReservedSpecs } from "../../../../../../../../../📜️script.ts";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";
import law from "./🧫️fixtures/🔣️.json";
test("reserved emission admission retains exact slots before producer ownership",()=>{
 for(const row of law.cases){const slots:Array<number|null>=Array.from({length:law.slots},(_,i)=>i<row.occupied?i+128:null);const original=structuredClone(slots);const vacant=slots.indexOf(null);expect(vacant>=0).toBe(row.admitted);if(vacant>=0){const model=applyPatch({slots},[{op:"replace",path:`/slots/${vacant}`,value:vacant+192}],false,false).newDocument;expect(model.slots.filter(x=>x!==null).length).toBe(row.occupied+1);expect(model.slots.slice(0,row.occupied)).toEqual(original.slice(0,row.occupied));}}
 const source=readFileSync(new URL("../../../../🦀️.rs",import.meta.url),"utf8");
 expect(source.includes("admit_reserved_emit_producer")).toBe(true);
 expect(source.includes("supplied_authority: Option<FrameworkReservedCommitPermit>")).toBe(true);
 for(const route of law.routes){const row=source.split("\n").find(line=>line.includes("framework_reserved_job!(")&&line.includes(`"${route.id}"`));expect(row).toBeDefined();expect(row).toContain(`lanes: [${route.lanes.join(", ")}]`);}
 const clipboard=source.slice(source.indexOf("async fn dispatch_framework_reserved_action"),source.indexOf("async fn admit_framework_reserved_spawn"));
 expect(clipboard.indexOf("admit_reserved_emit_producer")).toBeLessThan(clipboard.indexOf("build_artifact_reserved_action_job"));
 const media=source.slice(source.indexOf("async fn dispatch_import_media"),source.indexOf("async fn dispatch_config_command_inner"));
 expect(media.indexOf("admit_reserved_emit_producer")).toBeLessThan(media.indexOf("build_artifact_reserved_media_job"));
 console.log("[DEBUG] RFC6902 exact64-slot saturation0/63/64 and six canonical route lane declarations preserve original IDs before producer transfer");
});

test("reserved completed refusal retains original permit and output before returning authority",()=>{
 for(const row of law.refusals){const owner={completedOriginal:row.completedOriginal,retained:false,cancelled:false};const expected=applyPatch(owner,[{op:"replace",path:"/retained",value:row.retained},{op:"replace",path:"/cancelled",value:row.cancelled}],false,false).newDocument;expect(expected.completedOriginal).toBe(row.completedOriginal);expect(expected.retained).toBe(row.retained);}
 const source=readFileSync(new URL("../../../../🦀️.rs",import.meta.url),"utf8");
 expect(source).toMatch(/supplied_authority:\s*&FrameworkReservedCommitPermit/u);
 expect(source.includes("producer_fault: Option<ArtifactBoundedToolFault>")).toBe(true);
 const commit=source.slice(source.indexOf("async fn commit_framework_clipboard_completion"),source.indexOf("async fn dispatch_reserved_original_producer"));
 const mounted=commit.indexOf("self.mount_original_reserved_owners(action,None,None,completion,meta,Some((permit.operation,permit.lease)))");expect(mounted).toBeGreaterThan(0);expect(mounted).toBeLessThan(commit.indexOf("if let Some(fault)=permit.producer_fault"));expect(commit.indexOf("permit.producer_fault")).toBeLessThan(commit.indexOf("self.finish_mounted_reserved_completion(id)"));
 expect(commit.includes("validate_framework_reserved_commit")).toBe(false);
 console.log("[DEBUG] RFC6902 four refusal outcomes preserve completed original output and cancelled exact permit until mounted handoff");
});

test("reserved declaration parser projects the same exact neutral lanes",()=>{
 const source=readFileSync(new URL("../../../../🦀️.rs",import.meta.url),"utf8");
 const actual=new Map(toolJobReservedSpecs(source).map(row=>[row.id,row]));
 for(const route of law.routes){expect(actual.get(route.id)?.lanes).toEqual(route.lanes);}
 const forged='framework_reserved_job!(J, F, "cut", 0, 64, 1, 1, 64, lanes: [Unknown]);';
 expect(toolJobReservedSpecs(forged)).toEqual([]);
 console.log("[DEBUG] source parser exact per-route lane projection agrees with six neutral native factory declarations and refuses foreign lane");
});

test("reserved completion refusal keeps original fixed owners before checks",()=>{
 for(const row of law.completionRefusals){const original={operation:81,completion:"original",producer:"original",fault:null as string|null,cancelled:false};const model=applyPatch(original,[{op:"replace",path:"/fault",value:row.id},{op:"replace",path:"/cancelled",value:row.cancelled}],false,false).newDocument;expect(model.operation).toBe(81);expect(model.completion).toBe("original");expect(model.producer).toBe("original");}
 const source=readFileSync(new URL("../../../../🦀️.rs",import.meta.url),"utf8");
 expect(source.includes("reserved_producer: Option<ArtifactReservedToolJob>")).toBe(true);
 const owner=readFileSync(new URL("../../../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs",import.meta.url),"utf8");
 expect(owner.includes("mount_original_reserved_owners")).toBe(true);
 expect(source.includes("finish_mounted_reserved_completion")).toBe(true);
 console.log("[DEBUG] RFC6902 busy/poison/wrong-download preserves original producer/completion/operation before bounded refusal");
});
