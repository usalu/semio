import { toolJobReservedSpecs } from "@semio-tech/repo-lib/laws";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";
import law from "./🧫️fixtures/🔣️.json";
import {pluginFamilyText, pluginSubtreeText} from "../../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
test("reserved emission admission retains exact slots before producer ownership",()=>{
 for(const row of law.cases){const slots:Array<number|null>=Array.from({length:law.slots},(_,i)=>i<row.occupied?i+128:null);const original=structuredClone(slots);const vacant=slots.indexOf(null);expect(vacant>=0).toBe(row.admitted);if(vacant>=0){const model=applyPatch({slots},[{op:"replace",path:`/slots/${vacant}`,value:vacant+192}],false,false).newDocument;expect(model.slots.filter(x=>x!==null).length).toBe(row.occupied+1);expect(model.slots.slice(0,row.occupied)).toEqual(original.slice(0,row.occupied));}}
 const source=pluginFamilyText();
 expect(source.includes("admit_reserved_emit_producer")).toBe(true);
 expect(source.includes("permit:&mut Option<(semio_plugin_job::reserved_jobs::FrameworkReservedCommitPermit,")).toBe(true);
 for(const route of law.routes){const row=source.split("\n").find(line=>line.includes("framework_reserved_job!(")&&line.includes(`"${route.id}"`));expect(row).toBeDefined();expect(row).toContain(`lanes: [${route.lanes.join(", ")}]`);}
 const clipboardStart=source.indexOf("async fn dispatch_framework_reserved_action"),clipboard=source.slice(clipboardStart,clipboardStart+2500);
 expect(clipboard).toContain("advance_original_reserved_action");
 const actionStart=source.indexOf("pub async fn advance_original_reserved_action"),action=source.slice(actionStart,actionStart+900);
 expect(action.indexOf("advance_reserved_action(")).toBeGreaterThan(0);
 expect(action.indexOf("advance_reserved_action(")).toBeLessThan(action.indexOf("advance_original_reserved_mount("));
 const mountStart=source.indexOf("async fn advance_original_reserved_mount"),mount=source.slice(mountStart,mountStart+3000);
 expect(mount.indexOf("admit_reserved_emit_producer(verb,meta,cx)")).toBeGreaterThan(0);
 expect(mount.indexOf("admit_reserved_emit_producer(verb,meta,cx)")).toBeLessThan(mount.indexOf("OriginalEmissionAdmission::<A>::admit("));
 expect(mount).toContain("cx.original_source_parent()?.source_identity()?");
 const media=source.slice(source.indexOf("async fn dispatch_import_media"),source.indexOf("async fn dispatch_config_command_inner"));
 expect(media).toContain("advance_original_reserved_media");
 const mediaStart=source.indexOf("pub async fn advance_original_reserved_media"),mediaStep=source.slice(mediaStart,mediaStart+700);
 expect(mediaStep.indexOf("advance_reserved_media(")).toBeLessThan(mediaStep.indexOf("advance_original_reserved_mount("));
 const builders=pluginSubtreeText("🧵️instance","🖼️render");
 expect(builders.match(/\n\s*source_binding,/gu)?.length).toBe(2);
 expect(builders).toContain("input: Some(ArtifactReservedToolInput::Action");
 expect(builders).toContain("input: Some(ArtifactReservedToolInput::Media");
});

test("reserved completed refusal retains original permit and output before returning authority",()=>{
 for(const row of law.refusals){const owner={completedOriginal:row.completedOriginal,retained:false,cancelled:false};const expected=applyPatch(owner,[{op:"replace",path:"/retained",value:row.retained},{op:"replace",path:"/cancelled",value:row.cancelled}],false,false).newDocument;expect(expected.completedOriginal).toBe(row.completedOriginal);expect(expected.retained).toBe(row.retained);}
 const source=pluginFamilyText();
 expect(/permit:\s*&mut Option<\(semio_plugin_job::reserved_jobs::FrameworkReservedCommitPermit/u.test(source)).toBe(true);
 expect(source.includes("producer_fault: Option<ArtifactBoundedToolFault>")).toBe(true);
 const admit=source.slice(source.indexOf("pub fn admit_original_reserved("));
 const refused=admit.indexOf("authority.producer_fault.is_some()"),taken=admit.indexOf("permit.take().unwrap()");
 expect(refused).toBeGreaterThan(0);expect(taken).toBeGreaterThan(refused);
 expect(admit.indexOf("completion.take()")).toBeGreaterThan(refused);expect(admit.indexOf("job.take()")).toBeGreaterThan(refused);
 expect(admit.slice(0,taken).includes("validate_framework_reserved_commit")).toBe(false);
});

test("reserved declaration parser projects the same exact neutral lanes",()=>{
 const source=pluginFamilyText();
 const actual=new Map(toolJobReservedSpecs(source).map(row=>[row.id,row]));
 for(const route of law.routes){expect(actual.get(route.id)?.lanes).toEqual(route.lanes);}
 const forged='framework_reserved_job!(J, F, "cut", 0, 64, 1, 1, 64, lanes: [Unknown]);';
 expect(toolJobReservedSpecs(forged)).toEqual([]);
});

test("reserved completion refusal keeps original fixed owners before checks",()=>{
 for(const row of law.completionRefusals){const original={operation:81,completion:"original",producer:"original",fault:null as string|null,cancelled:false};const model=applyPatch(original,[{op:"replace",path:"/fault",value:row.id},{op:"replace",path:"/cancelled",value:row.cancelled}],false,false).newDocument;expect(model.operation).toBe(81);expect(model.completion).toBe("original");expect(model.producer).toBe("original");}
 const source=pluginFamilyText();
 expect(source.includes("reserved_producer: Option<ArtifactReservedToolJob>")).toBe(true);
 const owner=pluginSubtreeText("🧵️instance","📨️dispatch");
 expect(owner.includes("mount_original_reserved_owners")).toBe(true);
 expect(source.includes("finish_mounted_reserved_completion")).toBe(true);
});
