import{createHash}from"node:crypto";
import{availableParallelism,freemem}from"node:os";
import{spawn as nodeSpawn}from"node:child_process";
import{fileURLToPath}from"node:url";
import{rowKeyV1}from"../../../🕰️generations/🧮️fact-rows/🟨️.mjs";
import{planFarmWorkersV1,reserveFarmV1}from"../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import{localMachineSpeedFactorV1,scaledMillisecondsV1}from"../../../../🐢️speed/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🏭️ The declared limits of the closure walk farm; the schema file is the single source. */
export const WALK_FARM_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));

const partIds=new WeakMap();
/** 🪪️ The identity of one part of a closure's support (the rows one module node observed, or the rows of one shared computation): the digest of its row keys, kept per array, so a part shared by thousands of closures is identified once. The settler and the walk children use this one function, so a child names a part the parent already holds by its id alone. */
export function partIdOfV1(rows){
 let id=partIds.get(rows);
 if(id===undefined){const hash=createHash("sha256");for(const row of rows)hash.update(rowKeyV1(row)).update("\n");id=hash.digest("hex").slice(0,20);partIds.set(rows,id);}
 return id;
}

/** 📏️ How many walk children this process may run: the granted processors minus the one the parent uses, as many as the granted memory carries after the reserve, never more than the policy maximum; below two there is no farm. A syscall-bound walk gains from every child, so the host load is not subtracted: the grant is what the scheduler admitted.
 * @param {{cpus:number;availableBytes:number;reserveBytes?:number}} capacity */
export function walkFarmSizeV1(capacity,policy=WALK_FARM_V1){
 const memory=Math.floor(Math.max(0,capacity.availableBytes-(capacity.reserveBytes??0))/policy.childBytes),size=Math.min(policy.maximumChildren,capacity.cpus-1,memory);
 return size<2?0:size;
}

/** 🚫️ Whether the environment switched the farm off. */
export const walkFarmOffV1=(environment=process.env)=>environment[WALK_FARM_V1.environment]===WALK_FARM_V1.disabledValue;

const CHILD=fileURLToPath(new URL("./👷️child/🟨️.mjs",import.meta.url));
const farmFailure=(code,message)=>Object.assign(new Error(message),{code});

/**
 * 🏭️ Walks closure questions in child processes (`./👷️child`) so independent closures run on several processors: `derive(row, signal)` answers `{fact, parts:[{id, rows?}]}` (a part the child already sent to this parent comes by its id alone, so a closure of a hundred thousand rows costs the wire a few kilobytes) or rejects with code `WALK_FARM_*` so the caller walks the question itself. At most `size` children, started on demand, each fed up to `policy.batch` questions at once so it never waits for a round trip; a child that dies, stalls past the request limit or ends on memory is replaced on the next request; a child whose stdin closes ends, so none outlives the parent. Messages are lines `<marker>{json}`; anything else a child prints is ignored.
 * @see ./🧬️schema/🔣️.json
 * @param {{root:string;size:number;module?:string;spawn?:typeof nodeSpawn;policy?:typeof WALK_FARM_V1;lease?:{release():void}}} options
 */
export function createWalkFarmV1({root,size,module,spawn=nodeSpawn,policy=WALK_FARM_V1,lease,speed=localMachineSpeedFactorV1()}){
 const command=[process.execPath,CHILD,root,...module===undefined?[]:[module]],marker=policy.marker;
 const children=new Set(),queue=[],stats={started:0,answered:0,failed:0,recycled:0,bytes:0,byPid:new Map()};
 let closed=false,sequence=0,failedStarts=0;
 const waiters=new Set();
 const finish=(child,error)=>{
  if(!children.delete(child))return;
  clearTimeout(child.startTimer);
  for(const job of child.jobs.values()){clearTimeout(job.timer);stats.failed++;job.reject(error??farmFailure("WALK_FARM_CHILD_ENDED","A walk child ended while it walked a question"));}
  child.jobs.clear();
  pump();
 };
 const settle=(child,message)=>{
  const job=child.jobs.get(message.id);if(job===undefined)return;
  child.jobs.delete(message.id);clearTimeout(job.timer);
  if(message.answer!==undefined){stats.answered++;stats.byPid.set(child.pid,(stats.byPid.get(child.pid)??0)+1);job.resolve(message.answer);}
  else{stats.failed++;job.reject(Object.assign(farmFailure("WALK_FARM_QUESTION_FAILED",message.failure?.message??"A walk child could not walk the question"),{childCode:message.failure?.code,childName:message.failure?.name}));}
  pump();
 };
 const read=(child,line)=>{
  const at=line.lastIndexOf(marker);if(at<0)return;
  stats.bytes+=line.length;
  let message;try{message=JSON.parse(line.slice(at+marker.length));}catch{return;}
  if(message.ready===true){child.ready=true;clearTimeout(child.startTimer);for(const wake of[...waiters])wake();pump();}
  else if(message.recycle===true){stats.recycled++;child.recycling=true;}
  else settle(child,message);
 };
 const start=()=>{
  stats.started++;
  const process_=spawn(command[0],command.slice(1),{stdio:["pipe","pipe","inherit"],windowsHide:true}),child={process:process_,pid:process_.pid,ready:false,jobs:new Map(),recycling:false,startTimer:undefined,pending:[]};
  children.add(child);
  child.startTimer=setTimeout(()=>{failedStarts++;process_.kill("SIGKILL");finish(child,farmFailure("WALK_FARM_START_TIMEOUT","A walk child did not become ready"));},policy.startTimeoutMilliseconds);child.startTimer.unref?.();
  process_.stdout.setEncoding("utf8");process_.unref?.();
  process_.stdout.on("data",chunk=>{
   let from=0;
   for(let cut=chunk.indexOf("\n",from);cut>=0;cut=chunk.indexOf("\n",from)){child.pending.push(chunk.slice(from,cut));read(child,child.pending.join(""));child.pending.length=0;from=cut+1;}
   if(from<chunk.length)child.pending.push(chunk.slice(from));
  });
  process_.stdin.on("error",()=>undefined);
  process_.on("error",()=>{failedStarts++;finish(child,farmFailure("WALK_FARM_START_FAILED","A walk child could not start"));});
  process_.on("exit",()=>finish(child));
 };
 function pump(){
  while(queue.length){
   if(closed){queue.shift().reject(farmFailure("WALK_FARM_CLOSED","The walk farm is closed"));continue;}
   if(failedStarts>=policy.failuresToStop){queue.shift().reject(farmFailure("WALK_FARM_UNAVAILABLE","The walk children cannot start"));continue;}
   let target;for(const child of children)if(child.ready&&!child.recycling&&child.jobs.size<policy.batch&&(target===undefined||child.jobs.size<target.jobs.size))target=child;
   if(target===undefined){if(children.size<size&&[...children].filter(child=>!child.ready).length<queue.length)start();return;}
   const job=queue.shift();
   if(job.signal?.aborted){job.reject(job.signal.reason);continue;}
   target.jobs.set(job.id,job);
   job.timer=setTimeout(()=>{target.process.kill("SIGKILL");finish(target,farmFailure("WALK_FARM_REQUEST_TIMEOUT","A walk child took too long for a question"));},scaledMillisecondsV1(policy.requestTimeoutMilliseconds,speed));job.timer.unref?.();
   try{target.process.stdin.write(marker+JSON.stringify({id:job.id,row:job.row})+"\n");}catch{target.process.kill("SIGKILL");}
  }
 }
 return Object.freeze({
  size,
  batch:policy.batch,
  warm(){while(children.size<size&&!closed&&failedStarts<policy.failuresToStop)start();},
  ready:()=>[...children].some(child=>child.ready&&!child.recycling),
  whenReady:milliseconds=>new Promise(resolve=>{const ok=()=>[...children].some(child=>child.ready&&!child.recycling);if(ok())return resolve(true);const done=()=>{clearTimeout(timer);waiters.delete(wake);resolve(ok());},wake=()=>done(),timer=setTimeout(done,milliseconds);timer.unref?.();waiters.add(wake);}),
  derive:(row,signal)=>new Promise((resolve,reject)=>{
   if(closed)return reject(farmFailure("WALK_FARM_CLOSED","The walk farm is closed"));
   signal?.throwIfAborted();
   const job={id:sequence++,row:{kind:row.kind,path:row.path,fact:row.fact},resolve,reject,signal,timer:undefined};
   signal?.addEventListener("abort",()=>{const at=queue.indexOf(job);if(at>=0){queue.splice(at,1);reject(signal.reason);}else{for(const child of children)if(child.jobs.has(job.id)){child.process.kill("SIGKILL");finish(child,signal.reason);}}},{once:true});
   queue.push(job);pump();
  }),
  stats:()=>({started:stats.started,answered:stats.answered,failed:stats.failed,recycled:stats.recycled,bytes:stats.bytes,children:children.size,byPid:Object.fromEntries(stats.byPid)}),
  close(){closed=true;lease?.release();for(const child of[...children]){clearTimeout(child.startTimer);try{child.process.stdin.end();}catch{}child.process.kill("SIGTERM");}pump();},
 });
}

/** 🪢️ How many walk children the memory plan of this process leaves: the same bound the project farm obeys (`planFarmWorkersV1`: the whole process tree stays within the tree share of the physical memory, and the cut of a load that grew too big, `SEMIO_GRAPH_CUT_CONCURRENCY`, caps it), with a child costing `childBytes`; Infinity where the plan has no limit (a machine that serves the fleet). The granted memory alone sized the farm to eight children on a 16 GB worker, whose tree then stood at about 10 GB during the verification and was refused although the work could not be cut any further.
 * @param {{environment?:Record<string,string|undefined>;hostMegabytes?:number;residentMegabytes?:number;policy?:typeof WALK_FARM_V1}} [options] */
export function walkFarmCapV1({environment=process.env,hostMegabytes,residentMegabytes,policy=WALK_FARM_V1}={}){
 return planFarmWorkersV1({kind:"walk-farm",workerMegabytes:policy.childBytes/1048576,environment,...hostMegabytes===undefined?{}:{hostMegabytes},...residentMegabytes===undefined?{}:{residentMegabytes}});
}

/** 🧮️ Takes the walk children of a farm from the one memory plan of the process tree (`reserveFarmV1`, kind `walk-farm`): `wanted` children, fewer where the project farm, the cut or the share leave less; `yield` is called when the project farm needs the memory back. The farm hands the lease to `createWalkFarmV1`, which releases it at its close.
 * @param {number} wanted @param {{yield?:()=>void;environment?:Record<string,string|undefined>;hostMegabytes?:number;residentMegabytes?:number;policy?:typeof WALK_FARM_V1}} [options] */
export function reserveWalkFarmV1(wanted,{yield:onYield,environment=process.env,hostMegabytes,residentMegabytes,policy=WALK_FARM_V1}={}){
 return reserveFarmV1({kind:"walk-farm",workerMegabytes:policy.childBytes/1048576,wanted,environment,yield:onYield,...hostMegabytes===undefined?{}:{hostMegabytes},...residentMegabytes===undefined?{}:{residentMegabytes}});
}

let capacitySource=()=>({cpus:availableParallelism(),availableBytes:freemem()});
/** 🎟️ Binds the capacity the shared farm of closure recipes is sized from (the grant the scheduler admitted to this process). */
export function bindWalkFarmCapacityV1(source){capacitySource=source;}

const recipeFarms=new Map();
/** 📊️ The counters of the walk farms of this process (one per workspace and module): children running, questions answered and failed, children recycled; empty while no farm was started. */
export const recipeFarmStatsV1=()=>[...recipeFarms].map(([key,held])=>({key:key.slice(0,64),calls:Number.isFinite(held.calls)?held.calls:0,farm:held.farm!==undefined,...held.farm?.stats?.()}));
const closeRecipeFarm=key=>{const held=recipeFarms.get(key);if(held===undefined)return;recipeFarms.delete(key);clearTimeout(held.idle);held.farm?.close();};
process.on("exit",()=>{for(const key of[...recipeFarms.keys()])closeRecipeFarm(key);});

/**
 * 🏭️ Walks one shared closure entry (a recipe: the command, target or provider closures that owners resolve) in a walk child instead of on the event loop of this process: `{value, rows, layers, sidecar}` (the in-process detached walk plus the walk facts and members of its layers), or `undefined` (farm off, fewer than two children granted, the first `after` recipes of a process, any child failure) so the caller walks it itself. The farm is shared by every caller of the process, lives while recipes keep coming and closes after `policy.idleMilliseconds` without one; the parent only parses the answer and stores it.
 * @param {string} workspaceRoot @param {unknown} recipe @param {{module?:string;size?:number;after?:number;spawn?:typeof nodeSpawn}} [options]
 */
export async function offloadClosureRecipeV1(workspaceRoot,recipe,options={}){
 if(walkFarmOffV1())return undefined;
 const key=workspaceRoot+"\0"+(options.module??"");
 let held=recipeFarms.get(key);
 if(held===undefined){held={calls:0,farm:undefined,idle:undefined};recipeFarms.set(key,held);}
 clearTimeout(held.idle);held.idle=setTimeout(()=>closeRecipeFarm(key),WALK_FARM_V1.idleMilliseconds);held.idle.unref?.();
 if(held.farm===undefined){
  if(++held.calls<=(options.after??WALK_FARM_V1.recipesAfter))return undefined;
  const lease=options.size===undefined?reserveWalkFarmV1(walkFarmSizeV1(capacitySource()),{yield:()=>closeRecipeFarm(key)}):undefined,size=options.size??lease.workers;
  if(size<2){lease?.release();held.calls=-Infinity;return undefined;}
  held.farm=createWalkFarmV1({root:workspaceRoot,size,...lease?{lease}:{},...options.module?{module:options.module}:{},...options.spawn?{spawn:options.spawn}:{}});held.farm.warm();
 }
 if(!await held.farm.whenReady(WALK_FARM_V1.recipeStartWaitMilliseconds))return undefined;
 try{
  const answer=await held.farm.derive({kind:"recipe",path:JSON.stringify(recipe),fact:null});
  const[rows,...layers]=answer.parts.map(part=>part.rows??[]);
  return{value:answer.fact.value===null?undefined:answer.fact.value,sidecar:answer.fact.sidecar??undefined,rows:rows??[],layers};
 }catch{return undefined;}
}
