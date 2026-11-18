/** 🧮️ Contract: a reservation is what a job needs, not a worst case. The read holds its parent's measured need (3 GB to start, never the 14336 MB its inline inference once peaked at), the memory a process already holds is out of what the host reports available and is never charged again, and the farm of the read gets up to cores minus one children while the memory above the floor carries them, whatever foreign load the host shows; where it does not carry them the waiting line says how many megabytes are short and who holds them, in both languages.
 * @see ../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts */
import {expect,test} from "bun:test";
import {evaluateHostCapacityV1,hostClassBoundV1,keptHostMegabytesV1,sizeHostRequestV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1 as policy} from "../../🟦️.ts";

const now=1_800_000_000_000,gb=1024;
const read=sizeHostRequestV1(policy,{id:"showholder1",operation:"show"})!;
const sample=(totalGb:number,cores:number,freeGb:number,load=51):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores,totalMegabytes:totalGb*gb,availableMegabytes:freeGb*gb,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load,openFiles:null,openFilesMaximum:null,attachedImages:null}) as HostSampleV1;
const parent=(observed:number,reserved=Math.max(read.fixedMegabytes,read.minimumMegabytes??0)):HostGrantV1=>({id:"showholder1",operation:"show",jobs:1,cpu:2,reservedMegabytes:reserved,admittedAtMs:now-120_000,observedMegabytes:observed,pid:4200,bootAtMs:1_700_000_000_000,label:"projects",rampMilliseconds:1_800_000}) as HostGrantV1;
const ask=(cores:number,change:Record<string,unknown>={}):HostWaiterV1=>({request:{id:"farmreq0001",operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:cores-1,fixedMegabytes:0,perJobMegabytes:1536,cpuPerJob:1,elastic:true,label:"graph-farm",holder:"showholder1",...change},arrivedAtMs:now-1000,pid:4300}) as HostWaiterV1;
const jobs=(host:HostSampleV1,grants:HostGrantV1[],waiter:HostWaiterV1)=>{const row=evaluateHostCapacityV1(policy,host,grants,[waiter],now).decisions.find(each=>each.id==="farmreq0001")!;return row.verdict==="admit"?row.jobs:0;};

test("the read starts at its parent's need: three gigabytes, and a learned peak of the old inline inference never lifts it above four",()=>{
 expect(read.fixedMegabytes).toBeLessThanOrEqual(3072);
 const store={version:1 as const,classes:{"show:projects":{samples:[14243,12808,8941,8370,4563].map((peakMegabytes,at)=>({atMs:now-at*3600_000,ratioBasisPoints:20000,activeMilliseconds:600_000,peakMegabytes}))}}};
 expect(hostClassBoundV1(policy,store,["show","show:projects"],now).megabytes).toBeLessThanOrEqual(4096);
});

test("32 GB host, 27 GB free, read parent at 3 GB, load 51: at least nine children of 1.5 GB",()=>{
 expect(jobs(sample(32,10,27),[parent(3072)],ask(10,{essential:true}))).toBeGreaterThanOrEqual(9);
});

test("16 GB host with 9 GB free: at least three children; 8 GB host with 4 GB free: at least the essential one, never zero",()=>{
 expect(jobs(sample(16,8,9),[parent(3072)],ask(8,{essential:true}))).toBeGreaterThanOrEqual(3);
 expect(jobs(sample(8,4,4),[parent(3072)],ask(4,{essential:true}))).toBeGreaterThanOrEqual(1);
 expect(jobs(sample(8,4,2),[parent(3072)],ask(4,{essential:true}))).toBeGreaterThanOrEqual(1);
});

test("memory the parent already holds is not charged twice: a parent that holds more than it reserved takes nothing more from the room",()=>{
 expect(jobs(sample(32,10,27),[parent(5000)],ask(10,{essential:true}))).toBe(jobs(sample(32,10,27),[parent(3072)],ask(10,{essential:true})));
});

test("a request that is not essential never takes the host below the floor, at any free memory",()=>{
 for(const [total,cores] of [[32,10],[16,8],[8,4]] as const)for(let free=1;free<=total;free+=0.5){
  const host=sample(total,cores,free),floor=evaluateHostCapacityV1(policy,host,[parent(3072)],[],now).floorMegabytes,count=jobs(host,[parent(3072)],ask(cores));
  expect(count*1536,`${total} GB, ${free} GB free`).toBeLessThanOrEqual(Math.max(0,host.availableMegabytes-floor));
 }
});

test("a reservation is released to what the job uses: a read that reserved 14336 MB and holds 1875 MB for minutes is charged at three gigabytes, a young or busy one in full",()=>{
 const grant=(observed:number,age:number)=>({...parent(observed,14336),admittedAtMs:now-age});
 expect(keptHostMegabytesV1(policy,grant(1875,600_000),now)).toBe(3072);
 expect(keptHostMegabytesV1(policy,grant(1875,5_000),now)).toBe(14336);
 expect(keptHostMegabytesV1(policy,grant(9000,600_000),now)).toBe(14336);
 expect(keptHostMegabytesV1(policy,grant(2800,600_000),now)).toBe(4200);
 expect(jobs(sample(32,10,17),[grant(1875,600_000)],ask(10,{essential:true}))).toBeGreaterThanOrEqual(7);
});

test("the live read of 12:05: 18290 MB available, nine farm children resident about 4 GB in all, parent 2.2 GB: every child is admitted with more than 4 GB spare",()=>{
 const farm=(id:string,jobs:number,age:number):HostGrantV1=>({id,operation:"exec",jobs,cpu:jobs,reservedMegabytes:jobs*1536,admittedAtMs:now-age,observedMegabytes:0,pid:4300,bootAtMs:1_700_000_000_000,label:"graph-farm"}) as HostGrantV1;
 const read={...parent(6200,3072),reservedMegabytes:3072};
 const ledger=[read,farm("farmgrant01",5,20_000),farm("farmgrant02",3,15_000),{...farm("farmgrant03",1,30_000),holder:"showholder1"} as HostGrantV1];
 const host=sample(32,10,17.86,30),evaluation=evaluateHostCapacityV1(policy,host,ledger,[ask(10,{preferredJobs:1,essential:false})],now);
 expect(evaluation.unrealizedMegabytes).toBeLessThanOrEqual(9*512+3072);
 expect(evaluation.headroomMegabytes).toBeGreaterThan(4096);
 expect(evaluation.decisions[0]).toMatchObject({verdict:"admit"});
 expect(jobs(host,ledger,ask(10))).toBeGreaterThanOrEqual(2);
});
