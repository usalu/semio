/** 🔄️ Tests for the refresh of stale image volumes. */
import{test,expect}from"bun:test";
import{validateJsonSchemaSubset}from"../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{nativeImageRootV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import type{NativeImageReclaimPortV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{refreshStaleImageIfQuiescentV1,type ImageRefreshPortsV1}from"../🟦️.ts";
const control:ResourceLeaseControl={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>{},yieldContinuation:async()=>{}};
const directory="/r/rust",leaseDirectory="/r/leases";
const run=(row:typeof fixture.scenarios[number],log:string[],overrides:Partial<ImageRefreshPortsV1>={})=>{
 let attached=true,reads=0;
 const port:NativeImageReclaimPortV1={
  async attachedDevice(){return attached?"/dev/disk3":null;},
  async detach(){log.push("detach");attached=false;},
  async attach(){log.push("attach");attached=true;},
  allocatedBytes:()=>0,
  volumeAvailableBytes:()=>row.volumeAvailableBytes,
  dispose(){},
  mountsCanonically:true
 };
 const ports:ImageRefreshPortsV1={
  platform:row.platform,now:()=>1000000,exists:()=>row.imageExists,retirementPending:()=>"retirementPending" in row&&row.retirementPending===true,mounted:()=>row.mounted,
  volumeAvailableBytes:()=>reads++<2?row.volumeAvailableBytes:row.afterVolumeAvailableBytes,
  hostFreeBytes:()=>row.hostFreeBytes,
  containerFreeBytes:()=>(row as{containerFreeBytes?:number}).containerFreeBytes,
  lastCycleAt:()=>row.msSinceLastCycle===null?undefined:1000000-row.msSinceLastCycle,
  stampCycle:()=>{},
  lease:async()=>{log.push("lease");return row.leaseHeld?undefined:{release(){log.push("unlease");}};},
  port:()=>port,
  releaseReserve:()=>{log.push("release-reserve");},
  ensureReserve:()=>{log.push("ensure-reserve");},
  ...overrides
 };
 return refreshStaleImageIfQuiescentV1({directory,leaseDirectory,control,ports});
};
test("the scenario corpus is schema-valid and names the root of the image",()=>{
 expect(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/Corpus"},fixture)).toEqual([]);
 expect(nativeImageRootV1(directory)).toContain(directory);
});
for(const row of fixture.scenarios)test("refresh scenario corpus: "+row.name,async()=>{
 const log:string[]=[],receipt=await run(row,log);
 expect(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/Receipt"},receipt)).toEqual([]);
 expect(log).toEqual(row.calls);
 const outcome=receipt.outcome==="refreshed"?(receipt.cured?"refreshed":"refreshed-uncured"):receipt.reason;
 expect(outcome).toBe(row.outcome);
});
test("a refreshed receipt reports the figures the volume gave before and after the cycle",async()=>{
 const row=fixture.scenarios[0]!,log:string[]=[],receipt=await run(row,log);
 expect(receipt).toEqual({version:1,kind:"repository-image-refresh",outcome:"refreshed",beforeAvailableBytes:0,afterAvailableBytes:0,cured:true});
});
test("a failing cycle is a skipped receipt that still gives the lease and the reserve back",async()=>{
 const row=fixture.scenarios[0]!,log:string[]=[];
 const receipt=await run(row,log,{port:()=>({
  async attachedDevice(){return"/dev/disk3";},async detach(){throw Error("hdiutil exited with status 16");},async attach(){},allocatedBytes:()=>0,dispose(){},mountsCanonically:true
 })});
 expect(receipt).toEqual({version:1,kind:"repository-image-refresh",outcome:"skipped",reason:"refresh-failed"});
 expect(log).toEqual(["lease","release-reserve","ensure-reserve","unlease"]);
});
test("cancellation is the only error a refresh attempt raises",async()=>{
 const row=fixture.scenarios[0]!,controller=new AbortController(),cancelled:ResourceLeaseControl={...control,signal:controller.signal};
 controller.abort(Error("cancelled"));
 await expect(refreshStaleImageIfQuiescentV1({directory,leaseDirectory,control:cancelled,ports:{platform:row.platform}})).rejects.toThrow("cancelled");
});
