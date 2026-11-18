import{machinePathV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {spawnSync} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {tmpdir} from "node:os";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join,resolve,sep} from "node:path";
import {fileURLToPath} from "node:url";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {hostProfileV1} from "../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import {PLAN_STAGE_POLICY_V1,planStageLimitBytesV1} from "../../🏗️build/🧠️memory/🪜️plan-stage/🟨️.mjs";
import {ownerPoolChildLimitsV1} from "../../🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import {graphEntryKeyV1} from "../../🏗️build/🪪️graph-state/🛰️exchange/🟦️.ts";
import type {GraphStateV1} from "../../🏗️build/🪪️graph-state/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored numbers of the rehearsal. @see ./🧬️schema/🔣️.json */
export const REHEARSAL_POLICY_V1=schema.definitions.Policy.const;
const P=REHEARSAL_POLICY_V1,MEGABYTE=1048576,sha=(text:string)=>createHash("sha256").update(text).digest("hex");

/** 🎛️ What a rehearsal is asked to do. */
export type RehearsalOptionsV1=Readonly<{machines:number;memoryMegabytes:16384|8192;cores:number;owners:number;ownerSeconds:number;ownerMegabytes?:number;scenario:"steady"|"landing";landingAfterSeconds?:number;landingMovesRevision?:boolean;timeoutSeconds:number;out?:string;sampleMilliseconds?:number}>;
/** 📖️ Reads the options of a rehearsal and refuses anything the schema does not declare. */
export function readRehearsalOptionsV1(value:unknown):RehearsalOptionsV1{
 const{sampleMilliseconds:_ignored,...declared}=(value??{}) as Record<string,unknown>;
 const problems=validateJsonSchemaSubset(schema.definitions.Options,declared,schema);
 if(problems.length>0)throw Error("Closed rehearsal options required: "+JSON.stringify(problems).slice(0,300));
 return Object.freeze(value as RehearsalOptionsV1);
}

/** 🪪️ The state an owner of the rehearsal is stored under (the shape the owner pool uses for its own entries), by the revision of the record code. */
export function rehearsalOwnerStateV1(name:string,revision:string):GraphStateV1{
 const zero="0".repeat(64);
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("rehearsal:"+name),implementation:zero,platform:"neutral",extra:Object.freeze({})}),names:zero,content:zero,files:0,contentFiles:0,digest:sha(JSON.stringify(["rehearsal",name,revision]))}) as unknown as GraphStateV1;
}

/** 🖥️ One simulated machine: its identity and every directory it owns. */
export type RehearsalMachinePlanV1=Readonly<{origin:string;index:number;id:string;work:string;state:string;temporary:string;runtime:string;capacity:string;registry:string;store:string;memoryMegabytes:number;cores:number}>;
/** 🗂️ Names the machines of a run: each one has its own state directory (what `.cache/f` is on a worker), temporary directory, runtime directory of the resident service and capacity directory; the store is the one shared directory. */
export function rehearsalMachinePlansV1(options:RehearsalOptionsV1,id:string,work:string):readonly RehearsalMachinePlanV1[]{
 return Array.from({length:options.machines},(_,at)=>{
  const index=at+1,origin=`${P.originPrefix}-m${index}`,home=join(work,"machines",`m${index}`);
  return Object.freeze({origin,index,id,work,state:join(home,"state"),temporary:join(home,"tmp"),runtime:join(home,"run"),capacity:join(home,"capacity"),registry:join(home,"leases"),store:join(work,"store"),memoryMegabytes:options.memoryMegabytes,cores:options.cores});
 });
}

/** 🌿️ The environment of one machine: the identity and directories above, the declared memory and cores (the seam that every limit of the build derives from), no launchd service, no cache credential. */
export function rehearsalEnvironmentV1(plan:RehearsalMachinePlanV1,base:Readonly<Record<string,string|undefined>>=process.env):Record<string,string>{
 const kept=Object.fromEntries(Object.entries(base).filter(([name,value])=>value!==undefined&&!/^SEMIO_(REMOTE_CACHE|FLEET|HOST_CAPACITY|REHEARSAL)/u.test(name)&&name!=="TMPDIR"&&name!=="XDG_RUNTIME_DIR")) as Record<string,string>;
 return{...kept,SEMIO_FLEET_ORIGIN:plan.origin,SEMIO_FLEET_GROUP:P.group,SEMIO_FLEET_STORE_ROOT:plan.store,SEMIO_FLEET_ROOT:plan.work,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:String(plan.memoryMegabytes),SEMIO_HOST_CAPACITY_CORES:String(plan.cores),SEMIO_HOST_CAPACITY_DIRECTORY:plan.capacity,SEMIO_LEASE_REGISTRY_DIRECTORY:plan.registry,SEMIO_MACHINE_STATE_DIRECTORY:plan.state,SEMIO_SERVICE_LAUNCHD:"off",SEMIO_REHEARSAL_ID:plan.id,SEMIO_REHEARSAL_MACHINE:plan.origin,TMPDIR:plan.temporary,XDG_RUNTIME_DIR:plan.runtime};
}

/** 🪞️ The machine a process plans for under this environment: the real host shrunk to the declared memory and cores, which is what every limit of the build derives from. */
export const rehearsalHostV1=(environment:Readonly<Record<string,string|undefined>>)=>hostProfileV1(environment);

/** 🧱️ The limits a worker of this size gets, read from the build's own seams: the footprint limit of a plan (`treeShare` of the declared machine, the 6553 MB of a 16 GB worker), the managed bytes a plan may add (`planStageLimitBytesV1`) and the limits of the pool's children. */
export function rehearsalLimitsV1(memoryMegabytes:number):Readonly<{planMegabytes:number;managedMegabytes:number;child:Readonly<{recycleMegabytes:number;hardMegabytes:number}>}>{
 return Object.freeze({planMegabytes:Math.floor(memoryMegabytes*PLAN_STAGE_POLICY_V1.treeShare),managedMegabytes:Math.round(planStageLimitBytesV1({SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:String(memoryMegabytes)},memoryMegabytes)/MEGABYTE),child:ownerPoolChildLimitsV1(memoryMegabytes)});
}

/** 🔬️ One process of this run read from the process table. */
export type RehearsalProcessV1=Readonly<{pid:number;ppid:number;rssMegabytes:number;cpu:number;machine:string}>;
/** 🔎️ The rows of `ps -A -o pid=,ppid=,rss=,pcpu=,command=` that carry the marker of this run in their arguments (an environment cannot be read on every system, arguments can). */
export function markedProcessRowsV1(text:string,id:string):readonly RehearsalProcessV1[]{
 const rows:RehearsalProcessV1[]=[];
 for(const line of text.split("\n")){
  const match=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+([\d.]+)\s+(.*)$/u.exec(line);
  if(match===null||!match[5]!.includes(`${P.markerFlag}${id}`))continue;
  const machine=new RegExp(`${P.machineFlag}(\\S+)`,"u").exec(match[5]!)?.[1]??"";
  rows.push({pid:Number(match[1]),ppid:Number(match[2]),rssMegabytes:Number(match[3])/1024,cpu:Number(match[4]),machine});
 }
 return rows;
}
const processTable=():string=>{const run=spawnSync("ps",["-A","-ww","-o","pid=,ppid=,rss=,pcpu=,command="],{encoding:"utf8",maxBuffer:64*MEGABYTE});return run.status===0?run.stdout:"";};

/** 🔖️ One sample of one machine. */
export type RehearsalSampleV1=Readonly<{atSeconds:number;machine:string;ledger:Readonly<{total:number;present:number;fetched:number;here:number;wanted:number;done:number}>;pool:Readonly<{state:string;reason:string|null;running:number;open:number}>;footprintMegabytes:number;cpu:number}>;
/** 🧾️ The verdict of a run. */
export type RehearsalVerdictV1=Readonly<{pass:boolean;allReady:boolean;readyAtSeconds:Readonly<Record<string,number>>;duplicateOwners:readonly string[];inferredByMachine:Readonly<Record<string,number>>;fetchedByMachine:Readonly<Record<string,number>>;inferredTotal:number;peakFootprintMegabytes:Readonly<Record<string,number>>;limitMegabytes:number;withinLimit:Readonly<Record<string,boolean>>;ownersPerMinute:number;elapsedSeconds:number;orphans:number;workRemoved:boolean;landing?:Readonly<{atSeconds:number;storedBefore:number;storedLost:number;poolIdleSeconds:number;movedRevision:boolean}>}>;

/** ⚖️ Judges a run: every machine ready, no owner stored twice, the footprint of each machine within the plan limit, the throughput, no process left behind, the work directory removed; after a landing, the stored results that no longer count and the time the pools sat idle with owners open. */
export function rehearsalVerdictV1(input:Readonly<{options:RehearsalOptionsV1;limits:ReturnType<typeof rehearsalLimitsV1>;machines:readonly string[];totalOwners:number;samples:readonly RehearsalSampleV1[];readyAtSeconds:Readonly<Record<string,number>>;puts:readonly Readonly<{machine:string;key:string;atSeconds:number}>[];claims:readonly unknown[];orphans:number;workRemoved:boolean;elapsedSeconds:number;landing?:Readonly<{atSeconds:number;storedBefore:number;movedRevision:boolean}>}>):RehearsalVerdictV1{
 const counts=new Map<string,number>();for(const put of input.puts)counts.set(put.key,(counts.get(put.key)??0)+1);
 const duplicateOwners=[...counts].filter(([,count])=>count>1).map(([key])=>key).sort();
 const peak=Object.fromEntries(input.machines.map(machine=>[machine,Math.max(0,...input.samples.filter(sample=>sample.machine===machine).map(sample=>sample.footprintMegabytes))])),within=Object.fromEntries(input.machines.map(machine=>[machine,peak[machine]!<=input.limits.planMegabytes]));
 const finalOf=(machine:string)=>[...input.samples].reverse().find(sample=>sample.machine===machine)?.ledger,inferred=Object.fromEntries(input.machines.map(machine=>[machine,finalOf(machine)?.here??0])),fetchedBy=Object.fromEntries(input.machines.map(machine=>[machine,finalOf(machine)?.fetched??0]));
 const allReady=input.machines.every(machine=>input.readyAtSeconds[machine]!==undefined),step=(input.options.sampleMilliseconds??P.sampleMilliseconds)/1000;
 const landing=input.landing===undefined?undefined:Object.freeze({atSeconds:input.landing.atSeconds,storedBefore:input.landing.storedBefore,movedRevision:input.landing.movedRevision,storedLost:input.landing.movedRevision?input.landing.storedBefore:0,poolIdleSeconds:input.samples.filter(sample=>sample.atSeconds>input.landing!.atSeconds&&sample.pool.running===0&&sample.pool.open>=P.idleReasonOpenMinimum).length*step});
 const elapsed=Math.max(1,input.elapsedSeconds);
 return Object.freeze({pass:allReady&&duplicateOwners.length===0&&Object.values(within).every(Boolean)&&input.orphans===0&&input.workRemoved,allReady,readyAtSeconds:input.readyAtSeconds,duplicateOwners,inferredByMachine:inferred,fetchedByMachine:fetchedBy,inferredTotal:Object.values(inferred).reduce((sum,count)=>sum+count,0),peakFootprintMegabytes:peak,limitMegabytes:input.limits.planMegabytes,withinLimit:within,ownersPerMinute:Math.round(input.totalOwners/(elapsed/60)*10)/10,elapsedSeconds:elapsed,orphans:input.orphans,workRemoved:input.workRemoved,...landing===undefined?{}:{landing}});
}

/** 🛂️ Which state of the live checkout a simulated machine shares with the real machine, and why; checked against the tree and the mount table. */
export function auditIsolationV1(root:string):Readonly<{redirected:readonly Readonly<{path:string;how:string}>[];shared:readonly Readonly<{path:string;reason:string}>[]}>{
 const library=join(root,"🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build"),redirected:{path:string;how:string}[]=[],shared:{path:string;reason:string}[]=[];
 const mounts=spawnSync("mount",{encoding:"utf8"}).stdout??"",mounted=mounts.split("\n").filter(line=>line.includes(machinePathV1(root,".cache","b"))).length;
 shared.push({path:".cache/b",reason:`native storage images are mounted at <checkout>/.cache/b/<partition>/m (${mounted} mounts now) and the graph facts of a read are written to .cache/b/g/m below the checkout (repositoryBuildGraphDataDirectoryV1), under one writer custody per checkout: machines on one checkout would share records and queue behind the real machine`});
 shared.push({path:".nx",reason:"Nx workspace data and the installed tooling are keyed by the checkout root"});
 if(existsSync(join(library,"🏠️machine-state","🟦️.ts")))redirected.push({path:".cache/f",how:"SEMIO_MACHINE_STATE_DIRECTORY (machineStateDirectoryV1)"});
 else shared.push({path:".cache/f",reason:"machine.json (identity), cache facts, ready, owner pool, graph state and slice report are spelled `<checkout>/.cache/f/...` at about 40 sites; no variable moves them (the seam is staged, not landed)"});
 for(const name of["SEMIO_FLEET_ORIGIN","SEMIO_FLEET_GROUP","SEMIO_FLEET_STORE_ROOT","SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES","SEMIO_HOST_CAPACITY_CORES","SEMIO_HOST_CAPACITY_DIRECTORY","SEMIO_LEASE_REGISTRY_DIRECTORY","SEMIO_SERVICE_LAUNCHD","TMPDIR","XDG_RUNTIME_DIR"])redirected.push({path:name,how:"environment variable of the machine"});
 return Object.freeze({redirected,shared});
}

const here=dirname(fileURLToPath(import.meta.url));
type Put={machine:string;key:string;atSeconds:number};type Claim={machine:string;key:string;state:string;atSeconds:number};
const readJson=<T>(path:string):T|undefined=>{try{return JSON.parse(readFileSync(path,"utf8")) as T;}catch{return undefined;}};
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

/** 🏟️ Runs the rehearsal: starts the shared cache server (the real claim table) and one logging proxy per machine, starts the machines as separate process trees, samples them, judges the run, stops every process it started and removes its directories. */
export async function runRehearsalV1(options:RehearsalOptionsV1,io:Readonly<{say?(line:string):void;keep?:boolean;signal?:AbortSignal}>={}):Promise<Readonly<{workDirectory:string;timeline:readonly RehearsalSampleV1[];verdict:RehearsalVerdictV1;events:Readonly<{puts:readonly Put[];claims:readonly Claim[]}>}>>{
 if(process.platform==="win32")throw Error("The rehearsal lists processes with ps and runs on macOS and Linux / Die Probe listet Prozesse mit ps und läuft auf macOS und Linux");
 const say=io.say??(()=>{}),started=Date.now(),id=randomBytes(3).toString("hex"),temporary=realpathSync(tmpdir()),work=realpathSync(mkdtempSync(join(temporary,P.temporaryPrefix)));
 if(!work.startsWith(temporary+sep)||work.includes("Documents"))throw Error("A temporary work directory outside the checkout is required");
 const plans=rehearsalMachinePlansV1(options,id,work),limits=rehearsalLimitsV1(options.memoryMegabytes),step=options.sampleMilliseconds??P.sampleMilliseconds,controller=new AbortController();
 for(const plan of plans)for(const path of[plan.state,plan.temporary,plan.runtime,plan.capacity,plan.registry,plan.store])mkdirSync(path,{recursive:true});
 const puts:Put[]=[],claims:Claim[]=[],timeline:RehearsalSampleV1[]=[],ready:Record<string,number>={},drivers=new Map<string,ReturnType<typeof Bun.spawn>>();
 const seconds=()=>Math.round((Date.now()-started)/100)/10;
 const{startRemoteCacheServerV1}=await import("../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts"),{readRemoteCachePolicyV1}=await import("../../🏗️build/🌐️remote-cache/🟦️.ts"),{createCasStoreV1}=await import("../../🏗️build/🗄️cas/🗃️store/🟦️.ts");
 const all=readRemoteCachePolicyV1(),policy=all.server;
 const server=await startRemoteCacheServerV1({hostStore:createCasStoreV1({root:join(work,"cas")}),directory:join(work,"cache"),host:"127.0.0.1",port:0,maximumBytes:policy.maximumBytes,maximumEntryBytes:Math.min(policy.maximumEntryBytes,Math.floor(policy.maximumBytes/2)),maximumAgeDays:policy.maximumAgeDays,graceMilliseconds:policy.graceSeconds*1000,defaultInstance:all.defaults.instance,tokens:{},validation:policy.actionResults as "complete",maximumActionResultBytes:policy.maximumActionResultBytes,maintenanceSeconds:policy.maintenanceSeconds,signal:controller.signal});
 const proxies=plans.map(plan=>Bun.serve({hostname:"127.0.0.1",port:0,async fetch(request){
  const url=new URL(request.url),body=request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer(),answer=await fetch(server.url+url.pathname+url.search,{method:request.method,headers:request.headers,...body===undefined?{}:{body}}),claim=/\/claim\/ac\/([0-9a-f]+)$/u.exec(url.pathname),put=/\/ac\/([0-9a-f]+)$/u.exec(url.pathname);
  if(request.method==="POST"&&claim!==null){let state="?";try{state=((await answer.clone().json()) as {state?:string}).state??"?";}catch{}claims.push({machine:plan.origin,key:claim[1]!,state,atSeconds:seconds()});}
  if(request.method==="PUT"&&put!==null&&answer.status===200)puts.push({machine:plan.origin,key:put[1]!,atSeconds:seconds()});
  return new Response(answer.body,{status:answer.status,headers:answer.headers});
 }}));
 const names=Array.from({length:options.owners},(_,at)=>`@rehearsal/o${String(at).padStart(4,"0")}`),ownerKeys=new Set(["r1","r2"].flatMap(revision=>names.map(name=>graphEntryKeyV1(rehearsalOwnerStateV1(name,revision)))));
 let landing:{atSeconds:number;storedBefore:number;movedRevision:boolean}|undefined,orphans=0,elapsed=0;
 const cleanup=async():Promise<number>=>{
  for(const[,child]of drivers)try{child.kill("SIGTERM");}catch{}
  const mine=()=>markedProcessRowsV1(processTable(),id).filter(row=>row.pid!==process.pid);
  const deadline=Date.now()+P.stopGraceMilliseconds;
  while(mine().length>0&&Date.now()<deadline)await pause(200);
  for(const row of mine())try{process.kill(row.pid,"SIGKILL");}catch{}
  await pause(400);
  return mine().length;
 };
 try{
  for(const plan of plans){
   const config={origin:plan.origin,proxy:`http://127.0.0.1:${proxies[plan.index-1]!.port}`,instance:all.defaults.instance,state:plan.state,memoryMegabytes:plan.memoryMegabytes,cores:plan.cores,owners:names,ownerMilliseconds:Math.round(options.ownerSeconds*1000),ownerMegabytes:options.ownerMegabytes??64,revision:"r1",work,sampleMilliseconds:P.driverSampleMilliseconds,id,slot:P.slot};
   const file=join(plan.state,"config.json");writeFileSync(file,JSON.stringify(config));
   const log=join(plan.state,"driver.log");
   drivers.set(plan.origin,Bun.spawn([process.execPath,join(here,"👷️machine","🟦️.ts"),`${P.markerFlag}${id}`,`${P.machineFlag}${plan.origin}`,`--config=${file}`],{cwd:plan.work,env:rehearsalEnvironmentV1(plan,process.env),stdin:"ignore",stdout:Bun.file(log),stderr:Bun.file(log)}));
   say(`machine ${plan.origin} started / Maschine ${plan.origin} gestartet`);
  }
  const deadline=Date.now()+options.timeoutSeconds*1000;
  while(Date.now()<deadline&&io.signal?.aborted!==true){
   await pause(step);
   const table=markedProcessRowsV1(processTable(),id),at=seconds();
   for(const plan of plans){
    const ledger=readJson<{sources?:{owners?:{ledger?:RehearsalSampleV1["ledger"]}}}>(join(plan.state,"slice-memory.json"))?.sources?.owners?.ledger??{total:options.owners,present:0,fetched:0,here:0,wanted:options.owners,done:0},pool=readJson<RehearsalSampleV1["pool"]>(join(plan.state,"pool.json"))??{state:"starting",reason:null,running:0,open:options.owners},rows=table.filter(row=>row.machine===plan.origin);
    timeline.push({atSeconds:at,machine:plan.origin,ledger,pool,footprintMegabytes:Math.round(rows.reduce((sum,row)=>sum+row.rssMegabytes,0)),cpu:Math.round(rows.reduce((sum,row)=>sum+row.cpu,0)*10)/10});
    if(ready[plan.origin]===undefined&&existsSync(join(plan.state,"ready.json"))&&(landing===undefined||at>landing.atSeconds+1))ready[plan.origin]=at;
    const child=drivers.get(plan.origin);if(child!==undefined&&child.exitCode!==null&&ready[plan.origin]===undefined)throw Error(`${plan.origin} ended with ${child.exitCode}: ${readFileSync(join(plan.state,"driver.log"),"utf8").slice(-600)}`);
   }
   if(options.scenario==="landing"&&landing===undefined&&at>=(options.landingAfterSeconds??30)){
    landing={atSeconds:at,storedBefore:new Set(puts.map(put=>put.key)).size,movedRevision:options.landingMovesRevision??true};
    writeFileSync(join(work,"landing.json"),JSON.stringify({atMs:Date.now(),revision:landing.movedRevision?"r2":"r1"}));
    for(const plan of plans)rmSync(join(plan.state,"ready.json"),{force:true});
    for(const key of Object.keys(ready))delete ready[key];
    say(`landing at ${at} s / Landung bei ${at} s`);
   }
   if(plans.every(plan=>ready[plan.origin]!==undefined))break;
  }
  await pause(Math.min(P.settleMilliseconds,2000));
 }finally{
  elapsed=seconds();orphans=await cleanup();
  controller.abort();for(const proxy of proxies)proxy.stop(true);await server.stop().catch(()=>{});
  if(options.out!==undefined){mkdirSync(options.out,{recursive:true});writeFileSync(join(options.out,"timeline.jsonl"),timeline.map(row=>JSON.stringify(row)).join("\n")+"\n");writeFileSync(join(options.out,"events.json"),JSON.stringify({puts,claims}));}
  const guard=resolve(work);if(io.keep!==true&&guard.startsWith(join(temporary,P.temporaryPrefix))&&!guard.includes(sep+".."+sep))rmSync(guard,{recursive:true,force:true});
 }
 const verdict=rehearsalVerdictV1({options,limits,machines:plans.map(plan=>plan.origin),totalOwners:options.owners,samples:timeline,readyAtSeconds:ready,puts:puts.filter(put=>ownerKeys.has(put.key)),claims,orphans,workRemoved:!existsSync(work),elapsedSeconds:elapsed,...landing===undefined?{}:{landing}});
 return Object.freeze({workDirectory:work,timeline,verdict,events:{puts,claims}});
}
