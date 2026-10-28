import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import authored from "../../🔣️policy.json";
import fixture from "../🧫️fixtures/🧩️component-driver/🔣️.json";
import interleavings from "../🧫️fixtures/🧩️component-driver/🔀️interleavings.json";
import {checkHoldAndWaitV1,checkLeaseOrderV1,checkStrictWaitV1,classifyLeaseResourceV1,leaseModesConflictV1,readLeaseOrderPolicyV1} from "../../🟦️.ts";

type StepV1=Readonly<{op:"acquire"|"release";resource:string;mode?:"shared"|"exclusive";waitedMilliseconds?:number}>;
type RowV1=Readonly<{kind:string;held:string;requested:string}>;
const here=dirname(fileURLToPath(import.meta.url)),policyPath=join(here,"..","..","🔣️policy.json"),live=readLeaseOrderPolicyV1(authored);

/** 🧩️ Replays an acquisition sequence through the product's own order and hold-and-wait checks. */
function replay(steps:readonly StepV1[]):RowV1[]{
 const held:{directory:string;resource:string;mode:"shared"|"exclusive";atMs:number;task:number}[]=[],found:RowV1[]=[];
 steps.forEach((step,index)=>{
  if(step.op==="release"){const at=held.findIndex(row=>row.resource===step.resource);if(at>=0)held.splice(at,1);return;}
  const request={directory:fixture.directory,resource:step.resource,mode:step.mode!,nonblocking:false},order=checkLeaseOrderV1(live,held,request),wait=checkHoldAndWaitV1(live,held,request,step.waitedMilliseconds??0);
  if(order)found.push({kind:order.kind,held:order.held,requested:order.requested});
  if(wait)found.push({kind:wait.kind,held:wait.held,requested:wait.requested});
  held.push({directory:fixture.directory,resource:step.resource,mode:step.mode!,atMs:index+1,task:1});
 });
 return found;
}

const oracle=(()=>{const run=spawnSync("python3",["-I",join(here,"🐍️.py"),policyPath],{input:JSON.stringify(fixture),encoding:"utf8"});if(run.status!==0)throw Error("Python lock order oracle failed: "+run.stderr);return JSON.parse(run.stdout) as Record<string,RowV1[]>;})();

for(const row of fixture.sequences)test(`component driver lease sequence ${row.id} matches the product checks and the independent Python oracle`,()=>{
 expect<unknown>(replay(row.steps as readonly StepV1[])).toEqual(row.violations);
 expect<unknown>(oracle[row.id]).toEqual(row.violations);
});

test("the registry reader precedes the compiler writer, which precedes the exclusive registry fetch lease",()=>{
 const rank=(resource:string)=>classifyLeaseResourceV1(live,resource).rank;
 expect(rank("cargo:registry:/h")).toBeLessThan(rank("repo:compiler-writer:/m"));
 expect(rank("repo:compiler-writer:/m")).toBeLessThan(rank("cargo:registry-fetch:/h:-"));
 expect(classifyLeaseResourceV1(live,"cargo:registry-fetch:/h:-").id).toBe("registry-fetch");
 expect(live.allowed.some(edge=>edge.from==="registry-fetch")).toBe(false);
 expect(live.allowed.some(edge=>edge.to==="registry-fetch")).toBe(false);
});

test("the build-directory custody is its own class: its holder may wait for every class but the registry fetch and the inert ones nobody waits for, and no other class may wait for it (it is taken first, before the writer and the cores)",()=>{
 const custody='["semio.cargo.custody-lease/v1","/w/.cache/rust/fixed/m/b/p","prototype"]';
 expect(classifyLeaseResourceV1(live,custody).id).toBe("cargo-custody");
 expect(classifyLeaseResourceV1(live,'["semio.cargo.build-lease/v1","/b","prototype"]').id).toBe("cargo-coordination");
 expect(live.allowed.filter(edge=>edge.from==="cargo-custody").map(edge=>edge.to).sort()).toEqual(live.classes.filter(row=>row.id!=="registry-fetch"&&row.inert!==true).map(row=>row.id).sort());
 expect(live.allowed.filter(edge=>edge.to==="cargo-custody").map(edge=>edge.from)).toEqual(["cargo-custody"]);
 expect(live.allowed.some(edge=>edge.from==="registry-fetch"||edge.to==="registry-fetch")).toBe(false);
});

const owners=["🏗️build/🧩️driver-stages/🟦️.ts","📦️model/🟦️.ts"].map(path=>join(here,"..","..","..","..","..","..","🛍️products","💻️os","🔨️modules","🔌️plugin",path));

test("component builds hold the compiler writer shared so they run beside ordinary Cargo commands; exclusive writer custody stays with maintenance",()=>{
 expect(leaseModesConflictV1("shared","shared")).toBe(false);
 expect(leaseModesConflictV1("exclusive","shared")).toBe(true);
 for(const path of owners){const text=readFileSync(path,"utf8");expect(text.includes("withCompilerWorkspaceWriteV1"),path).toBe(false);expect(text.includes("withCompilerWorkspaceSharedWriteV1"),path).toBe(true);}
});

test("the warm component driver compiles in a private lane of its agent, never in the shared build directory: its shared target and build and the pass inputs it requests come from one lane, so its build-directory custody is never contended by another driver",()=>{
 const text=readFileSync(owners[0]!,"utf8");
 expect(text.includes("withRepositoryComponentLaneV1({storage:store.storage,root:store.root,workspace:repoRoot,environment:options.environment,control}")).toBe(true);
 expect(text.includes("shared:{target:lane.target,build:lane.build}")).toBe(true);
 expect(text.includes("build:lane.build,target:lane.target")).toBe(true);
 expect(text.includes("CARGO_TARGET_DIR!,build:store.buildDirectory")).toBe(false);
 expect(text.includes("shared:{target:store.environment.CARGO_TARGET_DIR")).toBe(false);
 expect(text.includes("build:store.buildDirectory")).toBe(false);
});

const deadlocks=(()=>{const run=spawnSync("python3",["-I",join(here,"🔀️interleaving-oracle/🐍️.py")],{input:JSON.stringify(interleavings),encoding:"utf8",maxBuffer:1<<26});if(run.status!==0)throw Error("Python interleaving oracle failed: "+run.stderr);return JSON.parse(run.stdout) as Record<string,{states:number;deadlock:boolean}>;})();

/** 🔀️ The tasks of a scenario whose acquisition sequence the strict wait pairs of the authored policy would refuse once a wait passes the grace period. */
function refusedTasks(scenario:(typeof interleavings.scenarios)[number]):string[]{
 return scenario.processes.filter(process=>{
  const held:{directory:string;resource:string;mode:"shared"|"exclusive";atMs:number;task:number}[]=[];
  for(const [index,step] of process.steps.entries()){
   const request={directory:interleavings.directory,resource:step.resource,mode:step.mode as "shared"|"exclusive",nonblocking:false};
   if(checkStrictWaitV1(live,held,request,20_000))return true;
   held.push({directory:interleavings.directory,resource:step.resource,mode:request.mode,atMs:index+1,task:1});
  }
  return false;
 }).map(process=>process.name);
}

for(const scenario of interleavings.scenarios)test(`interleavings of ${scenario.id}: the Python state-space search and the strict wait pairs agree`,()=>{
 expect(deadlocks[scenario.id]!.deadlock,"oracle").toBe(scenario.deadlock);
 const refused=refusedTasks(scenario);
 if(scenario.deadlock)expect(refused.length,"a deadlocking mix contains a task the product refuses").toBeGreaterThan(0);
 if(scenario.id==="declared-order-only"||scenario.id==="registry-before-writer")expect(refused).toEqual([]);
 if(scenario.id==="fetch-first-only")expect(refused.length).toBeGreaterThan(0);
});
