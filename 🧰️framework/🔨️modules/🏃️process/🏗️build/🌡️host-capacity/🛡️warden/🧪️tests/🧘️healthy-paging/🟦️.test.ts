import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {decideHostWardenV1,hostWardenHoldsV1,HOST_WARDEN_POLICY_V1,type HostWardenHostV1,type HostWardenPreviousV1,type HostWardenSubjectV1} from "../../🟦️.ts";
import sample from "./🔣️.json";

const policy=HOST_WARDEN_POLICY_V1;

/** 🛁️ Runs the rows of healthy paging against a real shortage through the decision, tick by tick, and asks the gate whether a 256 MB unit is admitted at the last tick. */
const run=(item:(typeof sample.cases)[number])=>{
 let previous:HostWardenPreviousV1={level:"calm",calmTicks:0},cancelled=false,last!:ReturnType<typeof decideHostWardenV1>,host!:HostWardenHostV1;
 for(const rate of item.swaps){
  host={...item.host,swapOutMegabytesPerSecond:rate,intervalMilliseconds:item.intervalMilliseconds} as HostWardenHostV1;
  last=decideHostWardenV1(policy,host,item.subjects as HostWardenSubjectV1[],previous);
  previous={level:last.level,calmTicks:last.calmTicks,swapMilliseconds:last.swapMilliseconds};
  cancelled=cancelled||last.actions.some(action=>action.action==="cancel");
 }
 const state={hold:last.hold,atMs:1000,level:last.level,guardMegabytes:last.guardMegabytes,host:{availableMegabytes:host.availableMegabytes}};
 return{level:last.level,trigger:last.trigger,cancel:cancelled,admitted:!hostWardenHoldsV1(policy,state,sample.unitMegabytes,1000,"cargo")};
};

test("healthy macOS paging admits and a real shortage cancels, as the rows and the independent oracle say",()=>{
 const oracle=Bun.spawnSync([testPythonV1()??Bun.which("python3")??"python3","-I",fileURLToPath(new URL("../🐍️.py",import.meta.url)),"--paging"]);
 expect(oracle.exitCode).toBe(0);
 const expected=JSON.parse(oracle.stdout.toString()) as unknown[];
 expect(expected).toHaveLength(sample.cases.length);
 sample.cases.forEach((item,index)=>{
  expect<unknown>(run(item),item.name).toEqual(item.expect);
  expect(expected[index],item.name).toEqual(item.expect);
 });
});

test("the shipped numbers sit above the measured healthy paging and below the measured thrash",()=>{
 expect(policy.swapActiveMegabytesPerSecond).toBeGreaterThan(7.5*4);
 expect(policy.swapActiveMegabytesPerSecond).toBeLessThan(285);
 expect(policy.swapSustainedSeconds).toBeGreaterThanOrEqual(10);
 expect(policy.compressedGuardBasisPoints).toBeGreaterThan(Math.ceil(4800*10000/16384));
 expect(policy.compressedDangerBasisPoints).toBeLessThan(Math.floor(6500*10000/16384));
});
