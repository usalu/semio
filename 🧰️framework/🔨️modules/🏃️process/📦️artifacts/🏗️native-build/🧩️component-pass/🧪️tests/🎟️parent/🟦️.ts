import {expect} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {uptime} from "node:os";
import {join} from "node:path";
import {registerIsolatedBuildTestV1} from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {artifacts,runStub,stubCargo,stubWorkspace} from "../🎭️stub/🟦️.ts";
import {ParentComponentMemoryRequiredErrorV1} from "../../🧵️job-slots/🟦️.ts";

for(const fleet of[false,true])registerIsolatedBuildTestV1(import.meta.path,fleet?"strict fleet fallback refuses local compilation above the verified parent grant":"a first local final needs the verified parent grant",{bodyMilliseconds:30000,scope:{kind:"body"}},async checkpoint=>{
 const grant=2048,peak=4096;
  const directory=mkdtempSync(join(artifacts,"pg-")),ledger=join(directory,"ledger"),id="native_pass_parent";
  mkdirSync(join(ledger,"grants"),{recursive:true});
  writeFileSync(join(ledger,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:2,cpu:2,reservedMegabytes:grant,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000}));
  const environment={SEMIO_HOST_CAPACITY_DIRECTORY:ledger,SEMIO_HOST_CAPACITY_GRANT:id,SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1,id,jobs:2,cpu:2,memoryMegabytes:grant,fixedMegabytes:256,floorMegabytes:2048})};
  const cargo=stubCargo(directory),pass=stubWorkspace(directory,"workspace",["component-a"],"release");
  let refusal:unknown;
  try{
   await runStub(directory,{version:1,passes:[pass]},cargo,{
    environment,
    localJobs:{maximum:1,peakMegabytes:()=>peak,sample:async()=>({availableMegabytes:16000,ownedMegabytes:0,pressure:"normal"})},
    ...fleet?{fleetJobs:async()=>({jobs:2,local:1,remote:1,reason:"fleet-all" as const}),fleetSlots:async()=>1}:{},
   },"fleet-fallback");
  }catch(error){
   let cause:unknown=error;
   while(cause instanceof Error&&!(cause instanceof ParentComponentMemoryRequiredErrorV1)&&cause.cause!==undefined)cause=cause.cause;
   refusal=cause;
  }
  expect(refusal).toBeInstanceOf(ParentComponentMemoryRequiredErrorV1);
  expect(refusal).toMatchObject({allowanceMegabytes:grant});
  expect((refusal as ParentComponentMemoryRequiredErrorV1).requiredMegabytes).toBeGreaterThan(peak);
  if(fleet){
   const calls=readFileSync(cargo.calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
   expect(calls.filter(row=>row.at)).toHaveLength(1);
   expect(calls.find(row=>row.reply)?.reply).toMatchObject({op:"refused",reason:"fleet-placement"});
  }else expect(existsSync(cargo.calls)).toBe(false);
  checkpoint();
  throw refusal;
},/Component build needs \d+ MiB for the local pass, but the parent granted 2048 MiB/u);
