import{appendFileSync,readFileSync,renameSync,statSync,writeFileSync}from"node:fs";
import{join}from"node:path";

const FILE="exit.log",MAXIMUM_BYTES=16384,MAXIMUM_ROWS=32,MEMORY_EVENTS="memory.events",STARTING_MILLISECONDS=120000;

function storeRowsV1(file,rows,maximumBytes,maximumRows){
 const kept=rows.slice(-maximumRows);
 while(kept.length>1&&Buffer.byteLength(kept.join("\n"))>maximumBytes)kept.shift();
 const temporary=file+"."+process.pid+".tmp";writeFileSync(temporary,kept.join("\n")+"\n",{mode:0o600});renameSync(temporary,file);
}

/** 🧾️ The bounded exit log of the host service under its runtime directory: one JSON row per start and per exit (reason, code, signal, memory, cgroup events), the newest `MAXIMUM_ROWS` rows within `MAXIMUM_BYTES`. A service that is killed cannot write its own end, so the row of its start (pid, cgroup) is what lets a client explain the loss.
 * @see ../🏗️composition/🌐️host/🟨️.mjs the service that writes it, ../🚪️client/🛰️plan/🟨️.mjs the client that prints it */
export function appendServiceExitRowV1(directory,row,{maximumBytes=MAXIMUM_BYTES,maximumRows=MAXIMUM_ROWS}={}){
 const file=join(directory,FILE);
 try{
  appendFileSync(file,JSON.stringify(row)+"\n",{mode:0o600});
  const all=readFileSync(file,"utf8").split("\n").filter(Boolean);
  if(all.length<=maximumRows&&statSync(file).size<=maximumBytes)return true;
  storeRowsV1(file,all,maximumBytes,maximumRows);return true;
 }catch{return false;}
}

/** 🚦️ Completes the one start row of a start that the launcher wrote without a pid (a launchd job starts before its process exists) with the pid of the running service; a start whose row already names a pid is left as it is, so every start keeps exactly one start row. */
export function completeServiceStartRowV1(directory,{pid,...facts},{maximumBytes=MAXIMUM_BYTES,maximumRows=MAXIMUM_ROWS}={}){
 const file=join(directory,FILE);
 try{
  const rows=readServiceExitRowsV1(directory),last=rows.at(-1);
  if(last?.event!=="start"||typeof last.pid==="number")return false;
  storeRowsV1(file,[...rows.slice(0,-1),{...last,...facts,pid}].map(row=>JSON.stringify(row)),maximumBytes,maximumRows);return true;
 }catch{return false;}
}

/** 📖️ The rows of the exit log, oldest first; a missing or damaged file is no rows. */
export function readServiceExitRowsV1(directory){
 try{return readFileSync(join(directory,FILE),"utf8").split("\n").filter(Boolean).flatMap(line=>{try{return[JSON.parse(line)];}catch{return[];}});}catch{return[];}
}

/** 📐️ The kernel's verdict on a cgroup: the counters of its `memory.events` (`oom_kill`, `high`, ...). */
export function readCgroupEventsV1(path,read=file=>readFileSync(file,"utf8")){
 try{return Object.fromEntries(read(join(path,MEMORY_EVENTS)).split("\n").filter(Boolean).map(line=>{const[name,value]=line.split(" ");return[name,Number(value)];}));}catch{return undefined;}
}

/** 🔎️ Names why the service behind `directory` is gone: the reason its last exit row gives; a launchd start that names no process yet is still starting within `starting` milliseconds, else (its last row is a start whose process is gone) a kill without a trace with the cgroup's `oom_kill` count; otherwise nothing is known. */
export function explainServiceLossV1(directory,{alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error.code==="EPERM";}},events=readCgroupEventsV1,now=Date.now(),starting=STARTING_MILLISECONDS}={}){
 const rows=readServiceExitRowsV1(directory),last=rows.at(-1);
 if(!last)return"the service left no exit record";
 if(last.event==="exit"){
  const held=Array.isArray(last.workers)&&last.workers.length?last.workers.reduce((sum,row)=>sum+row.heapMb+row.extraMb,0):undefined,memory=(last.rssMegabytes===undefined?"":", "+last.rssMegabytes+" MB resident")+(held===undefined?"":", slice workers held "+Math.round(held)+" MB");
  return"the service exited: "+last.reason+(last.code===undefined?"":" (code "+last.code+")")+(last.signal?" (signal "+last.signal+")":"")+memory+(last.error?": "+String(last.error).split("\n")[0]:"");
 }
 if(last.event==="start"&&typeof last.pid!=="number"&&last.via==="launchd"&&now-last.at<starting)return"the service is still starting under launchd";
 if(last.event==="start"&&(typeof last.pid!=="number"||!alive(last.pid))){
  const cgroup=last.cgroup??rows.findLast(row=>row.event==="start"&&row.cgroup!==undefined)?.cgroup;
  if(cgroup===undefined)return"the service ended without an exit record (it was not placed in a cgroup: it failed before it could write one, e.g. while admitting its sources, or it was ended from outside; its log says which)";
  const counters=events(cgroup),oom=counters?.oom_kill;
  return"the service was killed without an exit record"+(oom>0?": its cgroup recorded "+oom+" oom kills (memory limit "+(rows.findLast(row=>row.limitMegabytes!==undefined)?.limitMegabytes??"?")+" MB)":counters===undefined?" (its cgroup is gone)":" (no oom kill in its cgroup: an external kill or a crash)");
 }
 return"the service is still recorded as running";
}
