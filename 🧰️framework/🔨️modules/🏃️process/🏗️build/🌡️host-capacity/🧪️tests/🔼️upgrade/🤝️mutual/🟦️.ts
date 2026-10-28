import {expect,test} from "bun:test";
import {evaluateHostCapacityV1,readHostCapacityPolicyV1,type HostGrantV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../../../🟦️.ts";
import decisionFixture from "../../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,memory:{...decisionFixture.policy.memory,maximumQuiet:4}});
const host:HostSampleV1={version:1,atMs:0,platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:12000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10};
const quiet=(id:string,arrivedAtMs:number):HostGrantV1=>({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:2048,admittedAtMs:arrivedAtMs,observedMegabytes:1500,pid:arrivedAtMs,bootAtMs:1,quiet:true,arrivedAtMs});
const grow=(id:string,arrivedAtMs:number,megabytes:number):HostWaiterV1=>({request:{id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1} satisfies HostRequestV1,arrivedAtMs,pid:arrivedAtMs,upgrade:true});

test("Two runs whose quiet reservations both have to grow never wait on each other: the earlier grows into what fits now, the later waits for it",()=>{
 const first=quiet("firstaaaa1",1000),second=quiet("secondaaa1",2000),floor=evaluateHostCapacityV1(policy,host,[],[],0).floorMegabytes,free=host.availableMegabytes-floor,wanted=free+2500;
 expect(wanted-first.observedMegabytes!,"each growth alone does not fit beside the other run's quiet reservation").toBeGreaterThan(free);
 expect(wanted,"each growth fits once the other run's quiet reservation is returned").toBeLessThanOrEqual(free+first.observedMegabytes!+second.observedMegabytes!);
 for(const nowMs of [3_000_000,3_000_000+policy.wait.bypassMilliseconds,3_000_000+4*3_600_000]){
  const decisions=evaluateHostCapacityV1(policy,{...host,atMs:nowMs},[first,second],[grow("firstaaaa1",1000,wanted),grow("secondaaa1",2000,wanted)],nowMs).decisions;
  const [earlier,later]=[decisions.find(row=>row.id==="firstaaaa1")!,decisions.find(row=>row.id==="secondaaa1")!];
  expect([earlier.verdict,earlier.reason,earlier.reservedMegabytes],`at ${nowMs} ms the earlier run is not held by a run that itself only waits to grow`).toEqual(["admit","override",first.observedMegabytes!+free]);
  expect(later.verdict,`at ${nowMs} ms the later run waits for the earlier one`).toBe("wait");
 }
 const grown:HostGrantV1={...first,quiet:undefined,reservedMegabytes:first.observedMegabytes!+free,upgradedAtMs:3_000_000,resumedAtMs:3_000_000};
 delete (grown as {quiet?:boolean}).quiet;
 const after=evaluateHostCapacityV1(policy,host,[grown,second],[grow("secondaaa1",2000,wanted)],3_001_000).decisions[0]!;
 expect([after.verdict,after.reason],"once the earlier run grew it really competes: the later run waits for memory").toEqual(["wait","memory"]);
});

test("A quiet run that does not ask to grow still holds an upgrade back: it executes nothing and ends on its own",()=>{
 const first=quiet("firstaaaa1",1000),idle=quiet("idleaaaaa1",2000),floor=evaluateHostCapacityV1(policy,host,[],[],0).floorMegabytes,wanted=host.availableMegabytes-floor+2500;
 const decision=evaluateHostCapacityV1(policy,host,[first,idle],[grow("firstaaaa1",1000,wanted)],3_000_000).decisions[0]!;
 expect([decision.verdict,decision.reason]).toEqual(["wait","memory"]);
});
