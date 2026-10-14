import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import fg from "fast-glob";
import * as pruning from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {testCachePrune} from "../../../🧪️tests/🧹️cache-prune/🟦️.ts";
import {phaseCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

test("Cooperative scanners retain every existing portable pruning and physical byte law",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original ticket artifact directory required");
 const control=phaseCompilerAdmissionControlV1(new AbortController().signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done)));await testCachePrune(output,control);
 console.log("[DEBUG] Existing portable cache planner and Python physical byte oracle passed through cooperative maintenance");
},30000);

test("Large cache walks preserve awaited original cancellation and deadlines",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original ticket artifact directory required");mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"c-")),area=join(root,"area"),consumer=join(area,fixture.consumer);mkdirSync(consumer,{recursive:true});
 const original=Error("Original maintenance cancellation"),controller=new AbortController();let deadline:number|null=30000,published=0,settled=0,yielded=0,completed=0,interrupt:string|undefined;
 const control:ResourceLeaseControl={signal:controller.signal,remainingMilliseconds:()=>deadline,publish:async()=>{published++;await new Promise<void>(done=>setImmediate(done));settled++;},yieldContinuation:async()=>{expect(settled).toBe(published);yielded++;await new Promise<void>(done=>setImmediate(done));if(interrupt==="signal")controller.abort(original);if(interrupt==="deadline")deadline=0;}};
 try{
  expect(new Ajv({strict:true}).compile(schema)(schema.definitions.Policy.const)).toBe(true);
  for(let index=0;index<fixture.files;index++)writeFileSync(join(consumer,index+".bin"),fixture.contents);
  const oracle=fg.sync("**/*",{cwd:consumer,onlyFiles:true,followSymbolicLinks:false}).reduce((sum,path)=>sum+statSync(join(consumer,path)).size,0);
  const units=await pruning.scanDirectoryUnits(area,control,new Set(),()=>completed++);
  expect(units).toHaveLength(fixture.expected.units);expect(units[0]!.bytes).toBe(oracle);expect(published).toBeGreaterThan(0);expect(yielded).toBe(published);expect(completed).toBe(1);
  completed=0;interrupt="deadline";await expect(pruning.scanDirectoryUnits(area,control,new Set(),()=>completed++)).rejects.toThrow("Original compiler admission deadline exhausted");expect(completed).toBe(fixture.expected.cancelledScanUnits);deadline=30000;
  interrupt="signal";await expect(pruning.scanDirectoryUnits(area,control,new Set(),()=>completed++)).rejects.toBe(original);expect(completed).toBe(fixture.expected.cancelledScanUnits);
  console.log("[DEBUG] Cooperative cache scan files="+fixture.files+" bytes="+oracle+" originalSignal=preserved originalDeadline=preserved progress=awaited");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("Large cache deletion stops at the caller continuation and resumes from physical files",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original ticket artifact directory required");mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"d-")),area=join(root,"area"),consumer=join(area,fixture.consumer);mkdirSync(consumer,{recursive:true});
 const controller=new AbortController(),original=Error("Original cache deletion cancellation");let interrupt=true,yielded=0;
 const control:ResourceLeaseControl={signal:controller.signal,remainingMilliseconds:()=>30000,publish:async()=>undefined,yieldContinuation:async()=>{yielded++;await new Promise<void>(done=>setImmediate(done));if(interrupt)controller.abort(original);}};
 const unit:pruning.CacheUnit={path:fixture.consumer,bytes:0,recencyMs:0,lockHeld:false,kind:"directory"};
 try{
  for(let index=0;index<fixture.files;index++)writeFileSync(join(consumer,index+".bin"),fixture.contents);writeFileSync(join(root,"outside.bin"),fixture.contents);
  await expect(pruning.deleteUnit(area,unit,control)).rejects.toBe(original);const retained=fg.sync("**/*",{cwd:consumer,onlyFiles:true});expect(retained.length).toBeGreaterThan(0);expect(retained.length).toBeLessThan(fixture.files);expect(yielded).toBe(1);
  interrupt=false;const resumed:ResourceLeaseControl={...control,signal:new AbortController().signal};mkdirSync(join(root,"outside"));writeFileSync(join(root,"outside/kept.bin"),fixture.contents);symlinkSync(join(root,"outside"),join(consumer,"foreign"),"junction");await pruning.deleteUnit(area,unit,resumed);expect(fg.sync("**/*",{cwd:area,onlyFiles:true})).toEqual([]);expect(readFileSync(join(root,"outside/kept.bin"),"utf8")).toBe(fixture.contents);
  await expect(pruning.deleteUnit(area,{...unit,path:"../outside.bin"},resumed)).rejects.toThrow("Cache unit must remain inside its area");expect(readFileSync(join(root,"outside.bin"),"utf8")).toBe(fixture.contents);
  symlinkSync(join(root,"outside"),join(area,"foreign"),"junction");await expect(pruning.deleteUnit(area,{...unit,path:"foreign/kept.bin",kind:"cargo-target-file"},resumed)).rejects.toThrow("Cache unit parent must remain a physical directory");expect(readFileSync(join(root,"outside/kept.bin"),"utf8")).toBe(fixture.contents);
  console.log("[DEBUG] Cooperative cache deletion partialFiles="+retained.length+" resumed=terminal escapedDeletion="+fixture.expected.escapedDeletion);
 }finally{rmSync(root,{recursive:true,force:true});}
});
