import snapshotSchema from "../🧬️schema/📸️snapshot.json" with {type:"json"};
import {lstat,readdir} from "node:fs/promises";
import type {BigIntStats} from "node:fs";
import {join} from "node:path";
import {churnLanguagesV1,churnProtocolV1,concurrentEditV1,retryOnConcurrentEditV1,type ChurnOptionsV1,type ConcurrentEditReasonV1} from "../🟦️.ts";

const protocol=Object.freeze(JSON.parse(JSON.stringify(snapshotSchema.definitions.Protocol.const)) as Readonly<{version:1;concurrency:number;maximumEntries:number;maximumDepth:number;texts:Readonly<Record<string,Readonly<{settle:string}>>>}>);

/** 📸️ The closed numbers and EN/DE texts of the stable snapshot, exactly as the schema declares them. */
export function stableTreeProtocolV1():typeof protocol{return protocol;}
/** 📣️ One announced wait: the entry that was edited last, how long ago, and how long the snapshot waits, rendered in every selected language. */
export type StableTreeEventV1=Readonly<{phase:"settle";path:string;ageMs:number;delayMs:number;text:string}>;
/** 🎛️ The ports of one observation: the clock, the settle window (and the wait for it, which is real time even when the retry pauses of the churn policy are replaced) and the walk concurrency, the progress report, and the cancellation and retry ports of the churn policy. */
export type StableTreeOptionsV1=ChurnOptionsV1&Readonly<{settleMilliseconds?:number;concurrency?:number;now?:()=>number;include?:(path:string)=>boolean;onSettle?:(event:StableTreeEventV1)=>void;settleSleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}>;
/** 🌳️ A consistent snapshot: the value consumed from every file, the directories that were listed and how many attempts it took. */
export type StableTreeV1<T>=Readonly<{files:ReadonlyMap<string,T>;directories:readonly string[];attempts:number}>;

type EntryV1=Readonly<{stamp:string;identity:string;newestMs:number}>;
type WalkV1=Readonly<{files:Map<string,EntryV1>;directories:Map<string,Readonly<EntryV1&{names:string}>>;newestMs:number;newestPath:string}>;
const stampOf=(row:BigIntStats):EntryV1=>({stamp:[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":"),identity:row.dev+":"+row.ino,newestMs:Number((row.mtimeNs>row.ctimeNs?row.mtimeNs:row.ctimeNs)/1000000n)});

async function walk(roots:readonly string[],options:StableTreeOptionsV1,concurrency:number):Promise<WalkV1>{
 const files=new Map<string,EntryV1>(),directories=new Map<string,EntryV1&{names:string}>();
 let newestMs=0,newestPath="",level=roots.map(root=>({path:root,depth:0}));
 const note=(path:string,entry:EntryV1):void=>{if(entry.newestMs>=newestMs){newestMs=entry.newestMs;newestPath=path;}if(files.size+directories.size>protocol.maximumEntries)throw Error("Bounded stable tree entry count required");};
 while(level.length){
  options.signal?.throwIfAborted();
  const next:{path:string;depth:number}[]=[];
  for(let at=0;at<level.length;at+=concurrency)await Promise.all(level.slice(at,at+concurrency).map(async({path,depth})=>{
   const row=await lstat(path,{bigint:true});
   if(!row.isDirectory()){const entry=stampOf(row);files.set(path,entry);note(path,entry);return;}
   if(depth>=protocol.maximumDepth)throw Error("Bounded stable tree depth required");
   const names=(await readdir(path)).sort(),entry={...stampOf(row),names:names.join("\0")};directories.set(path,entry);note(path,entry);
   for(const name of names){const child=join(path,name);if(options.include?.(child)!==false)next.push({path:child,depth:depth+1});}
  }));
  level=next;
 }
 return{files,directories,newestMs,newestPath};
}

function differ(left:WalkV1,right:WalkV1):void{
 for(const[path,entry]of left.files){const other=right.files.get(path);if(other===undefined)throw concurrentEditV1("vanished",path,undefined,"snapshot");if(other.stamp!==entry.stamp)throw concurrentEditV1(other.identity!==entry.identity?"identity":"bytes",path,undefined,"snapshot");}
 for(const path of right.files.keys())if(!left.files.has(path))throw concurrentEditV1("appeared",path,undefined,"snapshot");
 for(const[path,entry]of left.directories){const other=right.directories.get(path);if(other===undefined)throw concurrentEditV1("vanished",path,undefined,"snapshot");if(other.stamp!==entry.stamp||other.names!==entry.names)throw concurrentEditV1("roster",path,undefined,"snapshot");}
 for(const path of right.directories.keys())if(!left.directories.has(path))throw concurrentEditV1("appeared",path,undefined,"snapshot");
}

/** 💬️ The notice that a snapshot waits for an entry edited a moment ago, in every selected language. */
export function stableTreeSettleTextV1(path:string,ageMs:number,delayMs:number,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 return churnLanguagesV1(environment).map(language=>protocol.texts[language]!.settle.replace(/\{([a-z]+)\}/gu,(_,name:string)=>String({path,age:ageMs,delay:delayMs}[name as "path"|"age"|"delay"]))).join("\n");
}

const pause=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((accept,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const timer=setTimeout(()=>{signal?.removeEventListener("abort",abort);accept();},milliseconds),abort=():void=>{clearTimeout(timer);reject(signal!.reason);};
 signal?.addEventListener("abort",abort,{once:true});
});

/** 🔒️ Observes the trees under `roots` as ONE state: lists them, consumes every file exactly once (a file whose identity did not change since an earlier attempt keeps its value), lists them again and requires both listings to agree to the byte of every stamp, and waits out entries edited within the settle window before a third listing confirms. A difference is a concurrent edit of the `snapshot` layer, so the whole observation runs again as the churn policy declares (bounded, EN/DE announced, cancellable); a raw filesystem error of a vanished entry is classified by the same policy. A run that consumes the returned values never mixes two tree states. */
export async function observeStableTreeV1<T>(roots:readonly string[],consume:(path:string)=>Promise<T>,options:StableTreeOptionsV1={}):Promise<StableTreeV1<T>>{
 const settle=options.settleMilliseconds??churnProtocolV1().settleMilliseconds,concurrency=options.concurrency??protocol.concurrency,now=options.now??Date.now,environment=options.environment??process.env;
 const kept=new Map<string,Readonly<{stamp:string;settled:boolean;value:T}>>();
 return retryOnConcurrentEditV1("snapshot",async attempt=>{
  const first=await walk(roots,options,concurrency),values=new Map<string,T>(),paths=[...first.files.keys()].sort();
  for(let at=0;at<paths.length;at+=concurrency)await Promise.all(paths.slice(at,at+concurrency).map(async path=>{
   options.signal?.throwIfAborted();
   const entry=first.files.get(path)!,prior=kept.get(path);
   if(prior&&prior.stamp===entry.stamp&&prior.settled){values.set(path,prior.value);return;}
   const value=await consume(path);values.set(path,value);kept.set(path,{stamp:entry.stamp,settled:entry.newestMs<now()-settle,value});
  }));
  const second=await walk(roots,options,concurrency);differ(first,second);
  const newestMs=Math.max(first.newestMs,second.newestMs),wait=Math.min(settle,newestMs+settle-now());
  if(wait>0){
   const age=Math.max(0,now()-newestMs),path=second.newestMs>=first.newestMs?second.newestPath:first.newestPath;
   (options.onSettle??(event=>{try{process.stderr.write(event.text+"\n");}catch{}}))(Object.freeze({phase:"settle" as const,path,ageMs:age,delayMs:wait,text:stableTreeSettleTextV1(path,age,wait,environment)}));
   await(options.settleSleep??pause)(wait,options.signal);
   differ(second,await walk(roots,options,concurrency));
  }
  return Object.freeze({files:new Map(paths.map(path=>[path,values.get(path)!])),directories:Object.freeze([...first.directories.keys()].sort()),attempts:attempt});
 },options);
}
