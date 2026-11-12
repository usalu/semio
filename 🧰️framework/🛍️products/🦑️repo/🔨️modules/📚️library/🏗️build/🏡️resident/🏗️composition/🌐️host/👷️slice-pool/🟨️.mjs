/** 👷 A bounded pool of slice workers for the host service: loading a graph slice (membership, facts, re-inference of the stale owners) is long synchronous work, so it runs in its own process (never a thread: a thread busy in synchronous work cannot be stopped) and the service's event loop keeps answering status and cached plans. At most `maximum` loads run at once, loads of one workspace root run one after the other (they share one fact store), a load is cancellable (its process group gets SIGINT, then SIGKILL after 2 s, from the service main thread) and reports a heartbeat so a client sees progress.
 * @see ../../../🏛️service/🌐️host/🟨️.mjs */
import {createProcessWorkerV1} from "./🧵️process/🟨️.mjs";
import {createBoundedCacheV1} from "../../../../🧊️bounded/🟨️.mjs";
import {HOST_POLICY_V1} from "../../../📨️protocol/🎚️policy/🟨️.mjs";
import {sliceLimitPolicyV1} from "./🧱️limit/🟨️.mjs";
import {createPressureWatchV1} from "./🌡️pressure/🟨️.mjs";
import {SLICE_JOB_VARIABLE_V1,createReaperV1,deadServiceRowV1,listProcessesV1,sliceJobTokenV1} from "./🧹️reap/🟨️.mjs";

export function createSlicePoolV1({workerUrl,maximum=2,heartbeatMilliseconds=2000,budgetMegabytes=Infinity,marginMegabytes=0,elasticity={},headroom=()=>({limitBytes:Infinity,currentBytes:process.memoryUsage().rss}),sampleMilliseconds=1000,note=()=>undefined,createWorker=(url,data,env)=>createProcessWorkerV1(url,data,env,graceMilliseconds===undefined?{}:{graceMilliseconds}),pressure=()=>createPressureWatchV1(),limiter,learned={read:()=>0,write:()=>undefined},reserve,graceMilliseconds,reaper=createReaperV1(graceMilliseconds===undefined?{}:{graceMilliseconds})}){
 if(!Number.isSafeInteger(maximum)||maximum<1)throw new Error("Positive worker maximum required");
 if(!Number.isFinite(marginMegabytes)||marginMegabytes<0||!(budgetMegabytes>=0))throw new Error("Finite memory margin and budget required");
 const tails=createBoundedCacheV1({name:"host:slice-roots",maxEntries:64,maxBytes:64}),waiting=[],counters={started:0,completed:0,failed:0,cancelled:0,overBudget:0,pressured:0};
 let active=0,sequence=0,recent=[];const running=new Map(),loads=new Map(),stale=deadServiceRowV1(),reaping=new Set();
 const scopeOf=load=>({pids:load.pids,match:row=>row.token===load.token||process.platform==="win32"&&running.size===0&&row.ppid===process.pid&&!load.baseline.has(row.pid)});
 const reaped=load=>{const task=reaper.reap(scopeOf(load)).catch(()=>0).finally(()=>reaping.delete(task));reaping.add(task);return task;};
 const sweepAtExit=()=>{for(const load of loads.values())reaper.killNow(scopeOf(load));reaper.killNow({match:stale});};
 process.once("exit",sweepAtExit);
 void reaper.reap({match:stale}).catch(()=>0);
 const slot=signal=>new Promise((resolve,reject)=>{
  signal?.throwIfAborted();
  const entry={resolve:()=>{signal?.removeEventListener("abort",entry.abort);resolve();},abort:()=>{const at=waiting.indexOf(entry);if(at>=0)waiting.splice(at,1);reject(signal.reason);}};
  signal?.addEventListener("abort",entry.abort,{once:true});
  if(active<maximum){active++;entry.resolve();}else waiting.push(entry);
 });
 const reserving=async(megabytes,signal,onProgress)=>{
  const started=performance.now(),beat=setInterval(()=>{note();onProgress?.(Math.round(performance.now()-started),{phase:"reservation",done:0,total:megabytes,running:[],decision:{key:"waiting",used:megabytes,budget:Math.round((headroom().availableBytes??0)/1048576)}});},heartbeatMilliseconds);beat.unref?.();
  try{return await reserve({megabytes,signal});}finally{clearInterval(beat);}
 };
 const release=()=>{const next=waiting.shift();if(next)next.resolve();else active--;};
 const runWorker=(request,signal,onProgress,limit,reservation)=>new Promise((resolve,reject)=>{
  counters.started++;const index=++sequence,token=sliceJobTokenV1(process.pid,index),load={token,pids:new Set(),baseline:new Set(process.platform==="win32"?listProcessesV1().filter(row=>row.ppid===process.pid).map(row=>row.pid):[])},worker=createWorker(workerUrl,{...request,heartbeatMilliseconds},{...process.env,[HOST_POLICY_V1.memoryPurge.environmentVariable]:String(HOST_POLICY_V1.memoryPurge.delayMilliseconds),[SLICE_JOB_VARIABLE_V1]:token}),started=performance.now(),entry={index,sample:undefined};running.set(index,entry);loads.set(token,load);
  let settled=false;
  const finish=(done,value)=>{if(settled)return;settled=true;if(entry.peakMegabytes)learned.write(Math.max(learned.read(),Math.round(entry.peakMegabytes*(value?.code==='memory-budget'?(elasticity.learnedGrowthBasisPoints??10000)/10000:1))));running.delete(index);if(entry.sample!==undefined)recent=[{index,sample:entry.sample},...recent].slice(0,maximum);clearInterval(beat);clearInterval(guard);signal?.removeEventListener("abort",abort);void Promise.resolve(worker.terminate?.()).catch(()=>undefined).then(()=>reaped(load)).then(()=>limit?.release()).catch(()=>undefined).finally(()=>{loads.delete(token);reservation?.release();});done(value);};
  const abort=()=>{counters.cancelled++;finish(reject,signal.reason??new Error("cancelled"));};
  let detail;const watch=pressure(),base=headroom().currentBytes,beat=setInterval(()=>{note();onProgress?.(Math.round(performance.now()-started),detail);},heartbeatMilliseconds),guard=setInterval(()=>{const verdict=watch.sample();if(verdict.tripped){counters.pressured++;finish(reject,Object.assign(new Error("Slice load stalled by memory pressure"),{code:"memory-pressure",kind:verdict.kind,value:verdict.value,limit:verdict.limit,seconds:verdict.seconds}));return;}const{limitBytes,currentBytes}=headroom(),grown=Math.round((currentBytes-base)/1048576),current=Math.round(currentBytes/1048576),peak=(entry.peakMegabytes=Math.max(entry.peakMegabytes??0,current)),limit=limitBytes===Infinity?Infinity:Math.round(limitBytes/1048576);if(grown>budgetMegabytes||current+marginMegabytes>limit){counters.overBudget++;finish(reject,Object.assign(new Error("Slice load would cross the memory limit"),{code:"memory-budget",currentMegabytes:current,limitMegabytes:limit===Infinity?budgetMegabytes:limit,marginMegabytes,grownMegabytes:grown}));}},sampleMilliseconds);beat.unref?.();guard.unref?.();
  signal?.addEventListener("abort",abort,{once:true});
  if(reservation&&worker.pid!==undefined)reservation.attach(worker.pid);
  if(limit){void limit.attach(worker.pid).catch(()=>undefined);limit.onBreach(breach=>{counters.overBudget++;entry.peakMegabytes=Math.max(entry.peakMegabytes??0,breach.limitMegabytes);finish(reject,Object.assign(new Error("Slice load overran its memory limit"),{code:"memory-budget",currentMegabytes:breach.peakMegabytes,limitMegabytes:breach.limitMegabytes,marginMegabytes,grownMegabytes:breach.peakMegabytes}));});}
  worker.on("message",message=>{
   note();if(message?.type==="child"&&Number.isSafeInteger(message.pid)){load.pids.add(message.pid);return;}
   if(message?.type==="progress"){detail={phase:message.phase,done:message.done,total:message.total,...message.running===undefined?{}:{running:message.running,...message.decision===undefined?{}:{decision:message.decision}}};return;}
   if(message?.type==="memory"){entry.sample={heapMb:message.heapMb,extraMb:message.extraMb,objectCount:message.objectCount,objectTypes:message.objectTypes,at:Date.now()};return;}
   if(message?.type==="done"){counters.completed++;finish(resolve,{graph:message.graph,...message.generationPin===undefined?{}:{generationPin:message.generationPin},...message.stats===undefined?{}:{stats:message.stats}});}
   else if(message?.type==="error"){counters.failed++;finish(reject,Object.assign(new Error(message.message),{stack:message.stack},message.refusal===undefined?{}:{name:"RepoError",...message.refusal}));}
  });
  worker.on("error",error=>{counters.failed++;finish(reject,error);});
  worker.on("exit",code=>{if(!settled){counters.failed++;finish(reject,new Error("Slice worker exited with code "+code));}});
 });
 return Object.freeze({
  async run(request,{signal,onProgress}={}){
   const key=String(request.root),previous=tails.get(key)??Promise.resolve(),turn=previous.catch(()=>undefined).then(async()=>{
    await slot(signal);
    let reservation;
    try{
     const policy=limiter===undefined?undefined:sliceLimitPolicyV1(headroom(),marginMegabytes,elasticity,learned.read());
     reservation=policy===undefined||reserve===undefined?undefined:await reserving(policy.memoryMegabytes,signal,onProgress);
     const limit=policy===undefined?undefined:await limiter.create(policy,"slice-"+(sequence+1)).catch(()=>undefined);
     const owned=reservation;reservation=undefined;
     return await runWorker(request,signal,onProgress,limit,owned);
    }catch(error){reservation?.release();throw error;}finally{release();}
   });
   tails.set(key,turn.catch(()=>undefined));
   return turn;
  },
  stats(){return{active,queued:waiting.length,...counters};},
  memory(now=Date.now()){const row=(index,sample,live)=>({index,heapMb:sample.heapMb,extraMb:sample.extraMb,objectCount:sample.objectCount,objectTypes:sample.objectTypes,ageMilliseconds:Math.max(0,now-sample.at),running:live}),live=[...running.values()].filter(entry=>entry.sample!==undefined).map(entry=>row(entry.index,entry.sample,true));return[...live,...recent.filter(entry=>!running.has(entry.index)).map(entry=>row(entry.index,entry.sample,false))].slice(0,maximum);},
  busy(){return active+waiting.length;},
  /** 🛑 Waits for the reaping of every finished load and kills what a load left behind. */
  async close(){await Promise.all([...reaping]);sweepAtExit();process.off("exit",sweepAtExit);},
 });
}
