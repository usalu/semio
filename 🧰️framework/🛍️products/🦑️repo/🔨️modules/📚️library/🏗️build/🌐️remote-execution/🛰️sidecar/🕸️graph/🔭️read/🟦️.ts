import{spawn}from"node:child_process";
import{closeSync,fstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,readSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{wantedTextV1,readReadRecordV1,type ReadRecordV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{createGraphReadWatchV1,graphReadEndTextV1,graphReadStopRowV1,wantedServerPolicyV1,type WantedInferV1,type WantedServerPolicyV1}from"../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{GRAPH_WAVE_POLICY_V1,renderGraphWaveTextV1}from"../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import type{GraphWaveRowV1}from"../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{memoryOverrunV1}from"../🔬️stall/🌳️tree/🟦️.ts";
import{ownerCostContextV1,recordOwnerCostFileV1,waveEvidenceV1}from"../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";

/** 📁️ The three files of the read in flight below the directory of the graph server: the read record the runner keeps fresh, the progress output of the read and the watch file the server touches while it looks (a runner whose watch file is not touched for the orphan time stops its read). */
export const GRAPH_READ_FILES_V1=Object.freeze({record:"read-record.json",log:"read.log",watch:"read-watch",stop:"read-stop.json"});

/** 🔭️ Everything a graph read of the server needs: the command, the directory of the tree, the environment of the read, why it runs, the cancellation of the server (which only lets go of the read, see {@link followGraphReadV1}), the language, where the read keeps its files and the hooks of the server. */
export type GraphReadInputV1=Readonly<{command:string;cwd:string;environment:Readonly<Record<string,string|undefined>>;reason:"request"|"refresh";signal:AbortSignal|undefined;language:"en"|"de";directory?:string;steerDirectory?:string;costPath?:string;policy?:WantedServerPolicyV1;now?:()=>number;say?:(line:string)=>void;onWave?:(row:GraphWaveRowV1)=>void;diagnose?:(input:Readonly<{pid:number;silentMilliseconds:number;lastOutput:string}>)=>Promise<readonly string[]>;measure?:(pid:number)=>Promise<Readonly<{footprintMegabytes:number|undefined;grantMegabytes:number|undefined;pressure:string}>|undefined>}>;

const lives=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}};
const filesOf=(directory:string)=>({record:join(directory,GRAPH_READ_FILES_V1.record),log:join(directory,GRAPH_READ_FILES_V1.log),watch:join(directory,GRAPH_READ_FILES_V1.watch),stop:join(directory,GRAPH_READ_FILES_V1.stop)});

/** 🗂️ Keeps the log of the read that ended: `read.log` becomes `read.<modification time>.log` and only the newest `keep` of those stay, so a new read never truncates the evidence of the previous one. */
export function rotateGraphReadLogV1(directory:string,keep:number):string|undefined{
 const log=join(directory,GRAPH_READ_FILES_V1.log);let moved:string|undefined;
 try{const at=Math.round(statSync(log).mtimeMs);if(statSync(log).size>0){moved=join(directory,`read.${at}.log`);renameSync(log,moved);}}catch{}
 try{const old=readdirSync(directory).filter(name=>/^read\.\d+\.log$/u.test(name)).sort((a,b)=>Number(b.slice(5,-4))-Number(a.slice(5,-4)));for(const name of old.slice(Math.max(0,keep)))rmSync(join(directory,name),{force:true});}catch{}
 return moved;
}
const recordOf=(directory:string):ReadRecordV1|undefined=>{try{return readReadRecordV1(JSON.parse(readFileSync(filesOf(directory).record,"utf8")));}catch{return undefined;}};
const wait=(milliseconds:number):Promise<void>=>new Promise<void>(done=>{setTimeout(done,milliseconds);});
const REPLAY_BYTES=4*1048576;

/** 👁️ The read of `directory` that a process can attach to: a runner that lives and beat within the last 20 beats, or a read that ended while nobody watched and ended less than `readOrphanMilliseconds` ago (its result is still wanted). Anything else is a leftover and is removed. */
export function liveGraphReadV1(directory:string,policy:WantedServerPolicyV1=wantedServerPolicyV1,now:()=>number=Date.now):ReadRecordV1|undefined{
 const record=recordOf(directory);
 if(record===undefined)return undefined;
 const fresh=record.status===null?lives(record.runner)&&now()-record.beatMs<policy.readBeatMilliseconds*20:record.endedMs!==null&&now()-record.endedMs<policy.readOrphanMilliseconds;
 if(!fresh)rmSync(filesOf(directory).record,{force:true});
 return fresh?record:undefined;
}

/** 🚀️ Starts the runner of a read in a session of its own and returns its first record (the runner has started the read and knows its pid): the runner and the read outlive this process. */
async function startGraphReadV1(input:GraphReadInputV1,directory:string,policy:WantedServerPolicyV1):Promise<ReadRecordV1>{
 mkdirSync(directory,{recursive:true});
 const files=filesOf(directory),runner=join(import.meta.dir,"..","🏃️runner","🟦️.ts");
 rmSync(files.record,{force:true});rmSync(files.stop,{force:true});rotateGraphReadLogV1(directory,policy.readLogsKept);writeFileSync(files.watch,"");
 const environment={...input.environment,SEMIO_GRAPH_FLEET_READ:"off",SEMIO_GRAPH_BACKGROUND:"1",SEMIO_LEASE_PRIORITY:input.reason==="request"?"interactive":"background",SEMIO_GRAPH_WAVE_ROWS:"1",...input.steerDirectory===undefined?{}:{SEMIO_GRAPH_STEER_DIRECTORY:input.steerDirectory}},env=Object.fromEntries(Object.entries(environment).filter((entry):entry is[string,string]=>entry[1]!==undefined));
 const child=spawn(process.execPath,[runner,files.record,files.log,files.watch,String(policy.readOrphanMilliseconds),String(policy.readBeatMilliseconds),input.cwd,input.reason,input.command,"show","projects"],{cwd:input.cwd,detached:true,stdio:"ignore",windowsHide:true,env});
 let failed:string|undefined;child.on("error",error=>{failed=error.message;});child.on("exit",code=>{if(code!==0&&code!==null)failed=`runner exit ${code}`;});child.unref();
 for(let waited=0;waited<30_000;waited+=25){
  const record=recordOf(directory);
  if(record!==undefined&&record.runner===child.pid&&record.read!==null)return record;
  if(failed!==undefined)throw Error(failed);
  await wait(25);
 }
 throw Error("the read runner did not report its read");
}

/** 🔭️ Follows one read through its record and its log until it ends: every tick it reads the new output of the log (waves are progress, as before), touches the watch file, asks for the stall, the maximum, the memory overrun and the diagnosis, and stops the read through its runner (SIGINT, the runner kills after ten seconds, this process after fifteen) when the watch says so. The cancellation of the server does NOT stop the read: it lets go, the answer is `detached`, and the next process of the server attaches to the same read (`adoptGraphReadV1`); a read that was attached to starts from the output it wrote, without telling the waves again. The answer is the exit status the runner recorded, or `1` when the runner vanished without one. */
function followGraphReadV1(input:GraphReadInputV1,directory:string,first:ReadRecordV1,policy:WantedServerPolicyV1,attached:boolean):Promise<WantedInferV1>{
 const now=input.now??Date.now,files=filesOf(directory);
 let replaying=attached,offset=0,tail="",ended:"stalled"|"too-long"|"memory-overrun"|undefined,overrun:{footprintMegabytes:number|undefined;grantMegabytes:number|undefined}|undefined,sampling=false,dumpedAt=-Infinity,record=first,stopped=false,done=false;
 const watch=createGraphReadWatchV1(policy,now,row=>{if(replaying)return;try{input.onWave?.(row);}catch{}input.say?.(renderGraphWaveTextV1(row,input.language));const proof=waveEvidenceV1(row);if(proof!==undefined&&input.costPath!==undefined)recordOwnerCostFileV1(input.costPath,{name:proof.name,kind:"wave",megabytes:proof.megabytes,seconds:row.seconds,rows:0,stopped:false,atMs:now()},ownerCostContextV1().ceilingMegabytes);},first.startedMs);
 const decoder=new TextDecoder();
 const pump=():void=>{
  let fd:number;try{fd=openSync(files.log,"r");}catch{return;}
  try{
   const size=fstatSync(fd).size;
   if(size<offset)offset=0;
   if(replaying&&offset===0&&size>REPLAY_BYTES)offset=size-REPLAY_BYTES;
   while(offset<size){
    const buffer=Buffer.allocUnsafe(Math.min(1048576,size-offset)),count=readSync(fd,buffer,0,buffer.length,offset);
    if(count<=0)break;
    offset+=count;const text=decoder.decode(buffer.subarray(0,count),{stream:true});watch.feed(text);tail=(tail+text).slice(-4096);
   }
  }catch{}finally{try{closeSync(fd);}catch{}}
 };
 const pid=():number=>record.read??record.runner;
 const stop=():void=>{
  if(stopped)return;stopped=true;
  try{const row=graphReadStopRowV1(watch,ended==="stalled"||ended==="too-long"||ended==="memory-overrun"?ended:"stalled",record.runner,record.read,now());writeFileSync(files.stop,JSON.stringify(row)+"\n");input.say?.(`graph-server: stopping the read (pid ${pid()}): ${row.verdict}, ${Math.round(row.silentMilliseconds/1000)} s without progress, generation ${row.generation??"-"}, counters ${JSON.stringify(row.counters)}`);}catch{}
  try{process.kill(record.runner,"SIGINT");}catch{}
  setTimeout(()=>{if(record.status===null&&lives(record.runner)){try{process.kill(record.runner,"SIGKILL");}catch{}if(record.read!==null)try{process.kill(record.read,"SIGKILL");}catch{}}},15_000).unref();
 };
 if(attached)input.say?.(wantedTextV1(first.status===null?"adopted":"adoptedEnded",input.language,{pid:pid(),seconds:Math.round((now()-first.startedMs)/1000),status:first.status??0}));
 return new Promise<WantedInferV1>(resolve=>{
  const finish=(result:WantedInferV1,keep:boolean):void=>{
   if(done)return;done=true;clearInterval(timer);input.signal?.removeEventListener("abort",detach);
   if(!keep)rmSync(files.record,{force:true});
   resolve(result);
  };
  const detach=():void=>{
   input.say?.(wantedTextV1("detached",input.language,{pid:pid()}));
   finish({ok:false,status:1,seconds:(now()-first.startedMs)/1000,generation:watch.generation(),detail:"detached"},true);
  };
  const answer=(status:number):WantedInferV1=>({ok:status===0&&ended===undefined,status,seconds:(now()-first.startedMs)/1000,generation:watch.generation(),detail:ended==="memory-overrun"?wantedTextV1("memoryOverrun",input.language,{footprint:overrun?.footprintMegabytes??0,grant:Math.round(overrun?.grantMegabytes??0),minutes:(watch.silentMilliseconds()/60000).toFixed(1)}):ended!==undefined?graphReadEndTextV1(ended,input.language,policy):status===0?input.reason:tail.trim().split("\n").filter(line=>!line.includes(GRAPH_WAVE_POLICY_V1.marker)).at(-1)?.slice(0,200)??""});
  const tick=():void=>{
   if(done)return;
   pump();replaying=false;
   try{const at=new Date();utimesSync(files.watch,at,at);}catch{try{writeFileSync(files.watch,"");}catch{}}
   record=recordOf(directory)??record;
   if(record.status!==null){pump();finish(answer(record.status),false);return;}
   if(!lives(record.runner)){pump();finish(answer(1),false);return;}
   const verdict=watch.verdict();if(verdict!=="go"&&ended===undefined){ended=verdict;stop();}
   if(input.measure!==undefined&&ended===undefined&&!sampling&&watch.silentMilliseconds()>=policy.memoryOverrunMilliseconds){
    sampling=true;void input.measure(pid()).then(sample=>{if(sample!==undefined&&ended===undefined&&memoryOverrunV1({...sample,silentMilliseconds:watch.silentMilliseconds(),policy})){overrun=sample;ended="memory-overrun";stop();}}).catch(()=>undefined).finally(()=>{setTimeout(()=>{sampling=false;},policy.memorySampleMilliseconds).unref();});
   }
   if(input.diagnose!==undefined&&ended===undefined&&watch.silentMilliseconds()>=policy.stallDumpMilliseconds&&now()-dumpedAt>=policy.stallDumpRepeatMilliseconds){
    dumpedAt=now();void input.diagnose({pid:pid(),silentMilliseconds:watch.silentMilliseconds(),lastOutput:tail}).then(lines=>{for(const line of lines)input.say?.(line);}).catch(()=>undefined);
   }
  };
  const timer=setInterval(tick,Math.min(policy.watchMilliseconds,policy.readPollMilliseconds));
  if(input.signal?.aborted)detach();else input.signal?.addEventListener("abort",detach,{once:true});
  tick();
 });
}

/** 🔭️ One graph read of the server (the wave that had exactly one project at its memory peak proves that project's cost, which `costPath` keeps for the planning of every reader): `repo show projects` as a background refresh (it gives way between waves, `SEMIO_GRAPH_BACKGROUND=1`, and neither asks the fleet nor waits for it), run by a detached runner so that a restart of the server, the build agent or the launcher for new code never loses it, watched by its progress and not by a fixed budget: it runs for as long as it publishes waves and is stopped (SIGINT, SIGKILL later) only after `inferStallMilliseconds` without one or after `inferMaximumMilliseconds`; what is published stays, so the next read starts from it. A read that lives already (a second server process, a restart that overlapped) is attached to and not started a second time. */
export async function inferGraphReadV1(input:GraphReadInputV1):Promise<WantedInferV1>{
 const policy=input.policy??wantedServerPolicyV1,own=input.directory===undefined?mkdtempSync(join(tmpdir(),"graph-read-")):undefined,directory=input.directory??own!;
 try{
  const found=liveGraphReadV1(directory,policy,input.now),record=found??await startGraphReadV1(input,directory,policy);
  return await followGraphReadV1(input,directory,record,policy,found!==undefined);
 }finally{
  if(own!==undefined){const left=recordOf(own);if(left!==undefined&&left.status===null)try{process.kill(left.runner,"SIGINT");}catch{}rmSync(own,{recursive:true,force:true});}
 }
}

/** 🧲️ Attaches to the read that a former process of the server left running (or that ended while no server watched): `undefined` when there is none. The answer says whether it was a request or a refresh and gives the promise of its result, which is the one {@link inferGraphReadV1} would have given. */
export function adoptGraphReadV1(input:GraphReadInputV1):Readonly<{reason:"request"|"refresh";done:Promise<WantedInferV1>}>|undefined{
 const policy=input.policy??wantedServerPolicyV1;
 if(input.directory===undefined)return undefined;
 const record=liveGraphReadV1(input.directory,policy,input.now);
 if(record===undefined)return undefined;
 return Object.freeze({reason:record.reason,done:followGraphReadV1({...input,reason:record.reason},input.directory,record,policy,true)});
}
