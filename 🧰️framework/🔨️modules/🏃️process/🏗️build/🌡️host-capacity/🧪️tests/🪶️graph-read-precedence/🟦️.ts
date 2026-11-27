/** 🪶️ Contract: the graph read every waiting run depends on is not held back by those runs. On a 16 GB worker (m08: 5.1 GB available, 2 GB floor, 2.8 GB reserved by a ramping run that only waits for this very read) a read with a shrinkable minimum is served before older batch requests, passes the head-of-line barrier and, once it waited the bypass time, is admitted at its minimum against the reservation of a ramping grant that holds no memory yet (the host warden pauses that work if the memory is needed); a read without a minimum keeps the old order.
 * @see ../../📏️decision/🟦️.ts `decideHostCapacityV1` */
import {expect,test} from "bun:test";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import {decideHostCapacityV1,readHostCapacityPolicyV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../📏️decision/🟦️.ts";

const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),now=1_000_000_000,total=16384;
const host=(availableMegabytes:number):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:2,openFiles:100,openFilesMaximum:100000,attachedImages:0,compressedMegabytes:0} as HostSampleV1);
const ramping=(id:string,megabytes:number):HostGrantV1=>({id,operation:"run-many",jobs:1,cpu:1,reservedMegabytes:megabytes,admittedAtMs:now-5000,observedMegabytes:100,pid:100,bootAtMs:0,label:"run-many"} as HostGrantV1);
const request=(id:string,operation:string,megabytes:number,over:Partial<HostRequestV1>={}):HostRequestV1=>({id,operation,priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1,label:operation,...over} as HostRequestV1);
const waiting=(row:HostRequestV1,waitedMilliseconds:number,pid:number):HostWaiterV1=>({arrivedAtMs:now-waitedMilliseconds,pid,request:row});
const read=(minimum:number|undefined,priority:"batch"|"interactive"="interactive")=>request("read","graph-plan",5918,{priority,retainReservation:true,...minimum===undefined?{}:{minimumMegabytes:minimum,floorMegabytes:Math.min(2048,minimum)}});
const ahead=waiting(request("ahead","run-many",2663),policy.wait.headOfLineMilliseconds!+60000,501);
const decide=(readRequest:HostRequestV1,waited:number,available=5100)=>decideHostCapacityV1(policy,host(available),[ramping("held",2663)],[ahead,waiting(readRequest,waited,502)],now).decisions.find(row=>row.id==="read")!;

test("a read with a shrinkable minimum is admitted at that minimum once it waited the bypass time, although a ramping run reserved memory it does not hold yet and an older request waits ahead",()=>{
 const decision=decide(read(2367),policy.wait.bypassMilliseconds+1000);
 expect(decision,JSON.stringify(decision)).toMatchObject({verdict:"admit",reason:"shrunk"});
 expect(decision.reservedMegabytes).toBeGreaterThanOrEqual(2367);
 expect(decision.reservedMegabytes).toBeLessThan(5918);
});

test("before the bypass time the read waits for the memory like any request, and a read without a minimum never overlaps a ramping reservation",()=>{
 expect(decide(read(2367),1000).verdict).toBe("wait");
 expect(decide(read(undefined),policy.wait.bypassMilliseconds+1000).verdict).toBe("wait");
});

test("the same request under another operation keeps waiting: only the graph read overlaps",()=>{
 const other=request("read","run-many",5918,{minimumMegabytes:2367,floorMegabytes:2048});
 expect(decide(other,policy.wait.bypassMilliseconds+1000).verdict).toBe("wait");
});

test("a read whose minimum does not fit even the real free memory still waits",()=>{
 expect(decide(read(5000),policy.wait.bypassMilliseconds+1000,3000).verdict).toBe("wait");
});

test("a read ranks ahead of older batch requests and the others lose nothing they had",()=>{
 const order=decideHostCapacityV1(policy,host(14000),[],[ahead,waiting(read(2367),500,502)],now).decisions.map(row=>row.id);
 expect(order[0]).toBe("read");
});
