/** 🔭 Watch supervisor: runs the native watcher (or the portable fallback), keeps batches ordered, provides fences, restarts with replay, reports trust. */
import {spawn} from "node:child_process";
import {mkdirSync,unlinkSync,watch as watchTree,writeFileSync} from "node:fs";
import {createFrameDecoderV1} from "../📨️protocol/🟨️.mjs";

const sleep=milliseconds=>new Promise(resolve=>setTimeout(resolve,milliseconds));

function readRows(raw){
 if(!Array.isArray(raw))return undefined;
 const rows=[];
 for(const row of raw){
  if(!Array.isArray(row)||row.length!==3||typeof row[0]!=="string"||typeof row[2]!=="string"||row[2].startsWith("/")||row[2].split("/").includes(".."))return undefined;
  rows.push([row[0],row[1]===1?1:0,row[2]]);
 }
 return rows;
}

/** 🔭 Creates the supervisor; nothing runs until `start()`. */
export function createWatchSupervisorV1(options){
 const{root,policy,filter,rooted=[],names=[],binary,backend:wanted="auto",latencyMs=policy.watcherLatencyMilliseconds,fenceDirectory,onBatch,onLost,onTrust=()=>{},log=()=>{},spawnProcess=spawn,restore}=options;
 const state={backend:"none",trust:"none",cursor:"",uuid:"",reason:""};
 let child,stopped=false,queue=Promise.resolve(),waiters=new Map(),fenceCounter=0,restarts=0,portable,current,next,readyResolve,readyReject,pollTimer;
 const stats={batches:0,rows:0,fences:0,fenceMisses:0,restarts:0,lost:0,lastBatchAt:0};
 const prefix=fenceDirectory+"/";
 let limited=false;
 const setTrust=(trust,reason="")=>{if(limited&&trust==="exact")return;if(state.trust!==trust||state.reason!==reason){state.trust=trust;state.reason=reason;onTrust({...state});}};

 const enqueue=task=>{queue=queue.then(task).catch(error=>log("watch-handler-failed",String(error?.stack??error)));return queue;};

 const deliver=(rows,at)=>{
  stats.batches++;stats.rows+=rows.length;stats.lastBatchAt=Date.now();
  const hits=[];
  for(const row of rows)if(row[2].startsWith(prefix)&&waiters.has(row[2]))hits.push(row[2]);
  enqueue(async()=>{
   if(rows.length)await onBatch(rows,at);
   for(const hit of hits){const waiter=waiters.get(hit);if(waiter){waiters.delete(hit);waiter.resolve();}}
  });
 };

 const handleFrame=frame=>{
  switch(frame?.t){
   case"ready":
    state.backend=frame.backend;state.cursor=frame.cursor??"";state.uuid=frame.uuid??"";state.replay=Boolean(frame.replay);readyResolve?.(frame);readyResolve=undefined;break;
   case"batch":{
    const rows=readRows(frame.c);
    if(!rows||typeof frame.at!=="number"){enqueue(()=>onLost("","read-error"));break;}
    if(typeof frame.cursor==="string"&&frame.cursor!=="")state.cursor=frame.cursor;
    deliver(rows,frame.at);break;
   }
   case"lost":stats.lost++;if(frame.reason==="watch-limit"){limited=true;setTrust("verify","watch-limit");}enqueue(()=>onLost(typeof frame.scope==="string"?frame.scope:"",String(frame.reason)));break;
   case"history-done":if(typeof frame.cursor==="string")state.cursor=frame.cursor;break;
   case"error":readyReject?.(new Error(String(frame.code)+": "+String(frame.message)));readyReject=undefined;break;
   default:break;
  }
 };

 const launchNative=()=>new Promise((resolve,reject)=>{
  const args=["watch","--root",root,"--latency-ms",String(latencyMs)];
  for(const item of rooted)args.push("--exclude",item);
  for(const item of names)args.push("--exclude-name",item);
  if(state.cursor!==""&&state.uuid!=="")args.push("--since",state.cursor,"--uuid",state.uuid);
  const process_=spawnProcess(binary,args,{stdio:["pipe","pipe","ignore"]});
  child=process_;
  let settled=false;
  const decoder=createFrameDecoderV1({maximumFrameBytes:64<<20,onFrame:handleFrame,onError:error=>{log("watch-protocol",error.message);}});
  readyResolve=frame=>{settled=true;resolve(frame);};
  readyReject=error=>{if(!settled){settled=true;reject(error);}};
  process_.stdout.on("data",chunk=>decoder.push(chunk));
  process_.on("error",error=>{readyReject?.(error);});
  process_.on("exit",code=>{
   readyReject?.(new Error("watcher exited before ready ("+code+")"));
   if(child===process_){child=undefined;if(!stopped)void recover(code);}
  });
  setTimeout(()=>readyReject?.(new Error("watcher start timed out")),policy.fenceTimeoutMilliseconds*4).unref?.();
 });

 const launchPortable=()=>{
  state.backend="portable";
  const pending=new Map();let timer;
  const flush=()=>{timer=undefined;const rows=[...pending.values()];pending.clear();if(rows.length)deliver(rows,Date.now());};
  portable=watchTree(root,{recursive:true,persistent:true},(_event,filename)=>{
   if(!filename)return;
   const path=String(filename).split("\\").join("/");
   if(filter.excluded(path))return;
   pending.set(path,["m",0,path]);
   timer??=setTimeout(flush,latencyMs);
  });
  portable.on("error",error=>{log("portable-watch-failed",String(error));setTrust("verify","portable-failed");});
 };

 async function recover(code){
  restarts++;stats.restarts++;
  setTrust("verify","watcher-exit:"+code);
  if(restarts>policy.watcherRestartsBeforePortable){
   log("watch-degrade","native watcher kept failing; using the portable backend");
   enqueue(()=>onLost("","watcher-exit"));
   try{launchPortable();await canary();}catch(error){setTrust("verify","portable:"+String(error?.message??error));}
   return;
  }
  await sleep(policy.watcherRestartBackoffMilliseconds[Math.min(restarts-1,policy.watcherRestartBackoffMilliseconds.length-1)]);
  if(stopped)return;
  try{
   const frame=await launchNative();
   if(!frame.replay)enqueue(()=>onLost("","watcher-exit"));
   await canary();
  }catch(error){log("watch-restart-failed",String(error?.message??error));void recover("restart");}
 }

 async function canary(){
  const result=await fence();
  if(result.ok){restarts=0;setTrust("exact","");}else setTrust("verify","canary");
  return result.ok;
 }

 function startFence(){
  const id=options.instance+"-"+(++fenceCounter),path=prefix+id;
  const entry={path,startedAt:performance.now(),resolve:undefined,promise:undefined};
  entry.promise=new Promise(resolve=>{entry.resolve=resolve;});
  waiters.set(path,entry);
  try{writeFileSync(root+"/"+path,"");}catch(error){waiters.delete(path);return Promise.resolve({ok:false,ms:0,reason:String(error?.code??error)});}
  stats.fences++;
  const timeout=new Promise(resolve=>setTimeout(()=>resolve("timeout"),policy.fenceTimeoutMilliseconds).unref?.());
  return Promise.race([entry.promise.then(()=>"event"),timeout]).then(outcome=>{
   waiters.delete(path);
   try{unlinkSync(root+"/"+path);}catch{}
   if(outcome==="timeout"){stats.fenceMisses++;return{ok:false,ms:performance.now()-entry.startedAt,reason:"timeout"};}
   return{ok:true,ms:performance.now()-entry.startedAt};
  });
 }

 /** 🚧 Resolves once every event that happened before the call has been delivered and applied; concurrent callers share the next fence. */
 function fence(){
  if(state.backend==="none"||state.backend==="poll")return Promise.resolve({ok:false,ms:0,reason:"no-watcher"});
  if(!current){
   current=startFence().finally(()=>{current=undefined;});
   return current;
  }
  next??=current.then(()=>{next=undefined;current=startFence().finally(()=>{current=undefined;});return current;});
  return next;
 }

 return{
  async start(){
   mkdirSync(root+"/"+fenceDirectory,{recursive:true});
   if(wanted==="none"){state.backend="none";setTrust("none","disabled");return{...state};}
   if(wanted==="poll"){state.backend="poll";setTrust("verify","poll");return{...state};}
   const native=wanted!=="portable"&&binary!==undefined;
   if(native){
    try{
     if(restore){state.cursor=restore.cursor??"";state.uuid=restore.uuid??"";}
     const frame=await launchNative();
     if(!frame.replay&&restore)enqueue(()=>onLost("","restart"));
    }
    catch(error){log("watch-native-failed",String(error?.message??error));if(wanted==="native"){setTrust("verify","native-failed");return{...state};}launchPortable();}
   }else launchPortable();
   await canary();
   return{...state};
  },
  fence,
  recheck:canary,
  state:()=>({...state}),
  stats,
  drained:()=>queue,
  async stop(){
   stopped=true;
   if(child){const dying=child;child=undefined;try{dying.stdin.end();}catch{}const killer=setTimeout(()=>dying.kill("SIGKILL"),1000);killer.unref?.();await new Promise(resolve=>{dying.once("exit",resolve);setTimeout(resolve,1100).unref?.();});}
   portable?.close();clearInterval(pollTimer);
   await queue;
  },
 };
}
