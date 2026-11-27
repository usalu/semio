/** 🧮️ Contract: the gate never admits more than the machine can carry (what is admitted beside the grants already held never exceeds the total less the floor less what the machine uses beside the gate), whatever the queue, and a request that only grows a held grant (it names a minimum) waits while there is no room and takes no more than the room once there is.
 * @see ../../📏️decision/🟦️.ts `decideHostCapacityV1` */
import {expect,test} from "bun:test";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import {decideHostCapacityV1,hostFloorMegabytesV1,hostInteractiveReserveMegabytesV1,readHostCapacityPolicyV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../📏️decision/🟦️.ts";

const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),now=1_000_000_000,total=16384;
const sample=(availableMegabytes:number,unmanagedMegabytes:number):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:2,openFiles:100,openFilesMaximum:100000,attachedImages:0,compressedMegabytes:0,unmanagedMegabytes} as HostSampleV1);
const grant=(id:string,reservedMegabytes:number,observedMegabytes:number|null,over:Partial<HostGrantV1>={}):HostGrantV1=>({id,operation:"graph-plan",jobs:1,cpu:1,reservedMegabytes,admittedAtMs:now-600000,observedMegabytes,pid:100,bootAtMs:0,retainReservation:true,label:"graph-plan",...over} as HostGrantV1);
const waiter=(id:string,megabytes:number,minimumMegabytes?:number):HostWaiterV1=>({arrivedAtMs:now-1000,pid:500,request:{id,operation:"graph-plan",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1,retainReservation:true,label:"graph-plan",...minimumMegabytes===undefined?{}:{minimumMegabytes}} as HostRequestV1});

test("whatever the ledger, the host and the queue, what is admitted beside the held grants never exceeds the memory the machine can carry",()=>{
 let seed=7;const random=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed/0x7fffffff;},pick=(count:number)=>Math.floor(random()*count);
 let admittedRounds=0;
 for(let round=0;round<4000;round++){
  const unmanaged=pick(3000),host=sample(500+pick(12000),unmanaged);
  const ledger=Array.from({length:pick(6)},(_,index)=>{const reserved=500+pick(5000);return grant(`g${index}`,reserved,random()<0.3?null:pick(reserved+1500),{pid:100+index});});
  const queue=Array.from({length:1+pick(3)},(_,index)=>{const megabytes=500+pick(4000);return waiter(`w${index}`,megabytes,random()<0.5?Math.min(256,megabytes):undefined);});
  const decisions=decideHostCapacityV1(policy,host,ledger,queue,now).decisions.filter(row=>row.verdict==="admit");
  const held=ledger.reduce((sum,each)=>sum+Math.max(each.retainReservation===true?each.reservedMegabytes:0,each.observedMegabytes??each.reservedMegabytes),0),added=decisions.reduce((sum,row)=>sum+row.reservedMegabytes,0),carry=total-hostFloorMegabytesV1(policy,host)-unmanaged;
  if(added>0){admittedRounds++;expect(held+added,`round ${round}: ${held} held plus ${added} admitted against ${carry} MB`).toBeLessThanOrEqual(carry);}
 }
 expect(admittedRounds,"the property was exercised").toBeGreaterThan(200);
});

test("a request that grows a held grant waits while no memory is free and then takes at most the room, never more than it asked",()=>{
 const base=sample(0,0),carry=total-hostFloorMegabytesV1(policy,base)-hostInteractiveReserveMegabytesV1(policy,base),host=sample(total-carry-1000+hostInteractiveReserveMegabytesV1(policy,base),0),full=[grant("held",carry-1000,carry-1000)];
 const crowded=decideHostCapacityV1(policy,host,[grant("held",carry-100,carry-100)],[waiter("grow",1200,256)],now).decisions[0]!;
 expect(crowded).toMatchObject({verdict:"wait",reservedMegabytes:0,reason:"memory"});
 const roomy=decideHostCapacityV1(policy,host,full,[waiter("grow",1200,256)],now).decisions[0]!;
 expect(roomy,JSON.stringify(roomy)).toMatchObject({verdict:"admit"});
 expect(roomy.reservedMegabytes).toBeGreaterThanOrEqual(256);
 expect(roomy.reservedMegabytes).toBeLessThanOrEqual(1000);
});

test("no starvation: a large request that waited past the head-of-line bound is admitted once the holders release, however many small requests keep arriving",()=>{
 const base=sample(0,0),carry=total-hostFloorMegabytesV1(policy,base)-hostInteractiveReserveMegabytesV1(policy,base),bound=policy.wait.headOfLineMilliseconds!;
 expect(bound,"the shipped policy names the head-of-line bound").toBeGreaterThan(0);
 const large=(arrived:number):HostWaiterV1=>({...waiter("large",6553,6553),arrivedAtMs:arrived,pid:900}),small=(id:string,arrived:number):HostWaiterV1=>({...waiter(id,800,256),arrivedAtMs:arrived,pid:901});
 const host=sample(total-carry-3000+hostInteractiveReserveMegabytesV1(policy,base)+1500,0),holders=[grant("held",carry-3000,carry-3000)];
 const before=decideHostCapacityV1(policy,host,holders,[large(now-1000),small("s1",now-500)],now);
 expect(before.decisions.find(row=>row.id==="s1")!.verdict,"before the bound a small request may pass a waiting large one").toBe("admit");
 const aged=decideHostCapacityV1(policy,host,holders,[large(now-bound-1),small("s2",now-500),small("s3",now-400)],now);
 expect(aged.decisions.find(row=>row.id==="large")).toMatchObject({verdict:"wait",reason:"memory"});
 for(const id of["s2","s3"])expect(aged.decisions.find(row=>row.id===id),`${id} must not take the room the head waits for`).toMatchObject({verdict:"wait",reason:"queue-order"});
 const released=decideHostCapacityV1(policy,sample(total-carry+hostInteractiveReserveMegabytesV1(policy,base),0),[],[large(now-bound-1),small("s4",now-500)],now);
 expect(released.decisions.find(row=>row.id==="large")!.verdict,"the head is admitted as soon as the holders have released").toBe("admit");
 expect(released.decisions.find(row=>row.id==="large")!.reservedMegabytes).toBe(6553);
});

test("a full booking that can not be met starts reduced once it waited past the bound or once the host could never give it, and its later growth keeps the age of the first request",()=>{
 const base=sample(0,0),reserve=hostInteractiveReserveMegabytesV1(policy,base),carry=total-hostFloorMegabytesV1(policy,base)-reserve,bound=policy.wait.headOfLineMilliseconds!;
 const booked=(arrived:number,extra:Record<string,number>={}):HostWaiterV1=>{const each=waiter("booked",5143,5143);return{...each,arrivedAtMs:arrived,pid:900,request:{...each.request,floorMegabytes:2048,...extra} as HostRequestV1};};
 const holders=[grant("held",carry-4500,carry-4500)],host=sample(total-carry-2900+reserve+4500,0);
 const young=decideHostCapacityV1(policy,host,[grant("held",carry-4500,3000)],[booked(now-1000)],now).decisions[0]!;
 const aged=decideHostCapacityV1(policy,host,holders,[booked(now-bound-1)],now).decisions[0]!;
 expect(aged.verdict,JSON.stringify(aged)).toBe("admit");
 expect(aged.reservedMegabytes,"a reduced start: at least the floor, at most what is free").toBeGreaterThanOrEqual(2048);
 expect(aged.reservedMegabytes).toBeLessThan(5143);
 const patient=decideHostCapacityV1(policy,host,holders,[booked(now-bound-1,{expectedMilliseconds:bound*10})],now).decisions[0]!;
 expect(patient.verdict,"an operation expected to run long waits at least that long for its full booking").toBe("wait");
 const never=decideHostCapacityV1(policy,sample(total-carry-2900+reserve,0),[],[booked(now-1000)],now).decisions[0]!;
 expect(young.verdict==="wait"||young.reservedMegabytes===5143,"a young request is not reduced while the host can still give its booking").toBe(true);
 expect(never.verdict).toBe("admit");
 const growth:HostWaiterV1={...waiter("growth",900,256),arrivedAtMs:now-100,pid:901,request:{...waiter("growth",900,256).request,queuedSinceMs:now-bound-1} as HostRequestV1},newer=waiter("newer",800,256);
 const tight=[grant("held",carry-3000,carry-3000)];

 const ordered=decideHostCapacityV1(policy,sample(hostFloorMegabytesV1(policy,base)+1000,0),tight,[{...newer,arrivedAtMs:now-30000},growth],now).decisions;
 expect(ordered.find(row=>row.id==="newer")!.verdict,"the growth of an old booking is the head of the line, a request that queued later waits").toBe("wait");
});
