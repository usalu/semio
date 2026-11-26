import {processPidExistsV1} from "../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts"
import{execFile,spawn}from"node:child_process";
import{createHash}from"node:crypto";
import{closeSync,fstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,readSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{wantedTextV1,readReadRecordV1,readReadWatchCheckpointV1,type ReadCapacityPauseV1,type ReadRecordV1,type ReadWatchCheckpointV1,type ReadWatchObservationV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{createGraphReadWatchV1,graphReadEndTextV1,graphReadStopRowV1,wantedServerPolicyV1,type WantedInferV1,type WantedServerPolicyV1}from"../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{GRAPH_WAVE_POLICY_V1,renderGraphWaveTextV1}from"../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import type{GraphWaveRowV1}from"../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{memoryOverrunV1}from"../🔬️stall/🌳️tree/🟦️.ts";
import{ownerCostContextV1,recordOwnerCostFileV1,waveEvidenceV1}from"../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";

/** 📁️ Local files of one graph read beneath the graph server directory. The runner owns its record and log. The observer refreshes the heartbeat and checkpoints its progress on detach. */
export const GRAPH_READ_FILES_V1=Object.freeze({record:"read-record.json",log:"read.log",watch:"read-watch",stop:"read-stop.json",checkpoint:"read-checkpoint.json"});

/** 🔭️ Everything a graph read of the server needs: the command, the directory of the tree, the environment of the read, why it runs, the cancellation of the server (which only lets go of the read, see {@link followGraphReadV1}), the language, where the read keeps its files and the hooks of the server. */
export type GraphReadCapacityWaitV1=Readonly<{id:string;scope:string;arrivedAtMs:number;reservedMegabytes:number;bookedAtMs?:number}>;
export type GraphReadInputV1=Readonly<{command:string;cwd:string;environment:Readonly<Record<string,string|undefined>>;reason:"request"|"refresh";signal:AbortSignal|undefined;language:"en"|"de";directory?:string;steerDirectory?:string;costPath?:string;policy?:WantedServerPolicyV1;now?:()=>number;say?:(line:string)=>void;onWave?:(row:GraphWaveRowV1)=>void;diagnose?:(input:Readonly<{pid:number;silentMilliseconds:number;lastOutput:string}>)=>Promise<readonly string[]>;capacityWait?:(record:ReadRecordV1,signal:AbortSignal,prior?:ReadCapacityPauseV1)=>Promise<GraphReadCapacityWaitV1|undefined>;measure?:(pid:number)=>Promise<Readonly<{footprintMegabytes:number|undefined;grantMegabytes:number|undefined;pressure:string}>|undefined>}>;

/** 🧭️ Keeps native process and ledger inspection out of the graph observer's event loop. */
export function graphReadCapacityProbeV1(environment:GraphReadInputV1["environment"]):NonNullable<GraphReadInputV1["capacityWait"]>{
 const script=join(import.meta.dir,"⏳️wait","📜️script.ts");
 return (record,signal,prior)=>new Promise(resolve=>{
  if(record.read===null||signal.aborted)return resolve(undefined);
  execFile(process.execPath,[script,String(record.read),String(record.runner),String(record.startedMs),...(prior===undefined?[]:[prior.id,prior.scope,String(prior.arrivedAtMs)])],{signal,env:{...environment,TZ:"UTC",LC_ALL:"C"},windowsHide:true,maxBuffer:2048,timeout:7500,killSignal:"SIGKILL"},(error,output)=>{
   if(error||signal.aborted)return resolve(undefined);
   try{const value=JSON.parse(output) as GraphReadCapacityWaitV1;resolve(value);}catch{resolve(undefined);}
  });
 });
}

const lives=(pid:number):boolean=>processPidExistsV1(pid);
const filesOf=(directory:string)=>({record:join(directory,GRAPH_READ_FILES_V1.record),log:join(directory,GRAPH_READ_FILES_V1.log),watch:join(directory,GRAPH_READ_FILES_V1.watch),stop:join(directory,GRAPH_READ_FILES_V1.stop),checkpoint:join(directory,GRAPH_READ_FILES_V1.checkpoint)});

/** 🗂️ Keeps the log of the read that ended: `read.log` becomes `read.<modification time>.log` and only the newest `keep` of those stay, so a new read never truncates the evidence of the previous one. */
export function rotateGraphReadLogV1(directory:string,keep:number):string|undefined{
 const log=join(directory,GRAPH_READ_FILES_V1.log);let moved:string|undefined;
 try{const at=Math.round(statSync(log).mtimeMs);if(statSync(log).size>0){moved=join(directory,`read.${at}.log`);renameSync(log,moved);}}catch{}
 try{const old=readdirSync(directory).filter(name=>/^read\.\d+\.log$/u.test(name)).sort((a,b)=>Number(b.slice(5,-4))-Number(a.slice(5,-4)));for(const name of old.slice(Math.max(0,keep)))rmSync(join(directory,name),{force:true});}catch{}
 return moved;
}
const recordOf=(directory:string):ReadRecordV1|undefined=>{try{return readReadRecordV1(JSON.parse(readFileSync(filesOf(directory).record,"utf8")));}catch{return undefined;}};
const wait=(milliseconds:number):Promise<void>=>new Promise<void>(done=>{setTimeout(done,milliseconds);});
const REPLAY_BYTES=4*1048576,MAXIMUM_READ_OFFSET=BigInt(Number.MAX_SAFE_INTEGER);
const matchesRead=(left:ReadRecordV1|ReadWatchCheckpointV1,right:ReadRecordV1)=>left.runner===right.runner&&left.read===right.read&&left.startedMs===right.startedMs;
const digest=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const checkpointPrefix=(fd:number,offset:number):Buffer|undefined=>{
 const bytes=Buffer.allocUnsafe(Math.min(offset,4096));
 return readSync(fd,bytes,0,bytes.length,offset-bytes.length)===bytes.length&&(offset===0||bytes.at(-1)===10)?bytes:undefined;
};
const readCheckpoint=(files:ReturnType<typeof filesOf>,record:ReadRecordV1,policy:WantedServerPolicyV1,at:number):Readonly<{row:ReadWatchCheckpointV1;tail:string}>|undefined=>{
 let checkpointFd:number|undefined,logFd:number|undefined;
 try{
  checkpointFd=openSync(files.checkpoint,"r");const file=fstatSync(checkpointFd),config=policy.readProgress;
  if(!file.isFile()||file.size===0||file.size>4096+config.maximumPhases*128+config.maximumOwners*(config.maximumResourceCharacters+64))return;
  const text=Buffer.allocUnsafe(file.size);
  if(readSync(checkpointFd,text,0,text.length,0)!==text.length)return;
  const row=readReadWatchCheckpointV1(JSON.parse(text.toString("utf8")));
  if(row===undefined||!matchesRead(row,record)||!Number.isSafeInteger(row.offset))return;
  const {progressAt,generation,phases,owners}=row.observation;
  const pause=row.capacityPause;
  if(pause!==undefined&&(!Number.isSafeInteger(pause.progressAtMs)||!Number.isSafeInteger(pause.throughMs)||!Number.isSafeInteger(pause.creditedMs)||pause.progressAtMs!==progressAt||pause.throughMs>at||pause.throughMs<progressAt||pause.arrivedAtMs<record.startedMs||pause.arrivedAtMs>pause.throughMs||pause.creditedMs>pause.throughMs-progressAt))return;
  if(!Number.isSafeInteger(progressAt)||progressAt<record.startedMs||progressAt>at||generation!==null&&!Number.isSafeInteger(generation))return;
  if(phases.length>config.maximumPhases+Number(phases.some(phase=>phase.name==="owners:ledger"))||owners.length>config.maximumOwners||new Set(phases.map(phase=>phase.name)).size!==phases.length||new Set(owners.map(owner=>owner.name)).size!==owners.length)return;
  if(phases.some(({done,total})=>!Number.isSafeInteger(total)||done>total)||owners.some(({name,rows})=>name.length>config.maximumResourceCharacters||!Number.isSafeInteger(rows)||encodeURIComponent(decodeURIComponent(name))!==name))return;
  logFd=openSync(files.log,"r");const log=fstatSync(logFd,{bigint:true}),offset=BigInt(row.offset);
  if(!log.isFile()||String(log.dev)!==row.device||String(log.ino)!==row.inode||log.size<offset||log.size>MAXIMUM_READ_OFFSET||log.size-offset>BigInt(REPLAY_BYTES))return;
  const bytes=checkpointPrefix(logFd,row.offset);
  if(bytes!==undefined&&digest(bytes)===row.prefix)return{row,tail:bytes.toString("utf8")};
 }catch{}finally{if(logFd!==undefined)try{closeSync(logFd);}catch{}if(checkpointFd!==undefined)try{closeSync(checkpointFd);}catch{}}
};
type ReadPositionV1=Readonly<{offset:number;device:bigint;inode:bigint}>;
const writeCheckpoint=(files:ReturnType<typeof filesOf>,first:ReadRecordV1,position:ReadPositionV1,observation:ReadWatchObservationV1,capacityPause?:ReadCapacityPauseV1):void=>{
 const temporary=files.checkpoint+"."+process.pid+".tmp";let fd:number|undefined;
 try{
  fd=openSync(files.log,"r");const log=fstatSync(fd,{bigint:true});
  if(log.dev!==position.device||log.ino!==position.inode||log.size<BigInt(position.offset))return;
  const bytes=checkpointPrefix(fd,position.offset);
  if(bytes===undefined)return;
  const row:ReadWatchCheckpointV1={version:1,runner:first.runner,read:first.read,startedMs:first.startedMs,offset:position.offset,device:String(position.device),inode:String(position.inode),prefix:digest(bytes),observation,...capacityPause===undefined?{}:{capacityPause}};
  writeFileSync(temporary,JSON.stringify(row)+"\n");renameSync(temporary,files.checkpoint);
 }catch{try{rmSync(temporary,{force:true});}catch{}}finally{if(fd!==undefined)try{closeSync(fd);}catch{}}
};


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
 rmSync(files.record,{force:true});rmSync(files.stop,{force:true});rmSync(files.checkpoint,{force:true});rotateGraphReadLogV1(directory,policy.readLogsKept);writeFileSync(files.watch,"");
 const environment={...input.environment,SEMIO_BUILD_GRAPH_POLICY:"fresh",SEMIO_GRAPH_FLEET_READ:"off",SEMIO_GRAPH_BACKGROUND:"1",SEMIO_LEASE_PRIORITY:input.reason==="request"?"interactive":"background",SEMIO_GRAPH_WAVE_ROWS:"1",...input.steerDirectory===undefined?{}:{SEMIO_GRAPH_STEER_DIRECTORY:input.steerDirectory}},env=Object.fromEntries(Object.entries(environment).filter((entry):entry is[string,string]=>entry[1]!==undefined));
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
 const handoff=attached?readCheckpoint(files,first,policy,now()):undefined;
 let restoring=handoff?.row,offset=restoring?.offset??0,complete=offset,tail=handoff?.tail??"",position:ReadPositionV1|undefined;
 let replaying=attached,ended:"stalled"|"too-long"|"memory-overrun"|undefined,overrun:{footprintMegabytes:number|undefined;grantMegabytes:number|undefined}|undefined,sampling=false,dumpedAt=-Infinity,record=first,stopped=false,done=false;
 const createWatch=(observation?:ReadWatchObservationV1)=>createGraphReadWatchV1(policy,now,row=>{if(replaying||done)return;try{input.onWave?.(row);}catch{}input.say?.(renderGraphWaveTextV1(row,input.language));const proof=waveEvidenceV1(row);if(proof!==undefined&&input.costPath!==undefined)recordOwnerCostFileV1(input.costPath,{name:proof.name,kind:"wave",megabytes:proof.megabytes,seconds:row.seconds,rows:0,stopped:false,atMs:now()},ownerCostContextV1().ceilingMegabytes);},first.startedMs,observation);
 let capacityPause=restoring?.capacityPause,reverifyCapacity=capacityPause!==undefined,sampledAt=-Infinity,capacityProbe:AbortController|undefined,reservation:number|undefined;
 let watch=createWatch(restoring?.observation),decoder=new TextDecoder("utf-8",{ignoreBOM:offset>0});
 const pump=():boolean=>{
  position=undefined;
  let fd:number;try{fd=openSync(files.log,"r");}catch{return false;}
  try{
   const info=fstatSync(fd,{bigint:true});
   if(!info.isFile()||info.size>MAXIMUM_READ_OFFSET)return false;
   const size=Number(info.size);
   if(restoring!==undefined&&(String(info.dev)!==restoring.device||String(info.ino)!==restoring.inode||size<offset||size-restoring.offset>REPLAY_BYTES)){
    restoring=undefined;watch=createWatch();capacityPause=undefined;reverifyCapacity=false;offset=0;complete=0;tail="";decoder=new TextDecoder();
   }
   if(size<offset){offset=0;complete=0;decoder=new TextDecoder();}
   if(replaying&&restoring===undefined&&offset===0&&size>REPLAY_BYTES)offset=size-REPLAY_BYTES;
   while(offset<size){
    const buffer=Buffer.allocUnsafe(Math.min(1048576,size-offset)),count=readSync(fd,buffer,0,buffer.length,offset);
    if(count<=0)return false;
    const bytes=buffer.subarray(0,count),end=bytes.lastIndexOf(10);
    if(end>=0)complete=offset+end+1;
    offset+=count;const text=decoder.decode(bytes,{stream:true});watch.feed(text);tail=(tail+text).slice(-4096);
   }
   position={offset:complete,device:info.dev,inode:info.ino};restoring=undefined;return true;
  }catch{return false;}finally{try{closeSync(fd);}catch{}}
 };
 const pid=():number=>record.read??record.runner;
 const stop=():void=>{
  if(done||stopped)return;stopped=true;
  try{const row=graphReadStopRowV1(watch,ended==="stalled"||ended==="too-long"||ended==="memory-overrun"?ended:"stalled",record.runner,record.read,now());writeFileSync(files.stop,JSON.stringify(row)+"\n");input.say?.(`graph-server: stopping the read (pid ${pid()}): ${row.verdict}, ${Math.round(row.silentMilliseconds/1000)} s without progress, generation ${row.generation??"-"}, counters ${JSON.stringify(row.counters)}`);}catch{}
  try{process.kill(record.runner,"SIGINT");}catch{}
  setTimeout(()=>{if(record.status===null&&lives(record.runner)){try{process.kill(record.runner,"SIGKILL");}catch{}if(record.read!==null)try{process.kill(record.read,"SIGKILL");}catch{}}},15_000).unref();
 };
 if(attached)input.say?.(wantedTextV1(first.status===null?"adopted":"adoptedEnded",input.language,{pid:pid(),seconds:Math.round((now()-first.startedMs)/1000),status:first.status??0}));
 return new Promise<WantedInferV1>(resolve=>{
  const finish=(result:WantedInferV1,keep:boolean):void=>{
   if(done)return;done=true;capacityProbe?.abort();clearInterval(timer);input.signal?.removeEventListener("abort",detach);
   if(!keep){rmSync(files.record,{force:true});rmSync(files.checkpoint,{force:true});}
   resolve(result);
  };
  const detach=():void=>{
   const current=recordOf(directory);
   if(!stopped&&position!==undefined&&current?.status===null&&matchesRead(first,current))writeCheckpoint(files,first,position,watch.observation(),capacityPause?.progressAtMs===watch.progressAt()?capacityPause:undefined);
   input.say?.(wantedTextV1("detached",input.language,{pid:pid()}));
   finish({ok:false,status:1,seconds:(now()-first.startedMs)/1000,generation:watch.generation(),detail:"detached"},true);
  };
  const answer=(status:number):WantedInferV1=>({ok:status===0&&ended===undefined,status,seconds:(now()-first.startedMs)/1000,generation:watch.generation(),detail:ended==="memory-overrun"?wantedTextV1("memoryOverrun",input.language,{footprint:overrun?.footprintMegabytes??0,grant:Math.round(overrun?.grantMegabytes??0),minutes:(watch.silentMilliseconds()/60000).toFixed(1)}):ended!==undefined?graphReadEndTextV1(ended,input.language,policy):status===0?input.reason:tail.trim().split("\n").filter(line=>!line.includes(GRAPH_WAVE_POLICY_V1.marker)).at(-1)?.slice(0,200)??""});
  const tick=():void=>{
   if(done)return;
   const current=recordOf(directory);
   if(current!==undefined&&!matchesRead(first,current)){finish({ok:false,status:1,seconds:(now()-first.startedMs)/1000,generation:watch.generation(),detail:"detached"},true);return;}
   record=current??record;
   const caughtUp=pump();if(done)return;
   replaying=restoring!==undefined;
   try{const at=new Date();utimesSync(files.watch,at,at);}catch{try{writeFileSync(files.watch,"");}catch{}}
   if(record.status!==null){pump();finish(answer(record.status),false);return;}
   if(!lives(record.runner)){pump();finish(answer(1),false);return;}
   const at=now(),verdict=watch.verdict(at);if(!caughtUp&&restoring!==undefined&&verdict!=="too-long")return;
   const progressAt=watch.progressAt(),silent=at-progressAt;
   if(capacityPause?.progressAtMs!==progressAt)capacityPause=undefined;
   const activeBefore=silent-(capacityPause?.creditedMs??0),eligible=ended===undefined&&verdict!=="too-long"&&caughtUp&&input.capacityWait!==undefined&&(reverifyCapacity||reservation!==undefined||activeBefore>=Math.max(policy.stallDumpMilliseconds,policy.inferStallMilliseconds-30_000));
   if(eligible&&capacityProbe===undefined&&at-sampledAt>=Math.max(policy.readPollMilliseconds,Math.min(10_000,policy.stallDumpMilliseconds))){
    sampledAt=at;reverifyCapacity=false;
    const probe=capacityProbe=new AbortController(),read=record,timeout=setTimeout(()=>{probe.abort();if(capacityProbe===probe){capacityProbe=undefined;reservation=undefined;}},8000);timeout.unref();
    void input.capacityWait!(read,probe.signal,capacityPause).then(wait=>{
     if(done||probe.signal.aborted||capacityProbe!==probe||!matchesRead(read,record))return;
     const confirmedAt=now(),baseline=watch.progressAt(),fresh=recordOf(directory);
     if(wait===undefined||fresh?.status!==null||!matchesRead(first,fresh)||confirmedAt-fresh.beatMs>=policy.readBeatMilliseconds*20||!Number.isSafeInteger(wait.arrivedAtMs)||wait.arrivedAtMs<first.startedMs||wait.arrivedAtMs>confirmedAt||!Number.isSafeInteger(wait.reservedMegabytes)||wait.reservedMegabytes<=0){
      reservation=undefined;return;
     }
     reservation=wait.reservedMegabytes;
     const prior=capacityPause?.progressAtMs===baseline?capacityPause:undefined;
     const {id,scope,arrivedAtMs,bookedAtMs}=wait;
     if(bookedAtMs!==undefined){
      if(prior?.id===id&&prior.scope===scope&&prior.arrivedAtMs===arrivedAtMs&&Number.isSafeInteger(bookedAtMs)&&bookedAtMs>=Math.max(baseline,arrivedAtMs,prior.throughMs)&&bookedAtMs<=confirmedAt)
       capacityPause={progressAtMs:baseline,id,scope,arrivedAtMs,throughMs:bookedAtMs,creditedMs:bookedAtMs-baseline};
      reservation=undefined;return;
     }
     const from=Math.max(baseline,arrivedAtMs,prior?.throughMs??baseline);
     if(confirmedAt>=from)capacityPause={progressAtMs:baseline,id,scope,arrivedAtMs,throughMs:confirmedAt,creditedMs:(prior?.creditedMs??0)+confirmedAt-from};
    }).catch(()=>{if(capacityProbe===probe)reservation=undefined;}).finally(()=>{clearTimeout(timeout);if(capacityProbe===probe)capacityProbe=undefined;});
   }
   const activeSilent=silent-(capacityPause?.creditedMs??0);
   if(ended===undefined&&(verdict==="too-long"||verdict==="stalled"&&capacityProbe===undefined&&activeSilent>=policy.inferStallMilliseconds)){ended=verdict;stop();}
   if(input.measure!==undefined&&ended===undefined&&!sampling&&silent>=policy.memoryOverrunMilliseconds){
    sampling=true;void input.measure(pid()).then(sample=>{if(done||sample===undefined||ended!==undefined)return;const actual={...sample,grantMegabytes:sample.grantMegabytes??reservation};if(memoryOverrunV1({...actual,silentMilliseconds:watch.silentMilliseconds(),policy})){overrun=actual;ended="memory-overrun";stop();}}).catch(()=>undefined).finally(()=>{setTimeout(()=>{sampling=false;},policy.memorySampleMilliseconds).unref();});
   }
   if(input.diagnose!==undefined&&ended===undefined&&silent>=policy.stallDumpMilliseconds&&at-dumpedAt>=policy.stallDumpRepeatMilliseconds){
    dumpedAt=at;void input.diagnose({pid:pid(),silentMilliseconds:silent,lastOutput:tail}).then(lines=>{if(done)return;for(const line of lines)input.say?.(line);}).catch(()=>undefined);
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
