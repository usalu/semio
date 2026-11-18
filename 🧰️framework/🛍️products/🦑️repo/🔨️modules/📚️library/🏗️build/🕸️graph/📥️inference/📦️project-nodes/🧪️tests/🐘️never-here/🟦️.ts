/** 🐘️ Contract: an owner is refused up front only by what this machine measured (a recorded cost), never by a declared seed, and a read that infers owners in child processes judges that cost as the physical footprint of a child against the alone cap of a child (the cap the farm itself enforces), not against the managed bytes of the parent; the measured peak of a child is recorded in the unit of the cost table. Measured 2026-11-17 with the declared 512 MB heap of a child: s-dev-composition 858 MB of footprint, a seed said 27952.
 * @see ../../🟨️.mjs
 * @see ../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{STOPS_ATTRIBUTABLE_FROM_MS_V1,graphOwnerEnforcementV1,ownerMeasuredManagedBytesV1}from"../../🟨️.mjs";
import{farmAloneCapV1}from"../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import{recordOwnerCostFileV1}from"../../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";

const MB=1048576,scratch:string[]=[],HEAVY=["@semio-tech/s-dev-composition","@semio-tech/repo-lib","@semio-tech/framework-rs","workspace"];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const withTotal=async<T>(megabytes:number,work:()=>Promise<T>)=>{const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=String(megabytes);try{return await work();}finally{if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}};
const enforce=(root:string,host:number)=>withTotal(host,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,undefined,()=>host*MB*3/4,()=>host));

test("a declared seed never refuses an owner up front: the heavy seeds pass on a 16 GB and on an 8 GB machine, in process and in a child",async()=>{
 const root=mkdtempSync(join(tmpdir(),"never-seed-"));scratch.push(root);
 for(const host of [16384,8192]){
  const enforcement=await enforce(root,host);
  expect(enforcement.neverHere!(HEAVY),`in process on ${host} MB`).toEqual([]);
  expect(enforcement.neverHere!(HEAVY,farmAloneCapV1(host)),`in a child on ${host} MB`).toEqual([]);
 }
});

test("in a child only a recorded STOP above the alone cap refuses an owner up front: a measured cost that was never stopped (a wave estimate, a lax-heap peak) is run alone in a child and judged by the farm itself; in process the managed solo limit still decides",async()=>{
 const root=mkdtempSync(join(tmpdir(),"never-recorded-"));scratch.push(root);
 const path=join(root,".cache/f/graph-costs/owner-costs.3.json"),now=Math.max(Date.now(),STOPS_ATTRIBUTABLE_FROM_MS_V1);
 recordOwnerCostFileV1(path,{name:"@semio-tech/fits",kind:"project",megabytes:1500,seconds:90,rows:10,stopped:false,atMs:now});
 recordOwnerCostFileV1(path,{name:"@semio-tech/noisy",kind:"project",megabytes:3000,seconds:90,rows:10,stopped:false,atMs:now});
 recordOwnerCostFileV1(path,{name:"@semio-tech/over",kind:"project",megabytes:3000,seconds:90,rows:10,stopped:true,atMs:now});
 for(const host of [16384,8192]){
  const enforcement=await enforce(root,host),cap=farmAloneCapV1(host);
  expect(enforcement.neverHere!(["@semio-tech/fits","@semio-tech/noisy","@semio-tech/over","@semio-tech/unknown"],cap),`child on ${host} MB`).toEqual(["@semio-tech/over"]);
 }
 const small=await enforce(root,8192);
 expect(small.neverHere!(["@semio-tech/fits"])).toEqual(["@semio-tech/fits"]);
});

test("the peak a child reports is its physical footprint; the cost table holds managed megabytes, so it is recorded divided by the footprint factor",()=>{
 expect(ownerMeasuredManagedBytesV1(undefined,2)).toBeUndefined();
 expect(ownerMeasuredManagedBytesV1(858*MB,2)).toBe(429*MB);
 expect(ownerMeasuredManagedBytesV1(858*MB,undefined)).toBe(858*MB);
});

test("the footprint a refusal names is the one that decided it: the recorded stops only, never the larger managed peaks of lax-heap runs on bigger hosts; an owner nobody stopped names none",async()=>{
 const root=mkdtempSync(join(tmpdir(),"never-stopped-"));scratch.push(root);
 const path=join(root,".cache/f/graph-costs/owner-costs.3.json"),now=Math.max(Date.now(),STOPS_ATTRIBUTABLE_FROM_MS_V1);
 recordOwnerCostFileV1(path,{name:"@semio-tech/mixed",kind:"project",megabytes:3358,seconds:90,rows:10,stopped:false,atMs:now,hostMegabytes:32768});
 recordOwnerCostFileV1(path,{name:"@semio-tech/mixed",kind:"project",megabytes:1500,seconds:90,rows:10,stopped:true,atMs:now,hostMegabytes:16384});
 recordOwnerCostFileV1(path,{name:"@semio-tech/lax",kind:"project",megabytes:3358,seconds:90,rows:10,stopped:false,atMs:now,hostMegabytes:32768});
 const enforcement=await enforce(root,16384),cap=farmAloneCapV1(16384);
 expect(enforcement.stopped!("@semio-tech/mixed")).toBe(3000);
 expect(enforcement.stopped!("@semio-tech/lax")).toBeUndefined();
 expect(enforcement.stopped!("@semio-tech/unknown")).toBeUndefined();
 expect(enforcement.neverHere!(["@semio-tech/mixed","@semio-tech/lax"],cap)).toEqual([]);
});

test("a stop recorded before the fresh-child rule (an alone job could land on a child full of earlier jobs) proves nothing about the owner: it names no footprint and never refuses",async()=>{
 const root=mkdtempSync(join(tmpdir(),"never-old-stop-"));scratch.push(root);
 const path=join(root,".cache/f/graph-costs/owner-costs.3.json");
 recordOwnerCostFileV1(path,{name:"@semio-tech/old",kind:"project",megabytes:2400,seconds:90,rows:10,stopped:true,atMs:Date.parse("2026-11-17T23:00:00Z")});
 recordOwnerCostFileV1(path,{name:"@semio-tech/new",kind:"project",megabytes:2400,seconds:90,rows:10,stopped:true,atMs:Math.max(Date.now(),STOPS_ATTRIBUTABLE_FROM_MS_V1)});
 const enforcement=await enforce(root,16384),cap=farmAloneCapV1(16384);
 expect(enforcement.stopped!("@semio-tech/old")).toBeUndefined();
 expect(enforcement.neverHere!(["@semio-tech/old","@semio-tech/new"],cap)).toEqual(["@semio-tech/new"]);
});
