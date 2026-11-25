import {afterEach,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../🌡️host-capacity/🎛️control/🟦️.ts";
import {buildRemoteExecutionLoadReportV1,evaluateRemoteExecutionAdmissionV1,remoteExecutionPressureGateV1,REMOTE_EXECUTION_ADMISSION_POLICY_V1 as A,type RemoteExecutionHeldUnitV1,type RemoteExecutionHostViewV1} from "../../🌡️load/🟦️.ts";
import {withReadyStageV1,type ReadyStageClaimV1} from "../../🌡️load/🪑️stage-claim/🟦️.ts";
import {offloadFamilyPeakMegabytesV1,offloadKilledPeakMegabytesV1,offloadLearnPeakMegabytesV1,offloadUnitPeakMegabytesV1} from "../../🚀️offload/📐️policy/🟦️.ts";
import {offloadDeclaredThreadsV1,offloadPeakAtThreadsMegabytesV1,readFamilyPeakV1,readUnitPeakV1,readUnitPeaksV1,writeUnitPeakV1} from "../../🚀️offload/🚦️admission/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../../🟦️.ts";
import {placementHasHeadroomV1} from "../../🗓️scheduler/⚖️placement/🟦️.ts";
import fixture from "./🔣️.json";

const scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const NOW=1_800_000_000_000,SWAP=BUILD_HOST_CAPACITY_POLICY_V1.swap.activeBasisPointsPerSecond;
const F=readRemoteExecutionPolicyV1().worker.codegen.fairShareThreads,EXTRA=readRemoteExecutionPolicyV1().worker.codegen.extraMegabytesPerThread;
const host=(total:number,available:number,pressure:"normal"|"warn"|"critical",swap:number|null)=>({version:1,atMs:NOW,platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes:available,pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:swap,load:null,openFiles:1000,openFilesMaximum:200000,attachedImages:0});
const view=(total:number,available:number,pressure:"normal"|"warn"|"critical"="normal",swap:number|null=0):RemoteExecutionHostViewV1=>({host:host(total,available,pressure,swap),grants:[],waiters:[]} as never);

test("The pressure gate equals the Python oracle: critical refuses every unit, warn alone refuses a large unit or growing swap only",()=>{
 for(const row of fixture.gates)expect(remoteExecutionPressureGateV1(host(16384,8000,row.pressure as "normal",row.swapOutMegabytesPerSecond),row.megabytes,SWAP),JSON.stringify(row)).toBe(row.gate as never);
 expect(remoteExecutionPressureGateV1(host(16384,8000,"warn",0),A.warnMaximumMegabytes,SWAP)).toBe("relaxed");
 expect(remoteExecutionPressureGateV1(host(16384,8000,"warn",0),A.warnMaximumMegabytes+1,SWAP)).toBe("pressure-warn");
});

test("A unit is admitted by its own predicted peak against the measured spare beside the units that run, equal to the Python oracle in every case",()=>{
 for(const row of fixture.admissions){
  const one=evaluateRemoteExecutionAdmissionV1({view:view(row.totalMegabytes,row.availableMegabytes,row.pressure as "normal",row.swapOutMegabytesPerSecond),running:row.held.length,concurrency:99,nowMs:NOW,megabytes:row.megabytes,held:row.held});
  expect(one.spareMegabytes,JSON.stringify(row)).toBe(row.spareMegabytes);
  expect(one.admission.admit?"admit":one.admission.reason,JSON.stringify(row)).toBe(row.verdict);
 }
 const fits=fixture.admissions.filter(row=>row.verdict==="admit"),refused=fixture.admissions.filter(row=>row.verdict==="memory");
 expect(fits.length).toBeGreaterThan(50);expect(refused.length).toBeGreaterThan(10);
 expect(fixture.admissions.some(row=>row.pressure==="warn"&&row.verdict==="admit"),"a small unit passes under warn").toBe(true);
 expect(evaluateRemoteExecutionAdmissionV1({view:view(16384,12000),running:3,concurrency:3,nowMs:NOW,megabytes:256}).admission).toEqual({admit:false,reason:"slots"});
});

test("The load report of a machine under warn pressure with a running stage still offers headroom and reports the spare a unit may take",()=>{
 const now=Date.now(),stage:ReadyStageClaimV1={version:1,stage:"workspace",pid:4242,entitlementMegabytes:4915,observedMegabytes:2600,waiting:false,atMs:now};
 const report=(one:RemoteExecutionHostViewV1,held?:readonly RemoteExecutionHeldUnitV1[])=>buildRemoteExecutionLoadReportV1({origin:"g1-m03",identity:{key:"macos-arm64-none",os:"macos",arch:"arm64",toolchain:null,target:null},view:one,worker:{state:"idle",concurrency:4,running:held?.length??0,sandbox:"isolated-root",confinement:"none",network:"unenforced"},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}},nowMs:now,...held?{held}:{}});
 const warn={...view(16384,6000,"warn",0),host:{...view(16384,6000,"warn",0).host,atMs:now}} as RemoteExecutionHostViewV1,staged=withReadyStageV1(warn,stage,now);
 const row=report(staged);
 expect(row.headroom).toBe(true);
 expect(row.freeMegabytes,"6000 less the 650 the stage may still grow to, less the 2048 floor").toBe(6000-650-2048);
 expect(report(withReadyStageV1(warn,{...stage,waiting:true},now)).freeMegabytes,"a waiting stage holds nothing back").toBe(6000-2048);
 expect(report({...staged,host:{...staged.host,pressure:"critical"}} as RemoteExecutionHostViewV1).headroom).toBe(false);
 expect(report({...staged,host:{...staged.host,swapOutMegabytesPerSecond:52}} as RemoteExecutionHostViewV1).headroom,"growing swap").toBe(false);
 expect(report(staged,[{megabytes:6144,usedMegabytes:0}]).freeMegabytes,"the growth a running unit has not taken yet is the scheduler's charge, not reported here (single charge)").toBe(row.freeMegabytes);
 expect(report(staged,[{megabytes:6144,usedMegabytes:3000}]).freeMegabytes,"what a running unit holds is in the sample already").toBe(row.freeMegabytes);
});

test("The scheduler places a unit where its predicted peak fits into the reported spare, small units where a large one does not fit",()=>{
 const worker={id:"w",name:"w",origin:"g1-m03",properties:[],concurrency:4,running:0,draining:false,refusedUntilMs:0,waiting:true,committedMegabytes:0} as never,entry=(free:number)=>({report:{pressure:"warn",headroom:true,freeMegabytes:free},atMs:NOW} as never);
 expect(placementHasHeadroomV1(worker,entry(1500),NOW,512)).toBe(true);
 expect(placementHasHeadroomV1(worker,entry(1500),NOW,6144)).toBe(false);
});

test("The peak a unit is placed with, what a crate learns from a measurement and from a kill equal the Python oracle",()=>{
 for(const row of fixture.peaks)expect(offloadUnitPeakMegabytesV1(row.learned??undefined,row.wasmLinked?["cdylib"]:["lib"],row.wasmLinked?"wasm32-unknown-unknown":undefined),JSON.stringify(row)).toBe(row.placed);
 for(const row of fixture.learns)expect(offloadLearnPeakMegabytesV1(row.before??undefined,row.measured),JSON.stringify(row)).toBe(row.learned);
 for(const row of fixture.kills)expect(offloadKilledPeakMegabytesV1(row.exit,row.placed,row.learned??undefined),JSON.stringify(row)).toBe(row.taught??undefined);
});

test("A final of a crate never measured is placed with the estimate of its family, equal to the Python oracle, and a kill raises that estimate",()=>{
 const final=["cdylib"],wasm="wasm32-wasip2";
 for(const row of fixture.families){
  const family=offloadFamilyPeakMegabytesV1(row.samples);
  expect(family,JSON.stringify(row)).toBe(row.family??undefined);
  expect(offloadUnitPeakMegabytesV1(undefined,final,wasm,family),JSON.stringify(row)).toBe(row.unseen);
  expect(offloadUnitPeakMegabytesV1(undefined,["lib"],undefined,family),JSON.stringify(row)).toBe(row.unseenNative);
  expect(offloadUnitPeakMegabytesV1(3000,final,wasm,family),JSON.stringify(row)).toBe(row.learned);
 }
 for(const row of fixture.familyKills){
  const taught=offloadKilledPeakMegabytesV1(137,row.placed,undefined);
  expect(taught,JSON.stringify(row)).toBe(row.taught);
  expect(offloadFamilyPeakMegabytesV1(row.samples),JSON.stringify(row)).toBe(row.familyBefore);
  expect(offloadFamilyPeakMegabytesV1([...row.samples,taught!]),JSON.stringify(row)).toBe(row.familyAfter);
  expect(row.familyAfter,JSON.stringify(row)).toBeGreaterThan(row.familyBefore!);
  expect(offloadUnitPeakMegabytesV1(undefined,final,wasm,row.familyAfter),JSON.stringify(row)).toBe(row.unseenAfter);
 }
});

test("The family estimate is read from the recorded peaks of the family's crates on disk, the measured releases give the oracle's value, and a worker's kill of a fresh crate raises it for the next final",()=>{
 const root=mkdtempSync(join(testScratchRootV1("rexec-family-peak"),"scratch-"));scratch.push(root);
 expect(readFamilyPeakV1(root)).toBeUndefined();
 const crates=Object.entries(fixture.observed.crates);
 for(const[crate,megabytes]of crates.slice(0,2))writeUnitPeakV1(root,crate,megabytes,F);
 writeUnitPeakV1(root,"serde",9000,1);
 expect(readFamilyPeakV1(root),"two family crates are too few").toBeUndefined();
 for(const[crate,megabytes]of crates.slice(2))writeUnitPeakV1(root,crate,megabytes,F);
 expect(readFamilyPeakV1(root)).toBe(fixture.observed.family);
 const placed=offloadUnitPeakMegabytesV1(readUnitPeakV1(root,"semio_hub_new_component",F),["cdylib"],"wasm32-wasip2",readFamilyPeakV1(root));
 expect(placed).toBe(fixture.observed.unseen);
 const taught=offloadKilledPeakMegabytesV1(137,placed,readUnitPeakV1(root,"semio_hub_new_component",F));
 expect(taught).toBe(fixture.observed.killed.taught);
 writeUnitPeakV1(root,"semio_hub_new_component",taught!,F,true);
 expect(readFamilyPeakV1(root),"the family estimate rises").toBe(fixture.observed.killed.familyAfter);
 expect(offloadUnitPeakMegabytesV1(readUnitPeakV1(root,"semio_hub_other_component",F),["cdylib"],"wasm32-wasip2",readFamilyPeakV1(root))).toBe(fixture.observed.killed.unseenAfter);
 expect(offloadUnitPeakMegabytesV1(readUnitPeakV1(root,"semio_hub_new_component",F),["cdylib"],"wasm32-wasip2",readFamilyPeakV1(root)),"the killed crate itself").toBe(Math.min(16384,Math.ceil(taught!*1.25)));
});

test("The learned peak of a crate is recorded beside its cost: a larger measurement at once, a smaller one halfway, a kill replaces it",()=>{
 const root=mkdtempSync(join(testScratchRootV1("rexec-unit-peak"),"scratch-"));scratch.push(root);
 expect(readUnitPeakV1(root,"serde",1)).toBeUndefined();
 writeUnitPeakV1(root,"serde",800,1);expect(readUnitPeakV1(root,"serde",1)).toBe(800);
 writeUnitPeakV1(root,"serde",400,1);expect(readUnitPeakV1(root,"serde",1),"halfway down").toBe(600);
 writeUnitPeakV1(root,"serde",2000,1);expect(readUnitPeakV1(root,"serde",1),"up at once").toBe(2000);
 writeUnitPeakV1(root,"serde",9000,1,true);expect(readUnitPeakV1(root,"serde",1),"a kill replaces").toBe(9000);
 writeUnitPeakV1(root,"serde",0,1);expect(readUnitPeakV1(root,"serde",1)).toBe(9000);
 expect(readUnitPeakV1(root,"other",1)).toBeUndefined();
});

test("A crate's peak is learned per codegen thread count: the measurement at a count, else the nearest below plus the worker's price per thread, never above a measurement with more threads, equal to the Python oracle; the ledger keeps every count",()=>{
 for(const row of fixture.threads)expect(offloadPeakAtThreadsMegabytesV1(row.samples as Record<string,number>,row.threads,row.extra)??null,JSON.stringify(row)).toBe(row.peak as never);
 const root=mkdtempSync(join(testScratchRootV1("rexec-thread-peak"),"scratch-"));scratch.push(root);
 writeUnitPeakV1(root,"semio_hub_dag_component",4011,1);
 expect(readUnitPeakV1(root,"semio_hub_dag_component",1)).toBe(4011);
 expect(readUnitPeakV1(root,"semio_hub_dag_component",F),"one thread measured: the fair share is priced per thread").toBe(4011+(F-1)*EXTRA);
 writeUnitPeakV1(root,"semio_hub_dag_component",4500,F);
 expect(readUnitPeaksV1(root,"semio_hub_dag_component")).toEqual({"1":4011,[String(F)]:4500});
 expect(readUnitPeakV1(root,"semio_hub_dag_component",F),"the measurement at the fair share").toBe(4500);
 expect(readUnitPeakV1(root,"semio_hub_dag_component",2),"between two measurements: never above the larger count's").toBe(Math.min(4011+EXTRA,4500));
 writeUnitPeakV1(root,"semio_hub_dag_component",3000,F);
 expect(readUnitPeaksV1(root,"semio_hub_dag_component"),"a smaller measurement moves halfway at its own count only").toEqual({"1":4011,[String(F)]:3750});
 expect(offloadDeclaredThreadsV1(true)).toBe(F);expect(offloadDeclaredThreadsV1(false)).toBe(1);
});

test("The oracle vectors are current",()=>{
 const run=Bun.spawnSync(["python3",join(import.meta.dir,"🐍️.py"),"--check"]);
 expect(run.exitCode,String(run.stderr)).toBe(0);
});

test("While the readiness stage runs a machine lends by measured memory: every slot while the units fit beside the stage's reserve, a unit that does not fit is refused for memory, not for a count",()=>{
 const now=Date.now(),stage:ReadyStageClaimV1={version:1,stage:"workspace",pid:4242,entitlementMegabytes:4915,observedMegabytes:2600,waiting:false,atMs:now};
 const calm={...view(32768,24000),host:{...view(32768,24000).host,atMs:now}} as RemoteExecutionHostViewV1,staged=withReadyStageV1(calm,stage,now);
 const ask=(one:RemoteExecutionHostViewV1,running:number,megabytes=512)=>evaluateRemoteExecutionAdmissionV1({view:one,running,concurrency:4,nowMs:now,megabytes,held:Array.from({length:running},()=>({megabytes:512,usedMegabytes:512}))}).admission;
 expect("stageConcurrency" in A,"no fixed count per stage").toBe(false);
 for(let running=0;running<4;running++)expect(ask(staged,running),"a running stage lends every slot that fits").toEqual({admit:true});
 expect(ask(staged,4)).toEqual({admit:false,reason:"slots"});
 const tight={...view(16384,7000),host:{...view(16384,7000).host,atMs:now}} as RemoteExecutionHostViewV1,tightStaged=withReadyStageV1(tight,stage,now);
 expect(ask(tightStaged,1,4000),"7000 - 650 stage reserve - 2048 floor = 4302 fits a 4000 MB final beside a unit at its peak").toEqual({admit:true});
 expect(ask(tightStaged,1,4400),"4400 does not fit beside the stage's reserve").toEqual({admit:false,reason:"memory"});
});

test("A running unit is charged once by the worker: the part of its peak it has not grown into yet stays free, what it holds is already in the sample",()=>{
 const one=(held:readonly RemoteExecutionHeldUnitV1[],megabytes:number)=>evaluateRemoteExecutionAdmissionV1({view:view(16384,10000),running:held.length,concurrency:8,nowMs:NOW,megabytes,held});
 expect(one([],7000).admission,"10000 - 2048 floor leaves 7952").toEqual({admit:true});
 expect(one([{megabytes:5000,usedMegabytes:1000}],7000).admission,"a final that will still grow by 4000 MB keeps them free").toEqual({admit:false,reason:"memory"});
 expect(one([{megabytes:5000,usedMegabytes:1000}],3900).admission,"7952 - 4000 = 3952 fits 3900").toEqual({admit:true});
 expect(one([{megabytes:5000,usedMegabytes:5000}],5000).admission,"a unit at its peak holds what the sample shows, nothing more (the interactive reserve of the policy leaves 5240 of the budget)").toEqual({admit:true});
 expect(one([{megabytes:5000,usedMegabytes:1000}],256).spareMegabytes).toBe(10000-2048-4000);
});
