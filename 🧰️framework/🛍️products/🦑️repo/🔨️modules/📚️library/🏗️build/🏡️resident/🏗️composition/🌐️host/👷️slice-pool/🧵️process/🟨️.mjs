/** 🧵 A slice load runs in its own process, never in a thread of the service: a thread that is busy in synchronous work cannot be stopped from outside (`Worker.terminate` waits for it to yield, and the process cannot even exit meanwhile), a process can. The process leads its own process group, so one signal from the service's main thread reaches it and everything it started; the kill does not depend on the victim's cooperation (SIGINT, then SIGKILL after the grace period; `taskkill /T` on Windows).
 * @see ../🟨️.mjs */
import {fork} from "node:child_process";
import {fileURLToPath} from "node:url";
import {writeFileSync} from "node:fs";
import {readFile} from "node:fs/promises";
import {join} from "node:path";
import {createScratchDirectoryV1,removeScratchDirectoryV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧹️scratch/🟦️.ts";
import {SLICE_GRACE_MILLISECONDS_V1,signalProcessTreeV1} from "../🧹️reap/🟨️.mjs";

export const SLICE_REQUEST_VARIABLE_V1="SEMIO_SLICE_REQUEST";
export const SLICE_SPOOL_VARIABLE_V1="SEMIO_SLICE_SPOOL";

/** 📦 Why the result travels as a file: measured on generated graphs, JSON over the IPC pipe cost 0.9 s for 50 MB, 4 s for 200 MB and 43 s (2 GB peak in both processes) for 600 MB; stringifying into a file in the child and parsing it in the service cost 0.9 s, 2.9 s and 9.5 s.
 * 🚀 Starts the load script at `url` as a process with the request in its environment and an IPC channel; answers a `Worker`-shaped handle (`on("message"|"error"|"exit")`, `pid`, `terminate()`). */
export function createProcessWorkerV1(url,data,env,{spawn=fork,graceMilliseconds=SLICE_GRACE_MILLISECONDS_V1,platform=process.platform}={}){
 const spool=createScratchDirectoryV1("slice");
 let child;
 try{child=spawn(url instanceof URL?fileURLToPath(url):String(url),[],{execPath:process.execPath,env:{...env,[SLICE_REQUEST_VARIABLE_V1]:JSON.stringify(data),[SLICE_SPOOL_VARIABLE_V1]:spool},detached:platform!=="win32",windowsHide:true,stdio:["ignore","ignore","inherit","ipc"]});}catch(error){removeScratchDirectoryV1(spool);throw error;}
 const pending=new Set();
 let exited=child.exitCode!==null||child.signalCode!==null;
 const gone=new Promise(resolve=>{if(exited)resolve();else child.once("exit",()=>{exited=true;resolve();});});
 void gone.then(()=>Promise.allSettled([...pending])).then(()=>removeScratchDirectoryV1(spool));
 return{
  pid:child.pid,
  on:(event,listener)=>{
   if(event==="message")child.on("message",message=>{
    if(message?.graphFile===undefined){listener(message);return;}
    const{graphFile,...rest}=message;
    const reading=readFile(graphFile,"utf8").then(text=>listener({...rest,graph:JSON.parse(text)}),error=>listener({type:"error",message:"Slice result unreadable: "+String(error?.message??error)}));
    pending.add(reading);void reading.finally(()=>pending.delete(reading));
   });
   else if(event==="exit")child.on("exit",code=>{void Promise.allSettled([...pending]).then(()=>listener(code));});
   else child.on(event==="error"?event:"never",listener);
  },
  /** 🛑 SIGINT to the group, SIGKILL after the grace period whether or not the process answered; resolves when the process is gone. */
  terminate(){
   if(exited)return gone;
   signalProcessTreeV1(child.pid,"SIGINT",platform);
   const hard=setTimeout(()=>signalProcessTreeV1(child.pid,"SIGKILL",platform),graceMilliseconds);hard.unref?.();
   return gone.then(()=>{clearTimeout(hard);signalProcessTreeV1(child.pid,"SIGKILL",platform);});
  },
 };
}

/** 📥 For the load script: the request the service passed (`workerData`) and the channel back (`parentPort`). */
export const workerData=JSON.parse(process.env[SLICE_REQUEST_VARIABLE_V1]??"{}");
export const parentPort={postMessage:message=>{
 const spool=process.env[SLICE_SPOOL_VARIABLE_V1];
 if(message?.type==="done"&&message.graph!==undefined&&spool){const{graph,...rest}=message,file=join(spool,"graph.json");writeFileSync(file,JSON.stringify(graph));process.send?.({...rest,graphFile:file});return;}
 process.send?.(message);
}};
