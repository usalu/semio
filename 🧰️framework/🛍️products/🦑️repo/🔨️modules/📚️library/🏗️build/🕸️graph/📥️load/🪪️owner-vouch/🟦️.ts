import {readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {provenUnchangedV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";
import {linkIndexV1,type JournalCursorV1,type JournalFeedV1,type JournalRefusedV1,type JournalRowV1,type LinkStateV1} from "../🗞️journal-clean/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const limits=schema.$defs.Limits.const,PATH_KINDS:ReadonlySet<string>=new Set(["file","exists","stat","lstat","fileExists","directoryExists","readdir","readdirNames"]);
type LedgerV1=Readonly<{version:1;key:string;epoch:string;seq:number;owners:readonly string[];links?:LinkStateV1}>;
export type OwnerVouchPortsV1=Readonly<{feed:JournalFeedV1;root:string;file:string;key:string;insensitive?:boolean;timeoutMilliseconds?:number;signal?:AbortSignal;now?:()=>number}>;
export type OwnerVouchV1=Readonly<{
 cleanFor:(owner:string,rows:readonly JournalRowV1[])=>Promise<((row:JournalRowV1)=>boolean)|undefined>;
 record:(owner:string)=>void;
 flush:()=>void;
 clear:()=>void;
 counts:()=>Readonly<{vouched:number;recorded:number;skipped:number;changed:number|undefined}>;
}>;

const NONE:OwnerVouchV1=Object.freeze({cleanFor:async()=>undefined,record:()=>undefined,flush:()=>undefined,clear:()=>undefined,counts:()=>({vouched:0,recorded:0,skipped:0,changed:undefined})});
const within=<T>(work:Promise<T>,milliseconds:number):Promise<T|undefined>=>new Promise(resolve=>{const timer=setTimeout(()=>resolve(undefined),milliseconds);work.then(value=>{clearTimeout(timer);resolve(value);},()=>{clearTimeout(timer);resolve(undefined);});});
const readLedger=(file:string,key:string):LedgerV1|undefined=>{try{const value=JSON.parse(readFileSync(file,"utf8"));return validateJsonSchemaSubset(schema.$defs.Ledger,value,schema).length===0&&value.key===key?value:undefined;}catch{return undefined;}};

/** 🪪️ Opens the vouch ledger of one selected read: the owners an earlier worker of the same read verified stay vouched while the journal reports no change at, above or below a path row of theirs since the cursor of the ledger, so a restarted worker skips those probes and repeats no completed owner check. Without an exact journal, a ledger of another key or epoch, a gap or a refusal nothing is vouched and every row is probed as before; the cursor is taken before the first probe, so a change during a probe is reported by the next question.
 * @see ./🧪️tests/🟦️.ts
 * @see ../🗞️journal-clean/🟦️.ts the same trust model for the rows of an admission memo */
export async function openOwnerVouchV1(ports:OwnerVouchPortsV1):Promise<OwnerVouchV1>{
 const {feed,root,file,key,timeoutMilliseconds=limits.timeoutMilliseconds,signal,now=Date.now}=ports,insensitive=ports.insensitive??(process.platform==="darwin"||process.platform==="win32");
 const fold=(path:string)=>insensitive?path.toLowerCase():path,prefix=fold(root.replaceAll("\\","/").replace(/\/+$/,""))+"/";
 const at=await within(feed.cursor(signal),timeoutMilliseconds);
 if(at===undefined||(at as JournalRefusedV1).exact===false)return NONE;
 const stored=readLedger(file,key);
 let answer=stored===undefined?undefined:await within(feed.since({epoch:stored.epoch,seq:stored.seq},[""],signal),timeoutMilliseconds);
 if(answer!==undefined&&answer.exact!==true)answer=undefined;
 const lineage:JournalCursorV1=stored!==undefined&&answer!==undefined?{epoch:stored.epoch,seq:stored.seq}:at as JournalCursorV1,usable=stored!==undefined&&answer!==undefined;
 const vouched=new Set<string>(usable?stored!.owners:[]),recorded=new Set<string>(vouched),excluded=feed.excluder(),links=linkIndexV1(root,usable?stored!.links:undefined,usable?{rows:answer!.rows,excluded,insensitive}:undefined);
 const state={skipped:0};let lastFlush=now();
 const flush=()=>{try{const temporary=file+"."+process.pid+".tmp";writeFileSync(temporary,JSON.stringify({version:1,key,epoch:lineage.epoch,seq:lineage.seq,owners:[...recorded],links:links.state()}));renameSync(temporary,file);lastFlush=now();}catch{}};
 return Object.freeze({
  async cleanFor(owner,rows){
   if(!usable||!vouched.has(owner))return undefined;
   const found:{row:JournalRowV1;relative:string;folded:string}[]=[];
   for(const row of rows){
    if(!PATH_KINDS.has(row.kind))continue;
    const original=row.path.replaceAll("\\","/"),folded=fold(original);
    if(folded.startsWith(prefix)&&original.length>prefix.length)found.push({row,relative:original.slice(prefix.length),folded});
   }
   const linked=await links.among(found.map(item=>item.relative)),kept=provenUnchangedV1(found.map(item=>item.relative),answer!.rows as never,excluded,insensitive,linked),eligible=new Map<string,JournalRowV1>();
   for(const item of found)if(kept.has(item.relative))eligible.set(item.row.kind+"\0"+item.folded,item.row);
   return row=>{
    const known=eligible.get(row.kind+"\0"+fold(row.path.replaceAll("\\","/")));
    if(known===undefined||known!==row&&JSON.stringify(known.fact)!==JSON.stringify(row.fact))return false;
    state.skipped++;return true;
   };
  },
  record(owner){
   if(recorded.size>=limits.maximumOwners&&!recorded.has(owner))return;
   recorded.add(owner);
   if(now()-lastFlush>=limits.flushMilliseconds)flush();
  },
  flush,
  clear(){try{rmSync(file,{force:true});}catch{}},
  counts:()=>({vouched:vouched.size,recorded:recorded.size,skipped:state.skipped,changed:usable?answer!.rows.length:undefined}),
 });
}

/** 🪪️ The two hooks that tie one owner of a read to the ledger, shared by the live read and its tests: `hold` probes the rows (`probe` receives the vouch predicate, `undefined` when the owner is not vouched) and records the owner once they hold, `outcome` records it when its answer was served or discovered, because an owner that was discovered fresh was observed after the cursor of the ledger and is vouched like a verified one (a read cut or killed after its first completed owners then repeats none of them).
 * @see ./🧪️tests/🟦️.ts */
export function vouchedOwnerV1(vouch:OwnerVouchV1|undefined,owner:string,probe:(rows:readonly JournalRowV1[],clean:((row:JournalRowV1)=>boolean)|undefined)=>Promise<boolean>){
 return Object.freeze({
  async hold(rows:readonly JournalRowV1[]):Promise<boolean>{
   const held=await probe(rows,await vouch?.cleanFor(owner,rows));
   if(held)vouch?.record(owner);
   return held;
  },
  outcome():void{vouch?.record(owner);},
 });
}
