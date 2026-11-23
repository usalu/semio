import {randomUUID} from "node:crypto";
import {decodeProtobufV1,REAPI_V1} from "../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {placementAnyCandidateV1,placementHasHeadroomV1,placementIdleCountV1,placementIdleElsewhereV1,placementKeyV1,planDispatchV1,RemoteExecutionLoadTableV1,type PlacementOperationV1,type PlacementWorkerV1} from "./⚖️placement/🟦️.ts";
import type {RemoteExecutionLoadReportV1,RemoteExecutionWorkerStateV1} from "../🌡️load/🟦️.ts";
import {remoteExecutionWorkerWhyV1} from "../🌡️load/🔎️why/🟦️.ts";
import {OFFLOAD_REFUSAL_PREFIX_V1} from "../🚀️offload/🩺️failure/🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../🚀️offload/📐️policy/🟦️.ts";

const withoutWhyV1=(state:RemoteExecutionWorkerStateV1):Omit<RemoteExecutionWorkerStateV1,"why">=>{const {why:_why,...rest}=state;return rest;};
import {readRemoteExecutionPolicyV1,remoteExecutionControlV1,remoteExecutionPeakMegabytesV1,remoteExecutionDigestV1,remoteExecutionMatchPropertiesV1,remoteExecutionPlatformKeyV1,remoteExecutionPlatformMatchesV1,type RemoteExecutionDigestV1,type RemoteExecutionHeartbeatAnswerV1,type RemoteExecutionPollAnswerV1,type RemoteExecutionPropertyV1,type RemoteExecutionTaskResultV1,type RemoteExecutionTaskStatusV1,type RemoteExecutionTaskV1,type RemoteExecutionWorkerRegisteredV1,type RemoteExecutionWorkerRegistrationV1} from "../🟦️.ts";

/** 🎞️ The stages an operation passes through, in the protocol's own words. */
export type RemoteExecutionStageV1="CACHE_CHECK"|"QUEUED"|"EXECUTING"|"COMPLETED";

/** 🧾️ What a finished operation carries: the status, the serialized ActionResult when there is one, and whether it came from the action cache. */
export type RemoteExecutionOutcomeV1=Readonly<{status:RemoteExecutionTaskStatusV1;actionResult:Uint8Array|null;cached:boolean;worker:string}>;

/** 📸️ An immutable view of one operation. */
export type RemoteExecutionSnapshotV1=Readonly<{name:string;instance:string;action:RemoteExecutionDigestV1;stage:RemoteExecutionStageV1;outcome?:RemoteExecutionOutcomeV1}>;

/** 📣️ Something worth logging. */
export type RemoteExecutionSchedulerEventV1=Readonly<{stage:"submitted"|"cached"|"assigned"|"completed"|"retried"|"worker-joined"|"worker-left"|"worker-lost"|"unmatched"|"refused"|"declined";detail:string}>;

/** ⚙️ Bounds of the scheduler; all durations are seconds. */
export type RemoteExecutionSchedulerOptionsV1=Readonly<{content:RemoteExecutionContentV1;unmatchedSeconds:number;lingerSeconds:number;leaseSeconds:number;maximumAttempts:number;defaultActionSeconds:number;maximumActionSeconds:number;maximumQueued:number;heartbeatSeconds:number;pollSeconds:number;loadExpireSeconds:number;refusalCooldownSeconds:number;keepAliveSeconds?:number;cancelGraceSeconds?:number;stuckSeconds?:number;cache:Readonly<{url:string;relative:boolean}>;language:"en"|"de"|"none";sweepMilliseconds?:number;now?:()=>number;onEvent?:(event:RemoteExecutionSchedulerEventV1)=>void}>;

type Operation={
 id:string;instance:string;action:RemoteExecutionDigestV1;key:string;platform:RemoteExecutionPropertyV1[];timeoutSeconds:number;stage:RemoteExecutionStageV1;version:number;createdAt:number;queuedAt:number;
 attempt:number;failures:number;tried:Set<string>;requester?:string;peakMegabytes?:number;submittedBy?:string;offload:boolean;worker?:string;startedAt?:number;outcome?:RemoteExecutionOutcomeV1;subscribers:Set<()=>void>;cancelRequested:boolean;adopted?:true;cancelRequestedAt?:number;lingerTimer?:ReturnType<typeof setTimeout>;
};

type Worker={
 id:string;registration:RemoteExecutionWorkerRegistrationV1;lastSeen:number;running:Set<string>;cancel:Set<string>;announced:Set<string>;waiter?:(answer:RemoteExecutionPollAnswerV1)=>void;draining:boolean;refusedUntil:number;refused:number;
};

/** ⏳️ How long the answer "another machine is idle" holds for the machine that was told: its next units are queued instead of declined while the headroom that was promised may have been taken by someone else in between (the queue limit of the requester still bounds the wait). */
const RESERVATION_SECONDS=20;
const OK:RemoteExecutionTaskStatusV1={code:0,message:""},DECLINED:RemoteExecutionTaskStatusV1={code:8,message:"No other machine has headroom: run the action locally"},FAILED_BACK=(why:string):RemoteExecutionTaskStatusV1=>({code:14,message:`${why}; run the action locally`}),MISSING=(digest:RemoteExecutionDigestV1):RemoteExecutionTaskStatusV1=>({code:9,message:`Missing blob ${digest.hash}/${digest.size}`,missing:[digest]});

/** 🗓️ The scheduler: deduplicates identical actions, answers from the action cache, queues actions, hands each to the least loaded compatible long-polling worker that has headroom (see `⚖️placement`), tracks leases and heartbeats, reschedules lost work once on another worker and then fails it back to the requester (a worker that no longer holds an action, left, was lost or misses its heartbeats), repeats every waiting operation to its clients as a keep-alive, cancels abandoned work (a cancellation the worker does not acknowledge in time frees the operation, a client that joins an operation under cancellation revokes it), ends operations on an operator's word, keeps the live load table of the fleet and counters. Pure logic over a content store; transports are separate. */
export class RemoteExecutionSchedulerV1{
 private readonly operations=new Map<string,Operation>();
 private readonly byKey=new Map<string,Operation>();
 private readonly queue:Operation[]=[];
 private readonly workers=new Map<string,Worker>();
 private readonly reserved=new Map<string,number>();
 private readonly counters={queued:0,executing:0,completed:0,cached:0,failed:0,cancelled:0,retried:0,deduplicated:0,refused:0,offloadRejected:0,cancelledQueued:0,cancelledExecuting:0,workersReused:0,sweepStalls:0};
 private readonly waits:number[]=[];
 private lastTick=0;
 readonly load:RemoteExecutionLoadTableV1;
 private readonly clock:()=>number;
 private timer?:ReturnType<typeof setInterval>;
 readonly startedAt:string;

 readonly options:RemoteExecutionSchedulerOptionsV1;private readonly keepAliveMilliseconds:number;private readonly cancelGraceMilliseconds:number;private readonly stuckMilliseconds:number; constructor(options:RemoteExecutionSchedulerOptionsV1){this.options=options;
  const server=readRemoteExecutionPolicyV1().server;this.keepAliveMilliseconds=(options.keepAliveSeconds??server.keepAliveSeconds)*1000;this.cancelGraceMilliseconds=(options.cancelGraceSeconds??server.cancelGraceSeconds)*1000;this.stuckMilliseconds=(options.stuckSeconds??server.stuckSeconds)*1000;
  this.clock=options.now??Date.now;this.startedAt=new Date(this.clock()).toISOString();this.load=new RemoteExecutionLoadTableV1(options.loadExpireSeconds*1000,this.clock);
  const every=options.sweepMilliseconds??1000;
  this.lastTick=this.clock();
  if(every>0){this.timer=setInterval(()=>{this.tick(every);},every);this.timer.unref();}
 }

 /** 🫀️ One beat of the sweep timer: when the timer came back much later than its interval, this process was stalled (the machine is starved), the heartbeats of the workers wait unread in the socket queue and their last-seen marks are stale through no fault of theirs, so that beat judges no worker and only counts the stall; the next beat judges on fresh marks. */
 tick(every:number):void{const now=this.clock(),stalled=now-this.lastTick>every*5;this.lastTick=now;if(stalled)this.counters.sweepStalls++;this.sweep(stalled);}

 private emit(stage:RemoteExecutionSchedulerEventV1["stage"],detail:string):void{this.options.onEvent?.({stage,detail});}
 private touch(operation:Operation):void{operation.version++;for(const wake of operation.subscribers)wake();}
 private snapshot(operation:Operation):RemoteExecutionSnapshotV1{return Object.freeze({name:`operations/${operation.id}`,instance:operation.instance,action:operation.action,stage:operation.stage,...(operation.outcome?{outcome:operation.outcome}:{})});}
 private label(operation:Operation):string{return `${operation.action.hash.slice(0,12)}/${operation.action.size}`;}

 private finish(operation:Operation,outcome:RemoteExecutionOutcomeV1):void{
  if(operation.stage==="COMPLETED")return;
  operation.stage="COMPLETED";operation.outcome=outcome;
  const index=this.queue.indexOf(operation);if(index>=0)this.queue.splice(index,1);
  if(operation.worker){this.workers.get(operation.worker)?.running.delete(`${operation.id}.${operation.attempt}`);}
  if(operation.lingerTimer)clearTimeout(operation.lingerTimer);
  this.byKey.delete(operation.key);
  if(outcome.cached)this.counters.cached++;
  else if(outcome.status.code===0)this.counters.completed++;
  else if(outcome.status.code===1)this.counters.cancelled++;
  else this.counters.failed++;
  this.emit("completed",`${this.label(operation)} code ${outcome.status.code}${outcome.cached?" cached":""}`);
  this.touch(operation);
  setTimeout(()=>{this.operations.delete(operation.id);},60000).unref();
  this.dispatch();
 }

 private async prepare(operation:Operation,skipCache:boolean):Promise<void>{
  try{
   const bytes=await this.options.content.read(operation.instance,operation.action);
   if(!bytes)return this.finish(operation,{status:MISSING(operation.action),actionResult:null,cached:false,worker:""});
   const action=decodeProtobufV1(REAPI_V1.action,bytes);
   let platform:{name:string;value:string}[]=action.platform?.properties??[];
   if(platform.length===0&&action.command_digest){
    const commandDigest=remoteExecutionDigestV1(action.command_digest.hash,action.command_digest.size_bytes),command=await this.options.content.read(operation.instance,commandDigest);
    if(!command)return this.finish(operation,{status:MISSING(commandDigest),actionResult:null,cached:false,worker:""});
    platform=decodeProtobufV1(REAPI_V1.command,command).platform?.properties??[];
   }
   operation.platform=platform.map(row=>({name:row.name,value:row.value}));operation.key=`${operation.instance}|${operation.action.hash}`;
   const requester=remoteExecutionControlV1(operation.platform,"semio-origin");
   if(operation.submittedBy!==undefined&&requester!==undefined&&requester!==operation.submittedBy)return this.finish(operation,{status:{code:7,message:"The origin property differs from the authenticated submitter"},actionResult:null,cached:false,worker:""});
   if(requester!==undefined)operation.requester=requester;
   else if(operation.submittedBy!==undefined&&remoteExecutionControlV1(operation.platform,"semio-offload")!==undefined)operation.requester=operation.submittedBy;
   operation.offload=remoteExecutionControlV1(operation.platform,"semio-offload")==="if-idle";operation.peakMegabytes=remoteExecutionPeakMegabytesV1(operation.platform);
   const seconds=action.timeout?Math.ceil(Number(action.timeout.seconds)+Number(action.timeout.nanos)/1e9):0;
   operation.timeoutSeconds=seconds>0?Math.min(seconds,this.options.maximumActionSeconds):Math.min(this.options.defaultActionSeconds,this.options.maximumActionSeconds);
   if(!skipCache){
    const cached=await this.options.content.readActionResult(operation.instance,operation.action.hash).catch(()=>undefined);
    if(cached){
     try{decodeProtobufV1(REAPI_V1.actionResult,cached);this.emit("cached",this.label(operation));return this.finish(operation,{status:OK,actionResult:cached,cached:true,worker:""});}catch{}
    }
   }
   if(operation.stage!=="CACHE_CHECK")return;
   if(operation.offload&&!placementIdleElsewhereV1(this.view(operation),this.views(),this.load,this.clock())&&!((this.reserved.get(operation.requester??"")??0)>this.clock())){this.counters.offloadRejected++;const standing=placementIdleCountV1(this.view(operation),this.views(),this.load,this.clock());this.emit("declined",`${this.label(operation)}: ${standing.workers} workers, ${standing.compatible} compatible, ${standing.headroom} with headroom, requester ${operation.requester??"unknown"}`);return this.finish(operation,{status:DECLINED,actionResult:null,cached:false,worker:""});}
   if(operation.requester!==undefined&&!placementAnyCandidateV1(this.view(operation),this.views(),this.load))return this.finish(operation,{status:FAILED_BACK("No other machine can take the action"),actionResult:null,cached:false,worker:""});
   if(operation.requester!==undefined)this.load.count(operation.requester,"offloadedOut");
   if(this.queue.length>=this.options.maximumQueued)return this.finish(operation,{status:{code:8,message:"Execution queue is full"},actionResult:null,cached:false,worker:""});
   operation.stage="QUEUED";operation.queuedAt=this.clock();this.queue.push(operation);this.counters.queued++;this.touch(operation);this.dispatch();
  }catch(error){
   this.finish(operation,{status:{code:14,message:`Preparing the action failed: ${error instanceof Error?error.message:String(error)}`},actionResult:null,cached:false,worker:""});
  }
 }

 private attach(operation:Operation,wake:()=>void):void{
  operation.subscribers.add(wake);
  if(operation.lingerTimer){clearTimeout(operation.lingerTimer);delete operation.lingerTimer;}
  if(operation.cancelRequested&&operation.stage==="EXECUTING"){
   operation.cancelRequested=false;delete operation.cancelRequestedAt;
   if(operation.worker)this.workers.get(operation.worker)?.cancel.delete(`${operation.id}.${operation.attempt}`);
  }
 }

 private detach(operation:Operation,wake:()=>void):void{
  operation.subscribers.delete(wake);
  if(operation.subscribers.size>0||operation.stage==="COMPLETED")return;
  operation.lingerTimer=setTimeout(()=>{this.cancel(operation);},this.options.lingerSeconds*1000);
  operation.lingerTimer.unref();
 }

 private cancel(operation:Operation):void{
  if(operation.stage==="COMPLETED"||operation.subscribers.size>0)return;
  if(operation.stage==="EXECUTING")this.counters.cancelledExecuting++;else this.counters.cancelledQueued++;
  if(operation.stage==="EXECUTING"&&operation.worker){
   const worker=this.workers.get(operation.worker);
   if(worker){operation.cancelRequested=true;worker.cancel.add(`${operation.id}.${operation.attempt}`);operation.cancelRequestedAt=this.clock();worker.waiter?.({version:1,task:null,cancel:[...worker.cancel],known:true});delete worker.waiter;return;}
  }
  this.finish(operation,{status:{code:1,message:"Cancelled: no client waits for the result"},actionResult:null,cached:false,worker:operation.worker??""});
 }

 private async *watch(operation:Operation,signal:AbortSignal):AsyncGenerator<RemoteExecutionSnapshotV1>{
  let wake:()=>void=()=>{},seen=-1,beat=false;
  const changed=():void=>{wake();},aborted=():void=>{wake();};
  this.attach(operation,changed);
  signal.addEventListener("abort",aborted,{once:true});
  try{
   for(;;){
    if(operation.version!==seen||beat){seen=operation.version;beat=false;const shown=this.snapshot(operation);yield shown;if(shown.stage==="COMPLETED")return;}
    if(signal.aborted)return;
    let timer:ReturnType<typeof setTimeout>|undefined;
    await new Promise<void>(resolve=>{wake=resolve;timer=setTimeout(()=>{beat=true;resolve();},this.keepAliveMilliseconds);if(operation.version!==seen||signal.aborted)resolve();});
    clearTimeout(timer);
   }
  }finally{signal.removeEventListener("abort",aborted);this.detach(operation,changed);}
 }

 /** 🚀️ Submits an action (or joins the identical one already in flight) and streams its operation snapshots until it completes or the caller goes away. */
 execute(request:Readonly<{instance:string;action:RemoteExecutionDigestV1;skipCacheLookup:boolean;submittedBy?:string}>,signal:AbortSignal):AsyncGenerator<RemoteExecutionSnapshotV1>{
  const key=`${request.instance}|${request.action.hash}`,existing=this.byKey.get(key);
  if(existing&&existing.stage!=="COMPLETED"){this.counters.deduplicated++;return this.watch(existing,signal);}
  const operation:Operation={id:randomUUID(),instance:request.instance,action:request.action,key,platform:[],timeoutSeconds:this.options.defaultActionSeconds,stage:"CACHE_CHECK",version:0,createdAt:this.clock(),queuedAt:this.clock(),attempt:1,failures:0,tried:new Set(),offload:false,subscribers:new Set(),cancelRequested:false,...(request.submittedBy===undefined?{}:{submittedBy:request.submittedBy})};
  this.operations.set(operation.id,operation);this.byKey.set(key,operation);this.emit("submitted",this.label(operation));
  const watching=this.watch(operation,signal);
  void this.prepare(operation,request.skipCacheLookup);
  return watching;
 }

 /** 🔁️ Re-attaches to a known operation (`WaitExecution`). */
 wait(name:string,signal:AbortSignal):AsyncGenerator<RemoteExecutionSnapshotV1>|undefined{
  const operation=this.operations.get(name.replace(/^operations\//u,""));
  return operation?this.watch(operation,signal):undefined;
 }

 /** 👷️ Registers a worker. */
 register(registration:RemoteExecutionWorkerRegistrationV1):RemoteExecutionWorkerRegisteredV1{
  const known=[...this.workers.values()].find(worker=>worker.registration.name===registration.name&&worker.registration.origin===registration.origin);
  if(known){known.registration=registration;known.lastSeen=this.clock();this.counters.workersReused++;this.dispatch();return this.registered(known.id);}
  const id=randomUUID();
  this.workers.set(id,{id,registration,lastSeen:this.clock(),running:new Set(),cancel:new Set(),announced:new Set(),draining:false,refusedUntil:0,refused:0});
  for(const row of registration.resumed??[]){
   const dot=row.task.lastIndexOf("."),oldId=row.task.slice(0,dot),attempt=Number(row.task.slice(dot+1)),key=`${row.instance}|${row.action.hash}`,existing=this.byKey.get(key);
   if(existing?.stage==="EXECUTING"||this.operations.has(oldId))continue;
   if(existing){
    const index=this.queue.indexOf(existing);if(index>=0)this.queue.splice(index,1);
    this.operations.delete(existing.id);existing.id=oldId;existing.attempt=attempt;existing.stage="EXECUTING";existing.worker=id;existing.startedAt=this.clock();if(existing.platform.length===0)existing.adopted=true;
    this.operations.set(oldId,existing);this.workers.get(id)!.running.add(row.task);this.touch(existing);
    this.emit("assigned",`${this.label(existing)} adopted from ${registration.name}`);continue;
   }
   const operation:Operation={id:oldId,instance:row.instance,action:row.action,key,platform:[],timeoutSeconds:this.options.defaultActionSeconds,stage:"EXECUTING",version:0,createdAt:this.clock(),queuedAt:this.clock(),attempt,failures:0,tried:new Set(),offload:false,subscribers:new Set(),cancelRequested:false,adopted:true,worker:id,startedAt:this.clock()};
   this.operations.set(oldId,operation);this.byKey.set(key,operation);this.workers.get(id)!.running.add(row.task);
   operation.lingerTimer=setTimeout(()=>{this.cancel(operation);},this.options.lingerSeconds*1000);operation.lingerTimer.unref();
   this.emit("submitted",`${this.label(operation)} adopted from ${registration.name}`);
  }
  this.emit("worker-joined",`${registration.name} ${registration.platform} x${registration.concurrency} ${registration.confinement==="full"?registration.sandbox:"unconfined"}`);
  this.dispatch();
  return this.registered(id);
 }

 /** 🪪️ The answer to a registration: a machine that registers again under the same name and origin (its heartbeat or its poll was told "unknown", or two of its loops raced) keeps its one entry and its id instead of leaving an orphan entry that nothing heartbeats and the lease then reports as lost. */
 private registered(id:string):RemoteExecutionWorkerRegisteredV1{
  return {version:1,id,heartbeatSeconds:this.options.heartbeatSeconds,pollSeconds:this.options.pollSeconds,leaseSeconds:this.options.leaseSeconds,cache:this.options.cache,language:this.options.language};
 }

 /** 👋️ Removes a worker; whatever it still runs is requeued. */
 deregister(workerId:string):void{
  const worker=this.workers.get(workerId);
  if(!worker)return;
  this.drop(worker,"worker-left");
 }

 private drop(worker:Worker,stage:"worker-left"|"worker-lost"):void{
  this.workers.delete(worker.id);
  worker.waiter?.({version:1,task:null,cancel:[],known:false});delete worker.waiter;
  this.emit(stage,worker.registration.name);
  for(const id of worker.running){
   const operation=this.operations.get(id.split(".")[0]!);
   if(operation&&operation.stage==="EXECUTING"&&`${operation.id}.${operation.attempt}`===id){operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,`Worker ${worker.registration.name} left`,true);}
  }
  this.dispatch();
 }

 private workerView(worker:Worker):PlacementWorkerV1{
  return {id:worker.id,name:worker.registration.name,origin:worker.registration.origin,properties:worker.registration.properties,concurrency:worker.registration.concurrency,running:worker.running.size,waiting:worker.waiter!==undefined,draining:worker.draining,refusedUntilMs:worker.refusedUntil,committedMegabytes:this.committedMegabytes(worker)};
 }
 private views():readonly PlacementWorkerV1[]{return [...this.workers.values()].map(worker=>this.workerView(worker));}
 private view(operation:Operation):PlacementOperationV1{
  return {id:operation.id,platform:operation.platform,tried:operation.requester===undefined?new Set():operation.tried,requester:operation.requester,peakMegabytes:operation.peakMegabytes??0};
 }
 /** 💾️ The peak megabytes of the actions this scheduler placed on a worker that are still running there (the machine's next load report may not show them yet). */
 private committedMegabytes(worker:Worker):number{
  let sum=0;
  for(const task of worker.running)sum+=this.operations.get(task.slice(0,task.lastIndexOf(".")))?.peakMegabytes??0;
  return sum;
 }

 private requeue(operation:Operation,reason:string,failure:boolean):void{
  const name=operation.worker??"";
  if(operation.worker)this.workers.get(operation.worker)?.running.delete(`${operation.id}.${operation.attempt}`);
  if(operation.adopted===true)return this.finish(operation,{status:FAILED_BACK(reason),actionResult:null,cached:false,worker:name});
  if(operation.cancelRequested)return this.finish(operation,{status:{code:1,message:"Cancelled"},actionResult:null,cached:false,worker:name});
  if(failure&&++operation.failures>=this.options.maximumAttempts)return this.finish(operation,{status:FAILED_BACK(`${reason}; gave up after ${operation.failures} attempt${operation.failures===1?"":"s"}`),actionResult:null,cached:false,worker:name});
  if(operation.requester!==undefined&&!placementAnyCandidateV1(this.view(operation),this.views(),this.load))return this.finish(operation,{status:FAILED_BACK(`${reason} and no other machine can take the action`),actionResult:null,cached:false,worker:name});
  operation.attempt++;operation.stage="QUEUED";delete operation.worker;delete operation.startedAt;operation.queuedAt=this.clock();this.queue.unshift(operation);this.counters.retried++;
  this.emit("retried",`${this.label(operation)}: ${reason}`);this.touch(operation);
 }

 private dispatch():void{
  const plan=planDispatchV1(this.queue.filter(operation=>operation.lingerTimer===undefined).map(operation=>this.view(operation)),this.views(),this.load,this.clock());
  for(const step of plan){
   const operation=this.operations.get(step.operation),worker=this.workers.get(step.worker);
   if(!operation||!worker||!worker.waiter)continue;
   this.queue.splice(this.queue.indexOf(operation),1);
   const task:RemoteExecutionTaskV1={id:`${operation.id}.${operation.attempt}`,instance:operation.instance,action:operation.action,timeoutSeconds:operation.timeoutSeconds,attempt:operation.attempt};
   const waited=this.clock()-operation.queuedAt;this.waits.push(waited);if(this.waits.length>512)this.waits.shift();
   operation.stage="EXECUTING";operation.worker=worker.id;operation.startedAt=this.clock();this.counters.executing++;worker.running.add(task.id);this.load.count(worker.registration.origin,"offloadedIn");
   this.emit("assigned",`${this.label(operation)} -> ${worker.registration.name}`);
   this.touch(operation);
   const answer=worker.waiter;delete worker.waiter;answer({version:1,task,cancel:[...worker.cancel],known:true});
  }
 }

 /** 📥️ A worker's long poll: answers with a task as soon as one matches, with the tasks to cancel, or with nothing after `waitMilliseconds`. */
 poll(workerId:string,waitMilliseconds:number,signal:AbortSignal):Promise<RemoteExecutionPollAnswerV1>{
  const worker=this.workers.get(workerId);
  if(!worker)return Promise.resolve({version:1,task:null,cancel:[],known:false});
  worker.lastSeen=this.clock();
  const fresh=[...worker.cancel].filter(id=>!worker.announced.has(id));
  if(fresh.length>0){for(const id of fresh)worker.announced.add(id);return Promise.resolve({version:1,task:null,cancel:[...worker.cancel],known:true});}
  worker.waiter?.({version:1,task:null,cancel:[],known:true});
  return new Promise(resolve=>{
   let timer:ReturnType<typeof setTimeout>|undefined;
   const settle=(answer:RemoteExecutionPollAnswerV1):void=>{if(timer)clearTimeout(timer);signal.removeEventListener("abort",onAbort);if(worker.waiter===settle)delete worker.waiter;resolve(answer);},
    onAbort=():void=>{settle({version:1,task:null,cancel:[],known:this.workers.has(workerId)});};
   worker.waiter=settle;timer=setTimeout(()=>{worker.lastSeen=this.clock();settle({version:1,task:null,cancel:[...worker.cancel],known:this.workers.has(workerId)});},waitMilliseconds);
   signal.addEventListener("abort",onAbort,{once:true});
   this.dispatch();
  });
 }

 /** 💓️ Marks a worker alive and tells it which of its tasks to abandon. */
 heartbeat(workerId:string,running:readonly string[]):RemoteExecutionHeartbeatAnswerV1{
  const worker=this.workers.get(workerId);
  if(!worker)return {version:1,known:false,cancel:[]};
  worker.lastSeen=this.clock();
  for(const id of [...worker.running])if(!running.includes(id)){
   const operation=this.operations.get(id.split(".")[0]!);
   if(!operation||operation.stage!=="EXECUTING"||`${operation.id}.${operation.attempt}`!==id){worker.running.delete(id);continue;}
   if(this.clock()-(operation.startedAt??0)>this.options.leaseSeconds*1000){operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,"Worker no longer runs the task",true);}
  }
  const answer={version:1 as const,known:true,cancel:[...worker.cancel].filter(id=>running.includes(id))};
  for(const id of [...worker.cancel])if(!running.includes(id)){worker.cancel.delete(id);worker.announced.delete(id);}
  return answer;
 }

 /** 🏁️ Takes a worker's report of a task. */
 report(workerId:string,taskId:string,result:RemoteExecutionTaskResultV1):boolean{
  const worker=this.workers.get(workerId),operation=this.operations.get(taskId.split(".")[0]!);
  if(worker){worker.lastSeen=this.clock();worker.running.delete(taskId);worker.cancel.delete(taskId);worker.announced.delete(taskId);}
  if(!operation||operation.stage!=="EXECUTING"||`${operation.id}.${operation.attempt}`!==taskId||operation.worker!==workerId)return false;
  const bytes=result.actionResult===null?null:new Uint8Array(Buffer.from(result.actionResult,"base64")),name=worker?.registration.name??"";
  if(result.outcome==="refused"){
   if(worker){worker.refusedUntil=this.clock()+this.options.refusalCooldownSeconds*1000;worker.refused++;this.load.count(worker.registration.origin,"refused");this.load.refuse(worker.registration.origin);}
   this.counters.refused++;this.emit("refused",`${this.label(operation)} by ${name}: ${result.refusal??"headroom"}`);
   if(operation.offload){
    if(worker)operation.tried.add(placementKeyV1(this.workerView(worker)));
    if(placementIdleElsewhereV1(this.view(operation),this.views(),this.load,this.clock()))this.requeue(operation,`Worker ${name} refused the action, another machine has headroom`,false);
    else{this.counters.offloadRejected++;this.finish(operation,{status:DECLINED,actionResult:null,cached:false,worker:name});}
   }
   else this.requeue(operation,`Worker ${name} refused the action`,false);
   return true;
  }
  if(result.outcome==="cancelled"){if(operation.cancelRequested)this.finish(operation,{status:{code:1,message:"Cancelled"},actionResult:null,cached:false,worker:name});else{if(worker)operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,"Worker abandoned the task",true);}return true;}
  if(result.outcome==="failed"){
   const policy=result.status.code===9&&operation.requester!==undefined&&result.status.message.startsWith(OFFLOAD_REFUSAL_PREFIX_V1);
   if(([10,14].includes(result.status.code)||policy)&&operation.failures+1<this.options.maximumAttempts){if(worker)operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,result.status.message,true);}
   else this.finish(operation,{status:result.status,actionResult:null,cached:false,worker:name});
   return true;
  }
  if(bytes===null){this.finish(operation,{status:{code:13,message:"Worker reported success without a result"},actionResult:null,cached:false,worker:name});return true;}
  let cacheable=false;
  try{cacheable=decodeProtobufV1(REAPI_V1.actionResult,bytes).exit_code===0;}catch{this.finish(operation,{status:{code:13,message:"Worker returned a malformed result"},actionResult:null,cached:false,worker:name});return true;}
  const finish=():void=>{this.finish(operation,{status:OK,actionResult:bytes,cached:false,worker:name});};
  if(cacheable)void this.options.content.writeActionResult(operation.instance,operation.action.hash,bytes).catch(()=>{}).finally(finish);else finish();
  return true;
 }

 /** 🧹️ Applies timeouts: dead workers, tasks beyond their deadline, actions no worker can ever run. */
 sweep(grace=false):void{
  const now=this.clock(),lease=this.options.leaseSeconds*1000;
  for(const worker of grace?[]:[...this.workers.values()])if(!worker.waiter&&now-worker.lastSeen>lease||worker.waiter&&now-worker.lastSeen>lease*3)this.drop(worker,"worker-lost");
  for(const operation of this.operations.values()){
   if(operation.stage==="EXECUTING"&&operation.startedAt!==undefined&&now-operation.startedAt>(operation.timeoutSeconds*1000+lease)){
    const worker=operation.worker?this.workers.get(operation.worker):undefined;worker?.cancel.add(`${operation.id}.${operation.attempt}`);
    if(operation.failures+1>=this.options.maximumAttempts)this.finish(operation,{status:{code:4,message:`Action exceeded its timeout of ${operation.timeoutSeconds}s`},actionResult:null,cached:false,worker:worker?.registration.name??""});
    else{if(worker)operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,`Action exceeded its deadline on ${worker?.registration.name??"its worker"}`,true);}
   }
   if(operation.stage==="EXECUTING"&&operation.startedAt!==undefined){
    const worker=operation.worker?this.workers.get(operation.worker):undefined,task=`${operation.id}.${operation.attempt}`;
    if(operation.cancelRequested&&operation.cancelRequestedAt!==undefined&&now-operation.cancelRequestedAt>this.cancelGraceMilliseconds){worker?.running.delete(task);this.finish(operation,{status:{code:1,message:"Cancelled: the worker did not acknowledge in time"},actionResult:null,cached:false,worker:worker?.registration.name??""});continue;}
    if((worker===undefined||!worker.running.has(task))&&now-operation.startedAt>lease&&!grace){if(worker)operation.tried.add(placementKeyV1(this.workerView(worker)));this.requeue(operation,"The worker of the action no longer holds it",true);continue;}
   }
   if(operation.stage==="QUEUED"&&now-operation.queuedAt>this.options.unmatchedSeconds*1000&&!placementAnyCandidateV1(this.view(operation),this.views(),this.load)){
    this.emit("unmatched",`${this.label(operation)}: ${remoteExecutionPlatformKeyV1(remoteExecutionMatchPropertiesV1(operation.platform))}`);
    this.finish(operation,{status:{code:9,message:`No worker offers the platform ${remoteExecutionPlatformKeyV1(remoteExecutionMatchPropertiesV1(operation.platform))||"(any)"}; ${this.workers.size} worker(s) are registered`},actionResult:null,cached:false,worker:""});
   }
  }
  this.dispatch();
 }

 /** 🛑️ Ends operations on an operator's word and tells their waiting clients to run the action locally (status `CANCELLED`, so no retry): one operation by its id or by at least 8 characters of its id or action digest, or `all-stuck`, every unfinished operation older than `stuckSeconds` (counted from its start, or from its queueing). A worker that still runs one is told to stop it. Answers what was ended. */
 abort(target:string):readonly Readonly<{id:string;action:string;stage:"QUEUED"|"EXECUTING";worker:string|null;elapsedSeconds:number}>[]{
  const now=this.clock(),named=target.length>=8?target:"",ended:{id:string;action:string;stage:"QUEUED"|"EXECUTING";worker:string|null;elapsedSeconds:number}[]=[];
  for(const operation of [...this.operations.values()]){
   if(operation.stage==="COMPLETED")continue;
   const age=now-(operation.startedAt??operation.queuedAt);
   if(target==="all-stuck"?age<=this.stuckMilliseconds:!(operation.id===target||(named!==""&&(operation.id.startsWith(named)||operation.action.hash.startsWith(named)))))continue;
   const worker=operation.worker?this.workers.get(operation.worker):undefined,task=`${operation.id}.${operation.attempt}`;
   ended.push({id:operation.id,action:operation.action.hash.slice(0,12),stage:operation.stage as "QUEUED"|"EXECUTING",worker:worker?.registration.origin??null,elapsedSeconds:Math.max(0,Math.round(age/1000))});
   if(worker&&operation.stage==="EXECUTING")worker.cancel.add(task);
   this.finish(operation,{status:{code:1,message:"Cancelled by an operator; run the action locally"},actionResult:null,cached:false,worker:worker?.registration.name??""});
   worker?.waiter?.({version:1,task:null,cancel:[...worker.cancel],known:true});if(worker)delete worker.waiter;
  }
  return ended;
 }

 /** 📊️ A secret-free view of the scheduler for `GET /status`. */
 status():Readonly<{counters:Record<string,number>;workers:readonly Record<string,unknown>[];platforms:readonly {key:string;queued:number;workers:number}[];queued:number;running:number}>{
  const now=this.clock(),platforms=new Map<string,{key:string;queued:number;workers:number}>();
  const row=(key:string)=>{let found=platforms.get(key);if(!found){found={key,queued:0,workers:0};platforms.set(key,found);}return found;};
  for(const operation of this.queue)row(remoteExecutionPlatformKeyV1(remoteExecutionMatchPropertiesV1(operation.platform))).queued++;
  const workers=[...this.workers.values()].map(worker=>{
   for(const key of new Set([...platforms.keys()].filter(key=>remoteExecutionPlatformMatchesV1(key===""?[]:key.split(";").map(part=>({name:part.slice(0,part.indexOf("=")),value:part.slice(part.indexOf("=")+1)})),worker.registration.properties))))row(key).workers++;
   const entry=this.load.get(worker.registration.origin);
   return {id:worker.id,name:worker.registration.name,host:worker.registration.host,platform:worker.registration.platform,properties:worker.registration.properties,concurrency:worker.registration.concurrency,running:worker.running.size,sandbox:worker.registration.sandbox,lastSeenSeconds:Math.max(0,Math.round((now-worker.lastSeen)/1000)),draining:worker.draining,origin:worker.registration.origin,confinement:worker.registration.confinement,network:worker.registration.network,headroom:placementHasHeadroomV1(this.workerView(worker),entry,now),offloadedIn:entry?.offloadedIn??0,refused:worker.refused};
  });
  let running=0;for(const worker of this.workers.values())running+=worker.running.size;
  return {counters:{...this.counters,...this.waitStats(),queued:this.queue.length,executing:running},workers,platforms:[...platforms.values()],queued:this.queue.length,running};
 }

 /** 📏️ How long the actions that were placed waited in the queue: the last 512 assignments, as the number of samples and the median, the 90th percentile and the maximum in milliseconds (the measure the queue limit of the requesters is sized from). */
 private waitStats():Readonly<{queueWaitSamples:number;queueWaitP50Milliseconds:number;queueWaitP90Milliseconds:number;queueWaitMaxMilliseconds:number}>{
  const sorted=[...this.waits].sort((left,right)=>left-right),at=(share:number):number=>sorted.length===0?0:sorted[Math.min(sorted.length-1,Math.floor(sorted.length*share))]!;
  return {queueWaitSamples:sorted.length,queueWaitP50Milliseconds:at(0.5),queueWaitP90Milliseconds:at(0.9),queueWaitMaxMilliseconds:sorted.at(-1)??0};
 }

 /** 🔢️ How the workers stand for a unit of `requester` right now, by the very counting the submission check uses, for the action format of this version: what the build door asks before it admits cargo without a grant. */
 idleFor(requester:string):ReturnType<typeof placementIdleCountV1>{
  const property=OFFLOAD_POLICY_V1.property;
  const standing=placementIdleCountV1({id:"idle-probe",platform:[{name:property.format,value:property.formatValue}],tried:new Set<string>(),requester},this.views(),this.load,this.clock());
  if(standing.candidates>0)this.reserved.set(requester,this.clock()+RESERVATION_SECONDS*1000);
  return standing;
 }

 /** 📩️ Takes the load report of a machine into the live table and gives the queue a chance to use the new headroom. */
 acceptLoad(report:RemoteExecutionLoadReportV1):void{this.load.accept(report);this.dispatch();}

 /** 🌍️ The live table of the fleet (`FleetLoad` of the schema): one row per machine that reported within the expiry, and the actions the scheduler holds. */
 fleetLoad():Readonly<Record<string,unknown>>{
  const now=this.clock(),workers=new Map([...this.workers.values()].map(worker=>[worker.registration.origin,worker]));
  const machines=this.load.all().map(entry=>{
   const report=entry.report,worker=workers.get(report.origin),state=worker===undefined?{...report.worker,state:"absent" as const,running:0,why:report.worker.state==="absent"&&report.worker.why!==undefined?report.worker.why:remoteExecutionWorkerWhyV1("not-registered")}:{...(report.worker.state==="refusing"?report.worker:withoutWhyV1(report.worker)),running:worker.running.size,concurrency:worker.registration.concurrency,sandbox:worker.registration.sandbox,confinement:worker.registration.confinement,network:worker.registration.network,state:report.worker.state==="refusing"||now<worker.refusedUntil?"refusing" as const:worker.running.size>0?"busy" as const:"idle" as const};
   return {origin:report.origin,platform:report.platform,capacity:report.capacity,running:report.running,queued:report.queued,pressure:report.pressure,freeMegabytes:report.freeMegabytes,load:report.load,headroom:report.headroom,ageSeconds:Math.max(0,Math.round((now-entry.receivedAtMs)/1000)),worker:state,offloadedIn:entry.offloadedIn,offloadedOut:entry.offloadedOut,refused:entry.refused,counters:report.counters};
  });
  const owner=(id:string|undefined)=>id===undefined?null:this.workers.get(id)?.registration.origin??null;
  const tasks=[...this.operations.values()].filter(operation=>operation.stage==="QUEUED"||operation.stage==="EXECUTING").map(operation=>({id:operation.id,action:operation.action.hash.slice(0,12),stage:operation.stage as "QUEUED"|"EXECUTING",worker:owner(operation.worker),requester:operation.requester??null,elapsedSeconds:Math.max(0,Math.round((now-(operation.startedAt??operation.queuedAt))/1000))}));
  const view=this.status();
  return {version:1,service:"semio-remote-execution",wallMs:now,expireSeconds:this.options.loadExpireSeconds,machines,tasks,queued:view.queued,executing:view.running,counters:view.counters};
 }

 /** 🛑️ Stops timers and releases every waiting worker. */
 close():void{
  if(this.timer)clearInterval(this.timer);
  for(const worker of this.workers.values()){worker.waiter?.({version:1,task:null,cancel:[],known:false});delete worker.waiter;}
 }
}
