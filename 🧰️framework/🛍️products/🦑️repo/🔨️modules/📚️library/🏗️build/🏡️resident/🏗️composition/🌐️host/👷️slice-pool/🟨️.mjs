/** 👷 A bounded pool of slice workers for the host service: loading a graph slice (membership, facts, re-inference of the stale owners) is long synchronous work, so it runs in its own process (never a thread: a thread busy in synchronous work cannot be stopped) and the service's event loop keeps answering status and cached plans. At most `maximum` loads run at once, loads of one workspace root run one after the other (they share one fact store), a load runs under the collector target of a graph child (`graphChildHeapBytesV1`), not the larger one of the resident service it inherits: its garbage, not its data, was most of its resident size (7.7 GB resident for a live set of 0.5 GB), and a load that large leaves the capacity gate no memory for the farm workers that would shorten it; a load is cancellable (its process group gets SIGINT, then SIGKILL after 2 s, from the service main thread) and reports a heartbeat so a client sees progress.
 * @see ../../../🏛️service/🌐️host/🟨️.mjs */
import {createProcessWorkerV1,errorOfMessageV1,workerCauseV1} from "./🧵️process/🟨️.mjs";
import {GRAPH_MEMORY_POLICY_V1} from "../../../../🧠️memory/🟨️.mjs";
import {GRAPH_CUT_POLICY_V1,graphCutAfterV1,graphCutEnvironmentV1,graphCutLearnedV1,graphCutStartLevelV1,graphCutV1} from "../../../../🧠️memory/✂️cut/🟨️.mjs";
import {SLICE_RECYCLE_POLICY_V1,sliceRecycleEnvironmentV1} from "../../../../🧠️memory/♻️recycle/🟨️.mjs";
import {graphChildEnvironmentV1} from "../../../../🧠️memory/🐏️child-environment/🟨️.mjs";
import {READY_SLICE_REPORT_POLICY_V1,SLICE_REPORT_SAMPLE_MILLISECONDS_V1} from "../../../../🧠️memory/📋️slice-report/🏷️owner/🟦️.ts";
import {hostProfileV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import {HOST_POLICY_V1,renderBothV1} from "../../../📨️protocol/🎚️policy/🟨️.mjs";
import formatStampSchema from "../../../../🧷️format-stamps/🧬️schema/🔣️.json" with {type:"json"};

/** ⏳ What a slice worker says first when it ends because a source a peer just edited imports a module that is not there yet: the replica has not settled, so the restart is waited for (bounded by the settle budget) and never counted as the same deterministic failure. */
const UNSETTLED_PREFIX_V1=formatStampSchema.$defs.Protocol.const.loadFailure.unsettledPrefix;
/** 🧊️ The start of the cause of a worker whose source facts a repeated inference could not refresh; a load that ends this way the declared number of times in a row is started once more (the worker refreshes those facts in a targeted way) before it is refused.
 * @see ../../../../👁️observation/🟨️.mjs `staleSourceFactsErrorV1` */
const STALE_FACTS_PREFIX_V1="Stale source facts that a repeated inference cannot refresh";
import {planGuardVerdictV1,sliceAdmissionV1,sliceBookingV1,sliceCeilingMegabytesV1,sliceLimitPolicyV1,slicePlanCeilingMegabytesV1} from "./🧱️limit/🟨️.mjs";
import {createPressureWatchV1,hostPressureLevelV1} from "./🌡️pressure/🟨️.mjs";
import {awaitImplementationQuietV1,quietBudgetV1,implementationDriftPathsV1,implementationPathShownV1,implementationStampsV1} from "./🔕️quiet/🟨️.mjs";
import {SLICE_JOB_VARIABLE_V1,createReaperV1,deadServiceRowV1,listProcessesV1,sliceJobTokenV1} from "./🧹️reap/🟨️.mjs";

/** 🔑 The identity of a slice load: loads with the same root, arguments, tool and graph policy produce the same slice, so they are one load. */
/** 🚦 The urgency of a slice load (0 is most urgent, 5 when a client names none): the pool serves the most urgent waiter first, equal urgency in arrival order. */
export const slicePriorityV1=priority=>Number.isInteger(priority)&&priority>=0&&priority<=9?priority:5;

export const shareKeyV1=request=>JSON.stringify([String(request.root),request.arguments??null,request.tool??null,request.graphPolicy??null]);

export function createSlicePoolV1({workerUrl,maximum=2,heartbeatMilliseconds=2000,budgetMegabytes=Infinity,marginMegabytes=0,elasticity={},headroom=()=>({limitBytes:Infinity,currentBytes:process.memoryUsage().rss}),sampleMilliseconds=1000,note=()=>undefined,createWorker=(url,data,env)=>createProcessWorkerV1(url,data,env,graceMilliseconds===undefined?{}:{graceMilliseconds}),pressure=()=>createPressureWatchV1(),limiter,learned={read:()=>0,write:()=>undefined},reserve,observe,unlearnedMegabytes=0,hostMegabytes=hostProfileV1().totalMegabytes,graceMilliseconds,reaper=createReaperV1(graceMilliseconds===undefined?{}:{graceMilliseconds}),planGuard=HOST_POLICY_V1.slicePlanGuard,restartPolicy={backoffMilliseconds:HOST_POLICY_V1.sliceRestartBackoffMilliseconds,settleMilliseconds:HOST_POLICY_V1.sliceRestartSettleMilliseconds,progressMilliseconds:HOST_POLICY_V1.sliceRestartProgressMilliseconds,totalMaximum:HOST_POLICY_V1.sliceRestartTotalMaximum,sameCauseMaximum:HOST_POLICY_V1.sliceRestartSameCauseMaximum},quietPolicy={milliseconds:HOST_POLICY_V1.sliceRestartQuietMilliseconds,maximumMilliseconds:HOST_POLICY_V1.sliceRestartQuietMaximumMilliseconds,totalMaximumMilliseconds:HOST_POLICY_V1.sliceRestartQuietTotalMaximumMilliseconds,floorMilliseconds:HOST_POLICY_V1.sliceRestartQuietFloorMilliseconds},implementationStamps=implementationStampsV1,wallNow=()=>Date.now(),cutPolicy=GRAPH_CUT_POLICY_V1,recycle=SLICE_RECYCLE_POLICY_V1,hostPressure=()=>hostPressureLevelV1(),report=line=>{try{process.stderr.write(line+"\n");}catch{}},now=()=>performance.now(),planLimitMegabytes=hostMegabytes>0&&hostMegabytes<GRAPH_MEMORY_POLICY_V1.serveHostMegabytes&&process.env.SEMIO_GRAPH_PLAN_LIMIT_MB!=="off"?sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,maximumMegabytes:elasticity.maximumMegabytes}):Infinity}){
 if(!Number.isSafeInteger(maximum)||maximum<1)throw new Error("Positive worker maximum required");
 if(!Number.isFinite(marginMegabytes)||marginMegabytes<0||!(budgetMegabytes>=0))throw new Error("Finite memory margin and budget required");
 const waiting=[],counters={started:0,completed:0,failed:0,cancelled:0,overBudget:0,pressured:0};
 const learnedPeak=()=>Math.min(learned.read(),sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,maximumMegabytes:elasticity.maximumMegabytes}));
 let active=0,sequence=0,recent=[],learnedLevel;const sharing=new Map();const running=new Map(),loads=new Map(),stale=deadServiceRowV1(),reaping=new Set();
 const scopeOf=load=>({pids:load.pids,match:row=>row.token===load.token||process.platform==="win32"&&running.size===0&&row.ppid===process.pid&&!load.baseline.has(row.pid)});
 const reaped=load=>{const task=reaper.reap(scopeOf(load)).catch(()=>0).finally(()=>reaping.delete(task));reaping.add(task);return task;};
 const sweepAtExit=()=>{for(const load of loads.values())reaper.killNow(scopeOf(load));reaper.killNow({match:stale});};
 process.once("exit",sweepAtExit);
 void reaper.reap({match:stale}).catch(()=>0);
 const slot=(signal,rank=()=>5)=>new Promise((resolve,reject)=>{
  signal?.throwIfAborted();
  const entry={rank,resolve:()=>{signal?.removeEventListener("abort",entry.abort);resolve();},abort:()=>{const at=waiting.indexOf(entry);if(at>=0)waiting.splice(at,1);reject(signal.reason);}};
  signal?.addEventListener("abort",entry.abort,{once:true});
  if(active<maximum){active++;entry.resolve();}else waiting.push(entry);
 });
 const reserving=async(megabytes,minimumMegabytes,signal,onProgress,reduced=false)=>{
  const started=performance.now();
  let wait,announced=false;
  const say=granted=>{
   if(signal?.aborted)return;
   announced=true;note();
   try{onProgress?.(Math.round(performance.now()-started),{phase:"reservation",reservation:{requested:megabytes,...granted!==undefined?{granted}:wait!==undefined?{wait}:{}}},granted!==undefined);}catch{}
  };
  const beat=setInterval(say,heartbeatMilliseconds);beat.unref?.();
  try{
   const result=await reserve({megabytes,minimumMegabytes,signal,onWait:facts=>{wait=facts;}});
   if((announced||wait!==undefined||reduced||result?.megabytes<megabytes)&&Number.isSafeInteger(result?.megabytes))say(result.megabytes);
   return result;
  }finally{clearInterval(beat);}
 };
 const release=()=>{let at=0;for(let index=1;index<waiting.length;index++)if(waiting[index].rank()<waiting[at].rank())at=index;const next=waiting.splice(at,1)[0];if(next)next.resolve();else active--;};
 /** ⏳ Keeps a queued load audible: while it waits for the load ahead or for a free worker it reports a heartbeat decision to its client, and leaves the wait when the client goes away. */
 const holding=(wait,signal,onProgress,decision)=>{
  const started=performance.now(),beat=onProgress===undefined?undefined:setInterval(()=>{note();const row=decision();onProgress(Math.round(performance.now()-started),{phase:"queue",done:row.done??0,total:row.total??0,running:[],decision:row},false);},heartbeatMilliseconds);beat?.unref?.();
  const gate=signal===undefined?wait:new Promise((resolve,reject)=>{if(signal.aborted){reject(signal.reason);return;}const abort=()=>reject(signal.reason);signal.addEventListener("abort",abort,{once:true});wait.then(resolve,reject).finally(()=>signal.removeEventListener("abort",abort));});
  return gate.finally(()=>clearInterval(beat));
 };
 const runWorker=(request,signal,onProgress,limit,reservation,waveBasisPoints=10000,cut=graphCutV1(0,cutPolicy),outcome={},wantsReport=()=>false,carried=undefined)=>new Promise((resolve,reject)=>{
  const environment={...process.env};delete environment[READY_SLICE_REPORT_POLICY_V1.grantEnvironment];
  if(reservation?.megabytes>0&&environment.SEMIO_HOST_CAPACITY!=="off"){
   const{budgetEnvironment,minimumBudgetMegabytes,grantShare}=GRAPH_MEMORY_POLICY_V1;
   const managed=Math.max(minimumBudgetMegabytes,Math.floor(reservation.megabytes*grantShare)),configured=Number(environment[budgetEnvironment]);
   if(environment[budgetEnvironment]===undefined||Number.isFinite(configured)&&configured>managed)environment[budgetEnvironment]=String(managed);
  }
  counters.started++;const index=++sequence,token=sliceJobTokenV1(process.pid,index),load={token,pids:new Set(),baseline:new Set(process.platform==="win32"?listProcessesV1().filter(row=>row.ppid===process.pid).map(row=>row.pid):[])},worker=createWorker(workerUrl,{...request,heartbeatMilliseconds},{...environment,SEMIO_SLICE_WAVE_BASIS_POINTS:String(waveBasisPoints),...graphChildEnvironmentV1(),...graphCutEnvironmentV1(cut,cutPolicy),...recycle===null?{}:sliceRecycleEnvironmentV1(carried,recycle),...Number.isFinite(planLimitMegabytes)?{[GRAPH_MEMORY_POLICY_V1.planCeilingEnvironment]:String(Math.floor(planLimitMegabytes))}:{},[HOST_POLICY_V1.memoryPurge.environmentVariable]:String(HOST_POLICY_V1.memoryPurge.delayMilliseconds),[SLICE_JOB_VARIABLE_V1]:token}),started=now(),entry={index,sample:undefined};running.set(index,entry);loads.set(token,load);outcome.started=true;
  let settled=false;
  const finish=(done,value)=>{if(settled)return;settled=true;outcome.peakMegabytes=entry.peakMegabytes??0;const learnedMegabytes=done===resolve||value?.code==='memory-budget'?entry.peakMegabytes:entry.treePeakMegabytes;if(learnedMegabytes)learned.write(Math.round(learnedMegabytes*(value?.code==='memory-budget'?(elasticity.learnedGrowthBasisPoints??10000)/10000:1)));running.delete(index);if(entry.sample!==undefined)recent=[{index,sample:entry.sample},...recent].slice(0,maximum);clearInterval(beat);clearInterval(guard);signal?.removeEventListener("abort",abort);const gone=Promise.resolve(worker.terminate?.()).catch(()=>undefined).then(()=>reaped(load)).then(()=>limit?.release()).catch(()=>undefined).finally(()=>{loads.delete(token);reservation?.release();});if(done===reject&&value!==null&&typeof value==="object"){try{Object.defineProperty(value,"gone",{value:gone,enumerable:false,configurable:true});}catch{}}done(value);};
  const abort=()=>{counters.cancelled++;finish(reject,signal.reason??new Error("cancelled"));};
  let settling;
  const ceilingMegabytes=hostMegabytes>0?Math.max(Math.floor(hostMegabytes*planGuard.ceilingBasisPoints/10000),planLimitMegabytes):Infinity;
  const judge=megabytes=>{
   if(settled)return;
   const answered=settling!==undefined&&(settling.answered||performance.now()-settling.at>=planGuard.settleWaitMilliseconds),stage=entry.stage??detail?.phase??"?";
   if(settling!==undefined&&!answered&&megabytes<=ceilingMegabytes)return;
   if(answered&&!settling.said){settling.said=true;const lines=renderBothV1("host-graph-plan-settled",{stage,from:settling.from,to:Math.round(megabytes)});for(const language of ["en","de"])report(lines[language]);}
   const available=headroom().availableBytes,availableMegabytes=available===undefined?undefined:Math.round(available/1048576),done=worker.send===undefined||answered,over=megabytes>planLimitMegabytes&&megabytes<=ceilingMegabytes&&done;
   const verdict=planGuardVerdictV1({treeMegabytes:megabytes,limitMegabytes:planLimitMegabytes,ceilingMegabytes,availableMegabytes,reserveMegabytes:planGuard.reserveMegabytes,pressure:over?hostPressure():undefined,settled:done});
   if(verdict.action==="settle"){settling={at:performance.now(),from:Math.round(megabytes),answered:false,said:false};if(!worker.send({type:"settle"}))settling.answered=true;return;}
   if(verdict.action==="refuse"){counters.overBudget++;const cutTo=graphCutAfterV1({level:cut.level,treeMegabytes:megabytes,limitMegabytes:planLimitMegabytes},cutPolicy);if(cutTo!==undefined){finish(reject,Object.assign(new Error("Slice load is cut smaller"),{code:"plan-cut",cutTo,reason:verdict.reason,currentMegabytes:Math.round(megabytes),limitMegabytes:Math.round(planLimitMegabytes)}));return;}finish(reject,Object.assign(new Error("Slice load grew past the memory a plan may take"),{code:"plan-memory",cutLevel:cut.level,reason:verdict.reason,stage,operation:entry.operation??detail?.phase??"?",currentMegabytes:Math.round(megabytes),limitMegabytes:Math.round(planLimitMegabytes),availableMegabytes,reserveMegabytes:planGuard.reserveMegabytes,sample:entry.sample}));}
  };
  let treeSamplePending=false;
  const sampleTree=pid=>{
   treeSamplePending=true;
   void Promise.resolve().then(()=>settled?null:observe(pid)).then(megabytes=>{
    treeSamplePending=false;
    if(settled||!Number.isFinite(megabytes))return;
    if(megabytes>(entry.treePeakMegabytes??0))entry.treePeakMegabytes=megabytes;
    if(megabytes>(entry.peakMegabytes??0))entry.peakMegabytes=megabytes;
    judge(megabytes);
   },()=>{treeSamplePending=false;});
  };
  let detail,workerObserved=false,lastReportAt=-Infinity;const watch=pressure(),base=headroom().currentBytes,beat=setInterval(()=>{
   note();const observed=workerObserved;workerObserved=false;onProgress?.(Math.round(performance.now()-started),detail,observed);
   if(wantsReport()&&now()-lastReportAt>=SLICE_REPORT_SAMPLE_MILLISECONDS_V1)worker.send?.({type:"slice-report"});
  },heartbeatMilliseconds),guard=setInterval(()=>{if(worker.pid!==undefined&&observe!==undefined&&!treeSamplePending)sampleTree(worker.pid);const verdict=watch.sample();if(verdict.tripped){counters.pressured++;finish(reject,Object.assign(new Error("Slice load stalled by memory pressure"),{code:"memory-pressure",kind:verdict.kind,value:verdict.value,limit:verdict.limit,seconds:verdict.seconds}));return;}const{limitBytes,currentBytes}=headroom(),grown=Math.round((currentBytes-base)/1048576),current=Math.round(currentBytes/1048576),peak=(entry.peakMegabytes=Math.max(entry.peakMegabytes??0,grown)),limit=limitBytes===Infinity?Infinity:Math.round(limitBytes/1048576);if(grown>budgetMegabytes||current+marginMegabytes>limit){counters.overBudget++;finish(reject,Object.assign(new Error("Slice load would cross the memory limit"),{code:"memory-budget",currentMegabytes:current,limitMegabytes:limit===Infinity?budgetMegabytes:limit,marginMegabytes,grownMegabytes:grown}));}},sampleMilliseconds);beat.unref?.();guard.unref?.();
  signal?.addEventListener("abort",abort,{once:true});
  if(reservation&&worker.pid!==undefined)reservation.attach(worker.pid);
  if(limit){void limit.attach(worker.pid).catch(()=>undefined);limit.onBreach(breach=>{counters.overBudget++;entry.peakMegabytes=Math.max(entry.peakMegabytes??0,breach.limitMegabytes);finish(reject,Object.assign(new Error("Slice load overran its memory limit"),{code:"memory-budget",currentMegabytes:breach.peakMegabytes,limitMegabytes:breach.limitMegabytes,marginMegabytes,grownMegabytes:breach.peakMegabytes}));});}
  worker.on("message",message=>{
   if(message?.type==="slice-report"){
    if(!settled&&wantsReport()&&typeof message.report==="string"&&Buffer.byteLength(message.report)<=READY_SLICE_REPORT_POLICY_V1.fileBytes){lastReportAt=now();onProgress?.(0,{sliceReport:message.report});}
    return;
   }
   if(message?.type==="recycle"){note();finish(reject,Object.assign(new Error("Slice worker recycled at a pass boundary"),{code:"slice-recycle",recycle:message.state}));return;}
   note();if(message?.type==="child"&&Number.isSafeInteger(message.pid)){load.pids.add(message.pid);return;}
   if(message?.type==="progress"&&message.decision?.key==="round"){workerObserved=true;onProgress?.(0,{phase:message.phase,done:message.done,total:message.total,running:[],decision:message.decision},true);return;}
   if(message?.type==="progress"){workerObserved=true;detail={phase:message.phase,done:message.done,total:message.total,...message.leaseWait===undefined?{}:{leaseWait:message.leaseWait},...message.running===undefined?{}:{running:message.running,...message.decision===undefined?{}:{decision:message.decision}}};return;}
   if(message?.type==="settled"){if(settling!==undefined)settling.answered=true;return;}
   if(message?.type==="stage"){entry.stage=String(message.stage);entry.operation=String(message.operation??message.stage);return;}
   if(message?.type==="memory"){entry.sample={heapMb:message.heapMb,extraMb:message.extraMb,objectCount:message.objectCount,objectTypes:message.objectTypes,at:Date.now()};return;}
   if(message?.type==="done"){counters.completed++;finish(resolve,{graph:message.graph,...message.generationPin===undefined?{}:{generationPin:message.generationPin},...message.stats===undefined?{}:{stats:message.stats}});}
   else if(message?.type==="error"){const error=errorOfMessageV1(message),cutTo=error.key==="build.graph.plan-too-heavy"?graphCutAfterV1({level:cut.level,treeMegabytes:Number(error.values?.megabytes),limitMegabytes:Number(error.values?.limit)},cutPolicy):undefined;if(cutTo!==undefined){counters.overBudget++;finish(reject,Object.assign(new Error("Slice plan is cut smaller"),{code:"plan-cut",cutTo,reason:"stage",currentMegabytes:Number(error.values?.megabytes)||0,limitMegabytes:Number(error.values?.limit)||0}));}else{counters.failed++;finish(reject,error);}}
  });
  worker.on("error",error=>{counters.failed++;finish(reject,error);});
  worker.on("exit",(code,signal)=>{if(!settled){counters.failed++;const killed=code===null&&typeof signal==="string"&&signal!=="",tail=typeof worker.stderrTail==="function"?worker.stderrTail():"";finish(reject,Object.assign(new Error("Slice worker exited with code "+code+(killed?" (killed by "+signal+")":"")),{exit:code,...killed?{code:"slice-killed",signal,currentMegabytes:entry.treePeakMegabytes}:{},restart:killed||code===HOST_POLICY_V1.sliceRestartExitCode,ranMilliseconds:Math.round(now()-started),workerCause:killed?"killed by "+signal+" from outside (resource-transient)":tail===""?"":workerCauseV1(tail),drift:implementationDriftPathsV1(tail)}));}});
 });
 const execute=async(request,{signal,onProgress,wantsReport,rank}={})=>{
  const turn=(async()=>{
   let ownsSlot=false,heavyHandoffs=0,recycles=0,carried,totalRestarts=0,quietSpent=0,churnSince,streak=0,lastCause="",sameCause=0;const drifted=new Set(),refreshed=new Set();
   let level=learnedLevel??Math.min(cutPolicy.levels,graphCutStartLevelV1({limitMegabytes:planLimitMegabytes},cutPolicy));
   try{for(let restarts=0;;restarts++){
   if(!ownsSlot){
    await holding(slot(signal,rank),undefined,onProgress,()=>({key:"queued-pool",used:active,budget:maximum,limit:waiting.length}));
    ownsSlot=true;
   }
   signal?.throwIfAborted();
   let reservation,limit;
   try{
    const room=headroom(),policy=limiter===undefined?undefined:sliceLimitPolicyV1(room,marginMegabytes,{...elasticity,hostMegabytes,planCeilingMegabytes:slicePlanCeilingMegabytesV1({hostMegabytes,ceilingBasisPoints:planGuard.ceilingBasisPoints,planLimitMegabytes})},learnedPeak());
    const{wantedMegabytes:booked,minimumMegabytes}=sliceBookingV1({declaredMegabytes:policy?.memoryMegabytes,learnedMegabytes:learnedPeak(),unlearnedMegabytes,maximumMegabytes:elasticity.maximumMegabytes,hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,minimumMegabytes:elasticity.minimumMegabytes,serviceLeftMegabytes:policy===undefined&&Number.isFinite(room.limitBytes)?Math.floor((room.limitBytes-room.currentBytes)/1048576)-marginMegabytes:Infinity});
    const available=headroom().availableBytes,admission=minimumMegabytes>0||elasticity.hostShareBasisPoints===undefined?{megabytes:booked,reduced:false,waveBasisPoints:10000}:sliceAdmissionV1({requestedMegabytes:booked,availableMegabytes:available===undefined?undefined:available/1048576,floorMegabytes:elasticity.hostFloorMegabytes,shareBasisPoints:elasticity.hostShareBasisPoints});
    reservation=reserve===undefined||!(booked>0)?undefined:await reserving(admission.megabytes,Math.min(minimumMegabytes,admission.megabytes),signal,onProgress,admission.reduced);
    signal?.throwIfAborted();
    limit=policy===undefined?undefined:await limiter.create(policy,"slice-"+(sequence+1)).catch(()=>undefined);
    signal?.throwIfAborted();
    const granted=Number(reservation?.megabytes),waveBasisPoints=Number.isFinite(granted)&&granted>0?Math.min(admission.waveBasisPoints,Math.max(500,Math.floor(granted*10000/booked))):admission.waveBasisPoints;
    const cut=graphCutV1(level,cutPolicy),outcome={},work=runWorker(request,signal,onProgress,limit,reservation,Math.min(waveBasisPoints,cut.waveBasisPoints),cut,outcome,wantsReport,carried);
    if(outcome.started){reservation=undefined;limit=undefined;}
    const answer=await work;
    learnedLevel=graphCutLearnedV1({level,peakMegabytes:outcome.peakMegabytes??0,limitMegabytes:planLimitMegabytes},cutPolicy);
    return answer;
   }catch(error){
    if(limit!==undefined)await Promise.resolve().then(()=>limit.release()).catch(()=>undefined);
    reservation?.release();
    await error?.gone;
    if(error?.code==="plan-cut"&&!signal?.aborted){level=error.cutTo;restarts--;note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision:{key:"cut",used:level,budget:cutPolicy.levels,kind:String(error.currentMegabytes),limit:error.limitMegabytes}});continue;}
    if(error?.code==="slice-recycle"&&recycle!==null&&!signal?.aborted&&recycles<recycle.maximumRecycles){recycles++;carried=error.recycle;restarts--;note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision:{key:"recycled",used:recycles,budget:recycle.maximumRecycles,kind:String(carried?.heapMegabytes??""),limit:recycle.heapMegabytes}});continue;}
    if(error?.key==="build.graph.owner-too-heavy"&&!signal?.aborted&&Number(error.values?.producers)!==0&&heavyHandoffs<cutPolicy.heavyHandoffs){heavyHandoffs++;restarts--;note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision:{key:"handoff",used:heavyHandoffs,budget:cutPolicy.heavyHandoffs,kind:String(error.values?.named??"")}});continue;}
    const squeezed=(error?.code==="memory-budget"||error?.code==="memory-pressure"||error?.code==="slice-killed")&&!signal?.aborted?graphCutAfterV1({level,treeMegabytes:Number(error.currentMegabytes),limitMegabytes:Number(error.limitMegabytes)},cutPolicy):undefined;
    if(squeezed!==undefined){level=squeezed;restarts--;note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision:{key:"cut",used:level,budget:cutPolicy.levels,kind:String(error.currentMegabytes??""),limit:error.limitMegabytes??0}});continue;}
    if(error?.restart!==true||signal?.aborted)throw error;
    totalRestarts++;
    const lasted=error.ranMilliseconds>=restartPolicy.progressMilliseconds;
    const cause=String(error.workerCause??""),unsettled=cause.includes(UNSETTLED_PREFIX_V1);sameCause=unsettled||error.code==="slice-killed"?0:cause!==""&&cause===lastCause?sameCause+1:cause!==""?1:0;lastCause=cause;
    if(sameCause>=(restartPolicy.sameCauseMaximum??Infinity)&&cause.includes(STALE_FACTS_PREFIX_V1)&&!refreshed.has(cause)){refreshed.add(cause);sameCause=0;}
    if(cause!==""&&sameCause>=(restartPolicy.sameCauseMaximum??Infinity))throw Object.assign(new Error(Object.values(renderBothV1("host-slice-worker-failing",{count:sameCause,exit:error.exit,cause})).join("\n")),{code:"slice-worker-failing",cause,count:sameCause,exit:error.exit});
    if(lasted){churnSince=now();streak=0;}
    churnSince??=now()-(error.ranMilliseconds??0);
    const settleWaited=now()-churnSince;
    if(!lasted&&settleWaited>=restartPolicy.settleMilliseconds||totalRestarts>restartPolicy.totalMaximum)throw Object.assign(new Error(Object.values(renderBothV1("host-slice-restarts",{restarts:totalRestarts,exit:error.exit})).join("\n")),{code:"slice-restarts",restarts:totalRestarts,exit:error.exit});
    const pause=restartPolicy.backoffMilliseconds[Math.min(streak++,restartPolicy.backoffMilliseconds.length-1)],decision={key:cause===""?"restarted":"restarted-cause",used:Math.round(settleWaited/1000),budget:Math.round(restartPolicy.settleMilliseconds/1000),kind:String(error.exit),limit:Math.round(pause/1000),...cause===""?{}:{cause}};
    ownsSlot=false;release();
    for(const path of error.drift??[])drifted.add(path);
    if(drifted.size>0&&!signal?.aborted){
     const settlePolicy=quietBudgetV1(quietPolicy,quietSpent);
     const waited=await awaitImplementationQuietV1({paths:drifted,policy:settlePolicy,heartbeatMilliseconds,signal,stamps:implementationStamps,now:wallNow,announce:(verdict,elapsed)=>{note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision:{key:"settling",used:Math.floor(verdict.quietForMilliseconds/1000),budget:Math.round(quietPolicy.milliseconds/1000),kind:String(error.exit),limit:Math.round(quietPolicy.maximumMilliseconds/1000),cause:implementationPathShownV1(request.root,verdict.newestPath)}},elapsed===0);}});
     signal?.throwIfAborted();
     quietSpent+=waited;
     if(waited>0)churnSince=now();
    }
    for(let waited=0;waited<pause&&!signal?.aborted;waited+=heartbeatMilliseconds){
     note();onProgress?.(0,{phase:"reservation",done:0,total:0,running:[],decision},waited===0);
     await new Promise(done=>{
      if(signal?.aborted){done();return;}
      const finish=()=>{clearTimeout(timer);signal?.removeEventListener("abort",finish);done();};
      const timer=setTimeout(finish,Math.min(heartbeatMilliseconds,pause-waited));
      signal?.addEventListener("abort",finish,{once:true});
     });
    }
   }
   }}finally{if(ownsSlot)release();}
  })();
  return turn;
 };

 /** 🤝 Runs a slice load once for every client that asks for the same slice (root, arguments, tool, graph policy): a client that comes while that load is queued or running joins it, sees its progress (a late one gets the last row at once) and gets its answer, so nothing is computed twice. A client that leaves only leaves; the load is stopped, its processes reaped and its slot released (the leaving call settles after that) when the last client has left, so a load whose first asker was cut still finishes for the others and a restarted try never starts at zero. */
 const share=async(request,{signal,onProgress,wantsReport}={})=>{
  signal?.throwIfAborted();
  const key=shareKeyV1(request),root=String(request.root);
  let load=sharing.get(key);
  if(load===undefined){
   const controller=new AbortController(),clients=new Set(),mine={clients,controller,last:undefined,priority:slicePriorityV1(request.priority)};
   const lead=(milliseconds,detail,observed)=>{
    if(detail?.sliceReport!==undefined){for(const client of clients)if(client.wantsReport?.())client.onProgress?.(milliseconds,detail);return;}
    mine.last=[milliseconds,detail];for(const client of[...clients])client.onProgress?.(milliseconds,detail,observed);
   };
   mine.promise=execute(request,{rank:()=>mine.priority,signal:controller.signal,onProgress:lead,wantsReport:()=>[...clients].some(client=>client.wantsReport?.())});
   const clear=()=>{if(sharing.get(key)===mine)sharing.delete(key);};
   mine.promise.then(clear,clear);sharing.set(key,mine);load=mine;
  }
  return new Promise((resolve,reject)=>{
   const client={onProgress,wantsReport};load.priority=Math.min(load.priority,slicePriorityV1(request.priority));
   const leave=()=>{
    load.clients.delete(client);
    const reason=signal.reason??new Error("cancelled");
    if(load.clients.size>0){reject(reason);return;}
    if(sharing.get(key)===load)sharing.delete(key);
    load.controller.abort(reason);
    load.promise.then(()=>reject(reason),()=>reject(reason));
   };
   load.clients.add(client);
   signal?.addEventListener("abort",leave,{once:true});
   if(load.last!==undefined)onProgress?.(...load.last,false);
   load.promise.then(resolve,reject).finally(()=>signal?.removeEventListener("abort",leave));
  });
 };
 return Object.freeze({
  run:share,
  stats(){return{active,queued:waiting.length,...counters};},
  memory(now=Date.now()){const row=(index,sample,live)=>({index,heapMb:sample.heapMb,extraMb:sample.extraMb,objectCount:sample.objectCount,objectTypes:sample.objectTypes,ageMilliseconds:Math.max(0,now-sample.at),running:live}),live=[...running.values()].filter(entry=>entry.sample!==undefined).map(entry=>row(entry.index,entry.sample,true));return[...live,...recent.filter(entry=>!running.has(entry.index)).map(entry=>row(entry.index,entry.sample,false))].slice(0,maximum);},
  busy(){return active+waiting.length;},
  /** 🛑 Waits for the reaping of every finished load and kills what a load left behind. */
  async close(){await Promise.all([...reaping]);sweepAtExit();process.off("exit",sweepAtExit);},
 });
}
