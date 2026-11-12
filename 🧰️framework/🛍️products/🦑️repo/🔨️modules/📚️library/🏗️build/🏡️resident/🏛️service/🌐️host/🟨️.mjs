import {preparePrivateDirectoryV1} from "../../📨️protocol/📍️endpoint/🔐️directory/🟨️.mjs";
/** 🌐 The per-host graph service: one process per user holds the fact index once and answers plan and status requests from every agent, worktree and workspace over one local socket or pipe. */

import {createServer,connect} from "node:net";
import {chmodSync,existsSync,lstatSync,mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {createHash,randomBytes} from "node:crypto";
import {join} from "node:path";
import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1,renderTextV1,validatorV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {authorizeCapabilityV1,createRootKeyV1,verifyCapabilityV1} from "../../📨️protocol/🔑️capability/🟨️.mjs";
import {workspaceIdV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {BusyErrorV1,createGovernorV1} from "../🚦️governor/🟨️.mjs";
import {createIndexPoolV1} from "./🧊️pool/🟨️.mjs";
import {boundedCacheStatsV1} from "../../../🧊️bounded/🟨️.mjs";
/** 🏃 Whether a build or test run of this machine still holds a live capacity grant (its row is touched by the run's heartbeat): the service hosts the cache endpoint of that run, so a silent run, a long test suite that prints nothing, is not idle time, and the service must outlive it. */
async function runInFlightV1(){
 try{
  const{hostCapacityDirectoryV1}=await import("../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts"),directory=join(hostCapacityDirectoryV1(process.env),"grants"),at=Date.now();
  return readdirSync(directory).some(name=>name.endsWith(".json")&&at-statSync(join(directory,name)).mtimeMs<120000);
 }catch{return false;}
}

const jsc=await import("bun:jsc").catch(()=>undefined);
const RELEVANT_OBJECT_TYPES=16,mebibytes=bytes=>Math.round(bytes/(1024*1024)*10)/10;
/** 📏 What this process holds: resident and peak pages, collector heap and external bytes, the most frequent object types (cheap counters, no heap walk) and every bounded cache. */
function memoryStatusV1(workers=[]){
 const usage=process.memoryUsage(),peak=(()=>{try{return Number(/^VmHWM:\s+(\d+) kB/mu.exec(readFileSync("/proc/self/status","utf8"))?.[1]);}catch{return NaN;}})(),heap=jsc?.heapStats?.();
 return{rssMb:mebibytes(usage.rss),...Number.isFinite(peak)?{hwmMb:mebibytes(peak*1024)}:{},heapMb:mebibytes(heap?.heapSize??usage.heapUsed),extraMb:mebibytes(heap?.extraMemorySize??usage.external),objectCount:heap?.objectCount??0,objectTypes:Object.entries(heap?.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,RELEVANT_OBJECT_TYPES).map(([type,count])=>({type,count})),caches:boundedCacheStatsV1().slice(0,64),workers:workers.slice(0,2)};
}

const validateClient=validatorV1("ClientFrame");
const validateReply=validatorV1("PlanReply");

/** 🚨 Another live host service already owns the endpoint. */
export class HostRunningErrorV1 extends Error{constructor(endpoint){super("A host graph service already listens on "+endpoint);this.name="HostRunningErrorV1";this.code="running";}}

const MiB=1024*1024;
/** 🔑 What makes two plan requests the same job: the digest of the index, the content scope (the digest of the captured inputs where the planner can bind a reply to each workspace, else the one workspace), revision, parent, tool, graph policy, arguments, targets and the dirty set. */
const jobKeyV1=(indexKey,scope,frame)=>sha(JSON.stringify([indexKey,scope,frame.revision??null,frame.parent??null,frame.tool??null,frame.graphPolicy??null,frame.arguments??null,[...new Set(frame.targets)].sort(),frame.dirty??null]));
const sha=text=>createHash("sha256").update(text).digest("hex");
const hangUp=socket=>{socket.end();setTimeout(()=>socket.destroy(),100).unref?.();};
const alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}};

function probe(endpoint,timeout=500){
 return new Promise(resolve=>{
  const socket=connect(endpoint),done=value=>{socket.destroy();resolve(value);};
  socket.setTimeout(timeout,()=>done(false));socket.once("connect",()=>done(true));socket.once("error",()=>done(false));
 });
}

function atomicWrite(path,text,mode=0o600){
 const temporary=path+"."+randomBytes(4).toString("hex")+".tmp";
 writeFileSync(temporary,text,{mode});renameSync(temporary,path);
}

/** 🌐 Creates the service; call `start()`, then wait for `closed`. `ports.planner` is `{identify?,load,plan,contentDigest?,bind?}` (`contentDigest` and `bind` together let workspaces with equal captured inputs share one job, `bind` answering each client's own pin, manifest and revision) (see `createGraphPlannerPortV1`); `ports.admission` defaults to the fair governor. */
export function createHostServiceV1(options){
 const{runtime,ports,implementation,log=()=>{},now=()=>Date.now(),requireCapability=true,onExit=()=>{}}=options;
 const policy=Object.freeze({...HOST_POLICY_V1,...(options.policy??{})});
 const posix=process.platform!=="win32",instance=randomBytes(8).toString("hex"),startedAt=now();
 const governor=ports.admission??createGovernorV1({maximumInFlight:policy.maximumInFlight,maximumQueued:policy.maximumQueued});
 const pool=createIndexPoolV1({maximumBytes:policy.maximumIndexBytes,maximumEntries:policy.maximumWorkspaces,now,onEvict:key=>{for(const memoKey of[...memo.keys()])if(memoKey.startsWith(key))memo.delete(memoKey);}});
 const memo=new Map(),workspaces=new Map(),connections=new Set(),spills=join(runtime.directory,"plans-"+runtime.tag);
 const counters={plans:0,hits:0,refused:0,evictedWorkspaces:0,jobs:0,coalesced:0,abandoned:0},jobs=new Map(),planned={budgetRefusals:0,load:undefined,computed:0,milliseconds:0,lastMilliseconds:0,captureFiles:0,captureProbes:0,unstaged:0};
 let server,rootKey,closeResolve,closing=false,lastActivity=now(),idleTimer,residentTimer,supersededSince;
 const closed=new Promise(resolve=>{closeResolve=resolve;});
 const touch=()=>{lastActivity=now();};

 const send=(connection,frame)=>{
  const socket=connection.socket;
  if(socket.destroyed||!socket.writable)return Promise.resolve();
  if(socket.write(encodeFrameV1(frame)))return Promise.resolve();
  return new Promise(resolve=>{const done=()=>{socket.off("drain",done);socket.off("close",done);resolve();};socket.once("drain",done);socket.once("close",done);});
 };
 const refuse=(connection,id,code,key,fields,retry)=>{
  counters.refused++;
  return send(connection,{t:"refused",...(id!==undefined?{id}:{}),code,message:renderBothV1(key,fields),retry});
 };
 const text=(connection,key,fields)=>renderTextV1(key,fields,connection.languages??["en","de"])??renderBothV1(key,fields);

 const spill=plan=>{
  const body=JSON.stringify(plan),digest=createHash("sha256").update(body).digest("hex");
  mkdirSync(spills,{recursive:true,mode:0o700});
  const path=join(spills,digest+".json");
  if(!existsSync(path))atomicWrite(path,body);else utimesSync(path,new Date(),new Date());
  const files=readdirSync(spills).filter(name=>name.endsWith(".json")).map(name=>({name,at:statSync(join(spills,name)).mtimeMs})).sort((a,b)=>b.at-a.at);
  for(const stale of files.slice(policy.planMemoEntries))rmSync(join(spills,stale.name),{force:true});
  return{ref:{hash:digest,size:Buffer.byteLength(body)}};
 };

 const openWorkspace=frame=>{
  let workspace=workspaces.get(frame.workspace);
  if(workspace){workspaces.delete(frame.workspace);workspaces.set(frame.workspace,workspace);workspace.lastUsedAt=now();return workspace;}
  if(workspaceIdV1(frame.root)!==frame.workspace)return undefined;
  try{if(!lstatSync(frame.root).isDirectory())return undefined;}catch{return undefined;}
  while(workspaces.size>=policy.maximumWorkspaces)closeWorkspace(workspaces.keys().next().value);
  workspace={id:frame.workspace,root:frame.root,lastUsedAt:now(),plans:0,handle:undefined,indexKey:undefined};
  workspaces.set(frame.workspace,workspace);
  return workspace;
 };
 const closeWorkspace=id=>{
  const workspace=workspaces.get(id);if(!workspace)return;
  workspaces.delete(id);counters.evictedWorkspaces++;
  workspace.handle?.release();workspace.handle=undefined;
 };
 const indexKeyOf=async(workspace,signal)=>workspace.handle?workspace.indexKey:(await ports.planner.identify?.({root:workspace.root,signal}))??sha("workspace\0"+workspace.id);
 const scopeOf=async(workspace,frame,signal)=>{
  const digest=ports.planner.contentDigest&&ports.planner.bind?await ports.planner.contentDigest({root:workspace.root,workspace:workspace.id,targets:frame.targets,arguments:frame.arguments,tool:frame.tool,graphPolicy:frame.graphPolicy,dirty:frame.dirty,signal}):undefined;
  return digest===undefined?"workspace:"+workspace.id+"\0"+workspace.root:"content:"+digest;
 };
 const ensureIndex=async(workspace,signal,phases,progress,key)=>{
  if(workspace.handle)return{hit:true};
  const started=performance.now();
  await progress("load",0,1,"host-load",{workspace:workspace.id});
  const handle=await pool.acquire(key,loadSignal=>ports.planner.load({root:workspace.root,workspace:workspace.id,signal:loadSignal}),signal);
  if(workspaces.get(workspace.id)!==workspace){handle.release();throw Object.assign(new Error("closed"),{code:"closed"});}
  if(workspace.handle)handle.release();else{workspace.handle=handle;workspace.indexKey=key;}
  phases.push({phase:"load",ms:Math.round((performance.now()-started)*1000)/1000});
  return{hit:handle.hit};
 };

 async function runPlan(connection,frame){
  counters.plans++;touch();
  if(closing)return refuse(connection,frame.id,"shutdown","shutdown",{},"fallback");
  if(supersededSince!==undefined&&connection.implementation!==implementation)return refuse(connection,frame.id,"stale","host-stale",{},"fallback");
  if(requireCapability){
   const token=frame.cap??connection.cap,caveats=token?verifyCapabilityV1(rootKey,token):undefined;
   if(!caveats)return refuse(connection,frame.id,"capability","capability",{},"never");
   const denial=authorizeCapabilityV1(caveats,{commandClass:"query",command:"plan",cwd:frame.root,now:now(),projects:frame.targets.map(target=>target.split(":")[0]),worktree:frame.workspace,agent:frame.agent});
   if(denial)return refuse(connection,frame.id,denial.code,denial.code==="scope"?"scope":"capability",{class:"query"},"never");
  }
  const controller=new AbortController();connection.requests.set(frame.id,controller);
  const deadline=frame.deadlineMs?setTimeout(()=>controller.abort(Object.assign(new Error("deadline"),{code:"deadline"})),frame.deadlineMs):undefined;
  const client={connection,frame,controller};
  try{
   await send(connection,{t:"accepted",id:frame.id,queue:governor.queued});
   controller.signal.throwIfAborted();
   const workspace=openWorkspace(frame);
   if(!workspace){await refuse(connection,frame.id,"invalid","host-unknown-workspace",{workspace:frame.workspace},"never");return;}
   const outcome=await attach(workspace,client,await indexKeyOf(workspace,controller.signal),await scopeOf(workspace,frame,controller.signal));
   if(outcome.error)throw outcome.error;
  }catch(error){
   if(error instanceof BusyErrorV1)await refuse(connection,frame.id,"busy","busy",{inFlight:error.inFlight,queued:error.queued},"later");
   else if(error?.code==="memory-pressure"){planned.budgetRefusals++;await refuse(connection,frame.id,"memory","host-memory-pressure-"+error.kind,{value:error.value,limit:error.limit,seconds:error.seconds},"later");}
   else if(error?.code==="memory-budget"){planned.budgetRefusals++;await refuse(connection,frame.id,"memory","host-memory-budget",{current:error.currentMegabytes,limit:error.limitMegabytes,margin:error.marginMegabytes},"later");}
   else if(error?.name==="RepoError"&&error.code==="stalled")await refuse(connection,frame.id,"stalled","host-stalled",{minutes:error.values?.minutes??0},error.values?.retry==="later"?"later":"never");
   else if(error?.code==="unknown-target")await refuse(connection,frame.id,"invalid","host-unknown-target",{target:error.target},"never");
   else if(controller.signal.aborted||error?.name==="AbortError"){
    const reason=controller.signal.reason;
    if(reason?.code==="deadline")await refuse(connection,frame.id,"deadline","deadline",{milliseconds:frame.deadlineMs},"never");
    else await refuse(connection,frame.id,"shutdown","shutdown",{},"never");
   }else{const reason=String(error?.stack??error),reference=createHash("sha256").update(reason).digest("hex").slice(0,8);log("plan-failed",reference+" "+reason);await refuse(connection,frame.id,"internal","internal",{reference},"later");}
  }finally{clearTimeout(deadline);connection.requests.delete(frame.id);touch();}
 }

 /** 🤝 Attaches a client to the running job of an identical request, or starts the job; resolves when the job answered this client, rejects-by-outcome when it failed, and detaches the client (cancelling the job with its last client) when the client goes away first. */
 function attach(workspace,client,indexKey,scope){
  const key=jobKeyV1(indexKey,scope,client.frame);
  client.workspace=workspace;
  let job=jobs.get(key);
  if(job)counters.coalesced++;else{job=startJob(key,workspace,client.frame,indexKey);}
  job.clients.add(client);
  for(const replay of job.progress.values())void sendProgress(client,replay);
  return new Promise(resolve=>{
   client.resolve=resolve;
   const gone=()=>{
    job.clients.delete(client);
    if(job.clients.size===0){if(jobs.get(key)===job)jobs.delete(key);job.controller.abort(Object.assign(new Error("abandoned"),{code:"abandoned"}));counters.abandoned++;}
    resolve({error:client.controller.signal.reason??new Error("aborted")});
   };
   if(client.controller.signal.aborted)gone();else client.controller.signal.addEventListener("abort",gone,{once:true});
   job.settled.then(()=>client.controller.signal.removeEventListener("abort",gone));
  });
 }

 const ownerLine=row=>row.name+" ("+(row.kind??"-")+", "+Math.round(row.seconds)+", "+(row.rows??0)+")";
 const sendProgress=(client,{phase,done,total,key,fields})=>{const{running,decision,...rest}=fields,owners=running===undefined?undefined:running.slice(0,3).map(ownerLine).join(", ");return send(client.connection,{t:"progress",id:client.frame.id,phase,done,total,message:text(client.connection,owners===undefined?key:decision===undefined?"host-slice-owners":"host-slice-owners-"+decision.key,{...rest,...owners===undefined?{}:{owners},...decision===undefined?{}:{used:decision.used,budget:decision.budget,kind:decision.kind??""}})});};

 function startJob(key,workspace,frame,indexKey){
  const controller=new AbortController(),job={key,controller,clients:new Set(),progress:new Map()};
  jobs.set(key,job);counters.jobs++;
  const progress=(phase,done,total,messageKey,fields)=>{
   const row={phase,done,total,key:messageKey,fields};job.progress.set(phase,row);
   return Promise.all([...job.clients].map(client=>sendProgress(client,row)));
  };
  const finish=outcome=>{for(const client of[...job.clients])client.resolve?.(outcome);job.clients.clear();if(jobs.get(key)===job)jobs.delete(key);};
  job.settled=run(job,workspace,frame,progress,indexKey).then(()=>finish({}),error=>finish({error}));
  return job;
 }

 async function run(job,workspace,frame,progress,indexKey){
  const phases=[];let release;
  try{
   const queued=performance.now();
   release=await governor.enter(frame.agent+"/"+frame.priority,job.controller.signal,position=>{for(const client of job.clients)void send(client.connection,{t:"progress",id:client.frame.id,phase:"queued",done:0,total:0,message:text(client.connection,"queued",{position})});});
   phases.push({phase:"queued",ms:Math.round((performance.now()-queued)*1000)/1000});
   job.controller.signal.throwIfAborted();
   const{hit}=await ensureIndex(workspace,job.controller.signal,phases,progress,indexKey);
   const memoKey=workspace.indexKey+"\0"+workspace.id+"\0"+(frame.revision??"")+"\0"+(frame.parent??"")+"\0"+(frame.tool??"")+"\0"+(frame.graphPolicy??"")+"\0"+(frame.arguments?.join("\u0001")??"")+"\0"+[...frame.targets].sort().join(","),started=performance.now();
   const cursor=frame.dirty?.cursor,live=frame.dirty!==undefined;
   let plan=live?undefined:memo.get(memoKey),planHit=plan!==undefined;
   if(planHit){memo.delete(memoKey);memo.set(memoKey,plan);}
   else{
    await progress("plan",0,frame.targets.length,"host-plan",{targets:frame.targets.length});
    const answer=await ports.planner.plan({index:workspace.handle,targets:frame.targets,revision:frame.revision,parent:frame.parent,dirty:frame.dirty,cursor,agent:frame.agent,workspace:workspace.id,root:workspace.root,arguments:frame.arguments,tool:frame.tool,graphPolicy:frame.graphPolicy,signal:job.controller.signal,progress:(milliseconds,detail)=>progress("slice",milliseconds,0,detail?"host-slice-detail":"host-slice",{seconds:Math.round(milliseconds/1000),...detail?{phase:detail.phase,done:detail.done,total:detail.total,...detail.running===undefined?{}:{running:detail.running,...detail.decision===undefined?{}:{decision:detail.decision}}}:{}})});
    planned.computed++;planned.lastMilliseconds=Math.round(performance.now()-started);planned.milliseconds+=planned.lastMilliseconds;planned.captureFiles=answer.stats?.captureFiles??0;planned.captureProbes=answer.stats?.captureProbes??0;planned.unstaged=answer.stats?.unstaged??0;planned.load=answer.stats?.load;
    plan={revision:answer.revision,tasks:answer.tasks,...answer.slice?{slice:spill(answer.slice).ref}:{},...answer.manifest?{manifest:answer.manifest}:{},...answer.pin?{pin:answer.pin}:{}};
    if(!live&&!answer.pin){memo.set(memoKey,plan);while(memo.size>policy.planMemoEntries)memo.delete(memo.keys().next().value);}
   }
   phases.push({phase:"plan",ms:Math.round((performance.now()-started)*1000)/1000});
   workspace.plans++;if(hit&&planHit)counters.hits++;
   const reply={t:"plan-reply",revision:plan.revision,...plan.manifest?{manifest:plan.manifest}:{},...plan.pin?{pin:plan.pin}:{},...plan.slice?{slice:plan.slice}:{},plan:plan.tasks.length>policy.maximumInlinePlanTasks?spill({tasks:plan.tasks}):{tasks:plan.tasks},progress:phases,cacheHits:hit&&planHit?1:0};
   if(validateReply({...reply,id:1}).length)throw new Error("planner produced an invalid reply: "+validateReply({...reply,id:1})[0]);
   const bound=client=>ports.planner.bind?ports.planner.bind({root:client.workspace.root,workspace:client.workspace.id,agent:client.frame.agent,reply,signal:job.controller.signal}):undefined;
   await Promise.all([...job.clients].map(async client=>{
    const own=await bound(client),full={...reply,...own,id:client.frame.id};
    if(own&&validateReply(full).length)throw new Error("planner bound an invalid reply: "+validateReply(full)[0]);
    return send(client.connection,full);
   }));
  }finally{release?.();}
 }

 const status=id=>{
  const stats=pool.stats();
  return{t:"service-status",id,pid:process.pid,instance,implementation,startedAt,rssMb:Math.round(process.memoryUsage().rss/MiB*10)/10,indexBytes:stats.bytes,indexes:stats.indexes,
   workspaces:[...workspaces.values()].map(w=>({id:w.id,root:w.root,indexKey:w.indexKey??"0".repeat(64),indexBytes:w.indexKey===undefined?0:pool.size(w.indexKey),lastUsedAt:w.lastUsedAt,plans:w.plans})),memory:memoryStatusV1(ports.workers?.memory?.()),plans:{total:counters.plans,hits:counters.hits,computed:planned.computed,lastMilliseconds:planned.lastMilliseconds,totalMilliseconds:planned.milliseconds,captureFiles:planned.captureFiles,captureProbes:planned.captureProbes,unstaged:planned.unstaged,budgetRefusals:planned.budgetRefusals,...planned.load===undefined?{}:{load:planned.load}},clients:connections.size,queue:governor.queued,inFlight:governor.inFlight};
 };

 function handleConnection(socket){
  if(connections.size>=policy.maximumClients){socket.end(encodeFrameV1({t:"refused",code:"busy",message:renderBothV1("busy",{inFlight:governor.inFlight,queued:governor.queued}),retry:"later"}));return;}
  touch();
  const connection={socket,languages:undefined,cap:undefined,requests:new Map(),helloDone:false,implementation:undefined};
  connections.add(connection);
  const decoder=createFrameDecoderV1({maximumFrameBytes:policy.maximumFrameBytes,
   onFrame:frame=>{
    const errors=validateClient(frame);
    if(errors.length){void refuse(connection,typeof frame?.id==="number"?frame.id:undefined,"invalid","invalid",{reason:errors[0]},"never").then(()=>hangUp(socket));return;}
    if(frame.t==="hello"){
     if(connection.helloDone){void refuse(connection,undefined,"invalid","invalid",{reason:"hello twice"},"never");return;}
     if(frame.implementation!==undefined&&frame.implementation!==implementation){void refuse(connection,undefined,"stale","host-stale",{},"fallback").then(()=>hangUp(socket));return;}
     connection.helloDone=true;connection.languages=frame.languages;connection.cap=frame.cap;connection.implementation=frame.implementation;
     void send(connection,{t:"welcome",v:1,features:["plan","host-status"],service:{pid:process.pid,instance,startedAt,implementation,generation:null,watcher:{backend:"none",trust:"none",cursor:"0"}},limits:{maximumFrameBytes:policy.maximumFrameBytes,maximumInFlight:policy.maximumInFlight}});
     return;
    }
    if(!connection.helloDone){void refuse(connection,undefined,"invalid","invalid",{reason:"hello first"},"never").then(()=>hangUp(socket));return;}
    if(frame.t==="ping"){void send(connection,{t:"pong",n:frame.n});return;}
    if(frame.t==="cancel"){connection.requests.get(frame.id)?.abort(Object.assign(new Error("cancelled"),{code:"cancelled"}));return;}
    if(frame.t==="host-status"){touch();void send(connection,status(frame.id));return;}
    if(frame.t==="plan"){
     if(connection.requests.has(frame.id)){void refuse(connection,frame.id,"invalid","invalid",{reason:"request id in use"},"never");return;}
     void runPlan(connection,frame);return;
    }
    void refuse(connection,frame.id,"unknown-command","unknown-command",{command:frame.t==="request"?frame.command:frame.t},"never");
   },
   onError:error=>{void refuse(connection,undefined,"invalid","invalid",{reason:error.message},"never").then(()=>hangUp(socket));},
  });
  socket.on("data",chunk=>decoder.push(chunk));
  socket.on("end",()=>{if(connection.requests.size===0)socket.end();});
  socket.on("error",()=>{});
  socket.on("close",()=>{connections.delete(connection);for(const controller of connection.requests.values())controller.abort(Object.assign(new Error("disconnected"),{code:"disconnected"}));touch();});
 }

 let listening=false;
 async function listen(){
  const tryListen=()=>new Promise((resolve,reject)=>{
   server=createServer({allowHalfOpen:false},handleConnection);
   server.once("error",reject);server.listen(runtime.endpoint,()=>{server.off("error",reject);resolve();});
  });
  try{await tryListen();}
  catch(error){
   if(error?.code!=="EADDRINUSE")throw error;
   if(await probe(runtime.endpoint)){
    const stopping=()=>{try{return JSON.parse(readFileSync(runtime.descriptor,"utf8")).stopping===true;}catch{return false;}};
    if(!stopping())throw new HostRunningErrorV1(runtime.endpoint);
    const deadline=now()+policy.takeoverMilliseconds;
    while(now()<deadline&&await probe(runtime.endpoint))await new Promise(resolve=>setTimeout(resolve,50));
    if(await probe(runtime.endpoint))throw new HostRunningErrorV1(runtime.endpoint);
   }
   if(posix)rmSync(runtime.endpoint,{force:true});
   await tryListen();
  }
  listening=true;
  if(posix)try{chmodSync(runtime.endpoint,0o600);}catch{}
 }

 const writeDescriptor=(stopping=false)=>{
  try{atomicWrite(runtime.descriptor,JSON.stringify({v:1,pid:process.pid,instance,startedAt,endpoint:runtime.endpoint,protocol:1,implementation,...(stopping?{stopping:true}:{})}));}catch(error){log("descriptor-failed",String(error));}
 };

 const newerSibling=()=>{
  try{
   for(const name of readdirSync(runtime.directory)){
    if(!/^host-[0-9a-f]{12}\.json$/u.test(name)||name==="host-"+runtime.tag+".json")continue;
    const other=JSON.parse(readFileSync(join(runtime.directory,name),"utf8"));
    if(other.startedAt>startedAt&&!other.stopping&&alive(other.pid))return true;
   }
  }catch{}
  return false;
 };

 const sweep=()=>{
  const at=now();
  for(const workspace of[...workspaces.values()])if(at-workspace.lastUsedAt>policy.workspaceIdleMilliseconds)closeWorkspace(workspace.id);
  const rss=(ports.resident?.rss??(()=>process.memoryUsage().rss))(),governed=(ports.workers?.active?.()??0)>0;
  if(!governed&&rss>policy.maximumResidentBytes){
   for(const id of[...workspaces.keys()])closeWorkspace(id);
   pool.clear();memo.clear();globalThis.Bun?.gc?.(true);
   if((ports.resident?.rss??(()=>process.memoryUsage().rss))()>policy.maximumResidentBytes*1.25)void drainAndExit("memory");
  }
  if(newerSibling()){supersededSince??=at;}else supersededSince=undefined;
  if(supersededSince!==undefined&&connections.size===0&&governor.inFlight===0&&at-supersededSince>=policy.supersededGraceMilliseconds)void drainAndExit("superseded");
 };

 async function drainAndExit(reason){
  if(closing)return;
  closing=true;log("shutdown",reason);
  clearInterval(idleTimer);clearInterval(residentTimer);
  if(listening)writeDescriptor(true);
  for(const connection of connections)void send(connection,{t:"notice",kind:"draining",message:text(connection,"shutdown",{})});
  server?.close();
  const deadline=now()+5000;
  while((governor.inFlight>0||[...connections].some(c=>c.requests.size>0))&&now()<deadline)await new Promise(resolve=>setTimeout(resolve,20));
  for(const connection of connections){for(const controller of connection.requests.values())controller.abort(new Error("shutdown"));connection.socket.destroy();}
  for(const id of[...workspaces.keys()])closeWorkspace(id);
  pool.clear();memo.clear();
  if(listening){if(posix)rmSync(runtime.endpoint,{force:true});rmSync(runtime.descriptor,{force:true});}
  closeResolve(reason);onExit(reason);
 }

 return{
  instance,closed,pool,counters,
  async start(){
   preparePrivateDirectoryV1(runtime.directory);
   if(existsSync(runtime.key))rootKey=readFileSync(runtime.key);
   else{rootKey=createRootKeyV1();try{writeFileSync(runtime.key,rootKey,{flag:"wx",mode:0o600});}catch{rootKey=readFileSync(runtime.key);}}
   await listen();writeDescriptor();
   idleTimer=setInterval(()=>{if(connections.size===0&&governor.inFlight===0&&now()-lastActivity>policy.idleShutdownMilliseconds)void runInFlightV1().then(running=>{if(running)lastActivity=now();else if(connections.size===0&&governor.inFlight===0)void drainAndExit("idle");});},Math.max(250,Math.min(policy.idleShutdownMilliseconds/4,30000)));idleTimer.unref?.();
   residentTimer=setInterval(sweep,Math.max(50,policy.residentCheckMilliseconds));residentTimer.unref?.();
   return{instance,endpoint:runtime.endpoint};
  },
  stop:reason=>drainAndExit(reason??"stop"),
  rootKey:()=>rootKey,
  status:()=>status(1),
  sweep,
 };
}
