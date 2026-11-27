import{mkdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{dirname}from"node:path";
import type{GraphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{claimWantedV1,publishServedV1,readRegistryWatchRoundV1,readServeStateV1,wantedClaimKeyV1,wantedTextV1,type ReadWatchObservationV1,type ServeStateV1,type WantedHeadroomV1,type WantedPolicyV1,type WantedRequestV1,wantedPolicyV1}from"../🟦️.ts";
import{clearSteeringV1,readPendingV1,readPriorityV1,writePriorityV1}from"../../../🕸️graph/🕰️generations/🧭️steering/🟨️.mjs";
import{parseGraphWaveRowV1,type GraphWaveRowV1}from"../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{estimateOwnerCostV1,estimateRemainingMillisecondsV1,orderByCostV1,type OwnerCostTableV1}from"../💰️cost/🟦️.ts";
import{publishOwnerCostsV1}from"../💰️cost/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import parserSchema from"../../../🚀️bootstrap/📦️parsers/🧬️schema/🔣️.json"with{type:"json"};
import{platformClassV1}from"../../🌍️platform-class/🟦️.ts";

/** 🛠️ The numbers of the serving machine: round length, how long the worktree must be quiet before a background refresh (with a maximum age that overrides it), the stall limit and the maximum of one graph read, the pauses of a background refresh, the free memory a refresh needs, the smaller free memory a requested read needs (the read runs in waves that give way, so a 16 GB machine serves) and the physical memory a machine must have to serve at all (15000 MB: the 16 GB machines the fleet consists of). See `../🧬️schema/🔣️.json`. */
export const wantedServerPolicyV1=Object.freeze({...schema.definitions.Server.const});
export type WantedServerPolicyV1=typeof wantedServerPolicyV1;
export type WantedInferV1=Readonly<{ok:boolean;status:number;seconds:number;generation:number|null;detail:string}>;

/** 🩺️ The verdict over one background graph read: it goes on while it makes progress and ends only when it does not. */
export type GraphReadWatchV1=Readonly<{feed(text:string):void;verdict(at?:number):"go"|"stalled"|"too-long";generation():number|null;observation():ReadWatchObservationV1;progressAt():number;silentMilliseconds():number;counters():Readonly<Record<string,readonly number[]>>}>;

/** 🩺️ Watches one graph read for published generations, completed startup and inference work and new observations within an active owner. Replayed work, running age, concurrency and waits do not renew the stall clock. The absolute read limit applies even when work keeps advancing. */
export function createGraphReadWatchV1(policy:WantedServerPolicyV1=wantedServerPolicyV1,now:()=>number=Date.now,onWave?:(row:GraphWaveRowV1)=>void,startedMs:number=now(),observation?:ReadWatchObservationV1):GraphReadWatchV1{
 const config=policy.readProgress,ledgerKey="owners:ledger",high=new Map<string,[number,number]>(observation?.phases.map(({name,done,total})=>[name,[done,total]])),owners=new Map<string,number>(observation?.owners.map(({name,rows})=>[name,rows]));
 const phasePattern=new RegExp(config.phasePattern,"u"),ownerPattern=new RegExp(config.ownerRowPattern,"u"),sharedPattern=new RegExp(config.sharedClosuresPattern,"u"),ledgers=config.ledgerPatterns.map(pattern=>new RegExp(pattern,"u")),generations=config.generationPatterns.map(pattern=>new RegExp(pattern,"u"));
 const dependencyCounters=new RegExp(config.dependencyCounterPattern,"u"),storeOwnership=new RegExp(config.storeOwnershipPattern,"u"),sourcePattern=new RegExp(config.sourcePattern,"u"),sourceBatch=parserSchema.definitions.ProgressProtocol.const.operationBatch,startupCommands=new Set<string>(config.startupCommands);
 let progressAt=observation?.progressAt??now(),newest=observation?.generation??-1,registryRound=observation?.registryRound??0,pending="",dropping=false;
 const counted=(name:string,done:number,total:number):boolean=>{
  if(!Number.isSafeInteger(done)||!Number.isSafeInteger(total)||done>total)return false;
  const known=high.get(name);
  if(known===undefined&&high.size-Number(high.has(ledgerKey))>=config.maximumPhases&&name!==ledgerKey)return false;
  if(known!==undefined&&done<=known[0])return false;
  high.set(name,[done,total]);return done>0;
 };
 const rowsMoved=(list:string):boolean=>{
  let advanced=false;
  for(const token of list.split(",",config.maximumRunningRows)){
   const row=ownerPattern.exec(token)?.groups;if(row===undefined)continue;
   const name=row.name!,rows=Number(row.rows);
   if(!Number.isSafeInteger(rows))continue;
   try{if(encodeURIComponent(decodeURIComponent(name))!==name)continue;}catch{continue;}
   const known=owners.get(name);
   if(known===undefined){if(owners.size<config.maximumOwners)owners.set(name,rows);}
   else if(rows>known){owners.set(name,rows);advanced=true;}
  }
  return advanced;
 };
 const generation=(value:number):void=>{
  if(Number.isSafeInteger(value)&&value>newest){newest=value;progressAt=now();}
 };
 const observe=(line:string):void=>{
  if(line.startsWith("[graph-repair] ")){
   const round=readRegistryWatchRoundV1(line);
   if(round!==undefined&&round===registryRound+1){
    registryRound=round;
    for(const key of high.keys())if(key===ledgerKey||startupCommands.has(key)||key.startsWith("build:graph-phase:"))high.delete(key);
    owners.clear();
   }
   return;
  }
  const wave=parseGraphWaveRowV1(line);
  if(wave){
   const next=wave.generation>=newest;generation(wave.generation);
   if(next){progressAt=now();for(const key of high.keys())if(key===ledgerKey||key.startsWith("build:graph-phase:"))high.delete(key);owners.clear();}
   try{onWave?.(wave);}catch{}return;
  }
  for(const pattern of generations){
   const found=pattern.exec(line);if(found!==null){generation(Number(found[1]));return;}
  }
  if(line.startsWith("[owners] ")){
   for(const pattern of ledgers){
    const found=pattern.exec(line)?.groups;if(found===undefined)continue;
    const done=Number(found.done),total=Number(found.total),sum=Number(found.present)+Number(found.fetched)+Number(found.here);
    if(done===sum&&counted(ledgerKey,done,total))progressAt=now();
    return;
   }
  }
  if(!line.startsWith('[script] {')||!(/"resource":"(?:build:(?:graph-phase|parser-source):|dependencies:(?:workspace-edges|store-ownership|preservation-ownership):)|"stage":"complete"/u).test(line))return;
  let row:{owner?:unknown;command?:unknown;stage?:unknown;activity?:{resource?:unknown;mode?:unknown;elapsedMs?:unknown}};
  try{row=JSON.parse(line.slice("[script] ".length));}catch{return;}
  if(row.owner==="repository:build"&&row.stage==="complete"&&row.activity===undefined&&typeof row.command==="string"&&startupCommands.has(row.command)){
   if(counted(row.command,1,1))progressAt=now();
   return;
  }
  const resource=row.activity?.resource;
  if(typeof resource!=="string"||resource.length>config.maximumResourceCharacters)return;
  const ownership=storeOwnership.exec(resource)?.groups;
  if(ownership!==undefined){
   const done=Number(ownership.done),elapsed=row.activity?.elapsedMs;
   if(row.owner==="repository:build"&&row.command==="repository:command"&&row.stage==="running"&&row.activity?.mode==="exclusive"&&typeof elapsed==="number"&&Number.isFinite(elapsed)&&elapsed>=0&&done%config.storeOwnershipBatch===0&&counted("dependencies:store-ownership",done,done))progressAt=now();
   return;
  }
  const dependency=dependencyCounters.exec(resource)?.groups;
  if(dependency!==undefined){
   const done=Number(dependency.done),kind=dependency.kind;
   if(kind==="preservation-ownership"&&done%config.storeOwnershipBatch!==0)return;
   if(row.owner==="repository:build"&&row.command==="repository:command"&&row.stage==="running"&&row.activity?.mode==="exclusive"&&row.activity.elapsedMs===0&&counted("dependencies:"+kind,done,done))progressAt=now();
   return;
  }
  const source=sourcePattern.exec(resource)?.groups;
  if(source!==undefined){
   const done=Number(source.done);
   if(row.owner==="repository:build"&&row.command==="repository:command"&&row.stage==="running"&&row.activity?.mode==="shared"&&row.activity.elapsedMs===0&&done%sourceBatch===0&&counted("build:parser-source",done,done))progressAt=now();
   return;
  }
  const phase=phasePattern.exec(resource)?.groups;if(phase===undefined)return;
  const done=Number(phase.done),total=Number(phase.total);
  if(!Number.isSafeInteger(done)||!Number.isSafeInteger(total)||done>total)return;
  const name="build:graph-phase:"+phase.phase;
  if(done<(high.get(name)?.[0]??0))return;
  const shared=phase.owners===undefined?undefined:sharedPattern.exec(phase.owners)?.groups;
  if(shared!==undefined&&counted(name+":shared-closures",Number(shared.done),Number(shared.total)))progressAt=now();
  const completed=counted(name,done,total),advanced=phase.phase==="inference"&&phase.owners!==undefined&&rowsMoved(phase.owners);
  if(completed||advanced)progressAt=now();
 };
 return Object.freeze({
  feed(text:string):void{
   for(let start=0;start<text.length;){
    const end=text.indexOf("\n",start),length=(end<0?text.length:end)-start;
    if(!dropping){
     if(pending.length+length>config.maximumLineCharacters){pending="";dropping=true;}
     else pending+=text.slice(start,start+length);
    }
    if(end<0)break;
    if(!dropping)observe(pending.trimEnd());
    pending="";dropping=false;start=end+1;
   }
  },
  verdict(at:number=now()):"go"|"stalled"|"too-long"{return at-startedMs>=policy.inferMaximumMilliseconds?"too-long":at-progressAt>=policy.inferStallMilliseconds?"stalled":"go";},
  generation:()=>newest<0?null:newest,
  observation:()=>({progressAt,generation:newest<0?null:newest,...registryRound===0?{}:{registryRound},phases:[...high].map(([name,[done,total]])=>({name,done,total})),owners:[...owners].map(([name,rows])=>({name,rows}))}),
  progressAt:()=>progressAt,
  silentMilliseconds:()=>now()-progressAt,
  counters:()=>Object.fromEntries([...high].map(([name,numbers])=>[name,[...numbers]])),
 });
}

/** 🗨️ Words why the server stopped a graph read, in the language of the machine. */
export function graphReadEndTextV1(kind:"stalled"|"too-long",language:"en"|"de",policy:WantedServerPolicyV1=wantedServerPolicyV1):string{
 return wantedTextV1(kind==="stalled"?"stalled":"tooLong",language,{minutes:Math.round(policy.inferStallMilliseconds/60000),hours:Math.round(policy.inferMaximumMilliseconds/3600000)});
}

/** 🧘️ What the capacity gate shows a background refresh: the pressure level and the requests that wait. */
export type GraphYieldViewV1=Readonly<{pressure:string;waiters:readonly Readonly<{request:Readonly<{id:string;elastic?:boolean;quiet?:boolean}>}>[]}>;

const pause=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal!.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/** 🧘️ Lets a background refresh give way between two waves: while another command of this machine waits for memory (any request but small forecast grants, the workers of a build and its own) or the pressure is raised, the refresh pauses, tells so every `yieldNoticeMilliseconds` and looks again every `yieldPollMilliseconds`; after `yieldMaximumMilliseconds` it goes on with one wave, so a machine that is busy for good still advances. A gate that cannot be read never holds the refresh. Answers the milliseconds paused. */
export async function yieldToInteractiveV1(input:Readonly<{view():Promise<GraphYieldViewV1>;own:string|undefined;generation:number;language:"en"|"de";say(line:string):void;signal:AbortSignal|undefined;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;policy?:WantedServerPolicyV1}>):Promise<number>{
 const now=input.now??Date.now,policy=input.policy??wantedServerPolicyV1,sleep=input.sleep??pause,started=now();let noticed=-Infinity;
 for(;;){
  if(input.signal?.aborted)return now()-started;
  let waiting:number;
  try{
   const view=await input.view(),others=view.waiters.filter(row=>row.request.elastic!==true&&row.request.quiet!==true&&row.request.id!==input.own).length;
   waiting=others>0?others:view.pressure==="normal"?0:1;
  }catch{return now()-started;}
  if(waiting===0||now()-started>=policy.yieldMaximumMilliseconds)return now()-started;
  if(now()-noticed>=policy.yieldNoticeMilliseconds){noticed=now();input.say(wantedTextV1("paused",input.language,{generation:input.generation,waiters:waiting,seconds:Math.round((now()-started)/1000)}));}
  try{await sleep(policy.yieldPollMilliseconds,input.signal);}catch{return now()-started;}
 }
}

/** 🧘️ The yield of a background refresh bound to the capacity gate of this machine: called by the graph host after every published wave of a read the server started (`SEMIO_GRAPH_BACKGROUND=1`). */
export async function yieldBackgroundGraphV1(input:Readonly<{generation:number;signal:AbortSignal|undefined;language:"en"|"de";environment?:NodeJS.ProcessEnv}>):Promise<number>{
 const environment=input.environment??process.env,{readRemoteExecutionHostViewV1}=await import("../../../🌐️remote-execution/🌡️load/🟦️.ts");
 return yieldToInteractiveV1({view:async()=>{const view=await readRemoteExecutionHostViewV1(environment);return{pressure:view.host.pressure,waiters:view.waiters};},own:environment.SEMIO_HOST_CAPACITY_GRANT,generation:input.generation,language:input.language,say:line=>{process.stderr.write(line+"\n");},signal:input.signal});
}

/** 🗃️ The state file of a graph server (see `ServeState` in `../🧬️schema/🔣️.json`): read back only when it has the shape of the schema, replaced atomically, never throws (a server does not depend on its notes). */
export function serveStateFileV1(file:string):Readonly<{load():ServeStateV1|undefined;save(state:ServeStateV1):void}>{
 return Object.freeze({
  load():ServeStateV1|undefined{try{return readServeStateV1(JSON.parse(readFileSync(file,"utf8")));}catch{return undefined;}},
  save(state:ServeStateV1):void{
   const temporary=`${file}.${process.pid}.tmp`;
   try{mkdirSync(dirname(file),{recursive:true});writeFileSync(temporary,JSON.stringify(state));renameSync(temporary,file);}catch{try{rmSync(temporary,{force:true});}catch{}}
  },
 });
}

/** 🧵️ What the serving loop tells the code-change restart of its process and hears back: `report(busy)` is true while a read runs and requests are held (a serve nobody waits for any more, because every held request expired, is not worth waiting for) (the restart waits for the end of it) and is called again the moment it ended, `restartPending()` is true while the restart waits, so the loop takes no new request and starts no refresh meanwhile (held requests keep their read, renewal and answers). */
export type GraphServeWorkV1=Readonly<{restartPending():boolean;report(busy:boolean):void}>;

/** 🔁️ The long-lived serving loop of a machine with headroom. It serves the requests of every machine of its platform class (a slot is per requester, the request names the platform) and counts the others. A machine that may not serve (`mayServe` answers false: it could never run the read) claims no request, so the request stays for a machine that can. It never blocks on a graph read: every round (seconds) it takes over the live requests of the other machines, tells the running read which projects to infer first (the priority file of `steerDirectory`), tells each requester which of its projects are fresh already (the read writes which asked-for projects are still dirty; every other asked-for project is fresh in this machine's tree) and keeps a sign of life on the answer, so a request is seen within one round, answered at once from what exists, and answered again as waves publish. Every answer carries the progress record of its request (how many projects, when the work last advanced: a read started or a wave published, and the time left estimated from what this read has finished or else from the measured cost table), so the requester waits for as long as it advances and not for a fixed time. A request is idempotent: the requester renews it (same creation time, later expiry, fewer projects) and the server keeps its claim, its running read and its position; a request asked again as a new one joins the held one (what is fresh stays fresh, nothing is inferred twice); the claims of the held requests are renewed so no other machine serves them meanwhile; the projects are ordered by how many held requests ask for them, so a project two workers wait for is inferred once and first. When no read runs and a request is held it starts one with the priority (headroom and pressure permitting; a failing read answers nobody and is held off for the quiet time); a read that ends well answers every held request completely with its generation. When no request is held it refreshes the fleet generation in the background after the worktree went quiet for `refreshQuietMilliseconds` (or the generation is older than `refreshIntervalMilliseconds`: edits arrive constantly, so a trailing debounce with a maximum age), only while its headroom allows it and no other machine holds the refresh slot of the interval (`mayRefresh`). The loop never throws except on cancellation. */
export async function runWantedServerV1(input:Readonly<{client:GraphFleetClientV1;owner:string;platform:string;language:"en"|"de";steerDirectory:string;requesters():Promise<readonly string[]>;headroom():Promise<WantedHeadroomV1>;mayServe?():Promise<boolean>;infer(reason:"request"|"refresh",signal:AbortSignal|undefined,published?:readonly string[]):Promise<WantedInferV1>;adopt?(signal:AbortSignal|undefined):Readonly<{reason:"request"|"refresh";done:Promise<WantedInferV1>}>|undefined;state?:Readonly<{load():ServeStateV1|undefined;save(state:ServeStateV1):void}>;quietMilliseconds():Promise<number>;refreshDeferred?():boolean|Promise<boolean>;mayRefresh?():Promise<boolean>;costs?():OwnerCostTableV1|undefined;say(line:string):void;signal:AbortSignal;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;now?:()=>number;rounds?:number;policy?:WantedPolicyV1;server?:WantedServerPolicyV1;work?:GraphServeWorkV1}>):Promise<void>{
 const now=input.now??Date.now,policy=input.policy??wantedPolicyV1,server=input.server??wantedServerPolicyV1,{language}=input;
 type Held={request:WantedRequestV1;list:readonly string[];told:boolean;toldAt:number;toldProgressAt:number;heldAt:number;claimedAt:number};
 type Running={reason:"request"|"refresh";startedAt:number;controller:AbortController;finished:boolean;done:Promise<void>};
 const held=new Map<string,Held>(),told=new Set<string>();
 const persisted=input.state?.load();
 if(persisted!==undefined&&persisted.owner===input.owner&&persisted.platform===input.platform){
  for(const row of persisted.held)if(row.request.expiresMs>now())held.set(row.request.requester,{request:row.request,list:row.list,told:row.told,toldAt:row.toldAt,toldProgressAt:-Infinity,heldAt:row.heldAt,claimedAt:-Infinity});
  for(const key of persisted.told)told.add(key);
  if(held.size>0)input.say(wantedTextV1("resumed",language,{held:held.size,fresh:[...held.values()].reduce((sum,entry)=>sum+entry.list.length,0)}));
 }
 let refreshedAt=-Infinity,progressAt=-Infinity,dirtyCount=-1,round=0,holdUntil=-Infinity,running:Running|undefined,priorityKey="",priorityAt=0,tick=0,seen=0,costsKey="",costsAt=-Infinity;
 const foreignNow=new Map<string,number>(),toldForeign=new Set<string>(),toldCross=new Set<string>(),noteForeign=(request:WantedRequestV1)=>{foreignNow.set(request.requester,request.projects.length);const key=wantedClaimKeyV1(request);if(toldForeign.has(key))return;toldForeign.add(key);input.say(wantedTextV1("foreign",language,{count:request.projects.length,requester:request.requester,platform:request.platform,class:platformClassV1(request.platform),own:platformClassV1(input.platform)}));};
 const crossed:(()=>void)[]=[],noteCross=(request:WantedRequestV1,split:Readonly<{served:readonly string[];left:readonly string[]}>)=>{const key=wantedClaimKeyV1(request);if(toldCross.has(key))return;toldCross.add(key);crossed.push(()=>input.say(wantedTextV1("cross",language,{requester:request.requester,platform:request.platform,class:platformClassV1(request.platform),own:platformClassV1(input.platform),served:split.served.length,left:split.left.length,named:split.left.length===0?"":": "+split.left.join(", ")})));};
 const tell=():void=>{try{input.work?.report(running!==undefined&&!running.finished&&running.reason==="request"&&held.size>0);}catch{}};
 const failed=(result:WantedInferV1)=>input.say(wantedTextV1("failed",language,{status:result.status,seconds:Math.round(result.seconds),detail:result.detail}));
 let savedKey="";
 const persist=():void=>{
  if(input.state===undefined)return;
  const state:ServeStateV1={version:1,owner:input.owner,platform:input.platform,savedMs:0,held:[...held.values()].map(entry=>({request:entry.request,list:entry.list,told:entry.told,toldAt:Number.isFinite(entry.toldAt)?entry.toldAt:0,heldAt:entry.heldAt})),told:[...told].slice(-4096),reading:running===undefined||running.finished?"none":running.reason},key=JSON.stringify(state);
  if(key===savedKey)return;
  savedKey=key;input.state.save({...state,savedMs:now()});
 };
 const remaining=(entry:Held,complete:boolean):number|null=>{
  if(complete)return 0;
  const done=new Set(entry.list),left=entry.request.projects.filter(name=>!done.has(name));
  return estimateRemainingMillisecondsV1({table:input.costs?.(),remaining:left,parallel:policy.waveProjects,...entry.list.length>0&&now()>entry.heldAt?{observed:{done:entry.list.length,milliseconds:now()-entry.heldAt}}:{}});
 };
 const answer=async(entry:Held,projects:readonly string[],generation:number|null)=>{
  entry.list=projects;entry.told=true;entry.toldAt=now();entry.toldProgressAt=progressAt;
  const complete=generation!==null;
  await publishServedV1({client:input.client,server:input.owner,request:entry.request,projects,generation,progress:{total:entry.request.projects.length,progressAtMs:Math.max(progressAt,entry.heldAt),remainingMilliseconds:remaining(entry,complete)},now}).catch(()=>false);
 };
 const start=(reason:"request"|"refresh",adopted?:Promise<WantedInferV1>)=>{
  if(adopted===undefined)clearSteeringV1(input.steerDirectory);priorityKey="";progressAt=now();dirtyCount=-1;
  const controller=new AbortController(),forward=()=>controller.abort(input.signal.reason);input.signal.addEventListener("abort",forward,{once:true});
  const state:Running={reason,startedAt:now(),controller,finished:false,done:Promise.resolve()};
  state.done=(adopted??input.infer(reason,controller.signal,[...new Set([...held.values()].flatMap(entry=>entry.list))].sort())).catch((error):WantedInferV1=>({ok:false,status:1,seconds:(now()-state.startedAt)/1000,generation:null,detail:String((error as Error)?.message??error).slice(0,200)})).then(async result=>{
   input.signal.removeEventListener("abort",forward);state.finished=true;
   if(input.signal.aborted)return;
   if(!result.ok){holdUntil=now()+server.refreshQuietMilliseconds;failed(result);if(reason==="refresh")refreshedAt=now();tell();return;}
   refreshedAt=now();
   for(const entry of held.values()){await answer(entry,entry.request.projects,result.generation);told.add(wantedClaimKeyV1(entry.request));input.say(wantedTextV1("served",language,{count:entry.request.projects.length,requester:entry.request.requester,seconds:Math.round(result.seconds)}));}
   if(reason==="refresh")input.say(wantedTextV1("refreshed",language,{seconds:Math.round(result.seconds)}));
   held.clear();tell();
  });
  running=state;
 };
 const steer=()=>{
  const table=input.costs?.(),asked=[...held.values()].sort((a,b)=>a.request.createdMs-b.request.createdMs),demand=new Map<string,number>();
  for(const entry of asked)for(const name of new Set(entry.request.projects))demand.set(name,(demand.get(name)??0)+1);
  const wanted=orderByCostV1(asked.flatMap(entry=>entry.request.projects),name=>estimateOwnerCostV1(table,name),new Set(asked.flatMap(entry=>entry.request.heavy??[])),demand),key=wanted.join("\n");
  if(key!==priorityKey){
   const same=priorityKey===""&&running!==undefined&&!running.finished?readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:input.steerDirectory}):undefined;
   priorityKey=key;
   if(same!==undefined&&same.projects.join("\n")===[...new Set(wanted)].join("\n"))priorityAt=same.atMs;
   else{priorityAt=now();writePriorityV1(input.steerDirectory,wanted,priorityAt);}
  }
 };
 const fresh=(request:WantedRequestV1):readonly string[]=>{
  const pending=readPendingV1(input.steerDirectory);
  if(pending===undefined||pending.priorityAtMs<priorityAt)return[];
  const dirty=new Set(pending.projects);
  return request.projects.filter(name=>!dirty.has(name));
 };
 const renewHeld=(request:WantedRequestV1)=>{
  const entry=held.get(request.requester);
  if(entry!==undefined&&wantedClaimKeyV1(entry.request)===wantedClaimKeyV1(request))entry.request=request;
 };
 const shareCosts=async()=>{
  const table=input.costs?.();if(table===undefined||Object.keys(table.owners).length===0)return;
  const key=JSON.stringify(table);if(key===costsKey||now()-costsAt<server.costsMilliseconds)return;
  costsKey=key;costsAt=now();await publishOwnerCostsV1({client:input.client,server:input.owner,table}).catch(()=>false);
 };
 const attached=input.signal.aborted?undefined:input.adopt?.(input.signal);
 if(attached!==undefined)start(attached.reason,attached.done);
 while(!input.signal.aborted&&(input.rounds===undefined||round++<input.rounds)){
  try{
   await shareCosts();
   if(running?.finished===true){await running.done;running=undefined;}
   foreignNow.clear();
   if(now()>=holdUntil&&(input.mayServe===undefined||await input.mayServe())){
    const taken=await claimWantedV1({client:input.client,owner:input.owner,platform:input.platform,foreign:noteForeign,cross:noteCross,live:renewHeld,requesters:await input.requesters().catch(()=>[]).then(list=>{seen=list.length;return list;}),known:key=>input.work?.restartPending()===true||told.has(key)||[...held.values()].some(entry=>wantedClaimKeyV1(entry.request)===key),signal:input.signal,now,policy});
    for(const request of taken){
     const before=held.get(request.requester);
     held.set(request.requester,{request,list:before?.list??[],told:before?.told??false,toldAt:before?.toldAt??-Infinity,toldProgressAt:before?.toldProgressAt??-Infinity,heldAt:before?.heldAt??now(),claimedAt:now()});
     if(before===undefined)input.say(wantedTextV1("serving",language,{count:request.projects.length,requester:request.requester}));
     else input.say(wantedTextV1("joined",language,{count:request.projects.length,arrived:before.list.length,requester:request.requester}));
    }
    for(const say of crossed.splice(0))say();
   }
   for(const [requester,entry] of held)if(entry.request.expiresMs<=now())held.delete(requester);
   for(const [requester,entry] of held){
    if(now()-entry.claimedAt<policy.claimRenewMilliseconds)continue;
    entry.claimedAt=now();
    const claimKey=wantedClaimKeyV1(entry.request);
    if(await input.client.renew(claimKey,input.owner,policy.claimTtlMilliseconds).catch(()=>"fault")==="lost"){
     const again=await input.client.claim(claimKey,input.owner,policy.claimTtlMilliseconds,policy.claimExpectedMilliseconds).catch(()=>undefined);
     if(again?.state==="granted"){entry.claimedAt=now();input.say(wantedTextV1("reclaimed",language,{requester}));}
     else if(again?.state==="held"&&again.by==="lifetime-cap")input.say(wantedTextV1("capped",language,{requester}));
     else if(again?.state==="held"){held.delete(requester);input.say(wantedTextV1("lost",language,{requester,by:again.by}));}
    }
   }
   if(held.size>0){
    steer();
    const pending=readPendingV1(input.steerDirectory);
    if(pending!==undefined&&pending.projects.length!==dirtyCount){dirtyCount=pending.projects.length;progressAt=now();}
    for(const entry of held.values()){
     const done=fresh(entry.request);
     if(!entry.told||done.length>entry.list.length){
      if(done.length>entry.list.length)input.say(wantedTextV1("answering",language,{arrived:done.length,count:entry.request.projects.length,requester:entry.request.requester}));
      await answer(entry,done,null);
     }else if(now()-entry.toldAt>=server.heartbeatMilliseconds||progressAt>entry.toldProgressAt)await answer(entry,entry.list,null);
    }
   }
   if(running===undefined&&!input.signal.aborted&&now()>=holdUntil){
    const room=await input.headroom();
    if(held.size>0&&room.pressure==="normal"&&room.freeMegabytes>=server.requestHeadroomMegabytes)start("request");
    else if(held.size===0&&input.work?.restartPending()!==true&&room.pressure==="normal"&&room.freeMegabytes>=server.refreshHeadroomMegabytes){
     const age=now()-refreshedAt,quiet=await input.quietMilliseconds();
     if((age>=server.refreshIntervalMilliseconds||(age>=server.refreshQuietMilliseconds&&quiet>=server.refreshQuietMilliseconds))&&!input.signal.aborted&&await input.refreshDeferred?.()!==true){
      if(input.mayRefresh!==undefined&&!await input.mayRefresh())refreshedAt=now();
      else{
       input.say(wantedTextV1("refreshing",language,{reason:`${Math.round(quiet/1000)} s quiet, ${Number.isFinite(age)?Math.round(age/1000)+" s":"never"} since the last`}));
       start("refresh");
      }
     }
    }
   }
  }catch(error){if(input.signal.aborted)break;holdUntil=now()+server.refreshQuietMilliseconds;input.say(wantedTextV1("failed",language,{status:1,seconds:0,detail:String((error as Error)?.message??error).slice(0,200)}));}
  persist();tell();
  if(++tick%server.aliveRounds===0)input.say(wantedTextV1("alive",language,{requesters:seen,held:held.size,cross:[...held.values()].filter(entry=>platformClassV1(entry.request.platform)!==platformClassV1(input.platform)).reduce((sum,entry)=>sum+entry.request.projects.length,0),left:[...held.values()].reduce((sum,entry)=>sum+(entry.request.left?.length??0),0)+[...foreignNow.values()].reduce((sum,count)=>sum+count,0),reading:running===undefined?"none":running.reason}));
  await input.sleep(server.roundMilliseconds,input.signal);
 }
 persist();
 if(running!==undefined){if(input.signal.aborted)running.controller.abort(input.signal.reason);await running.done;}
}


/** 🧾️ Why the server stops a graph read, as written to `read-stop.json` before the signal is sent (schema `ReadStop` in `../🧬️schema/🔣️.json`): the verdict, the milliseconds without progress, the newest generation and the highest counter of every phase. */
export function graphReadStopRowV1(watch:GraphReadWatchV1,verdict:"stalled"|"too-long"|"memory-overrun",runner:number,read:number|null,at:number=Date.now()):Readonly<{version:1;verdict:string;atMs:number;silentMilliseconds:number;generation:number|null;counters:Readonly<Record<string,readonly number[]>>;runner:number;read:number|null}>{
 return Object.freeze({version:1,verdict,atMs:at,silentMilliseconds:watch.silentMilliseconds(),generation:watch.generation(),counters:watch.counters(),runner,read});
}
