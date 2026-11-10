/** 🧺️ Contract: admission divides only what is left of the machine after the processes that run beside the gate.
 * @see ./🧫️fixtures/🔣️.json
 * @see ../../🧺️unmanaged/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {censusUnmanagedHostV1,evaluateHostCapacityV1,parseHostProcessRowsV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,unmanagedHostMegabytesV1,withUnmanagedHostV1,type HostGrantV1,type HostRequestV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const python=testPythonV1()!,oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));
const model=(mode:string,value:unknown):any=>{const done=Bun.spawnSync([python,"-I",oracle,mode,JSON.stringify(value)]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout));};
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"um-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1(decisionFixture.policy);
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:1,platform:"linux",cores:16,totalMegabytes:16000,availableMegabytes:15000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:100,openFilesMaximum:100000,attachedImages:null,...over});
const request:HostRequestV1={id:"a",operation:"run",priority:"batch",minimumJobs:1,preferredJobs:8,fixedMegabytes:1000,perJobMegabytes:1000,cpuPerJob:1};

test("the unmanaged memory equals the independent model",()=>{
 const expected=model("unmanaged",fixture.unmanaged);
 fixture.unmanaged.forEach((row,index)=>{
  expect(unmanagedHostMegabytesV1(row,row.ledger as never),row.name).toBe(row.expect);
  expect(row.expect,row.name).toBe(expected[index]);
 });
});

test("the census of a process table equals the independent model and leaves granted trees out",()=>{
 const rows=parseHostProcessRowsV1(fixture.processes.table),census=censusUnmanagedHostV1(rows,fixture.processes.grantRoots);
 expect(rows).toHaveLength(8);
 expect(census).toEqual(fixture.processes.expect as never);
 expect(census).toEqual(model("census",fixture.processes));
});

test("admission divides what is left after the unmanaged processes",()=>{
 const decide=(over:Partial<HostSampleV1>)=>evaluateHostCapacityV1(policy,host(over),[],[{request,arrivedAtMs:0,pid:1}],0);
 const free=decide({}),crowded=decide({unmanagedMegabytes:8000});
 expect(free.decisions[0]).toMatchObject({verdict:"admit",jobs:8});
 expect(crowded.decisions[0]).toMatchObject({verdict:"admit",jobs:2,reservedMegabytes:3000});
 expect(crowded.unmanagedMegabytes).toBe(8000);expect(free.unmanagedMegabytes).toBeUndefined();
 expect(crowded.headroomMegabytes).toBeLessThan(free.headroomMegabytes);
});

test("the gate measures the unmanaged memory live from the sample and its grants",async()=>{
 const grant:HostGrantV1={id:"g",operation:"run",jobs:1,cpu:1,reservedMegabytes:5000,admittedAtMs:0,observedMegabytes:3000,pid:1,bootAtMs:0};
 expect(withUnmanagedHostV1(host({availableMegabytes:6000}),[grant]).unmanagedMegabytes).toBe(7000);
 const status=await readHostCapacityStatusV1({directory:fresh(),policy,control:{signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined},sample:async()=>({host:host({availableMegabytes:6000}),swappedOutMegabytes:0})});
 expect(status.evaluation.unmanagedMegabytes).toBe(10000);
});
