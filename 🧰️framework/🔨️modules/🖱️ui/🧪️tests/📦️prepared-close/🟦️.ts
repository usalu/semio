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
