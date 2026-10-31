import{isAbsolute,resolve,join}from"node:path";
/** 🔭 Watch supervisor: runs the native watcher (or the portable fallback), keeps batches ordered, provides fences, restarts with replay, reports trust. */
import {spawn} from "node:child_process";
import {lstatSync,mkdirSync,unlinkSync,watch as watchTree,writeFileSync} from "node:fs";
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
 const{root,policy,filter,rooted=[],names=[],opaqueBasenames,binary,backend:wanted="auto",latencyMs=policy.watcherLatencyMilliseconds,fenceDirectory,onBatch,onLost,onTrust=()=>{},log=()=>{},spawnProcess=spawn,restore,privateFence=false,failureMode="recover"}=options;
 const state={backend:"none",trust:"none",cursor:"",uuid:"",reason:""};
 let child,stopped=false,queue=Promise.resolve(),waiters=new Map(),fenceCounter=0,restarts=0,portable,current,next,readyResolve,readyReject,pollTimer;
 const stats={batches:0,rows:0,fences:0,fenceMisses:0,restarts:0,lost:0,lastBatchAt:0};
 const prefix=fenceDirectory+"/";
 let terminalError;
 const refuse=(error)=>{terminalError??=error instanceof Error?error:new Error(String(error));setTrust("verify",terminalError.message);return terminalError;};
 let limited=false;
 const setTrust=(trust,reason="")=>{if((limited||terminalError)&&trust==="exact")return;if(state.trust!==trust||state.reason!==reason){state.trust=trust;state.reason=reason;onTrust({...state});}};

 const enqueue=task=>{queue=queue.then(task).catch(async error=>{log("watch-handler-failed",String(error?.stack??error));if(failureMode==="terminal"){refuse(error);try{await onLost("","handler-failed");}catch{}}});return queue;};

 const deliver=(rows,at)=>{
  stats.batches++;stats.rows+=rows.length;stats.lastBatchAt=Date.now();
  const hits=[];
  for(const row of rows)if(row[2].startsWith(prefix)&&waiters.has(row[2]))hits.push(row[2]);
  enqueue(async()=>{
   const visible=privateFence?rows.filter(row=>!row[2].startsWith(prefix)):rows;
   if(visible.length)await onBatch(visible,at);
   for(const hit of hits){const waiter=waiters.get(hit);if(waiter){waiters.delete(hit);waiter.resolve("event");}}
  });
 };

 const handleFrame=frame=>{
  switch(frame?.t){
   case"ready":
    state.backend=frame.backend;state.cursor=frame.cursor??"";state.uuid=frame.uuid??"";state.replay=Boolean(frame.replay);readyResolve?.(frame);readyResolve=undefined;break;
   case"batch":{
    const rows=readRows(frame.c);
    if(!rows||typeof frame.at!=="number"){if(failureMode==="terminal")refuse(new Error("watcher batch refused"));enqueue(()=>onLost("","read-error"));break;}
    if(typeof frame.cursor==="string"&&frame.cursor!=="")state.cursor=frame.cursor;
    deliver(rows,frame.at);break;
   }
   case"lost":stats.lost++;if(failureMode==="terminal")refuse(new Error("watcher lost coverage: "+frame.reason));if(frame.reason==="watch-limit"){limited=true;setTrust("verify","watch-limit");}enqueue(()=>onLost(typeof frame.scope==="string"?frame.scope:"",String(frame.reason)));break;
   case"history-done":if(typeof frame.cursor==="string")state.cursor=frame.cursor;break;
   case"error":readyReject?.(new Error(String(frame.code)+": "+String(frame.message)));readyReject=undefined;break;
   default:break;
  }
 };

 const launchNative=startup=>new Promise((resolve,reject)=>{
  const remaining=startup?startup.remainingMilliseconds():policy.fenceTimeoutMilliseconds*4;
  if(startup?.signal.aborted){reject(startup.signal.reason);return;}
  if(remaining===null||!Number.isFinite(remaining)||remaining<1||remaining>2147483647){reject(new Error("Finite funded watcher startup required"));return;}
  const args=["watch","--root",root,"--latency-ms",String(latencyMs)];
  if(privateFence)args.push("--private-fence-directory",fenceDirectory);
  args.push(...nativeExclusionArgumentsV1({rooted,names,opaqueBasenames}));
  if(state.cursor!==""&&state.uuid!=="")args.push("--since",state.cursor,"--uuid",state.uuid);
  const process_=spawnProcess(binary,args,{stdio:["pipe","pipe","ignore"]});
  child=process_;
  let settled=false,timer;
  const cleanup=()=>{clearTimeout(timer);startup?.signal.removeEventListener("abort",interrupt);if(readyResolve===accept)readyResolve=undefined;if(readyReject===decline)readyReject=undefined;};
  const accept=frame=>{if(settled)return;settled=true;cleanup();resolve(frame);},decline=error=>{if(settled)return;settled=true;cleanup();reject(error);},interrupt=()=>decline(startup.signal.reason);
  const decoder=createFrameDecoderV1({maximumFrameBytes:64<<20,onFrame:frame=>{if(child===process_)handleFrame(frame);},onError:error=>{if(child!==process_)return;log("watch-protocol",error.message);if(failureMode==="terminal"){refuse(error);enqueue(()=>onLost("","read-error"));}}});
  readyResolve=accept;readyReject=decline;
  process_.stdout.on("data",chunk=>decoder.push(chunk));
  process_.on("error",decline);
  process_.on("exit",code=>{
   decline(new Error("watcher exited before ready ("+code+")"));
   if(child===process_){child=undefined;if(!stopped){if(failureMode==="terminal"){refuse(new Error("watcher exited ("+code+")"));enqueue(()=>onLost("","watcher-exit"));}else void recover(code);}}
  });
  timer=setTimeout(()=>decline(new Error("watcher start timed out")),Math.floor(remaining));timer.unref?.();
  startup?.signal.addEventListener("abort",interrupt,{once:true});
  if(startup?.signal.aborted)interrupt();
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
  if(stopped)return Promise.resolve({ok:false,ms:0,reason:"stopped"});
  const id=options.instance+"-"+(++fenceCounter),path=prefix+id;
  const entry={path,startedAt:performance.now(),resolve:undefined,promise:undefined};
  entry.promise=new Promise(resolve=>{entry.resolve=resolve;});
  waiters.set(path,entry);
  try{writeFileSync(root+"/"+path,"");}catch(error){waiters.delete(path);return Promise.resolve({ok:false,ms:0,reason:String(error?.code??error)});}
  stats.fences++;
  let timer;const timeout=new Promise(resolve=>{timer=setTimeout(()=>resolve("timeout"),policy.fenceTimeoutMilliseconds);timer.unref?.();});
  return Promise.race([entry.promise,timeout]).then(outcome=>{
   clearTimeout(timer);waiters.delete(path);
   try{unlinkSync(root+"/"+path);}catch{}
   if(outcome!=="event"){stats.fenceMisses++;return{ok:false,ms:performance.now()-entry.startedAt,reason:outcome};}
   return{ok:true,ms:performance.now()-entry.startedAt};
  });
 }

 /** 🚧 Resolves once every event that happened before the call has been delivered and applied; concurrent callers share the next fence. */
 function fence(){
  if(terminalError&&failureMode==="terminal")return Promise.reject(terminalError);
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
   if(failureMode==="terminal"&&(wanted!=="native"||!binary))throw new Error("Exact native watcher required");
   if(privateFence)preparePrivateFenceV1(root,fenceDirectory);else mkdirSync(root+"/"+fenceDirectory,{recursive:true});
   if(wanted==="none"){state.backend="none";setTrust("none","disabled");return{...state};}
   if(wanted==="poll"){state.backend="poll";setTrust("verify","poll");return{...state};}
   const native=wanted!=="portable"&&binary!==undefined;
   if(native){
    try{
     if(restore){state.cursor=restore.cursor??"";state.uuid=restore.uuid??"";}
     const frame=await launchNative(options.startupControl);
     if(!frame.replay&&restore)enqueue(()=>onLost("","restart"));
    }
    catch(error){log("watch-native-failed",String(error?.message??error));if(failureMode==="terminal")throw refuse(error);if(wanted==="native"){setTrust("verify","native-failed");return{...state};}launchPortable();}
   }else launchPortable();
   const exact=await canary();
   if(failureMode==="terminal"&&(!exact||terminalError))throw refuse(terminalError??new Error("Native watcher physical canary failed"));
   return{...state};
  },
  fence,
  recheck:canary,
  state:()=>({...state}),
  stats,
  drained:()=>queue,
  async stop(){
   stopped=true;for(const waiter of waiters.values())waiter.resolve("stopped");waiters.clear();
   if(child){const dying=child;child=undefined;const exited=new Promise(resolve=>dying.once("exit",resolve));try{dying.stdin.end();}catch{}const killer=setTimeout(()=>dying.kill("SIGKILL"),1000);killer.unref?.();if(failureMode==="terminal"){await exited;clearTimeout(killer);}else await new Promise(resolve=>{dying.once("exit",resolve);setTimeout(resolve,1100).unref?.();});}
   portable?.close();clearInterval(pollTimer);
   await queue;
  },
 };
}

/** 🚧️ Projects the supplied exclusion protocol to both native watch and membership scan. */
export function nativeExclusionArgumentsV1({rooted=[],names=[],opaqueBasenames}={}){return[...rooted.flatMap(item=>["--exclude",item]),...names.flatMap(item=>["--exclude-name",item]),...(opaqueBasenames?opaqueBasenames.prefixes.flatMap(prefix=>["--exclude-hex-basename",prefix+":"+opaqueBasenames.digestHex]):[])];}

function preparePrivateFenceV1(root,directory){
 if(isAbsolute(directory)||directory.split("/").some(part=>!part||part==="."||part==="..")||directory.includes(String.fromCharCode(92)))throw Error("Relative private fence ancestry required");
 let current=resolve(root);const check=()=>{if(current.length>256||[...current].length>256)throw Error("Bounded private fence ancestry required");const status=lstatSync(current);if(!status.isDirectory()||status.isSymbolicLink())throw Error("No-follow private fence ancestry required");};check();
 for(const part of directory.split("/")){current=join(current,part);try{mkdirSync(current);}catch(error){if(error?.code!=="EEXIST")throw error;}check();}
}
