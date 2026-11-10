import{appendFileSync,readFileSync,renameSync,statSync,writeFileSync}from"node:fs";
import{join}from"node:path";

const FILE="exit.log",MAXIMUM_BYTES=16384,MAXIMUM_ROWS=32,MEMORY_EVENTS="memory.events";

/** 🧾️ The bounded exit log of the host service under its runtime directory: one JSON row per start and per exit (reason, code, signal, memory, cgroup events), the newest `MAXIMUM_ROWS` rows within `MAXIMUM_BYTES`. A service that is killed cannot write its own end, so the row of its start (pid, cgroup) is what lets a client explain the loss.
 * @see ../🏗️composition/🌐️host/🟨️.mjs the service that writes it, ../🚪️client/🛰️plan/🟨️.mjs the client that prints it */
export function appendServiceExitRowV1(directory,row,{maximumBytes=MAXIMUM_BYTES,maximumRows=MAXIMUM_ROWS}={}){
 const file=join(directory,FILE);
 try{
  appendFileSync(file,JSON.stringify(row)+"\n",{mode:0o600});
  const all=readFileSync(file,"utf8").split("\n").filter(Boolean);
  if(all.length<=maximumRows&&statSync(file).size<=maximumBytes)return true;
  const rows=all.slice(-maximumRows);
  while(rows.length>1&&Buffer.byteLength(rows.join("\n"))>maximumBytes)rows.shift();
  const temporary=file+"."+process.pid+".tmp";writeFileSync(temporary,rows.join("\n")+"\n",{mode:0o600});renameSync(temporary,file);return true;
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

/** 🔎️ Names why the service behind `directory` is gone: the reason its last exit row gives, else (its last row is a start whose process is gone) a kill without a trace with the cgroup's `oom_kill` count, else nothing is known. */
export function explainServiceLossV1(directory,{alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error.code==="EPERM";}},events=readCgroupEventsV1}={}){
 const rows=readServiceExitRowsV1(directory),last=rows.at(-1);
 if(!last)return"the service left no exit record";
 if(last.event==="exit"){
  const held=Array.isArray(last.workers)&&last.workers.length?last.workers.reduce((sum,row)=>sum+row.heapMb+row.extraMb,0):undefined,memory=(last.rssMegabytes===undefined?"":", "+last.rssMegabytes+" MB resident")+(held===undefined?"":", slice workers held "+Math.round(held)+" MB");
  return"the service exited: "+last.reason+(last.code===undefined?"":" (code "+last.code+")")+(last.signal?" (signal "+last.signal+")":"")+memory+(last.error?": "+String(last.error).split("\n")[0]:"");
 }
 if(last.event==="start"&&typeof last.pid==="number"&&!alive(last.pid)){
  const counters=last.cgroup===undefined?undefined:events(last.cgroup),oom=counters?.oom_kill;
  return"the service was killed without an exit record"+(oom>0?": its cgroup recorded "+oom+" oom kills (memory limit "+(last.limitMegabytes??"?")+" MB)":counters===undefined?" (its cgroup is gone)":" (no oom kill in its cgroup: an external kill or a crash)");
 }
 return"the service is still recorded as running";
}
