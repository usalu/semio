/** 🛡️ Independent RFC6902 and physical Buffer authority for denied original prepared pages. */
import {test,expect} from "bun:test";
import {applyPatch} from "fast-json-patch";
import fixture from "../../🧫️fixtures/📦️prepared-close/🔣️.json" with {type:"json"};
import physical from "../♻️physical-job-close/🧫️fixtures/🔣️.json" with {type:"json"};
test("original physical fixture agrees with independent Buffer extents",()=>{
 const atlas=physical.prepared.atlas;
 const pixels=Buffer.alloc(atlas.width*atlas.height*atlas.channels,91);
 expect(pixels.byteLength).toBe(atlas.byteLength);
 expect(pixels.byteLength/atlas.pageBytes).toBe(4);
 for(const copy of physical.copyGrants){
  const receipt={items:1,copy:0,capacity:0,release:atlas.pageBytes};
  expect(receipt.copy).toBeLessThanOrEqual(copy);
  expect(receipt.release).toBeGreaterThan(copy);
 }
});
test("prepared page release requires physical authority independently of payload copy",()=>{
 const original=Buffer.alloc(fixture.atlas.physicalPageBytes,7);
 for(const grant of fixture.grants){
  const owners=structuredClone(fixture.retained);
  const funded=grant.items>0&&grant.depth>0&&grant.release>=original.byteLength;
  expect(funded).toBe(false);
  expect(applyPatch(owners,[],true).newDocument).toEqual(fixture.retained);
  expect(Buffer.compare(original,Buffer.alloc(original.byteLength,7))).toBe(0);
  expect(fixture.expectedRetained).toEqual({result:"pending",copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
 }
 for(const copy of [0,1,3,64]){
  const grant={items:1,copy,capacity:0,release:original.byteLength,depth:1};
  expect(grant.release>=original.byteLength).toBe(true);
  const receipt={copiedItems:1,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:original.byteLength};
  expect(receipt.copiedBytes).toBeLessThanOrEqual(grant.copy);
  expect(receipt.releasedBytes).toBe(grant.release);
 }
 console.log(`[DEBUG] prepared page bytes=${original.byteLength} grants=${fixture.grants.length} originalOwner=unchanged copiedBytes=0 releaseIndependent=true`);
});

import rasterFixture from "../♻️raster-lease-close/🧫️fixtures/🔣️.json" with {type:"json"};
test("moved raster admission preserves separately admitted original backing",()=>{
 for(const row of rasterFixture.rasters){
  const backing=Buffer.alloc(row.capacityBytes,row.fill),pixels=backing.subarray(0,row.byteLength);
  expect(pixels.byteLength).toBe(row.width*row.height*4);
  expect(pixels.buffer).toBe(backing.buffer);
  expect(pixels.byteOffset).toBe(backing.byteOffset);
  const owner={logicalBytes:pixels.byteLength,backingBytes:backing.byteLength};
  expect(applyPatch(structuredClone(owner),[],true).newDocument).toEqual(owner);
  expect(owner.backingBytes>=owner.logicalBytes).toBe(true);
  for(const copy of rasterFixture.copyGrants){
   const grant={items:1,copy,capacity:0,release:owner.backingBytes,depth:1};
   expect(grant.release>=owner.backingBytes).toBe(true);
   expect(grant.release-1<owner.backingBytes).toBe(true);
   expect(Buffer.compare(pixels,Buffer.alloc(row.byteLength,row.fill))).toBe(0);
  }
 }
 console.log("[DEBUG] moved raster originalPointer=true callerCapacity=true logicalBytesIndependent=true refusalAllocation=0");
});

import Ajv from "ajv";
import {readFileSync} from "node:fs";
test("prepared outcome fixture admits paid loans with independent closed schema",()=>{
 const expectedSchema={type:"object",additionalProperties:false,required:["deniedReceipt","completeStateAbsent","completeOutputAbsent","faultLoanIdentity","acknowledgementItems"],properties:{deniedReceipt:{type:"object",additionalProperties:false,required:["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"],properties:{copiedItems:{const:0},copiedBytes:{const:0},retainedCapacityBytes:{const:0},releasedBytes:{const:0}}},completeStateAbsent:{const:true},completeOutputAbsent:{const:true},faultLoanIdentity:{const:true},acknowledgementItems:{const:1}}};
 const admitted=new Ajv({strict:true}).compile(expectedSchema);
 expect(admitted(physical.preparedOutcome.expected)).toBe(true);
 expect(applyPatch(structuredClone(physical.preparedOutcome.expected),[],true).newDocument).toEqual(physical.preparedOutcome.expected);
 const source=readFileSync(new URL("../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs",import.meta.url),"utf8");
 const implementation=source.slice(source.indexOf("impl InteractiveJob for PreparedRenderJob"),source.indexOf("//#endregion ⚙️PreparationJob"));
 expect(implementation.includes("Result<Option<JobOutcomeBorrow<'a>>, ValueError>")).toBe(true);
 expect(implementation.includes("fn borrow_outcome")).toBe(true);
 expect(implementation.includes("StepOutcome")).toBe(false);
 expect(source.includes("fault_payload.close_step_granted(grant)")).toBe(true);
 expect(source.includes("JobOutcomeBorrow::admit_fault(cx, self.fault_payload.published()")).toBe(true);
 console.log("[DEBUG] prepared outcome closed Ajv/RFC6902 fixture; canonical native borrowed trait and original funded fault owner source");
});
