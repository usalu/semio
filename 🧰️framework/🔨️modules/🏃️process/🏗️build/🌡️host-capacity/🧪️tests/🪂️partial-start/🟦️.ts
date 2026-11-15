import {expect,test} from "bun:test";
import {evaluateHostCapacityV1,hostFloorMegabytesV1,readHostCapacityPolicyV1} from "../../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";

/** 🪂️ Contract: a graph plan that asks for more memory than is free starts with what is free when that is at least its minimum, whatever the load of the host, as long as nothing of ours runs. */
const policy=readHostCapacityPolicyV1(fixture.policy),base=fixture.cases[0]!;
const decide=(host:object,fixedMegabytes=6553)=>{
 const request={...base.queue[0]!.request,id:"partial001",operation:"graph-plan",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes,perJobMegabytes:0,cpuPerJob:1,minimumMegabytes:2048};
 return evaluateHostCapacityV1(policy,{...base.host,...host} as never,[],[{...base.queue[0]!,request,arrivedAtMs:0,pid:base.queue[0]!.pid} as never],1000).decisions[0]!;
};
const floor=hostFloorMegabytesV1(policy,base.host);
test("5515 MB free above the floor admit a 6553 MB ask shrunk to what is free, on a quiet host and on a loaded one",()=>{
 for(const load of [0.5,40]){
  const host={availableMegabytes:floor+5515+0,load,pressure:"normal"},decision=decide(host);
  expect(decision.verdict,`load ${load}: ${JSON.stringify(decision)}`).toBe("admit");
  expect(decision.reservedMegabytes).toBeGreaterThanOrEqual(2048);
  expect(decision.reservedMegabytes).toBeLessThan(6553);
 }
});

test("a raised memory pressure (the normal state of a busy Mac) does not hold back the minimum start of a plan when nothing of ours runs, and still holds a full ask",()=>{
 const host={availableMegabytes:floor+5515,load:5,pressure:"warn"},decision=decide(host);
 expect(decision.verdict,JSON.stringify(decision)).toBe("admit");
 expect(decision.reason).toBe("shrunk");
 expect(decision.reservedMegabytes).toBeGreaterThanOrEqual(2048);
 expect(decide({...host,availableMegabytes:floor+9000},6553).verdict).toBe("wait");
 expect(decide({...host,availableMegabytes:floor+1000}).verdict).toBe("wait");
});
