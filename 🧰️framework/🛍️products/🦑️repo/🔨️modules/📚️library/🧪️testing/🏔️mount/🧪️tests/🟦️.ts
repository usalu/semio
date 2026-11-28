/** 🏔️ The fail-fast mount runner: language-neutral vectors agree with the Python oracle, the plan schema agrees with Ajv, and the real runner aborts at the first failing phase with an English and German reason, enforces per-phase deadlines and stops on cancellation. */
import Ajv2020 from "ajv/dist/2020.js";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {describe,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import vectors from "./🔣️.json" with {type:"json"};
import {assertMountedV1,foldMountPhasesV1,MountRefusalV1,mountProgressLineV1,parseMountPlanV1,runMountPlanV1,type MountObservationV1,type MountPlanV1} from "../🟦️.ts";

const oracle=():{fold:{passed:boolean;statuses:string[];failure:{phase:string;code:string}|null}[];plans:boolean[]}=>{
 const result=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(vectors),encoding:"utf8"});
 if(result.status!==0)throw Error(result.stderr);
 return JSON.parse(result.stdout);
};
const sleep=(ms:number,signal:AbortSignal)=>new Promise<void>((resolve,reject)=>{const timer=setTimeout(resolve,ms);signal.addEventListener("abort",()=>{clearTimeout(timer);reject(Error("aborted"));},{once:true});});
const plan=(phases:[string,number][]):MountPlanV1=>parseMountPlanV1({version:1,app:"forms",phases:phases.map(([id,budgetMs])=>({id,budgetMs}))});

describe("mount fold",()=>{
 const python=oracle();
 test("every vector folds the same in TypeScript and in the Python oracle",()=>{
  vectors.fold.forEach((row,index)=>{
   const receipt=foldMountPhasesV1(row.plan as MountPlanV1,row.observations as MountObservationV1[]),got:{passed:boolean;statuses:string[];failure:{phase:string;code:string}|null}={passed:receipt.passed,statuses:receipt.phases.map(phase=>phase.status),failure:receipt.failure===undefined?null:{phase:receipt.failure.phase,code:receipt.failure.code}};
   expect(got,row.name).toEqual({passed:row.passed,statuses:row.statuses,failure:row.failure});
   expect(python.fold[index],row.name+" python").toEqual(got);
  });
 });
 test("plan admission agrees with the Python oracle, and the schema agrees with Ajv on everything but the policy totals",()=>{
  const ajv=new Ajv2020({strict:false}).compile({...schema,$ref:"#/definitions/Plan"});
  vectors.plans.forEach((row,index)=>{
   let admitted=true;try{parseMountPlanV1(row.plan);}catch{admitted=false;}
   expect(admitted,row.name).toBe(row.valid);
   expect(python.plans[index],row.name+" python").toBe(row.valid);
   if(row.valid)expect(ajv(row.plan),row.name+" ajv").toBe(true);
   if(["unknown field","no phases","budget below the floor"].includes(row.name))expect(ajv(row.plan),row.name+" ajv").toBe(false);
  });
 });
 test("a failure names the app, the phase and the budget in English and German",()=>{
  const receipt=foldMountPhasesV1(plan([["shell",200]]),[{kind:"deadline",elapsedMs:200}]);
  expect(receipt.failure?.en).toBe("forms: phase shell did not finish within its budget of 200 ms");
  expect(receipt.failure?.de).toBe("forms: Phase shell endete nicht innerhalb ihres Budgets von 200 ms");
  expect(()=>assertMountedV1(receipt)).toThrow("forms: phase shell did not finish within its budget of 200 ms / forms: Phase shell endete");
  expect(mountProgressLineV1("de","forms",receipt.phases[0]!,0,1)).toBe("forms: Phase 1/1 shell failed in 200 ms");
 });
});

describe("mount runner",()=>{
 test("runs the phases in order and passes",async()=>{
  const seen:string[]=[];
  const receipt=await runMountPlanV1(plan([["navigate",500],["shell",500]]),{navigate:async(log:string[])=>{log.push("navigate");},shell:async(log:string[])=>{log.push("shell");}},seen);
  expect(receipt.passed).toBe(true);
  expect(seen).toEqual(["navigate","shell"]);
 });
 test("the first refusal aborts the run: later phases never start",async()=>{
  const started:string[]=[];
  const receipt=await runMountPlanV1(plan([["navigate",500],["shell",500],["settle",500]]),{navigate:async()=>{started.push("navigate");},shell:async()=>{started.push("shell");throw Error("data-shell-error: guest trapped");},settle:async()=>{started.push("settle");}},null);
  expect(started).toEqual(["navigate","shell"]);
  expect(receipt.phases.map(row=>row.status)).toEqual(["passed","failed","skipped"]);
  expect(receipt.failure).toMatchObject({phase:"shell",code:"refused",detail:"data-shell-error: guest trapped"});
  expect(receipt.failure?.de).toContain("Phase shell schlug nach");
 });
 test("a hanging phase fails at its own deadline, not at the total, and its signal aborts",async()=>{
  let aborted=false;const started=performance.now();
  const receipt=await runMountPlanV1(plan([["shell",120],["settle",5000]]),{shell:async(_:null,signal)=>{await sleep(10_000,signal).catch(()=>{aborted=signal.aborted;});},settle:async()=>{}},null);
  expect(performance.now()-started).toBeLessThan(1500);
  expect(receipt.failure).toMatchObject({phase:"shell",code:"deadline"});
  expect(receipt.phases[1]!.status).toBe("skipped");
  await Bun.sleep(5);
  expect(aborted).toBe(true);
 });
 test("cancellation stops the running phase and reports it",async()=>{
  const controller=new AbortController(),lines:string[]=[];
  setTimeout(()=>controller.abort(),50);
  const receipt=await runMountPlanV1(plan([["shell",5000],["settle",500]]),{shell:(_:null,signal)=>sleep(10_000,signal),settle:async()=>{}},null,{signal:controller.signal,onPhase:(row,index,count)=>lines.push(mountProgressLineV1("en","forms",row,index,count))});
  expect(receipt.failure?.code).toBe("cancelled");
  expect(receipt.phases[0]!.elapsedMs).toBeLessThan(1000);
  expect(lines).toHaveLength(1);
  expect(lines[0]).toStartWith("forms: phase 1/2 shell failed in");
 });
 test("a bilingual refusal keeps each language's own detail",async()=>{
  const receipt=await runMountPlanV1(plan([["landmarks",200]]),{landmarks:async()=>{throw new MountRefusalV1('no visible element is region "Design"','kein sichtbares Element ist region "Entwurf"');}},null);
  expect(receipt.failure?.en).toEndWith(': no visible element is region "Design"');
  expect(receipt.failure?.de).toEndWith(': kein sichtbares Element ist region "Entwurf"');
 });
 test("a plan whose steps differ from its phases is refused before anything runs",async()=>{
  await expect(runMountPlanV1(plan([["shell",100]]),{settle:async()=>{}},null)).rejects.toThrow("phases shell, steps settle");
 });
});
