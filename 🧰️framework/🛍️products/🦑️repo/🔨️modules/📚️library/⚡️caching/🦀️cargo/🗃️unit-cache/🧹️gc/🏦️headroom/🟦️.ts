import {randomBytes} from "node:crypto";
import {mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {processAliveV1,sleepV1} from "../../🔒️lock/🟦️.ts";
import {collectGarbageSingleFlightV1,measureVolumeV1} from "../🟦️.ts";
import {ensureBallastV1,releaseBallastV1,tryClaimWithBallastV1} from "../🛟️ballast/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const,layout=UNIT_CACHE_POLICY_V1.layout,GIB=1024**3;
/** 📐️ The authored policy of the image headroom admission.
 * @see ./🧬️schema/🔣️.json */
export const HEADROOM_POLICY_V1=policy;
/** 🧾️ One command in the headroom ledger: a waiting command has reserved nothing yet, a running command is still expected to write `reservedBytes` over its ramp, counted from `admittedMs`. */
export type HeadroomRowV1=Readonly<{id:string;state:"waiting"|"running";reservedBytes:number;rampMilliseconds:number;arrivedMs:number;admittedMs:number|null}>;
/** 📍️ What one finished command wrote to the image and how long it took. */
export type HeadroomSampleV1=Readonly<{bytes:number;wallMs:number;atMs:number}>;
/** 📚️ The newest samples of every class of command. */
export type HeadroomHistoryV1=Readonly<{version:1;classes:Readonly<Record<string,readonly HeadroomSampleV1[]>>}>;
/** 🌱️ What a class without enough samples is assumed to write. */
export type HeadroomPriorV1=Readonly<{bytes:number;rampMilliseconds:number}>;
/** 🔮️ The reservation a command asks for and the class its numbers were learned under (`null` for the prior). */
export type HeadroomEstimateV1=Readonly<{reservedBytes:number;rampMilliseconds:number;samples:number;class:string|null}>;
/** ⚖️ The verdict on one command: admitted because it fits, because nothing else runs and a cleanup found nothing more (`alone`), or kept waiting for space or for the commands ahead of it. */
export type HeadroomDecisionV1=Readonly<{verdict:"admit"|"wait";reason:"fits"|"alone"|"space"|"queue";floorBytes:number;heldBytes:number;aheadBytes:number;shortfallBytes:number;running:number}>;
/** 📋️ Everything the pure admission rule decides on. */
export type HeadroomInputV1=Readonly<{freeBytes:number;volumeBytes:number;nowMs:number;request:Readonly<{id:string;reservedBytes:number;arrivedMs:number}>;rows:readonly HeadroomRowV1[];reclaimed:boolean}>;

const clamp=(value:number,low:number,high:number):number=>Math.min(high,Math.max(low,value));

/** 🛟️ The free bytes that are never promised away: the floor share of the image, at least the minimum. */
export function headroomFloorBytesV1(volumeBytes:number,rules:Pick<typeof policy,"floorBasisPoints"|"floorMinimumBytes">=policy):number{return Math.max(rules.floorMinimumBytes,Math.floor(volumeBytes*rules.floorBasisPoints/10000));}

/** 📊️ The nearest-rank quantile of integers (the smallest value that at least `basisPoints` of ten thousand of the values do not exceed). */
export function headroomQuantileV1(values:readonly number[],basisPoints:number):number{
 const sorted=[...values].sort((left,right)=>left-right),rank=Math.max(1,Math.ceil(basisPoints*sorted.length/10000));
 return sorted[Math.min(sorted.length,rank)-1]!;
}

/** 🔮️ What a command of these classes (general to specific) is expected to write and how long it takes: the quantile of the most specific class with enough samples plus the margin, else the prior; the bytes never exceed the maximum share of the image and the ramp stays inside its bounds. Pure and deterministic. */
export function headroomEstimateV1(history:HeadroomHistoryV1,classes:readonly string[],prior:HeadroomPriorV1,volumeBytes:number,rules:Pick<typeof policy,"maximumShareBasisPoints"|"minimumBytes"|"estimate">=policy):HeadroomEstimateV1{
 const ceiling=Math.floor(volumeBytes*rules.maximumShareBasisPoints/10000),bytesOf=(bytes:number)=>clamp(bytes,rules.minimumBytes,ceiling),rampOf=(ms:number)=>clamp(ms,rules.estimate.minimumRampMilliseconds,rules.estimate.maximumRampMilliseconds);
 for(const name of [...classes].reverse()){
  const rows=(history.classes[name]??[]).slice(-rules.estimate.maximumSamples);
  if(rows.length>=rules.estimate.minimumSamples)return Object.freeze({reservedBytes:bytesOf(Math.floor(headroomQuantileV1(rows.map(row=>row.bytes),rules.estimate.quantileBasisPoints)*(10000+rules.estimate.marginBasisPoints)/10000)),rampMilliseconds:rampOf(headroomQuantileV1(rows.map(row=>row.wallMs),rules.estimate.quantileBasisPoints)),samples:rows.length,class:name});
 }
 return Object.freeze({reservedBytes:bytesOf(prior.bytes),rampMilliseconds:rampOf(prior.rampMilliseconds),samples:0,class:null});
}

/** ⏳️ The bytes a running command is still expected to write: its reservation shrinks linearly to zero over its ramp, because what it wrote since is already missing from the free space. A waiting command expects nothing yet. */
export function headroomOutstandingV1(row:HeadroomRowV1,nowMs:number):number{
 if(row.state!=="running"||row.admittedMs===null)return 0;
 const ramp=Math.max(1,row.rampMilliseconds),left=Math.max(0,ramp-Math.max(0,nowMs-row.admittedMs));
 return Math.floor(row.reservedBytes*Math.floor(left*10000/ramp)/10000);
}

/** 🚦️ Decides whether a command may start: the free space minus the floor, minus what running commands are still expected to write and minus the reservations of the waiting commands that arrived before it must cover its own reservation. A command that does not fit waits, except the only one left (nothing runs, nothing is ahead) once a cleanup has run: waiting can free nothing more, so it goes on and the out-of-space retry handles a real shortfall. Pure and deterministic. */
export function decideHeadroomV1(input:HeadroomInputV1,rules:Pick<typeof policy,"floorBasisPoints"|"floorMinimumBytes">=policy):HeadroomDecisionV1{
 const others=input.rows.filter(row=>row.id!==input.request.id),running=others.filter(row=>row.state==="running"),floorBytes=headroomFloorBytesV1(input.volumeBytes,rules);
 const heldBytes=running.reduce((sum,row)=>sum+headroomOutstandingV1(row,input.nowMs),0);
 const ahead=others.filter(row=>row.state==="waiting"&&(row.arrivedMs<input.request.arrivedMs||row.arrivedMs===input.request.arrivedMs&&row.id<input.request.id)),aheadBytes=ahead.reduce((sum,row)=>sum+row.reservedBytes,0);
 const room=input.freeBytes-floorBytes-heldBytes-aheadBytes,shortfallBytes=Math.max(0,input.request.reservedBytes-room),facts={floorBytes,heldBytes,aheadBytes,shortfallBytes,running:running.length};
 if(shortfallBytes===0)return Object.freeze({verdict:"admit",reason:"fits",...facts});
 if(running.length===0&&ahead.length===0&&input.reclaimed)return Object.freeze({verdict:"admit",reason:"alone",...facts});
 return Object.freeze({verdict:"wait",reason:aheadBytes>0&&input.freeBytes-floorBytes-heldBytes>=input.request.reservedBytes?"queue":"space",...facts});
}

const className=(name:string,temperature:string):string=>name.replace(/\s+/gu,"_").slice(0,policy.maximumClassLength)+"#"+temperature;
const gib=(bytes:number):string=>(bytes/GIB).toFixed(1);

/** 💬️ One operator message with its placeholders filled. */
export function headroomTextV1(kind:keyof typeof messages,parameters:Readonly<Record<string,string|number>>,language:"en"|"de"):string{return messages[kind][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(parameters[name]??""));}

const isSample=(row:unknown):row is HeadroomSampleV1=>typeof row==="object"&&row!==null&&Number.isSafeInteger((row as HeadroomSampleV1).bytes)&&(row as HeadroomSampleV1).bytes>=0&&Number.isSafeInteger((row as HeadroomSampleV1).wallMs)&&(row as HeadroomSampleV1).wallMs>=0&&Number.isSafeInteger((row as HeadroomSampleV1).atMs);
const historyPath=(directory:string):string=>join(directory,policy.historyFile);
const writeAtomic=(path:string,text:string):void=>{const temporary=path+"."+randomBytes(3).toString("hex")+".tmp";try{writeFileSync(temporary,text);renameSync(temporary,path);}catch(error){try{rmSync(temporary,{force:true});}catch{}throw error;}};
const roomy=<T>(root:string,body:()=>T):T=>{try{return body();}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOSPC"||!releaseBallastV1(root))throw error;return body();}};

/** 📖️ Reads the history of a headroom directory; a missing, unreadable or malformed file is an empty history, a malformed row is skipped. */
export function readHeadroomHistoryV1(directory:string):HeadroomHistoryV1{
 try{
  const parsed=JSON.parse(readFileSync(historyPath(directory),"utf8")) as {classes?:Record<string,unknown>},classes:Record<string,HeadroomSampleV1[]>={};
  for(const [name,rows] of Object.entries(parsed.classes??{}))if(Array.isArray(rows))classes[name]=rows.filter(isSample).map(row=>({bytes:row.bytes,wallMs:row.wallMs,atMs:row.atMs}));
  return Object.freeze({version:1,classes:Object.freeze(classes)});
 }catch{return Object.freeze({version:1,classes:Object.freeze({})});}
}

/** 🧮️ The history after one finished command is recorded under each of its classes: every class keeps its newest samples, and the classes beyond the maximum are dropped, least recently measured first. Pure. */
export function recordHeadroomSampleV1(history:HeadroomHistoryV1,names:readonly string[],sample:HeadroomSampleV1,rules:Pick<typeof policy,"estimate">=policy):HeadroomHistoryV1{
 const classes:Record<string,readonly HeadroomSampleV1[]>={...history.classes};
 for(const name of names)classes[name]=[...(classes[name]??[]),sample].slice(-rules.estimate.maximumSamples);
 const newest=(name:string)=>classes[name]!.reduce((latest,row)=>Math.max(latest,row.atMs),0),kept=Object.keys(classes).sort((left,right)=>newest(right)-newest(left)||(left<right?-1:left>right?1:0)).slice(0,rules.estimate.maximumClasses);
 return Object.freeze({version:1,classes:Object.freeze(Object.fromEntries(kept.sort().map(name=>[name,classes[name]!])))});
}

type RecordV1=Readonly<HeadroomRowV1&{version:1;pid:number;label:string}>;

function readRows(directory:string,realMs:number):HeadroomRowV1[]{
 let all:string[];try{all=readdirSync(directory);}catch{return[];}
 for(const name of all.filter(name=>name.endsWith(".tmp")))try{if(realMs-statSync(join(directory,name)).mtimeMs>policy.abandonedMilliseconds)rmSync(join(directory,name),{force:true});}catch{}
 const names=all.filter(name=>name.endsWith(".json")&&name!==policy.historyFile),rows:HeadroomRowV1[]=[];
 for(const name of names){
  const path=join(directory,name);
  try{
   const age=realMs-statSync(path).mtimeMs,row=JSON.parse(readFileSync(path,"utf8")) as RecordV1;
   if(age>policy.abandonedMilliseconds||age>policy.staleMilliseconds&&!processAliveV1(row.pid)){rmSync(path,{force:true});continue;}
   if(age<=policy.staleMilliseconds&&processAliveV1(row.pid)&&typeof row.id==="string"&&(row.state==="waiting"||row.state==="running")&&Number.isSafeInteger(row.reservedBytes)&&Number.isSafeInteger(row.rampMilliseconds)&&Number.isSafeInteger(row.arrivedMs))rows.push({id:row.id,state:row.state,reservedBytes:row.reservedBytes,rampMilliseconds:row.rampMilliseconds,arrivedMs:row.arrivedMs,admittedMs:Number.isSafeInteger(row.admittedMs)?row.admittedMs:null});
  }catch{}
 }
 return rows;
}

/** 🔭️ The live commands of the headroom ledger of a cache root: a record counts only while its process lives and keeps it fresh; abandoned records are removed. */
export function readHeadroomLedgerV1(root:string,realMs:number=Date.now()):HeadroomRowV1[]{return readRows(join(root,layout.headroom),realMs);}

/** 🎟️ An admitted command's hold on the image: `release` ends the reservation and records what the command wrote (`grown`, measured after it ended) under its classes. */
export type HeadroomLeaseV1=Readonly<{id:string;reservedBytes:number;waitedMs:number;release(grown?:()=>number):Promise<void>}>;
type SleepV1=(milliseconds:number,signal?:AbortSignal)=>Promise<void>;
/** 🔌️ What admission touches, replaceable by tests. `reclaim` frees the bytes it is given on demand and answers whether a collection ran. */
export type HeadroomPortsV1=Readonly<{now?:()=>number;sleep?:SleepV1;measure?:typeof measureVolumeV1;reclaim?:(demandBytes:number)=>Promise<boolean>;pid?:number;id?:string}>;

async function withClaim<T>(root:string,body:()=>T,sleep:SleepV1,attempts:number,signal?:AbortSignal):Promise<T|undefined>{
 for(let attempt=0;attempt<attempts;attempt++){
  signal?.throwIfAborted();
  const claim=tryClaimWithBallastV1(join(root,layout.locks),policy.claim);
  if(claim)try{return body();}finally{claim.release();}
  await sleep(policy.claimPollMilliseconds,signal);
 }
 return undefined;
}

/** 🏦️ Admits one Cargo command to the compiler image. The command reserves what its classes (`classes` general to specific, `temperature` of the lane it runs in) wrote before; while the free space does not cover that on top of what the running commands are still expected to write and the floor, it waits in arrival order, frees what it can on demand (idle lanes, incremental sessions and cold units, never anything a running command holds) and reports its wait; a cancellation withdraws it. Once admitted, the lease watches the image every `monitorMilliseconds` and frees on demand the moment the free space falls below the floor, and `release` ends the reservation and learns what the command wrote. An image that cannot be measured admits at once (nothing to protect). */
export async function admitImageHeadroomV1(options:Readonly<{root:string;capacityBytes:number;classes:readonly string[];temperature:"warm"|"cold";label:string;language:"en"|"de";report:(line:string)=>void;signal?:AbortSignal}&HeadroomPortsV1>):Promise<HeadroomLeaseV1>{
 const now=options.now??Date.now,sleep=options.sleep??sleepV1,measure=options.measure??measureVolumeV1,directory=join(options.root,layout.headroom),start=measure(options.root,options.capacityBytes);
 const unlimited:HeadroomLeaseV1=Object.freeze({id:"-",reservedBytes:0,waitedMs:0,release:async()=>{}});
 if(start.unmeasured)return unlimited;
 const names=options.classes.map(name=>className(name,options.temperature)),estimate=headroomEstimateV1(readHeadroomHistoryV1(directory),names,policy.prior[options.temperature],start.volumeBytes);
 const reclaim=options.reclaim??(async(demandBytes:number)=>!(await collectGarbageSingleFlightV1(options.root,options.capacityBytes,{demandBytes,signal:options.signal})).running);
 const id=options.id??randomBytes(8).toString("hex"),file=join(directory,id+".json"),arrivedMs=now(),pid=options.pid??process.pid;
 ensureBallastV1(options.root);
 const write=(state:"waiting"|"running",admittedMs:number|null)=>roomy(options.root,()=>writeAtomic(file,JSON.stringify({version:1,pid,label:options.label.slice(0,200),id,state,reservedBytes:estimate.reservedBytes,rampMilliseconds:estimate.rampMilliseconds,arrivedMs,admittedMs})));
 try{mkdirSync(directory,{recursive:true});write("waiting",null);}catch{return unlimited;}
 const beat=setInterval(()=>{try{const date=new Date();utimesSync(file,date,date);}catch{}},policy.heartbeatMilliseconds);beat.unref?.();
 let poll=policy.pollMilliseconds,reclaimed=false,lastReclaim=-Infinity,lastProgress=-Infinity,monitor:ReturnType<typeof setInterval>|undefined,admittedMs=0;
 try{
  for(;;){
   options.signal?.throwIfAborted();
   const decided=await withClaim(options.root,()=>{
    const volume=measure(options.root,options.capacityBytes),at=now(),decision=decideHeadroomV1({freeBytes:volume.freeBytes,volumeBytes:volume.volumeBytes,nowMs:at,request:{id,reservedBytes:estimate.reservedBytes,arrivedMs},rows:readRows(directory,Date.now()),reclaimed});
    if(decision.verdict==="admit"){admittedMs=at;write("running",at);}
    return{decision,volume,at};
   },sleep,Infinity,options.signal);
   const {decision,volume,at}=decided!,facts={need:gib(estimate.reservedBytes),free:gib(volume.freeBytes),held:gib(decision.heldBytes),running:decision.running,ahead:gib(decision.aheadBytes),floor:gib(decision.floorBytes),seconds:Math.round((at-arrivedMs)/1000)};
   if(decision.verdict==="admit"){
    if(decision.reason==="alone")options.report(headroomTextV1("alone",facts,options.language));else if(lastProgress>-Infinity)options.report(headroomTextV1("admitted",facts,options.language));
    break;
   }
   if(at-lastProgress>=policy.progressMilliseconds){lastProgress=at;options.report(headroomTextV1("waiting",facts,options.language));}
   if(decision.reason==="space"&&at-lastReclaim>=policy.reclaimIntervalMilliseconds){
    if(lastReclaim===-Infinity)options.report(headroomTextV1("reclaiming",{shortfall:gib(decision.shortfallBytes)},options.language));
    lastReclaim=at;try{if(await reclaim(decision.shortfallBytes))reclaimed=true;}catch(error){if(options.signal?.aborted)throw error;}
    continue;
   }
   await sleep(poll,options.signal);poll=Math.min(policy.pollMaximumMilliseconds,Math.ceil(poll*policy.pollBackoffBasisPoints/10000));
  }
 }catch(error){clearInterval(beat);rmSync(file,{force:true});throw error;}
 let inFlight=false;
 monitor=setInterval(()=>{
  if(inFlight)return;
  try{
   const volume=measure(options.root,options.capacityBytes),floor=headroomFloorBytesV1(volume.volumeBytes);
   if(!volume.unmeasured&&volume.freeBytes<floor){inFlight=true;void reclaim(2*floor-volume.freeBytes).catch(()=>false).finally(()=>{inFlight=false;});}
  }catch{}
 },policy.monitorMilliseconds);monitor.unref?.();
 let released=false;
 return Object.freeze({id,reservedBytes:estimate.reservedBytes,waitedMs:admittedMs-arrivedMs,async release(grown?:()=>number){
  if(released)return;released=true;clearInterval(beat);clearInterval(monitor);rmSync(file,{force:true});
  if(grown===undefined||options.signal?.aborted)return;
  try{
   const sample:HeadroomSampleV1={bytes:clamp(Math.floor(grown()),0,549755813888),wallMs:clamp(now()-admittedMs,0,86400000),atMs:now()};
   await withClaim(options.root,()=>roomy(options.root,()=>writeAtomic(historyPath(directory),JSON.stringify(recordHeadroomSampleV1(readHeadroomHistoryV1(directory),names,sample)))),sleep,policy.claimAttempts);
  }catch{}
 }});
}
