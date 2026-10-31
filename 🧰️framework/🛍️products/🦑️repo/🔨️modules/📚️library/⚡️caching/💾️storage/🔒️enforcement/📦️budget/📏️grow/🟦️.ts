import {appendFileSync,closeSync,existsSync,fsyncSync,lstatSync,openSync,readFileSync,renameSync,rmSync,statSync,statfsSync,writeFileSync,writeSync} from "node:fs";
import {spawn} from "node:child_process";
import {basename,join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import imageSchema from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const,messages=schema.definitions.Messages.const as Readonly<Record<string,Readonly<Record<string,string>>>>;
/** 🗣️ The languages of the grow texts. */
export type GrowLanguageV1="en"|"de";
/** 🔤️ Renders one declared grow text; `{name}` placeholders take the given values. */
export function growTextV1(language:GrowLanguageV1,key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const text=messages[language]?.[key];if(text===undefined)throw Error("Declared grow text required: "+key);
 return text.replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??"{"+name+"}"));
}

/** 🚫️ A refusal with its declared reason and the values its text names; nothing was changed when it is thrown before the first phase. */
export class GrowRefusalV1 extends Error{
 constructor(readonly reason:string,readonly variables:Readonly<Record<string,string|number>>={}){super(growTextV1("en","refused."+reason,variables)+"\n"+growTextV1("de","refused."+reason,variables));}
 /** 🏷️ The same refusal rendered for the named partition. */
 render(partition:string):string{const variables={...this.variables,partition};return growTextV1("en","refused."+this.reason,variables)+"\n"+growTextV1("de","refused."+this.reason,variables);}
}

/** 📐️ The bounds of one growth: what the policy declares for the partition, the elastic rule and the share of host free space it may take. */
export type GrowBoundsV1=Readonly<{currentBytes:number;declaredBytes:number;ruleMaximumBytes:number;hostFreeBytes:number;hostFreeReservePercent:number;requestedBytes?:number}>;
/** 🎯️ The size an image grows to, or the reason it cannot: the declared size and the rule maximum bound it, the host volume may give only `100 - reserve` percent of its free bytes, a whole number of MiB is kept. */
export function growTargetBytesV1(bounds:GrowBoundsV1):Readonly<{targetBytes:number;allowedBytes:number}>{
 const align=protocol.alignBytes,declared=Math.min(bounds.declaredBytes,bounds.ruleMaximumBytes),allowed=Math.floor(bounds.hostFreeBytes*(100-bounds.hostFreeReservePercent)/100),vars={from:bounds.currentBytes,declared:bounds.declaredBytes,maximum:bounds.ruleMaximumBytes,free:bounds.hostFreeBytes,allowed,reserve:bounds.hostFreeReservePercent,requested:bounds.requestedBytes??0};
 if(bounds.requestedBytes!==undefined){
  if(bounds.requestedBytes%align!==0)throw new GrowRefusalV1("misaligned",vars);
  if(bounds.requestedBytes>declared)throw new GrowRefusalV1("above-declared",vars);
  if(bounds.requestedBytes<=bounds.currentBytes)throw new GrowRefusalV1("nothing-to-grow",vars);
  if(bounds.requestedBytes-bounds.currentBytes>allowed)throw new GrowRefusalV1("host-free",vars);
  return Object.freeze({targetBytes:bounds.requestedBytes,allowedBytes:allowed});
 }
 if(declared<=bounds.currentBytes)throw new GrowRefusalV1("nothing-to-grow",vars);
 const target=Math.floor(Math.min(declared,bounds.currentBytes+allowed)/align)*align;
 if(target<=bounds.currentBytes)throw new GrowRefusalV1("host-free",vars);
 return Object.freeze({targetBytes:target,allowedBytes:allowed});
}

/** 🧾️ The receipt after a growth: size, filesystem bytes and the physical envelope follow the new size, everything else stays; a receipt that does not describe a fixed image of `fromBytes` is refused. */
export function growReceiptTextV1(text:string,fromBytes:number,toBytes:number):string{
 let value:Record<string,unknown>;try{value=JSON.parse(text) as Record<string,unknown>;}catch{throw new GrowRefusalV1("receipt",{partition:"",from:fromBytes,reason:"unreadable"});}
 const margin=protocol.physicalMarginBytes,grown=value["filesystemBytes"]===toBytes&&value["size"]===toBytes&&value["maximumPhysicalBytes"]===toBytes+margin;
 if(grown)return text;
 if(value["filesystemBytes"]!==fromBytes||value["size"]!==fromBytes||value["maximumPhysicalBytes"]!==fromBytes+margin)throw new GrowRefusalV1("receipt",{partition:"",from:fromBytes,reason:"sizes differ"});
 return JSON.stringify({...value,size:toBytes,filesystemBytes:toBytes,maximumPhysicalBytes:toBytes+margin});
}

/** 🪪️ A parent identity `[dev,ino,size,device]` with the new size and mount device. */
function identityText(text:string,from:Readonly<{size:number;device:number|null}>,to:Readonly<{size:number;device:number|null}>):string{
 let row:unknown;try{row=JSON.parse(text);}catch{return text;}
 if(!Array.isArray(row)||row.length<4)return text;
 const next=[...row];if(next[2]===from.size)next[2]=to.size;if(from.device!==null&&to.device!==null&&next[3]===from.device)next[3]=to.device;
 return JSON.stringify(next);
}
function restampValue(value:unknown,from:Readonly<{size:number;device:number|null}>,to:Readonly<{size:number;device:number|null}>,key:string):unknown{
 if(Array.isArray(value))return value.map(row=>restampValue(row,from,to,""));
 if(value!==null&&typeof value==="object")return Object.fromEntries(Object.entries(value as Record<string,unknown>).map(([name,row])=>[name,restampValue(row,from,to,name)]));
 if(key==="parentIdentity"&&typeof value==="string")return identityText(value,from,to);
 if(key==="dev"&&from.device!==null&&to.device!==null){if(value===from.device)return to.device;if(value===String(from.device))return String(to.device);}
 return value;
}
/** 📜️ The storage journal after a growth: every `parentIdentity` carries the new backing size and mount device and every `dev` of the old mount device names the new one; lines that name neither stay byte for byte. */
export function restampJournalTextV1(text:string,from:Readonly<{size:number;device:number|null}>,to:Readonly<{size:number;device:number|null}>):string{
 return text.split("\n").map(line=>{if(!line.trim())return line;try{const next=JSON.stringify(restampValue(JSON.parse(line),from,to,""));return next===JSON.stringify(JSON.parse(line))?line:next;}catch{return line;}}).join("\n");
}
/** 📎️ The receipt of a nested image after the mount device of its parent changed: the `dev` fields of the old device name the new one. */
export function restampChildReceiptTextV1(text:string,fromDevice:number|null,toDevice:number|null):string{
 if(fromDevice===null||toDevice===null||fromDevice===toDevice)return text;
 let value:unknown;try{value=JSON.parse(text);}catch{return text;}
 const next=JSON.stringify(restampValue(value,{size:-1,device:fromDevice},{size:-1,device:toDevice},""));return next===JSON.stringify(value)?text:next;
}
/** 📂️ The directories of the nested images the journal names, whose receipts follow the mount device of their parent. */
export function childDirectoriesV1(journalText:string):readonly string[]{
 const found=new Set<string>();
 for(const line of journalText.split("\n")){if(!line.trim())continue;try{const row=JSON.parse(line) as {allocation?:{child?:{directory?:unknown}}};const directory=row.allocation?.child?.directory;if(typeof directory==="string")found.add(directory);}catch{}}
 return Object.freeze([...found].sort());
}

/** 🖥️ One attached image of the kernel roster. */
export type GrowAttachedV1=Readonly<{imagePath:string;mountPoints:readonly string[];device:string}>;
/** 🔌️ The system tools the growth needs; production binds them to the closed programs of the schema, tests to a model of the host. */
export type GrowToolsV1=Readonly<{
 roster():readonly GrowAttachedV1[]|Promise<readonly GrowAttachedV1[]>;
 holders(mount:string):readonly string[]|Promise<readonly string[]>;
 stopCollectors?(mount:string):readonly number[]|Promise<readonly number[]>;
 detach(row:GrowAttachedV1):void|Promise<void>;
 /** The processes (id and full executable path) that dissent from a plain unmount of the row. */
 dissenters?(row:GrowAttachedV1):readonly Readonly<{pid:number;path:string}>[]|Promise<readonly Readonly<{pid:number;path:string}>[]>;
 signedBySystem?(path:string):boolean|Promise<boolean>;
 forceUnmount?(row:GrowAttachedV1):void|Promise<void>;
 resize(image:string,bytes:number):void|Promise<void>;
 attach(image:string,mount:string):void|Promise<void>;
 deviceOf(path:string):number|Promise<number>;
 volumeBytes(mount:string):number|Promise<number>;
}>;
/** 🔌️ Everything outside the files of the partition. `checkpoint` is called before and after every effect so a test can end the run there. */
export type GrowPortsV1=Readonly<{
 lease():Promise<Readonly<{release():void}>>;
 tools:GrowToolsV1;
 report?(line:string):void;
 checkpoint?(name:string):void;
 signal?:AbortSignal;
 sleep?(milliseconds:number):Promise<void>;
 now?():number;
}>;
/** 📓️ What a run did. */
export type GrowOutcomeV1=Readonly<{partition:string;fromBytes:number;toBytes:number;resumed:boolean;oldDevice:number|null;newDevice:number|null;forcedUnmount?:readonly string[]}>;
/** 🧭️ The request: the partition directory (receipt, backing file and mount directory), its id and the target. */
export type GrowRequestV1=Readonly<{partition:string;directory:string;targetBytes:number;language?:GrowLanguageV1;stopCollectors?:boolean}>;
export type GrowJournalV1=Readonly<{version:1;phase:typeof protocol.phases[number];partition:string;fromBytes:number;toBytes:number;oldDevice:number|null;newDevice:number|null;forcedUnmount?:readonly string[]}>;

function writeAtomic(path:string,text:string):void{
 const temporary=path+".tmp",descriptor=openSync(temporary,"w",0o600);
 try{writeSync(descriptor,text);fsyncSync(descriptor);}finally{closeSync(descriptor);}
 renameSync(temporary,path);
}
/** 📖️ The journal of an interrupted growth, or undefined when none exists; an unreadable journal is refused, never guessed. */
export function readGrowJournalV1(directory:string):GrowJournalV1|undefined{
 const path=join(directory,protocol.journalFile);if(!existsSync(path))return undefined;
 const size=lstatSync(path).size;if(size>protocol.maximumJournalBytes)throw Error("Bounded grow journal required");
 const value=JSON.parse(readFileSync(path,"utf8")) as GrowJournalV1;
 if(value.version!==1||!protocol.phases.includes(value.phase)||!Number.isSafeInteger(value.fromBytes)||!Number.isSafeInteger(value.toBytes))throw Error("Closed grow journal required");
 return value;
}
const phaseIndex=(phase:string)=>protocol.phases.indexOf(phase as never);

/** 🌱️ Grows the image of one partition offline and returns what it did. The run holds the exclusive image lease for its whole length, so new commands queue behind it and running ones finish first. Every phase is journalled beside the receipt before and after its effect and every effect checks the real state first, so a run killed anywhere is resumed by calling this function again: the journal fixes the target. */
export async function growPartitionV1(request:GrowRequestV1,ports:GrowPortsV1):Promise<GrowOutcomeV1>{
 const language=request.language??"en",{directory,partition}=request,receiptPath=join(directory,protocol.receiptFile),image=join(directory,protocol.imageFile),mount=join(directory,protocol.mountDirectory),eventsPath=join(mount,protocol.eventsFile),journalPath=join(directory,protocol.journalFile);
 const say=(key:string,variables:Readonly<Record<string,string|number>>={})=>ports.report?.(growTextV1(language,key,{partition,...variables})),at=(name:string)=>{ports.signal?.throwIfAborted();ports.checkpoint?.(name);};
 if(!existsSync(receiptPath)||!existsSync(image))throw new GrowRefusalV1("absent",{partition,directory});
 say("step.waiting");at("lease:before");
 const lease=await ports.lease();
 try{
  at("lease:after");
  const existing=readGrowJournalV1(directory),resumed=existing!==undefined,receiptText=readFileSync(receiptPath,"utf8");
  if(resumed)say("resumed",{phase:existing.phase});
  let journal:GrowJournalV1;
  if(existing){journal=existing;}
  else{
   const receipt=JSON.parse(receiptText) as {filesystemBytes?:number},fromBytes=receipt.filesystemBytes;
   if(!Number.isSafeInteger(fromBytes))throw new GrowRefusalV1("receipt",{partition,from:0,reason:"no size"});
   const actual=statSync(image).size;if(actual!==fromBytes)throw new GrowRefusalV1("state",{partition,actual,from:fromBytes as number,to:request.targetBytes});
   growReceiptTextV1(receiptText,fromBytes as number,fromBytes as number);
   if(request.targetBytes<=(fromBytes as number))throw new GrowRefusalV1("nothing-to-grow",{partition,from:fromBytes as number});
   const attached=(await ports.tools.roster()).some(row=>row.imagePath===image);
   journal=Object.freeze({version:1 as const,phase:"planned" as const,partition,fromBytes:fromBytes as number,toBytes:request.targetBytes,oldDevice:attached?(await ports.tools.deviceOf(mount)):null,newDevice:null});
   at("journal:planned:before");writeAtomic(journalPath,JSON.stringify(journal));at("journal:planned:after");
  }
  const record=(phase:GrowJournalV1["phase"],extra:Partial<GrowJournalV1>={})=>{journal=Object.freeze({...journal,...extra,phase});writeAtomic(journalPath,JSON.stringify(journal));};
  const attachedRows=async()=>(await ports.tools.roster()).filter(row=>row.imagePath===image||row.imagePath.startsWith(mount+"/"));
  if(phaseIndex(journal.phase)<phaseIndex("detached")){
   const attached=(await attachedRows()).some(row=>row.imagePath===image);
   if(attached&&resumed){const device=(await ports.tools.deviceOf(mount));if(device!==journal.oldDevice)record(journal.phase,{oldDevice:device});}
   if(attached){
    const sleep=ports.sleep??((milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds))),clock=ports.now??Date.now,started=clock();
    if(request.stopCollectors!==false&&ports.tools.stopCollectors)for(const pid of await ports.tools.stopCollectors(mount))say("step.collector",{pid});
    let holders=(await ports.tools.holders(mount));
    while(holders.length&&clock()-started<protocol.holderGraceMilliseconds){say("step.holders",{holders:holders.length});await sleep(protocol.holderPollMilliseconds);ports.signal?.throwIfAborted();holders=(await ports.tools.holders(mount));}
    if(holders.length){if(!resumed)rmSync(journalPath,{force:true});throw new GrowRefusalV1("busy",{partition,holders:holders.slice(0,protocol.maximumHolderLines).join(", ")});}
   }
   at("drained:before");if(journal.phase==="planned")record("drained");at("drained:after");
  }
  if(phaseIndex(journal.phase)<phaseIndex("detached")){
   at("detach:before");
   const rows=(await attachedRows()).sort((left,right)=>right.imagePath.length-left.imagePath.length),failed:string[]=[];
   for(const row of rows){
    for(let attempt=1;attempt<=protocol.detachAttempts;attempt++){
     try{(await ports.tools.detach(row));break;}
     catch(error){if(attempt===protocol.detachAttempts){let open:readonly string[]=[],unknown=false;try{open=(await ports.tools.holders(mount));}catch{unknown=true;}
      const dissent=open.length||unknown||!ports.tools.dissenters||!ports.tools.forceUnmount||!ports.tools.signedBySystem?[]:(await ports.tools.dissenters(row)),allowed=protocol.forcedUnmountDissenters as readonly string[];
      if(dissent.length&&await everyAsyncV1(dissent,async entry=>allowed.includes(entry.path)&&await ports.tools.signedBySystem!(entry.path))){record(journal.phase,{forcedUnmount:[...(journal.forcedUnmount??[]),row.imagePath+" <- "+dissent.map(entry=>entry.pid+":"+entry.path).join(",")].slice(-protocol.maximumForcedUnmountRecords)});try{(await ports.tools.forceUnmount!(row));(await ports.tools.detach(row));break;}catch(forced){failed.push(row.imagePath+": forced "+String((forced as Error).message));continue;}}
      failed.push(row.imagePath+": "+String((error as Error).message)+(open.length?" [open: "+open.slice(0,protocol.maximumHolderLines).join("; ")+"]":""));}else await (ports.sleep??((milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds))))(protocol.detachRetryMilliseconds);}
    }
   }
   const left=(await attachedRows());if(left.length)throw new GrowRefusalV1("children",{partition,children:left.map(row=>row.imagePath).join(", ")+(failed.length?" ("+failed.join("; ")+")":"")});
   at("detach:after");record("detached");say("step.detached");
  }
  if(phaseIndex(journal.phase)<phaseIndex("resized")){
   const actual=statSync(image);
   if(actual.size!==journal.fromBytes&&actual.size!==journal.toBytes)throw new GrowRefusalV1("state",{partition,actual:actual.size,from:journal.fromBytes,to:journal.toBytes});
   at("resize:before");
   if(actual.size===journal.fromBytes){(await ports.tools.resize(image,journal.toBytes));const after=statSync(image);if(after.size!==journal.toBytes||after.ino!==actual.ino)throw new GrowRefusalV1("state",{partition,actual:after.size,from:journal.fromBytes,to:journal.toBytes});}
   at("resize:after");record("resized");say("step.resized",{to:journal.toBytes});
  }
  if(phaseIndex(journal.phase)<phaseIndex("attached")){
   at("attach:before");
   if(!(await attachedRows()).some(row=>row.imagePath===image))(await ports.tools.attach(image,mount));
   const device=(await ports.tools.deviceOf(mount)),volume=(await ports.tools.volumeBytes(mount));
   if(volume*10000<journal.toBytes*protocol.minimumVolumeShareBasisPoints)throw new GrowRefusalV1("state",{partition,actual:volume,from:journal.fromBytes,to:journal.toBytes});
   at("attach:after");record("attached",{newDevice:device});say("step.attached",{volume});
  }
  if(phaseIndex(journal.phase)<phaseIndex("restamped")){
   const from={size:journal.fromBytes,device:journal.oldDevice},to={size:journal.toBytes,device:journal.newDevice};
   let events="";if(existsSync(eventsPath)){
    events=readFileSync(eventsPath,"utf8");
    at("events:before");const next=restampJournalTextV1(events,from,to);if(next!==events)writeAtomic(eventsPath,next);at("events:after");
    for(const child of childDirectoriesV1(events)){
     const cut=child.indexOf("/"+protocol.mountDirectory+"/");if(cut<0)continue;
     const receipt=join(mount,child.slice(cut+protocol.mountDirectory.length+2),protocol.receiptFile);
     if(!existsSync(receipt))continue;
     const text=readFileSync(receipt,"utf8"),next=restampChildReceiptTextV1(text,journal.oldDevice,journal.newDevice);
     at("child:before");if(next!==text)writeAtomic(receipt,next);at("child:after");
    }
   }
   at("receipt:before");const grown=growReceiptTextV1(readFileSync(receiptPath,"utf8"),journal.fromBytes,journal.toBytes);if(grown!==readFileSync(receiptPath,"utf8"))writeAtomic(receiptPath,grown);at("receipt:after");
   record("restamped");say("step.restamped");
  }
  if(journal.forcedUnmount?.length)appendFileSync(join(directory,protocol.auditFile),JSON.stringify({version:1,partition,fromBytes:journal.fromBytes,toBytes:journal.toBytes,forcedUnmount:journal.forcedUnmount})+"\n");
  at("journal:clear:before");rmSync(journalPath,{force:true});rmSync(journalPath+".tmp",{force:true});at("journal:clear:after");
  say("done",{from:journal.fromBytes,to:journal.toBytes});
  return Object.freeze({partition,fromBytes:journal.fromBytes,toBytes:journal.toBytes,resumed,oldDevice:journal.oldDevice,newDevice:journal.newDevice,...journal.forcedUnmount?.length?{forcedUnmount:journal.forcedUnmount}:{}});
 }finally{lease.release();}
}

/** ✅️ Whether `check` holds for every row, asked in order and stopping at the first refusal. */
async function everyAsyncV1<T>(rows:readonly T[],check:(row:T)=>Promise<boolean>):Promise<boolean>{for(const row of rows)if(!await check(row))return false;return true;}

/** 🛠️ Runs one closed program of the schema and returns its status and text. */
function run(program:string,args:readonly string[],input?:string,timeout:number=protocol.toolTimeoutMilliseconds,signal?:AbortSignal):Promise<Readonly<{status:number|null;stdout:string;stderr:string}>>{
 return new Promise((resolve,reject)=>{
  if(signal?.aborted){reject(signal.reason);return;}
  const child=spawn(program,[...args],{stdio:["pipe","pipe","pipe"],env:{...process.env,LC_ALL:"C"}}),out:Buffer[]=[],err:Buffer[]=[];let size=0,settled=false;
  const finish=(action:()=>void)=>{if(settled)return;settled=true;clearTimeout(timer);signal?.removeEventListener("abort",abort);action();};
  const stop=(reason:unknown)=>{try{child.kill("SIGKILL");}catch{}finish(()=>reject(reason));};
  const abort=()=>stop(signal!.reason),timer=setTimeout(()=>stop(Error(program+" timed out after "+timeout+" ms")),timeout);
  signal?.addEventListener("abort",abort,{once:true});
  child.stdout.on("data",(chunk:Buffer)=>{size+=chunk.length;if(size>(1<<26))stop(Error(program+" output too large"));else out.push(chunk);});
  child.stderr.on("data",(chunk:Buffer)=>{err.push(chunk);});
  child.on("error",error=>finish(()=>reject(Error(program+": "+error.message))));
  child.on("close",status=>finish(()=>resolve(Object.freeze({status,stdout:Buffer.concat(out).toString("utf8"),stderr:Buffer.concat(err).toString("utf8")}))));
  child.stdin.on("error",()=>undefined);child.stdin.end(input??"");
 });
}
const fill=(args:readonly string[],values:Readonly<Record<string,string>>)=>args.map(value=>value.replace(/\{(\w+)\}/gu,(_,name:string)=>values[name]??"{"+name+"}"));
/** 🖥️ The production tools: the closed programs of the schema, the attach flags of the image protocol and the kernel roster read as JSON through `plutil`. */
export function realGrowToolsV1(signal?:AbortSignal):GrowToolsV1{
 const tools=protocol.tools,exec=(program:string,args:readonly string[],input?:string,timeout?:number)=>run(program,args,input,timeout??protocol.toolTimeoutMilliseconds,signal);
 return Object.freeze({
  roster:async ()=>{
   const info=await exec(tools.roster.program,tools.roster.arguments);if(info.status!==0)throw Error("hdiutil info failed: "+info.stderr);
   const json=await exec(tools.plist.program,tools.plist.arguments,info.stdout);if(json.status!==0)throw Error("plutil failed: "+json.stderr);
   const rows=(JSON.parse(json.stdout) as {images?:readonly Record<string,unknown>[]}).images??[];
   return rows.map(row=>{const entities=(row["system-entities"] as readonly Record<string,unknown>[]|undefined)??[];return Object.freeze({imagePath:String(row["image-path"]??""),mountPoints:Object.freeze(entities.map(entity=>entity["mount-point"]).filter((value):value is string=>typeof value==="string")),device:String(entities[0]?.["dev-entry"]??"")});});
  },
  holders:async mount=>{const result=await exec(tools.holders.program,fill(tools.holders.arguments,{mount}));if(result.status!==0&&result.status!==1)throw Error("lsof failed: "+(result.stderr.trim()||String(result.status)));return result.stdout.split("\n").slice(1).filter(Boolean);},
  stopCollectors:async mount=>{
   const result=await exec(tools.holders.program,["-Fp",...fill(tools.holders.arguments,{mount})]),stopped:number[]=[];
   for(const pid of new Set(result.stdout.split("\n").filter(line=>line.startsWith("p")).map(line=>Number(line.slice(1))))){
    const command=(await exec("/bin/ps",["-o","command=","-p",String(pid)])).stdout;
    if(protocol.collector.signature.every(word=>command.includes(word))&&command.includes(mount)){try{process.kill(pid,"SIGTERM");stopped.push(pid);}catch{}}
   }
   return stopped;
  },
  detach:async row=>{const result=await exec(tools.detach.program,fill(tools.detach.arguments,{target:row.device||row.mountPoints[0]||""}));if(result.status!==0)throw Error(result.stderr.trim()||"detach failed");},
  dissenters:async row=>{const result=await exec(tools.unmountProbe.program,fill(tools.unmountProbe.arguments,{target:row.mountPoints[0]||row.device}));return[...(result.stderr+result.stdout).matchAll(/dissented by PID (\d+) \(([^)]*)\)/gu)].map(match=>Object.freeze({pid:Number(match[1]),path:match[2]!}));},
  signedBySystem:async path=>(await exec(tools.signature.program,fill(tools.signature.arguments,{path}))).status===0,
  forceUnmount:async row=>{const result=await exec(tools.unmountForce.program,fill(tools.unmountForce.arguments,{target:row.mountPoints[0]||row.device}));if(result.status!==0)throw Error(result.stderr.trim()||"forced unmount failed");},
  resize:async (image,bytes)=>{const result=await exec(tools.resize.program,fill(tools.resize.arguments,{bytes:String(bytes),image}),undefined,protocol.resizeTimeoutMilliseconds);if(result.status!==0)throw Error(result.stderr.trim()||result.stdout.trim()||"resize failed");},
  attach:async (image,mount)=>{const result=await exec(tools.attach.program,[...tools.attach.arguments,...imageSchema.definitions.ImageProtocol.const.attachArguments,"-mountpoint",mount,image]);if(result.status!==0)throw Error(result.stderr.trim()||"attach failed");},
  deviceOf:path=>statSync(path).dev,
  volumeBytes:mount=>{const row=statfsSync(mount);return row.blocks*row.bsize;}
 });
}
/** 📍️ The partition directory a partition id names below a workspace, from the directory the budget policy declares. */
export function growPartitionDirectoryV1(workspace:string,declaredDirectory:string):string{return join(workspace,declaredDirectory);}
export const GROW_PARTITIONS_V1:Readonly<Record<string,Readonly<{elastic:string;strict:string}>>>=protocol.partitions;
export const GROW_LEASE_PREFIX_V1=protocol.leaseResourcePrefix;
