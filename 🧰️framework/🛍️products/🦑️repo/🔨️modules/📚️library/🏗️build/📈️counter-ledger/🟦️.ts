import{appendFileSync,mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync}from"node:fs";
import{basename,dirname,join}from"node:path";
import{processPidExistsV1}from"../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The authored numbers of the counter ledger: the flush interval, the age beyond which a temporary file is retired and the names of the files.
 * @see ./🧬️schema/🔣️.json */
export const COUNTER_LEDGER_POLICY_V1=schema.definitions.Policy.const;
const P=COUNTER_LEDGER_POLICY_V1,NAME=new RegExp(schema.definitions.Name.pattern,"u"),TEMPORARY=new RegExp(P.temporaryPattern,"u");

/** 🧮️ Named counts in named groups, for example `{ac:{hits:3},cas:{hits:1}}`. */
export type CounterGroupsV1=Readonly<Record<string,Readonly<Record<string,number>>>>;
/** 📒️ The ledger file (schema definition `Ledger`): the counts of every run up to the last flush. */
export type CounterLedgerDocumentV1=Readonly<{version:1;name:string;sinceMs:number;updatedMs:number;restarts:number;groups:CounterGroupsV1}>;

const whole=(value:unknown):number=>typeof value==="number"&&Number.isFinite(value)&&value>0?Math.floor(value):0;
const isCount=(value:unknown):value is number=>typeof value==="number"&&Number.isSafeInteger(value)&&value>=0;
const isRecord=(value:unknown):value is Record<string,unknown>=>typeof value==="object"&&value!==null&&!Array.isArray(value);

/** 📖️ Reads a ledger file's text into its document; `undefined` for anything that is not of the closed shape of the schema (a restarting process then counts from zero instead of failing). */
export function parseCounterLedgerV1(text:string):CounterLedgerDocumentV1|undefined{
 try{
  const value=JSON.parse(text) as unknown;
  if(!isRecord(value)||Object.keys(value).some(key=>!["version","name","sinceMs","updatedMs","restarts","groups"].includes(key)))return undefined;
  if(value.version!==1||typeof value.name!=="string"||!NAME.test(value.name)||!isCount(value.sinceMs)||!isCount(value.updatedMs)||!isCount(value.restarts)||value.restarts<1||!isRecord(value.groups))return undefined;
  const groups:Record<string,Record<string,number>>={};
  for(const [group,counts] of Object.entries(value.groups)){
   if(!NAME.test(group)||!isRecord(counts))return undefined;
   const row:Record<string,number>={};
   for(const [name,count] of Object.entries(counts)){if(!NAME.test(name)||!isCount(count))return undefined;row[name]=count;}
   groups[group]=row;
  }
  return Object.freeze({version:1,name:value.name,sinceMs:value.sinceMs,updatedMs:value.updatedMs,restarts:value.restarts,groups});
 }catch{return undefined;}
}

/** ➕️ The sum of the counts of two group sets (names of either side kept except those the schema does not allow, which would make the whole file unreadable, a count below zero or not a number counts as zero, fractions are dropped), with the names in order so equal counts give equal text. */
export function addCounterGroupsV1(base:CounterGroupsV1,run:CounterGroupsV1):CounterGroupsV1{
 const sum:Record<string,Record<string,number>>={};
 for(const group of [...new Set([...Object.keys(base),...Object.keys(run)])].filter(name=>NAME.test(name)).sort()){
  const row:Record<string,number>={};
  for(const name of [...new Set([...Object.keys(base[group]??{}),...Object.keys(run[group]??{})])].filter(name=>NAME.test(name)).sort())row[name]=whole(base[group]?.[name])+whole(run[group]?.[name]);
  sum[group]=row;
 }
 return sum;
}

/** 🧹️ Removes the temporary files `<file name>.<pid>.tmp` beside `path` that a writer that died (or whose rename failed) left behind: those of a process that is gone and those older than the age the schema bounds, never the caller's own. Returns the names removed; never throws. */
export function retireCounterTemporariesV1(path:string,now:number=Date.now(),alive:(pid:number)=>boolean=processPidExistsV1):readonly string[]{
 const removed:string[]=[],stem=basename(path),directory=dirname(path);
 try{
  for(const name of readdirSync(directory)){
   const found=TEMPORARY.exec(name);if(found===null||found[1]!==stem)continue;
   const pid=Number(found[2]);if(pid===process.pid)continue;
   try{
    const file=join(directory,name);
    if(!alive(pid)||now-statSync(file).mtimeMs>P.temporaryMaximumAgeMilliseconds){rmSync(file,{force:true});removed.push(name);}
   }catch{}
  }
 }catch{}
 return removed;
}

/** 📒️ A ledger bound to its file: `total()` is the cumulative count, `flush()` writes it. */
export interface CounterLedgerV1{
 readonly path:string;
 readonly sinceMs:number;
 readonly restarts:number;
 bind(read:()=>CounterGroupsV1):void;
 total():CounterGroupsV1;
 flush(force?:boolean):void;
 close():void;
}

/** 📒️ Opens the ledger at `path` (a missing or broken file is an empty ledger), counts the open as a restart and writes the file at once, then again at most every `flushMilliseconds` while a count changed; `bind` names where the counts of this run are read from. The cumulative count never falls below the ledger, whatever the run reports. Never throws: a file that cannot be written costs the persistence and nothing else.
 * @see ./🧪️tests/🟦️.ts */
export function openCounterLedgerV1(input:Readonly<{path:string;name:string;now?:()=>number;flushMilliseconds?:number}>):CounterLedgerV1{
 if(!NAME.test(input.name))throw Error("Counter ledger name required");
 const now=input.now??Date.now;
 let previous:CounterLedgerDocumentV1|undefined;
 try{previous=parseCounterLedgerV1(readFileSync(input.path,"utf8"));}catch{}
 retireCounterTemporariesV1(input.path,now());
 const baseline:CounterGroupsV1=previous?.groups??{},sinceMs=previous?.sinceMs??now(),restarts=(previous?.restarts??0)+1;
 let read:()=>CounterGroupsV1=()=>({}),written="";
 const total=():CounterGroupsV1=>{let run:CounterGroupsV1={};try{run=read();}catch{}return addCounterGroupsV1(baseline,run);};
 const flush=(force=false):void=>{
  const groups=total(),text=JSON.stringify(groups);
  if(!force&&text===written)return;
  const temporary=input.path+"."+process.pid+".tmp",document:CounterLedgerDocumentV1={version:1,name:input.name,sinceMs,updatedMs:now(),restarts,groups};
  try{mkdirSync(dirname(input.path),{recursive:true});writeFileSync(temporary,JSON.stringify(document)+"\n");renameSync(temporary,input.path);written=text;}
  catch{try{rmSync(temporary,{force:true});}catch{}}
 };
 const timer=setInterval(()=>flush(),input.flushMilliseconds??P.flushMilliseconds);timer.unref?.();
 flush(true);
 return Object.freeze({path:input.path,sinceMs,restarts,bind(source:()=>CounterGroupsV1){read=source;},total,flush,close(){clearInterval(timer);flush(true);}});
}

/** 🔢️ Counts one event of `directory`: a single byte appended to the file of the event, which POSIX makes atomic across any number of processes, so the hot path takes no lock and loses nothing; the first count also writes the `since` time. Never throws; a name that is no event name is ignored. */
export function countEventV1(directory:string|undefined,event:string,now:()=>number=Date.now):void{
 if(!directory||!NAME.test(event))return;
 try{
  try{appendFileSync(join(directory,event),"1");}catch{mkdirSync(directory,{recursive:true});appendFileSync(join(directory,event),"1");}
  try{statSync(join(directory,P.sinceFile));}catch{writeFileSync(join(directory,P.sinceFile),String(now()),{flag:"wx"});}
 }catch{}
}

/** 📖️ The counts of the `events` of `directory` (the size of each file; 0 when the file is missing) and the time of the first count in milliseconds, `undefined` before it. */
export function readEventCountsV1(directory:string,events:readonly string[]):Readonly<{counts:Readonly<Record<string,number>>;sinceMs?:number}>{
 const counts:Record<string,number>={};
 for(const event of events){let size=0;try{size=statSync(join(directory,event)).size;}catch{}counts[event]=size;}
 let sinceMs:number|undefined;try{const value=Number(readFileSync(join(directory,P.sinceFile),"utf8"));if(isCount(value))sinceMs=value;}catch{}
 return Object.freeze({counts,...sinceMs===undefined?{}:{sinceMs}});
}
