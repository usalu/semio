import {appendFileSync,mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
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
export type ClaimJournalV1=Readonly<{restored:ClaimSnapshotV1;record(event:ClaimEventV1):void;bind(snapshot:()=>ClaimSnapshotV1):void}>;

const line=(event:ClaimEventV1,at:number):string=>JSON.stringify({at,...event})+"\n";

/** 💽️ Opens (and compacts) the claim journal `file`: every `set` and `drop` the table made is replayed, so a claim that was alive when the server stopped (killed included; a torn last line is ignored) is restored with `restoreGraceMilliseconds` more to let its holder reconnect and renew, while a claim that had already lapsed, or a cool-off that is over, is dropped. Appends are synchronous, one short line per change; after `journalCompactEvents` changes the file is rewritten from the live table. Any I/O fault disables the journal and never the table: a restart then only costs duplicate work, as before. */
export function openClaimJournalV1(file:string,options:Readonly<{now?:()=>number;graceMilliseconds?:number;compactEvents?:number}>={}):ClaimJournalV1{
 const now=options.now??Date.now,grace=options.graceMilliseconds??policy.restoreGraceMilliseconds,limit=options.compactEvents??policy.journalCompactEvents;
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
 const rewrite=(state:ClaimSnapshotV1):void=>{
  const temporary=file+".tmp",stamp=now();
  writeFileSync(temporary,[...state.claims.map(row=>line({t:"set",...row},stamp)),...state.cooling.map(row=>line({t:"cool",...row},stamp))].join(""));
  renameSync(temporary,file);events=0;
 };
 try{mkdirSync(dirname(file),{recursive:true});rewrite(restored);}catch{broken=true;}
 return Object.freeze({
  restored,
  record(event:ClaimEventV1):void{
   if(broken)return;
   try{
    appendFileSync(file,line(event,now()));
    if(++events>=limit&&snapshot!==undefined)rewrite(snapshot());
   }catch{broken=true;}
  },
  bind(read:()=>ClaimSnapshotV1):void{snapshot=read;},
 });
}
