import{lstatSync,readdirSync}from"node:fs";
import{rm}from"node:fs/promises";
import{join,relative}from"node:path";
import{bindCatalogV1,handleTextV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import{formatDiskBytesV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/💽️disk/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const P=schema.definitions.Policy.const;

/** 🧹️ The policy of the sweep: which folders of the tickets, how long idle, when the volume is short. */
export const TICKET_GENERATED_POLICY_V1=P;
/** 🗣️ The bilingual catalog of the sweep. */
export const TICKET_GENERATED_TEXTS_V1=bindCatalogV1("ticket-generated",schema.definitions.Messages.const);
/** 🧾️ What the sweep found in one `🗑️generated` folder: removed, would be removed (dry run), still active, too big to judge within the budget, or not removable. */
export type TicketGeneratedFindingV1=Readonly<{path:string;files:number;bytes:number;state:"removed"|"would-remove"|"active"|"unknown"|"failed";idleHours:number;message?:string}>;
export type TicketGeneratedVolumeV1=Readonly<{freeBytes:number|null;totalBytes:number|null}>;

const NAMES=new Set(P.names.map(name=>name.normalize("NFC")));
const list=(path:string)=>{try{return readdirSync(path,{withFileTypes:true});}catch{return[];}};

/** 🌡️ Whether the volume is short: free below the larger of `pressureFreeBytes` and `pressureFreeBasisPoints` of its size, so a nearly full disk judges ticket output by the short idle limit. A volume that cannot be measured is not short. */
export function ticketGeneratedPressureV1(volume:TicketGeneratedVolumeV1|null|undefined):boolean{
 if(volume===null||volume===undefined||typeof volume.freeBytes!=="number")return false;
 const total=typeof volume.totalBytes==="number"?volume.totalBytes:0;
 return volume.freeBytes<Math.max(P.pressureFreeBytes,Math.floor(total*P.pressureFreeBasisPoints/10000));
}

/** 📂️ Every `🗑️generated` folder of a ticket below the tickets of `root` (`levels` folders down: year, month, day, ticket, the folder), links never followed, sorted. */
export function ticketGeneratedFoldersV1(root:string):readonly string[]{
 const base=join(root,...P.directory),found:string[]=[];
 const walk=(path:string,level:number):void=>{
  for(const entry of list(path)){
   if(!entry.isDirectory()||entry.isSymbolicLink())continue;
   const child=join(path,entry.name);
   if(level===P.levels){if(NAMES.has(entry.name.normalize("NFC")))found.push(child);}
   else walk(child,level+1);
  }
 };
 walk(base,1);
 return Object.freeze(found.sort());
}

/** 🕰️ Measures one folder: the newest modification time of any entry and its files and bytes, stopping at the first entry newer than `activeAfterMs` (a lane still writes there) or when the budget or the signal ends the walk (`complete` false). A pure read.
 * @see ./🧪️tests/🟦️.ts */
export async function ticketGeneratedIdleV1(folder:string,activeAfterMs:number,options:Readonly<{budget:number;signal?:AbortSignal;yieldEvery?:number}>):Promise<Readonly<{active:boolean;complete:boolean;newestMs:number;files:number;bytes:number}>>{
 let budget=options.budget,files=0,bytes=0,newest=0,counted=0;
 const every=options.yieldEvery??P.yieldEvery,stack=[folder];
 const result=(active:boolean,complete:boolean)=>Object.freeze({active,complete,newestMs:newest,files,bytes});
 try{const own=lstatSync(folder);newest=own.mtimeMs;if(own.mtimeMs>activeAfterMs)return result(true,true);}catch{return result(false,false);}
 while(stack.length>0){
  const path=stack.pop()!;
  for(const entry of list(path)){
   if(budget--<=0||options.signal?.aborted===true)return result(false,false);
   if(++counted%every===0)await new Promise<void>(done=>setImmediate(done));
   const full=join(path,entry.name);
   let stats;try{stats=lstatSync(full);}catch{continue;}
   newest=Math.max(newest,stats.mtimeMs);
   if(stats.mtimeMs>activeAfterMs)return result(true,true);
   if(stats.isDirectory())stack.push(full);else{files++;bytes+=stats.size;}
  }
 }
 return result(false,true);
}

/** 🧹️ Removes every `🗑️generated` folder of a ticket in which nothing has been written for `idleMilliseconds` (the short `pressureIdleMilliseconds` while the volume is short). An open ticket is not spared: its output is regenerable by definition and a lane that still works keeps writing, which keeps its folder. The folder is judged by its newest entry, a folder too big for the entry budget is kept as `unknown`, a failing removal never stops the next, and the signal cancels between folders. `dryRun` only reports. `.gitignore` keeps these folders out of the shared worktree, so removing one deletes nothing on any other machine.
 * @see ./🧪️tests/🟦️.ts */
export async function sweepTicketGeneratedV1(root:string,options:Readonly<{now?:()=>number;volume?:()=>TicketGeneratedVolumeV1|null;dryRun?:boolean;signal?:AbortSignal;entryBudget?:number;yieldEvery?:number;idleMilliseconds?:number;pressureIdleMilliseconds?:number;language?:"en"|"de"|"both";report?(line:string):void;remove?(path:string):Promise<void>}>={}):Promise<readonly TicketGeneratedFindingV1[]>{
 const now=options.now??Date.now,volume=options.volume?.()??null,pressure=ticketGeneratedPressureV1(volume),idle=pressure?options.pressureIdleMilliseconds??P.pressureIdleMilliseconds:options.idleMilliseconds??P.idleMilliseconds;
 const language=options.language??"both",free=pressure?formatDiskBytesV1(volume!.freeBytes as number):"",erase=options.remove??((path:string)=>rm(path,{recursive:true,force:true,maxRetries:2}));
 const say=(lang:"en"|"de",key:string,values:Record<string,string|number>)=>handleTextV1(TICKET_GENERATED_TEXTS_V1,key,lang,{...values,why:handleTextV1(TICKET_GENERATED_TEXTS_V1,pressure?"sweep.pressure":"sweep.age",lang,{free})});
 const text=(key:string,values:Record<string,string|number>)=>language==="both"?say("en",key,values)+" / "+say("de",key,values):say(language,key,values);
 let budget=options.entryBudget??P.entryBudget;
 const findings:TicketGeneratedFindingV1[]=[];
 for(const folder of ticketGeneratedFoldersV1(root)){
  options.signal?.throwIfAborted();
  const at=now(),measured=await ticketGeneratedIdleV1(folder,at-idle,{budget,signal:options.signal,...options.yieldEvery===undefined?{}:{yieldEvery:options.yieldEvery}});
  budget=Math.max(0,budget-measured.files);
  const path=relative(root,folder),idleHours=Math.floor(Math.max(0,at-measured.newestMs)/3600000),row={path,files:measured.files,bytes:measured.bytes,idleHours};
  if(measured.active){findings.push({...row,state:"active"});continue;}
  if(!measured.complete){findings.push({...row,state:"unknown"});continue;}
  const values={path,files:measured.files,bytes:formatDiskBytesV1(measured.bytes),hours:idleHours};
  if(options.dryRun===true){findings.push({...row,state:"would-remove"});options.report?.(text("sweep.would-remove",values));continue;}
  try{await erase(folder);findings.push({...row,state:"removed"});options.report?.(text("sweep.removed",values));}
  catch(error){const message=String((error as Error)?.message??error).slice(0,200);findings.push({...row,state:"failed",message});options.report?.(text("sweep.failed",{path,message}));}
 }
 return Object.freeze(findings);
}

/** 📮️ Runs the sweep in the background of a worker, the first time `firstDelayMilliseconds` after the start and then every `intervalMilliseconds`, at most one at a time; a failing sweep is dropped and the next interval tries again. A machine whose volume is short sweeps with the short idle limit.
 * @see ./🧪️tests/🟦️.ts */
export function createTicketGeneratedSweepV1(input:Readonly<{root:string;now?:()=>number;volume?:()=>TicketGeneratedVolumeV1|null;report?(line:string):void;signal?:AbortSignal;sweep?:typeof sweepTicketGeneratedV1}>):Readonly<{tick():Promise<boolean>}>{
 const now=input.now??Date.now,run=input.sweep??sweepTicketGeneratedV1,startedAt=now();
 let running=false,lastAt:number|undefined;
 return Object.freeze({async tick():Promise<boolean>{
  if(running)return false;
  const at=now();
  if(lastAt===undefined?at-startedAt<P.firstDelayMilliseconds:at-lastAt<P.intervalMilliseconds)return false;
  running=true;
  try{await run(input.root,{now,...input.volume===undefined?{}:{volume:input.volume},...input.report===undefined?{}:{report:input.report},...input.signal===undefined?{}:{signal:input.signal}});lastAt=now();return true;}
  catch{lastAt=now();return false;}finally{running=false;}
 }});
}
