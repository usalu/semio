import {expect,test} from "bun:test";
import {evaluateHostCapacityV1,hostFloorMegabytesV1,hostSwapStopV1,readHostCapacityPolicyV1,sizeHostRequestV1,type HostActionKindV1,type HostSampleV1} from "../../📏️decision/🟦️.ts";
import library from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import fixture from "./🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1(library.hostCapacity);
const sample=(over:Partial<HostSampleV1>):HostSampleV1=>({version:1,atMs:1_800_000_000_000,platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});

test("The floor and the cap of every host size equal the Python oracle",()=>{
 for(const row of fixture.floors){const floor=hostFloorMegabytesV1(policy,{totalMegabytes:row.totalMegabytes});expect(floor,String(row.totalMegabytes)).toBe(row.floorMegabytes);expect(row.totalMegabytes-floor).toBe(row.capMegabytes);}
 const shares=fixture.floors.map(row=>row.floorMegabytes/row.totalMegabytes);expect(shares[0]!).toBeGreaterThan(shares[3]!);
});

test("Cold request sizes are clamped to a share of the room above the floor and equal the Python oracle on 8, 16, 32 and 64 GB hosts",()=>{
 for(const row of fixture.sizes){
  const request=sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:row.operation,preferredJobs:4,kinds:row.kinds as HostActionKindV1[],targets:row.targets,host:{totalMegabytes:row.totalMegabytes}})!;
  const label=`${row.totalMegabytes} ${row.operation} ${row.kinds.join()} ${row.targets.join()}`;
  expect(request.fixedMegabytes,label).toBe(row.expect.fixedMegabytes);
  expect(request.perJobMegabytes,label).toBe(row.expect.perJobMegabytes);
  expect(request.envelopePerJobMegabytes??null,label).toBe(row.expect.envelopePerJobMegabytes);
  const room=row.totalMegabytes-hostFloorMegabytesV1(policy,{totalMegabytes:row.totalMegabytes}),least=request.minimumMegabytes??request.fixedMegabytes+request.perJobMegabytes;
  expect(least,label+" fits the share").toBeLessThanOrEqual(Math.max(request.minimumMegabytes??0,Math.floor(room*policy.estimates.maximumShareBasisPoints/10000)));
 }
 const end=(total:number)=>sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",kinds:["test"],targets:["test-e2e-fleet"],host:{totalMegabytes:total}})!;
 expect(end(8192).fixedMegabytes+end(8192).perJobMegabytes).toBeLessThan(end(65536).fixedMegabytes+end(65536).perJobMegabytes);
 expect(sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",kinds:["test"],targets:["test-e2e-fleet"]})).toMatchObject({fixedMegabytes:1536,perJobMegabytes:6144});
});

test("Swap stops equal the Python oracle on every host size and never fire on swap that is merely occupied",()=>{
 for(const row of fixture.swaps)expect(hostSwapStopV1(policy,row.host as Pick<HostSampleV1,"totalMegabytes"|"availableMegabytes"|"swapUsedMegabytes"|"swapOutMegabytesPerSecond">)??null,row.name).toBe(row.expect);
 const reached=new Set(fixture.swaps.map(row=>row.expect));expect(reached.has("swap")&&reached.has("swap-activity")&&reached.has(null),"vectors reach every verdict").toBe(true);
 expect(fixture.swaps.filter(row=>row.name.includes("idle with old swap")).every(row=>row.expect===null)).toBe(true);
});

test("A coordinator that always carries 20 GB of swap admits while its memory is plentiful and waits only when swap is worked and memory is short",()=>{
 const request=sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"run",preferredJobs:1,kinds:["build"],host:{totalMegabytes:32768}})!,waiter=[{request,arrivedAtMs:1_800_000_000_000,pid:1}] as never;
 const verdict=(over:Partial<HostSampleV1>)=>evaluateHostCapacityV1(policy,sample({swapUsedMegabytes:20352,...over}),[],waiter,1_800_000_000_000).decisions[0]!;
 expect(verdict({availableMegabytes:15227,swapOutMegabytesPerSecond:30})).toMatchObject({verdict:"admit"});
 expect(verdict({availableMegabytes:15227,swapOutMegabytesPerSecond:0})).toMatchObject({verdict:"admit"});
 expect(verdict({availableMegabytes:4000,swapOutMegabytesPerSecond:30})).toMatchObject({verdict:"wait",reason:"swap"});
 expect(verdict({availableMegabytes:15227,swapOutMegabytesPerSecond:400})).toMatchObject({verdict:"wait",reason:"swap-activity"});
});
