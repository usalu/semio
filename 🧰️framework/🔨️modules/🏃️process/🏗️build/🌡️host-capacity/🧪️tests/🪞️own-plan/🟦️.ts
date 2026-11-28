/** 🪞️ Contract: a graph plan is never charged for the unrealized reservation of a graph plan its own process already holds (one process never double-charges itself), so a plan whose minimum fits the spare memory is admitted at once, while the same ask from another process still waits behind that reservation.
 * @see ../../📏️decision/🟦️.ts `evaluateHostCapacityV1` */
import {expect,test} from "bun:test";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import {evaluateHostCapacityV1,hostFloorMegabytesV1,readHostCapacityPolicyV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../📏️decision/🟦️.ts";

const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),now=1_000_000_000,total=16384;
const sample=(availableMegabytes:number):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:2,openFiles:100,openFilesMaximum:100000,attachedImages:0,compressedMegabytes:0,unmanagedMegabytes:0} as HostSampleV1);
const held=(pid:number):HostGrantV1=>({id:"held",operation:"graph-plan",jobs:1,cpu:1,reservedMegabytes:4299,admittedAtMs:now-57000,observedMegabytes:117,pid,bootAtMs:0,retainReservation:true,label:"graph-plan"} as HostGrantV1);
const ask=(pid:number):HostWaiterV1=>({arrivedAtMs:now-46000,pid,request:{id:"ask",operation:"graph-plan",priority:"interactive",minimumJobs:1,preferredJobs:1,fixedMegabytes:6090,perJobMegabytes:0,cpuPerJob:1,retainReservation:true,label:"graph-plan",minimumMegabytes:2436} as HostRequestV1});
const host=()=>sample(hostFloorMegabytesV1(policy,sample(0))+1300+4299-117);

test("a plan whose process holds a ramping plan is admitted shrunk with the memory that is spare, not made to wait behind its own reservation",()=>{
 const decision=evaluateHostCapacityV1(policy,host(),[held(100)],[ask(100)],now).decisions[0]!;
 expect(decision.verdict,JSON.stringify(decision)).toBe("admit");
 expect(decision.reservedMegabytes).toBeGreaterThanOrEqual(2436);
 expect(decision.reservedMegabytes).toBeLessThan(6090);
});

test("another process asking the same is not credited the reservation it does not own and waits",()=>{
 expect(evaluateHostCapacityV1(policy,host(),[held(100)],[ask(200)],now).decisions[0]!.verdict).toBe("wait");
});
