import {spawn,type ChildProcess} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {fleetFirstTaskV1,fleetReadyV1,type FleetReceiptV1,fleetDiskLawsV1,fleetHostV1,fleetMemoryLawsV1,fleetOutcomeV1,fleetPolicyV1,fleetScenariosV1,fleetTwinV1,fleetWarmV1,type FleetOutcomeV1,type FleetValuesV1} from "../🟦️.ts";
import {countingRustcV1,createFleetCheckoutV1,eventuallyV1,externHashV1,pressuredHostProbeV1,rustEnvironmentV1,startFleetServicesV1,withinV1,workerEnvironmentV1} from "../🧰️harness/🟦️.ts";
import {incrementalChildEnvironmentV1} from "../../../🏃️runner/🟦️.ts";
import {terminateOwnedChildTree} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

const policy=fleetPolicyV1,sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),both=(english:string,german:string)=>`${english}\n${german}`;
const self=fileURLToPath(import.meta.url),encoder=new TextEncoder();

type Server=Awaited<ReturnType<typeof startServer>>;
async function startServer(directory:string){
 const {startRemoteCacheServerV1}=await import("../../../../🌐️remote-cache/🖥️server/🟦️.ts");
 return startRemoteCacheServerV1({directory:join(directory,"cache"),host:"127.0.0.1",port:0,maximumBytes:1<<28,maximumEntryBytes:1<<26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:undefined,write:undefined}});
}
async function machine(server:Server,name:string,directory:string){
 const {createFleetClientV1}=await import("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts"),home=join(directory,name);
 mkdirSync(join(home,"state"),{recursive:true});
 return {home,cas:join(home,"cas"),client:createFleetClientV1({endpoint:server.url.replace(/\/+$/u,""),instance:"main",token:undefined,upload:true,timeoutMilliseconds:undefined,retries:undefined,stateDirectory:join(home,"state"),signedRoot:undefined,identity:name})};
}

const platform={version:1,os:"darwin",arch:"arm64",host:"aarch64-apple-darwin",rustc:"a".repeat(64),toolchain:"t",linker:"d",cpu:""} as never;
async function unitOf(label:string,outputs:readonly (readonly [string,Uint8Array])[]){
 const {packUnitV1}=await import("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📦️wire/🟦️.ts"),contents=new Map<string,Uint8Array>(),rows=outputs.map(([name,bytes])=>{const hash=sha(bytes);contents.set(hash,bytes);return {name,sha256:hash,bytes:bytes.length,executable:false};});
 const key=sha("key:"+label),manifest={version:1,key,localKey:sha("local:"+label),slot:sha("slot:"+label),crate:label.replace(/[^a-z0-9]/gu,""),platform,createdMs:1,costMs:1,dependencies:[],environment:[],outputs:rows,stdout:null,stderr:null} as never;
 return {key,localKey:sha("local:"+label),slot:sha("slot:"+label),pack:packUnitV1(manifest,contents)};
}
const bytesOf=(index:number,size:number):Uint8Array=>{const bytes=new Uint8Array(size);bytes.fill(index%251+1);new DataView(bytes.buffer).setUint32(0,index*977+size);return bytes;};
async function publish(client:{putBlob(hash:string,bytes:Uint8Array):Promise<boolean>;putResult(key:string,bytes:Uint8Array,slot:string):Promise<boolean|"diverged">},unit:Awaited<ReturnType<typeof unitOf>>):Promise<boolean|"diverged">{
 for(const [hash,bytes] of unit.pack.blobs)if(!await client.putBlob(hash,bytes))return false;
 return client.putResult(unit.key,unit.pack.result,unit.slot);
}

async function unitCacheWarm(directory:string):Promise<FleetValuesV1>{
 const server=await startServer(directory);
 try{
  const first=await machine(server,"first",directory),second=await machine(server,"second",directory),{installFleetUnitV1}=await import("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📥️adopt/🟦️.ts");
  const published=await Promise.all(Array.from({length:policy.units},(_,index)=>unitOf("warm"+index,[[`libw${index}.rlib`,bytesOf(index,40000)],[`libw${index}.rmeta`,bytesOf(index+100,6000)]]))),controls=await Promise.all(Array.from({length:policy.controls},(_,index)=>unitOf("cold"+index,[[`libc${index}.rlib`,bytesOf(index+200,1000)]])));
  process.env.SEMIO_CAS_ROOT=first.cas;
  for(const unit of published)if(await publish(first.client,unit)!==true)throw Error(both("The first workspace could not publish a unit","Der erste Arbeitsbereich konnte eine Einheit nicht veröffentlichen"));
  process.env.SEMIO_CAS_ROOT=second.cas;
  const target=(unit:Awaited<ReturnType<typeof unitOf>>)=>({client:second.client,root:join(second.home,"unit"),localKey:unit.localKey,key:unit.key,slot:unit.slot,platform,roots:[]});
  let hits=0;
  for(const unit of published)if(await installFleetUnitV1(target(unit))==="hit")hits++;
  for(const unit of controls)if(await installFleetUnitV1(target(unit))==="hit")throw Error(both("A unit that was never published was served","Eine nie veröffentlichte Einheit wurde geliefert"));
  return fleetWarmV1({published:published.length,hits});
 }finally{await server.stop();}
}

async function nxCacheWarm(directory:string):Promise<FleetValuesV1>{
 const server=await startServer(directory),base=`${server.url.replace(/\/+$/u,"")}/main/v1/cache`;
 try{
  const tasks=Array.from({length:policy.units},(_,index)=>({hash:sha("nx-task"+index).slice(0,20),bytes:bytesOf(index,30000)})),controls=Array.from({length:policy.controls},(_,index)=>sha("nx-cold"+index).slice(0,20));
  for(const task of tasks){const response=await fetch(`${base}/${task.hash}`,{method:"PUT",headers:{"content-type":"application/octet-stream"},body:task.bytes});if(!response.ok)throw Error(both(`The first workspace could not store a task: ${response.status}`,`Der erste Arbeitsbereich konnte eine Aufgabe nicht ablegen: ${response.status}`));}
  let hits=0;
  for(const task of tasks){const response=await fetch(`${base}/${task.hash}`);if(response.ok&&sha(new Uint8Array(await response.arrayBuffer()))===sha(task.bytes))hits++;}
  for(const hash of controls)if((await fetch(`${base}/${hash}`)).ok)throw Error(both("A task that was never stored was served","Eine nie abgelegte Aufgabe wurde geliefert"));
  return fleetWarmV1({published:tasks.length,hits});
 }finally{await server.stop();}
}

async function twinBuild(directory:string):Promise<FleetValuesV1>{
 const server=await startServer(directory);
 try{
  const source="pub fn a()->u32{1}\n#[inline]\npub fn gen<T:Default>()->T{T::default()}\npub fn b()->u32{a()+gen::<u32>()}\n",roots=["one","two"].map(label=>createFleetCheckoutV1("twin-"+label,directory));
  for(const checkout of roots)checkout.compile("a",source);
  const names=[`liba-${externHashV1("a")}.rlib`,`liba-${externHashV1("a")}.rmeta`],read=(checkout:typeof roots[number],name:string)=>new Uint8Array(readFileSync(join(checkout.deps,name))),first=await machine(server,"one",directory),second=await machine(server,"two",directory);
  const units=await Promise.all(roots.map(checkout=>unitOf("twin",names.map(name=>[name,read(checkout,name)] as const))));
  const answers=[await publish(first.client,units[0]!),await publish(second.client,units[1]!)];
  for(const answer of answers)if(answer===false)throw Error(both("A build of the twin unit could not be published","Eine Fassung der Zwillingseinheit konnte nicht veröffentlicht werden"));
  return fleetTwinV1({outputs:names.map(name=>({name,left:sha(read(roots[0]!,name)),right:sha(read(roots[1]!,name))})),answers:answers as (boolean|"diverged")[]});
 }finally{await server.stop();}
}

type UnitRunV1=Readonly<{status:number|null;milliseconds:number;localCalls:number;kinds:readonly string[];root:string;events:readonly Readonly<Record<string,unknown>>[]}>;
async function wrappedUnit(directory:string,environment:Readonly<Record<string,string>>,label:string,timeoutMilliseconds:number,options:Readonly<{pressed:boolean;nonce?:string;offload:"distribute"|"never";drain?:boolean}>):Promise<UnitRunV1>{
 const {wrapRustcV1}=await import("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts");
 const checkout=createFleetCheckoutV1(label,directory);checkout.compile("a","pub fn a()->u32{1}\n");checkout.compile("b","pub fn b()->u32{a::a()+1}\n",["a"]);checkout.crate("c","pub fn c()->u32{b::b()+"+(options.nonce??String(Date.now()%1000))+"}\n");
 const root=join(directory,"unit-"+label),counter=(mkdirSync(root,{recursive:true}),countingRustcV1(root));
 const unitEnvironment={...rustEnvironmentV1(checkout.environment("c")),SEMIO_UNIT_CACHE_ROOT:root,SEMIO_UNIT_CACHE_LANE:"t",SEMIO_UNIT_CACHE_INNER:JSON.stringify([counter.path]),SEMIO_UNIT_CACHE_SLOTS:"0",SEMIO_OFFLOAD:options.offload,SEMIO_CAS_ROOT:join(directory,"cas-"+label),...environment};
 const previous=process.cwd();process.chdir(checkout.root);
 const started=performance.now();
 try{
  const done=await withinV1(wrapRustcV1(["rustc",...checkout.arguments("c",["b"])],unitEnvironment,checkout.root,options.pressed?{admission:{probe:pressuredHostProbeV1()}}:{}),timeoutMilliseconds),milliseconds=performance.now()-started;
  if(options.drain&&done===0){const {drainUploadsV1}=await import("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts");await withinV1(drainUploadsV1(root,unitEnvironment),timeoutMilliseconds);}
  const events=existsSync(join(root,"x","offload.jsonl"))?readFileSync(join(root,"x","offload.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as {kind:string}):[];
  return {status:done==="timeout"?null:done,milliseconds,localCalls:counter.calls(),kinds:events.map(row=>row.kind),root,events:events.map(row=>({kind:row.kind,reason:(row as {reason?:string}).reason,detail:String((row as {detail?:string}).detail??"").slice(0,200)}))};
 }finally{process.chdir(previous);}
}

async function offloadPressure(directory:string):Promise<FleetValuesV1>{
 const entry=fileURLToPath(new URL("../../../../🌐️remote-execution/🏃️entry/🟦️.ts",import.meta.url)),services=await startFleetServicesV1(directory),children=new Set<ChildProcess>();
 const idle=join(directory,"idle-host.json");
 writeFileSync(idle,JSON.stringify({host:{version:1,atMs:Date.now()+86400000,platform:process.platform==="darwin"?"darwin":"linux",cores:16,totalMegabytes:65536,availableMegabytes:48000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:null,openFilesMaximum:null,attachedImages:null},grants:[],waiters:[]}));
 try{
  for(let index=0;index<policy.offload.workers;index++){
   const name="w"+index,work=join(directory,"worker-"+name);mkdirSync(work,{recursive:true});
   const child=spawn(process.execPath,[entry,"--join",services.server.workerUrl,"--name",name,"--origin",name,"--work-directory",join(work,"work"),"--concurrency","2"],{cwd:work,env:workerEnvironmentV1(idle),stdio:"ignore"});
   children.add(child);child.on("close",()=>children.delete(child));
  }
  await eventuallyV1(async()=>{const status=await (await fetch(services.server.workerUrl+"/status")).json() as {workers?:unknown[]};return (status.workers?.length??0)>=policy.offload.workers;},120000,"the worker to join");
  await eventuallyV1(async()=>{const response=await fetch(services.server.workerUrl+"/v1/fleet/load");if(!response.ok)return false;const table=await response.json() as {machines?:{headroom?:boolean}[]};return table.machines?.some(row=>row.headroom===true)===true;},120000,"the worker to report its load");
  const run=await wrappedUnit(directory,{SEMIO_REMOTE_EXECUTION_ENDPOINT:services.server.grpcUrl,SEMIO_REMOTE_CACHE_ENDPOINT:services.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main"},"offload",policy.offload.wallTimeoutMilliseconds,{pressed:true,offload:"distribute"}),moved=run.status===0&&run.localCalls===0&&run.kinds.includes("offload-run");
  return moved?{offloadedUnits:1,offloadMs:run.milliseconds}:{offloadedUnits:0};
 }finally{
  for(const child of children)terminateOwnedChildTree(child);
  await services.stop();
 }
}

async function offloadFallback(directory:string):Promise<FleetValuesV1>{
 const run=await wrappedUnit(directory,{SEMIO_REMOTE_EXECUTION_ENDPOINT:policy.fallback.deadEndpoint,SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_INSTANCE:"main"},"fallback",policy.fallback.wallTimeoutMilliseconds,{pressed:true,offload:"distribute"});
 return run.status===0&&run.localCalls===1?{executedActions:1,fallbackMs:run.milliseconds}:{executedActions:run.status===0?run.localCalls:0};
}

type LiveV1=Readonly<{root:string;environment:Record<string,string>;status:()=>Promise<Readonly<{divergences:number}>>}>;
async function liveFleet(root:string):Promise<LiveV1>{
 const {fleetRemoteEnvironmentV1}=await import("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts"),{remoteCacheFetchV1}=await import("../../../../🌐️remote-cache/🛰️fleet/🟦️.ts");
 const environment=fleetRemoteEnvironmentV1(process.env,root);
 if(environment.SEMIO_REMOTE_CACHE_ENDPOINT===undefined)throw Error(both("No fleet cache is configured on this machine","Auf diesem Rechner ist kein Flottencache eingerichtet"));
 const fetcher=remoteCacheFetchV1({...process.env,...environment},{root});
 return {root,environment,status:async()=>{
  const response=await fetcher(`${new URL(environment.SEMIO_REMOTE_CACHE_ENDPOINT!).origin}/status`,{signal:AbortSignal.timeout(policy.live.statusTimeoutMilliseconds)});
  if(!response.ok)throw Error(both(`The fleet cache answered status ${response.status}`,`Der Flottencache antwortete mit Status ${response.status}`));
  return {divergences:Number(((await response.json()) as {divergences?:{count?:number}}).divergences?.count??0)};
 }};
}

async function liveUnitCache(directory:string,root:string):Promise<FleetValuesV1>{
 const live=await liveFleet(root),before=await live.status(),nonce=`${Date.now()}${process.pid}`.slice(-9);
 const first=await wrappedUnit(directory,live.environment,"live-one",policy.live.unitTimeoutMilliseconds,{pressed:false,nonce,offload:"never",drain:true});
 if(first.status!==0)throw Error(both("The first workspace could not build the unit","Der erste Arbeitsbereich konnte die Einheit nicht bauen"));
 const second=await wrappedUnit(directory,live.environment,"live-two",policy.live.unitTimeoutMilliseconds,{pressed:false,nonce,offload:"never"}),after=await live.status();
 if(second.status!==0)throw Error(both("The second workspace could not build the unit","Der zweite Arbeitsbereich konnte die Einheit nicht bauen"));
 return {...fleetWarmV1({published:1,hits:second.localCalls===0?1:0}),divergentWrites:Math.max(0,after.divergences-before.divergences)};
}

async function liveNxCache(directory:string,root:string):Promise<FleetValuesV1>{
 const {provisionPristineNxV1,createScratchNxWorkspaceV1,wipeScratchNxCacheV1,runScratchNxV1}=await import("../../../../🌐️remote-cache/🧶️nx/🧰️scratch/🟦️.ts"),{nxRemoteCacheEnvironmentV1}=await import("../../../../🌐️remote-cache/🧶️nx/🟦️.ts"),{routeNxThroughFlightV1}=await import("../../../../🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts");
 const nx=provisionPristineNxV1(root);
 if(nx===undefined)throw Error(both("No pristine Nx package is cached on this machine","Auf diesem Rechner ist kein unverändertes Nx-Paket zwischengespeichert"));
 const live=await liveFleet(root),base={...process.env,...live.environment,SEMIO_WORKSPACE_ROOT:root} as Record<string,string>,routed={...base,...await nxRemoteCacheEnvironmentV1(base,{probeMilliseconds:policy.live.statusTimeoutMilliseconds})} as Record<string,string>,flight=await routeNxThroughFlightV1(routed);
 if(flight===undefined)throw Error(both("The fleet cache is not reachable for Nx","Der Flottencache ist für Nx nicht erreichbar"));
 try{
  const modules=join(nx,"..","..","..",".."),nonce=`${Date.now()}${process.pid}`,environment=flight.environment() as Record<string,string>,limit=policy.live.nxTimeoutMilliseconds;
  const first=createScratchNxWorkspaceV1(directory,modules,"one",nonce),second=createScratchNxWorkspaceV1(directory,modules,"two",nonce);
  const cold=await runScratchNxV1(nx,first,environment,limit);
  if(cold.status!==0)throw Error(both("The first Nx workspace could not build","Der erste Nx-Arbeitsbereich konnte nicht bauen"));
  wipeScratchNxCacheV1(first);
  const warm=await runScratchNxV1(nx,second,environment,limit);
  if(warm.status!==0||warm.total===0)throw Error(both("The second Nx workspace could not build","Der zweite Nx-Arbeitsbereich konnte nicht bauen"));
  return {hitRatio:warm.hits/warm.total,executedActions:warm.total-warm.hits};
 }finally{await flight.stop();}
}

async function liveBazelCache(directory:string,root:string):Promise<FleetValuesV1>{
 const {readdirSync}=await import("node:fs"),{startBazelRemoteCacheProxyV1,bazelRemoteCacheV1}=await import("../../../../🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts"),{resolveRemoteCacheV1}=await import("../../../../🌐️remote-cache/🛰️fleet/🟦️.ts");
 const live=await liveFleet(root),environment={...process.env,...live.environment,SEMIO_WORKSPACE_ROOT:root} as Record<string,string>,resolved=resolveRemoteCacheV1(environment);
 const binaries=join(root,".cache","b","t","m","bazel","bin"),found=environment.SEMIO_TOOL_BAZEL??(existsSync(binaries)?readdirSync(binaries).sort().map(name=>join(binaries,name,process.platform==="win32"?"bazel.exe":"bazel")).find(path=>existsSync(path)):undefined);
 if(found===undefined||resolved===undefined)throw Error(both("No Bazel binary or fleet cache is available on this machine","Auf diesem Rechner sind weder ein Bazel-Programm noch ein Flottencache verfügbar"));
 const proxy=await startBazelRemoteCacheProxyV1(environment);
 if(proxy.url===undefined)throw Error(both("The signing proxy for Bazel could not start","Der Signatur-Proxy für Bazel konnte nicht starten"));
 try{
  const workspace=join(directory,"ws"),rendering=bazelRemoteCacheV1(resolved.contract,resolved.environment,{helperDirectory:directory,proxy:proxy.url});
  mkdirSync(workspace,{recursive:true});
  writeFileSync(join(workspace,"MODULE.bazel"),'module(name="scratch",version="0.0.0")\n');writeFileSync(join(workspace,"in.txt"),`fleet ${Date.now()}${process.pid}\n`);
  writeFileSync(join(workspace,"BUILD.bazel"),'genrule(name="a",srcs=["in.txt"],outs=["a.txt"],cmd="cat $(SRCS) > $@ && echo a >> $@")\ngenrule(name="b",srcs=[":a"],outs=["b.txt"],cmd="cat $(SRCS) > $@ && echo b >> $@")\n');
  const bazel=(command:string[])=>new Promise<{status:number|null;text:string}>(accept=>{
   const child=spawn(found,[`--output_user_root=${join(directory,"root")}`,`--output_base=${join(directory,"ob")}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=15",...command,"--lockfile_mode=off","--incompatible_autoload_externally=",...(command[0]==="build"?rendering.flags:[])],{cwd:workspace,env:{PATH:process.env.PATH??"",HOME:process.env.HOME??""}}),parts:Buffer[]=[],timer=setTimeout(()=>child.kill("SIGKILL"),policy.live.bazelTimeoutMilliseconds);
   child.stderr.on("data",chunk=>parts.push(chunk));child.stdout.on("data",chunk=>parts.push(chunk));
   child.on("error",()=>{clearTimeout(timer);accept({status:null,text:""});});child.on("close",status=>{clearTimeout(timer);accept({status,text:Buffer.concat(parts).toString()});});
  });
  const counts=(text:string)=>{const line=/INFO: [0-9]+ processes?: ([^\n]*?)\./u.exec(text)?.[1]??"";let hits=0,executed=0;for(const part of line.split(", ")){const match=/^([0-9]+) (.+)$/u.exec(part.trim());if(match===null||match[2]==="internal")continue;if(match[2]==="remote cache hit")hits+=Number(match[1]);else executed+=Number(match[1]);}return {hits,executed};};
  try{
   const cold=await bazel(["build","//:b"]);
   if(cold.status!==0)throw Error(both("The first Bazel build failed","Der erste Bazel-Build ist gescheitert"));
   await bazel(["clean"]);
   const warm=await bazel(["build","//:b"]);
   if(warm.status!==0)throw Error(both("The second Bazel build failed","Der zweite Bazel-Build ist gescheitert"));
   const {hits,executed}=counts(warm.text),total=hits+executed;
   return {hitRatio:total===0?0:hits/total,executedActions:executed};
  }finally{await bazel(["shutdown"]);}
 }finally{await proxy.stop();}
}

async function liveOffload(directory:string,root:string):Promise<FleetValuesV1>{
 const live=await liveFleet(root),{resolveRemoteExecutionEndpointV1}=await import("../../../../🌐️remote-execution/🧭️endpoint/🟦️.ts"),endpoint=resolveRemoteExecutionEndpointV1({environment:{...process.env,...live.environment},root});
 if(endpoint===undefined)throw Error(both("No remote execution endpoint is configured on this machine","Auf diesem Rechner ist kein Endpunkt für Remote-Ausführung eingerichtet"));
 const run=await wrappedUnit(directory,{...live.environment,SEMIO_REMOTE_EXECUTION_ENDPOINT:endpoint.endpoint},"live-offload",policy.live.offloadTimeoutMilliseconds,{pressed:true,offload:"distribute"}),moved=run.status===0&&run.localCalls===0&&run.kinds.includes("offload-run");
 if(!moved)process.stderr.write(`[fleet] live offload stayed local: status=${run.status} local=${run.localCalls} events=${JSON.stringify(run.events)}\n`);
 return moved?{offloadedUnits:1,offloadMs:run.milliseconds}:{offloadedUnits:0};
}

async function liveRunWarm(root:string):Promise<FleetValuesV1>{
 const target=policy.live.runTarget,environment=incrementalChildEnvironmentV1(process.env,{SEMIO_LANGUAGE:process.env.SEMIO_LANGUAGE??"en"}),run=()=>new Promise<{lines:{atMs:number;text:string}[];wallMs:number}>((accept,reject)=>{
  const started=performance.now(),lines:{atMs:number;text:string}[]=[],pending=["",""];
  const child=spawn("repo",["run",target,"--tool=nx"],{cwd:root,env:environment,detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"]}),timer=setTimeout(()=>terminateOwnedChildTree(child),policy.live.runTimeoutMilliseconds);
  const scan=(at:0|1)=>(chunk:Buffer)=>{const now=performance.now()-started;pending[at]+=chunk.toString();for(let cut=pending[at]!.indexOf("\n");cut>=0;cut=pending[at]!.indexOf("\n")){lines.push({atMs:now,text:pending[at]!.slice(0,cut).replace(/\r$/u,"")});pending[at]=pending[at]!.slice(cut+1);}};
  child.stdout!.on("data",scan(0));child.stderr!.on("data",scan(1));
  child.on("error",error=>{clearTimeout(timer);reject(error);});
  child.on("close",(code,signal)=>{clearTimeout(timer);code===0?accept({lines,wallMs:performance.now()-started}):reject(Error(both(`repo run ${target} failed (exit ${code??signal})`,`repo run ${target} ist gescheitert (Exit ${code??signal})`)));});
 });
 await run();
 const measured=await run();
 return fleetFirstTaskV1(measured.lines,measured.wallMs);
}

async function fleetReady():Promise<FleetValuesV1>{
 const {readdirSync}=await import("node:fs"),receipts:FleetReceiptV1[]=[];
 for(const origin of readdirSync(policy.ready.origins)){
  try{const row=JSON.parse(readFileSync(join(policy.ready.origins,origin,"ready.json"),"utf8")) as {ready:boolean;startedMs:number;readyMs:number|null;updatedMs:number};receipts.push({ready:row.ready===true,startedMs:Number(row.startedMs),readyMs:row.readyMs===null||row.readyMs===undefined?null:Number(row.readyMs),updatedMs:Number(row.updatedMs)});}catch{}
 }
 return fleetReadyV1(receipts,Date.now(),policy.ready.maximumAgeMilliseconds);
}

async function host16g():Promise<FleetValuesV1>{
 const [{BUILD_HOST_CAPACITY_POLICY_V1:capacity},{evaluateHostCapacityV1,hostFloorMegabytesV1,sizeHostRequestV1},elastic,budget]=await Promise.all([import("../../../../🌡️host-capacity/🎛️control/🟦️.ts"),import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts"),import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📐️elastic/🟦️.ts"),import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json",{with:{type:"json"}})]);
 const sizing=policy.host,now=1_800_000_000_000,memory=[] as ReturnType<typeof fleetMemoryLawsV1>[];
 const sample=(free:number,pressure:"normal"|"warn"|"critical")=>({version:1,atMs:now,platform:"darwin",cores:sizing.cores,totalMegabytes:sizing.totalMegabytes,availableMegabytes:Math.floor(sizing.totalMegabytes*free),pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:null,openFilesMaximum:null,attachedImages:null}) as never;
 const waiters=Array.from({length:sizing.requests},(_,index)=>({request:{...sizeHostRequestV1(capacity,{id:"fleetreq"+String(index).padStart(3,"0"),operation:sizing.operation,preferredJobs:sizing.cores-1,kinds:["build"]})!},arrivedAtMs:now-1000-index,pid:5000+index})) as never;
 for(const [free,pressure] of [...sizing.freeFractions.map(fraction=>[fraction,"normal"] as const),[sizing.freeFractions[0]!,"critical"] as const]){
  const host=sample(free,pressure),evaluation=evaluateHostCapacityV1(capacity,host,[],waiters,now);
  memory.push(fleetMemoryLawsV1({cores:sizing.cores,floorMegabytes:hostFloorMegabytesV1(capacity,host),availableMegabytes:Math.floor(sizing.totalMegabytes*free),pressure,decisions:evaluation.decisions}));
 }
 const rules=budget.default as {partitions:Record<string,{capacityBytes:number;hostSharePercent?:number}>;elastic:Record<string,Parameters<typeof elastic.elasticCapacityV1>[0]>},born=Object.entries(rules.partitions).map(([id,row])=>{const rule=rules.elastic[id];return rule?elastic.elasticCapacityV1(rule,elastic.elasticMinimumV1(rule,rule.minimumBytes,sizing.freeDiskBytes),sizing.freeDiskBytes):elastic.hostShareCapacityV1(row.capacityBytes,row.hostSharePercent!,sizing.freeDiskBytes);});
 return fleetHostV1({memory,disk:fleetDiskLawsV1({freeBytes:sizing.freeDiskBytes,born,sharePercent:sizing.diskSharePercent,mebibyte:sizing.mebibyte}),mebibyte:sizing.mebibyte});
}

/** 🏁️ Runs one fleet scenario inside this process on a scratch directory: every machine is a second workspace and cache directory of this host, every server listens on loopback and every unit is a small slice; the values are closed against the scenario's declared metrics. */
export async function runFleetScenarioV1(id:string,directory:string,root:string=process.cwd()):Promise<FleetOutcomeV1>{
 const runners:Readonly<Record<string,()=>Promise<FleetValuesV1>>>={"unit-cache-warm":()=>unitCacheWarm(directory),"nx-cache-warm":()=>nxCacheWarm(directory),"twin-build":()=>twinBuild(directory),"offload-pressure":()=>offloadPressure(directory),"offload-fallback":()=>offloadFallback(directory),"host-16g":host16g,"live-unit-cache":()=>liveUnitCache(directory,root),"live-nx-cache":()=>liveNxCache(directory,root),"live-bazel-cache":()=>liveBazelCache(directory,root),"live-offload":()=>liveOffload(directory,root),"live-run-warm":()=>liveRunWarm(root),"fleet-ready":fleetReady};
 const run=runners[id];
 if(!run||!fleetScenariosV1.scenarios.some(row=>row.id===id))throw Error(both(`Unknown fleet scenario: ${id}`,`Unbekanntes Flottenszenario: ${id}`));
 return fleetOutcomeV1(id,await run());
}

/** 🚀️ Runs one fleet scenario in a child process of its own (a scenario sets loopback servers, a second cache root and the worker risk switch, none of which may reach the bench), kills the child with its whole tree when the signal aborts or the scenario timeout passes and answers the closed outcome the child printed. */
export async function runFleetScenarioChildV1(id:string,signal:AbortSignal,environment:Readonly<Record<string,string|undefined>>=process.env,live:string=process.cwd()):Promise<FleetOutcomeV1>{
 if(!fleetScenariosV1.scenarios.some(row=>row.id===id))throw Error(both(`Unknown fleet scenario: ${id}`,`Unbekanntes Flottenszenario: ${id}`));
 const directory=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"fleet-"))),fleetFacts=/^(SEMIO_REMOTE_|SEMIO_FLEET_|SEMIO_UNIT_CACHE_REMOTE$)/u,env=Object.fromEntries(Object.entries(incrementalChildEnvironmentV1(environment,id.startsWith("live-")?Object.fromEntries(Object.entries(environment).filter((row):row is [string,string]=>row[1]!==undefined&&fleetFacts.test(row[0]))):{})).filter(([name,value])=>value!==undefined&&(id.startsWith("live-")||!name.startsWith("SEMIO_REMOTE_"))&&name!=="SEMIO_CAS_ROOT")) as Record<string,string>;
 try{
  return await new Promise<FleetOutcomeV1>((accept,reject)=>{
   if(signal.aborted){reject(signal.reason);return;}
   const child=spawn(process.execPath,[self,id,directory,live],{cwd:directory,env,detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"],windowsHide:true}),out:Buffer[]=[],err:Buffer[]=[],stop=()=>terminateOwnedChildTree(child),timer=setTimeout(stop,policy.scenarioTimeoutSeconds*1000);
   signal.addEventListener("abort",stop,{once:true});
   child.stdout!.on("data",chunk=>out.push(chunk));child.stderr!.on("data",chunk=>err.push(chunk));
   child.on("error",error=>{clearTimeout(timer);signal.removeEventListener("abort",stop);reject(error);});
   child.on("close",code=>{
    clearTimeout(timer);signal.removeEventListener("abort",stop);
    if(signal.aborted){reject(signal.reason);return;}
    for(const note of Buffer.concat(err).toString().split("\n").filter(row=>row.startsWith("[fleet] ")))process.stderr.write(note+"\n");
    const line=Buffer.concat(out).toString().split("\n").filter(row=>row.startsWith("{\"version\":1")).at(-1);
    if(code!==0||line===undefined){reject(Error(both(`The fleet scenario ${id} failed (exit ${code}): ${Buffer.concat(err).toString().slice(-800)}`,`Das Flottenszenario ${id} ist gescheitert (Exit ${code}): ${Buffer.concat(err).toString().slice(-800)}`)));return;}
    try{const outcome=JSON.parse(line) as FleetOutcomeV1;accept(fleetOutcomeV1(id,outcome.values));}catch(error){reject(error);}
   });
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
}

if(import.meta.main){
 const [id,directory,root]=process.argv.slice(2);
 runFleetScenarioV1(id!,directory!,root).then(outcome=>{process.stdout.write(JSON.stringify(outcome)+"\n");process.exit(0);},error=>{process.stderr.write(String((error as Error)?.stack??error)+"\n");process.exit(1);});
}
