import {mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {appendFile,rename,rm,writeFile} from "node:fs/promises";
import {dirname} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const;

/** 📒️ One claim as the journal keeps it: the table address, the owner, when it was first granted, until when it holds and its lease length. */
export type ClaimRowV1=Readonly<{address:string;owner:string;grantedMs:number;untilMs:number;ttlMs:number;expectedMs?:number}>;
/** 🧊️ One lifetime-cap cool-off: the owner may not claim the address again until `untilMs`. */
export type ClaimCoolingV1=Readonly<{address:string;owner:string;untilMs:number}>;
/** 🧾️ One journal event of the claim table. */
export type ClaimEventV1=Readonly<{t:"set"}&ClaimRowV1>|Readonly<{t:"drop";address:string}>|Readonly<{t:"cool"}&ClaimCoolingV1>;
/** 📸️ The whole state of a claim table. */
export type ClaimSnapshotV1=Readonly<{claims:readonly ClaimRowV1[];cooling:readonly ClaimCoolingV1[]}>;

/** 💽️ The journal of one claim table: what a restarted server restores and the sink the table writes to. */
export type ClaimJournalV1=Readonly<{restored:ClaimSnapshotV1;record(event:ClaimEventV1):void;bind(snapshot:()=>ClaimSnapshotV1):void;flush():Promise<void>}>;

const line=(event:ClaimEventV1,at:number):string=>JSON.stringify({at,...event})+"\n",SLICE=2000;

/** 💽️ Opens (and compacts) the claim journal `file`: every `set` and `drop` the table made is replayed, so a claim that was alive when the server stopped (killed included; a torn last line is ignored) is restored with `restoreGraceMilliseconds` more to let its holder reconnect and renew, while a claim that had already lapsed, or a cool-off that is over, is dropped. A change is queued in memory and written off the event loop: every change of one `flushMilliseconds` window (a lapse sweep of thousands of claims included) is one append, so the table never waits for the disk; after `journalCompactEvents` changes the file is rewritten from the live table; `flush` waits for what is queued: a stop, a test and the route that grants a claim (the holder hears "granted" only once the grant is on disk, so a killed server still remembers it), while a crash loses at most the last window of renewals and lapses, which the restore grace covers. Any I/O fault disables the journal and never the table: a restart then only costs duplicate work, as before. */
export function openClaimJournalV1(file:string,options:Readonly<{now?:()=>number;graceMilliseconds?:number;compactEvents?:number;flushMilliseconds?:number}>={}):ClaimJournalV1{
 const now=options.now??Date.now,grace=options.graceMilliseconds??policy.restoreGraceMilliseconds,limit=options.compactEvents??policy.journalCompactEvents,window=options.flushMilliseconds??policy.journalFlushMilliseconds;
 const claims=new Map<string,ClaimRowV1>(),cooling=new Map<string,ClaimCoolingV1>();
 let last=0,events=0,broken=false,snapshot:(()=>ClaimSnapshotV1)|undefined;
 try{
  for(const text of readFileSync(file,"utf8").split("\n")){
   if(text==="")continue;
   let row:Record<string,unknown>;try{row=JSON.parse(text) as Record<string,unknown>;}catch{continue;}
   const at=Number(row.at);if(Number.isFinite(at))last=Math.max(last,at);
   const address=row.address;if(typeof address!=="string")continue;
   if(row.t==="set"&&typeof row.owner==="string"&&Number.isFinite(row.grantedMs)&&Number.isFinite(row.untilMs)&&Number.isFinite(row.ttlMs))claims.set(address,{address,owner:row.owner,grantedMs:Number(row.grantedMs),untilMs:Number(row.untilMs),ttlMs:Number(row.ttlMs),...(Number.isFinite(row.expectedMs)?{expectedMs:Number(row.expectedMs)}:{})});
   else if(row.t==="drop")claims.delete(address);
   else if(row.t==="cool"&&typeof row.owner==="string"&&Number.isFinite(row.untilMs))cooling.set(address,{address,owner:row.owner,untilMs:Number(row.untilMs)});
  }
 }catch{}
 const at=now();
 for(const [address,row] of[...claims])if(row.untilMs<last)claims.delete(address);else claims.set(address,{...row,untilMs:Math.max(row.untilMs,at+grace)});
 for(const [address,row] of[...cooling])if(row.untilMs<=at)cooling.delete(address);
 const restored:ClaimSnapshotV1={claims:[...claims.values()],cooling:[...cooling.values()]};
 const rewrite=(state:ClaimSnapshotV1):void=>{const temporary=file+".tmp",stamp=now();writeFileSync(temporary,[...state.claims.map(row=>line({t:"set",...row},stamp)),...state.cooling.map(row=>line({t:"cool",...row},stamp))].join(""));renameSync(temporary,file);events=0;};
 try{mkdirSync(dirname(file),{recursive:true});rewrite(restored);}catch{broken=true;}
 let queued:(readonly [ClaimEventV1,number])[]=[],timer:ReturnType<typeof setTimeout>|undefined,writing:Promise<void>=Promise.resolve();
 const encode=async(rows:readonly (readonly [ClaimEventV1,number])[]):Promise<string>=>{
  const parts:string[]=[];
  for(let from=0;from<rows.length;from+=SLICE){
   if(from>0)await new Promise<void>(done=>{setImmediate(done);});
   for(const [event,stamp] of rows.slice(from,from+SLICE))parts.push(line(event,stamp));
  }
  return parts.join("");
 };
 const drain=():Promise<void>=>{
  if(timer!==undefined){clearTimeout(timer);timer=undefined;}
  const batch=queued;queued=[];
  if(batch.length===0||broken)return writing;
  const compact=events>=limit&&snapshot!==undefined,state=compact?snapshot!():undefined,stamp=now();
  if(compact)events=0;
  return writing=writing.then(async()=>{
   try{
    if(state!==undefined){const temporary=file+".tmp";await writeFile(temporary,await encode([...state.claims.map(row=>[{t:"set",...row} as ClaimEventV1,stamp] as const),...state.cooling.map(row=>[{t:"cool",...row} as ClaimEventV1,stamp] as const)]));await rename(temporary,file);}
    else await appendFile(file,await encode(batch));
   }catch{broken=true;queued=[];await rm(file+".tmp",{force:true}).catch(()=>undefined);}
  });
 };
 return Object.freeze({
  restored,
  record(event:ClaimEventV1):void{
   if(broken)return;
   queued.push([event,now()]);events++;
   if(timer===undefined){timer=setTimeout(()=>{timer=undefined;void drain();},window);timer.unref?.();}
  },
  bind(read:()=>ClaimSnapshotV1):void{snapshot=read;},
  flush:()=>drain().then(()=>writing),
 });
}
