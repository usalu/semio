/** 👥️ Actual entity-edit transitions agree with an independent RFC6902 oracle over full u64 identities. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {applyPatch} from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {ToolRunEntityEdit} from "../🟦️.ts";
import versionSchema from "../📸️version/🧬️schema/🔣️.json";
import versionFixture from "../📸️version/🧫️fixtures/🔣️.json";
import {ToolRunEntityVersion,ToolRunEntityVersionEdit} from "../📸️version/🟦️.ts";
import provenanceSchema from "../🧾️provenance/🧬️schema/🔣️.json";
import provenanceFixture from "../🧾️provenance/🧫️fixtures/🔣️.json";
import {ToolRunEntityProvenanceEdit} from "../🧾️provenance/🟦️.ts";

test("original ToolRun rollback provenance retains earlier introductions and captured originals",()=>{
 expect(new Ajv({strict:true}).compile(provenanceSchema)(provenanceFixture)).toBe(true);
 for(const row of provenanceFixture.cases){
  const marks=row.original.filter(mark=>row.retractTo===null||mark.end<=row.retractTo);
  for(const entity of row.append)if(!marks.some(mark=>mark.end===row.end&&mark.entity===entity))marks.push({end:row.end,entity});
  marks.sort((a,b)=>a.end-b.end||(BigInt(a.entity)<BigInt(b.entity)?-1:BigInt(a.entity)>BigInt(b.entity)?1:0));
  const entities=[...new Set(marks.map(mark=>mark.entity))].sort((a,b)=>BigInt(a)<BigInt(b)?-1:BigInt(a)>BigInt(b)?1:0);
  const oracle=applyPatch({marks:row.original,entities:[] as string[]},[{op:"replace",path:"/marks",value:marks},{op:"replace",path:"/entities",value:entities}],true,false).newDocument;
  expect(oracle.marks).toEqual(row.expectedMarks);expect(oracle.entities).toEqual(row.expectedEntities);
  for(const stop of [undefined,...provenanceFixture.interruptAfter]){
   const job=new ToolRunEntityProvenanceEdit(row.original,row.retractTo,row.end,row.append);let turns=0;
   while(!job.complete&&turns<(stop??10000)){expect(job.advance(0)).toBe(0);expect(job.advance(1)).toBeLessThanOrEqual(1);turns++;}
   expect(job.original).toEqual(row.original);
   if(stop===undefined){expect(job.complete).toBe(true);expect(job.take(0)).toBeUndefined();expect(job.take(1)).toEqual(oracle);expect(job.take(1)).toBeUndefined();}else{job.cancel();expect(job.take(1)).toBeUndefined();}
   while(job.retained)expect(job.close(1)).toBeLessThanOrEqual(1);expect(job.original).toEqual(row.original);
  }
 }
 console.log("[DEBUG] Original ToolRun provenance rollback and deduplication matched independent BigInt/RFC6902 oracle");
});

test("original ToolRun entity versions publish once and preserve captured originals on cancellation",()=>{
 expect(new Ajv({strict:true}).compile(versionSchema)(versionFixture)).toBe(true);
 for(const row of fixture.cases){
  const original=new ToolRunEntityVersion(versionFixture.originalRevision,[...row.initial]);
  const oracle=applyPatch({entities:[...row.initial]},[{op:"replace",path:"/entities",value:row.expected}],true,false).newDocument.entities;
  for(const stop of [undefined,...fixture.interruptAfter]){
   const job=new ToolRunEntityVersionEdit(original,[...row.targets],row.kind as "append"|"retract");
   let turns=0;while(!job.complete&&turns<(stop??10000)){expect(job.advance(0)).toBe(0);expect(job.advance(1)).toBeLessThanOrEqual(1);turns++;}
   expect(original.entities).toEqual(row.initial);
   if(stop===undefined){expect(job.complete).toBe(true);expect(job.take(0)).toBeUndefined();const published=job.take(1)!;expect(published.revision).toBe(versionFixture.publishedRevision);expect(published.entities).toEqual(oracle);expect(job.take(1)).toBeUndefined();}
   else{job.cancel();expect(job.take(1)).toBeUndefined();}
   while(job.retained)expect(job.close(1)).toBeLessThanOrEqual(1);
   expect(original.entities).toEqual(row.initial);
  }
 }
 console.log("[DEBUG] Original ToolRun immutable entity versions match independent RFC6902 publication and cancellation outputs");
});

test("original ToolRun entity owner completes and cancels the shared append/retract corpus",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  let oracle={entities:[...row.initial]};
  for(const target of row.targets){const found=oracle.entities.indexOf(target);if(row.kind==="retract"){if(found>=0)oracle=applyPatch(oracle,[{op:"remove",path:`/entities/${found}`}],true,false).newDocument;}else if(found<0){const index=oracle.entities.findIndex(value=>BigInt(value)>BigInt(target));oracle=applyPatch(oracle,[{op:"add",path:`/entities/${index<0?oracle.entities.length:index}`,value:target}],true,false).newDocument;}}
  expect(oracle.entities).toEqual(row.expected);
  const source=[...row.initial],targets=[...row.targets],job=new ToolRunEntityEdit(source,targets,row.kind as "append"|"retract");
  let turns=0;while(!job.complete&&turns++<10000){const denied=job.advance(0);expect(denied).toBe(0);expect(job.advance(1)).toBeLessThanOrEqual(1);}
  expect(job.complete).toBe(true);expect(job.take()).toEqual(oracle.entities);expect(targets).toEqual(row.targets);
  for(const interrupt of fixture.interruptAfter){const original=[...row.initial],pending=new ToolRunEntityEdit(original,[...row.targets],row.kind as "append"|"retract");for(let turn=0;turn<interrupt&&!pending.complete;turn++)pending.advance(1);pending.cancel();expect(pending.take()).toBeUndefined();expect(pending.retained).toBe(true);while(pending.retained)expect(pending.close(1)).toBeLessThanOrEqual(1);}
 }
 console.log("[DEBUG] original ToolRun entity transitions agree with independent RFC6902 append/retract and full-u64 ordering");
});
         
