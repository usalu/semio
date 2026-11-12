import {appendFileSync,mkdirSync} from "node:fs";
import {appendFile,mkdir,readdir,readFile,rename,stat,unlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import type {CasPolicyV1} from "../🧬️schema/🟦️.ts";

const BASE="base.jsonl",SEGMENT=/^[A-Za-z0-9._-]+\.jsonl$/u;

/** 📒️ Directory of the append-only access segments of one shard; every writer owns its own segment file, so writers never contend. */
export const casLedgerShardV1=(root:string,shard:string):string=>join(root,"v1","l",shard);

/** 📒️ Appends one access event `{d,t}` to the segment of `writer`; a torn last line after a crash is skipped on read. */
export async function appendCasAccessV1(root:string,shard:string,writer:string,digest:string,atMs:number):Promise<void>{
 const directory=casLedgerShardV1(root,shard);
 try{await appendFile(join(directory,writer+".jsonl"),JSON.stringify({d:digest,t:atMs})+"\n");}
 catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;await mkdir(directory,{recursive:true});await appendFile(join(directory,writer+".jsonl"),JSON.stringify({d:digest,t:atMs})+"\n");}
}

async function readEvents(file:string,into:Map<string,number>):Promise<void>{
 let text;try{text=await readFile(file,"utf8");}catch{return;}
 for(const line of text.split("\n")){
  if(!line)continue;
  try{const event=JSON.parse(line) as {d?:unknown;t?:unknown};if(typeof event.d==="string"&&/^[0-9a-f]{64}$/u.test(event.d)&&Number.isFinite(event.t)){const at=event.t as number;if(at>(into.get(event.d)??-1))into.set(event.d,at);}}catch{}
 }
}

/** 📒️ Folds every segment of a shard into the latest access time per digest. */
export async function foldCasShardV1(root:string,shard:string):Promise<Map<string,number>>{
 const into=new Map<string,number>(),directory=casLedgerShardV1(root,shard);
 let names:string[];try{names=await readdir(directory);}catch{return into;}
 for(const name of names.sort())if(SEGMENT.test(name))await readEvents(join(directory,name),into);
 return into;
}

/** 🧹️ Rewrites the base segment of a shard to the digests that still exist, then removes the quiet segments it folded (a segment of a live writer is younger than the quiet time and stays). */
export async function compactCasShardV1(root:string,shard:string,live:ReadonlySet<string>,folded:ReadonlyMap<string,number>,nowMs:number,policy:Pick<CasPolicyV1,"ledgerQuietMs">):Promise<number>{
 const directory=casLedgerShardV1(root,shard);let names:string[];try{names=await readdir(directory);}catch{return 0;}
 const quiet:string[]=[];
 for(const name of names)if(SEGMENT.test(name)&&name!==BASE){try{if(nowMs-(await stat(join(directory,name))).mtimeMs>=policy.ledgerQuietMs)quiet.push(name);}catch{}}
 const rows=[...folded].filter(([digest])=>live.has(digest)).sort((left,right)=>left[0]<right[0]?-1:1).map(([d,t])=>JSON.stringify({d,t})).join("\n");
 const target=join(directory,BASE),temporary=join(directory,`.${BASE}.${process.pid}.${Math.random().toString(36).slice(2)}`);
 try{await writeFile(temporary,rows?rows+"\n":"");await rename(temporary,target);}catch(error){await unlink(temporary).catch(()=>undefined);throw error;}
 for(const name of quiet)await unlink(join(directory,name)).catch(()=>undefined);
 return quiet.length;
}

/** 👁️ A live view of one shard: the folded latest access per digest (`snapshot` is the whole map, `latest` one digest), refreshed by reading only the bytes appended to the segments since the last look (one directory listing and a few stats), so a collector sees an access that happened a moment ago and reads every segment of a run once instead of once per pass. */
export type CasShardViewV1=Readonly<{latest(digest:string):number;snapshot():ReadonlyMap<string,number>;refresh():Promise<void>}>;

const SEGMENT_READS=32;

/** 👁️ Opens the live view of a shard; the first look reads the segments in batches of `SEGMENT_READS` at a time, because a shard of a busy host holds thousands of small segments (one per writer). A segment that nobody wrote for `quietMs` before it was read is settled: later looks list the directory and stat only the segments that are still live or new, so looking again after every removal of a collection costs a listing, not thousands of stats. */
export async function openCasShardViewV1(root:string,shard:string,options:Readonly<{nowMs?:number;quietMs?:number}>={}):Promise<CasShardViewV1>{
 const directory=casLedgerShardV1(root,shard),latest=new Map<string,number>(),offsets=new Map<string,number>(),settled=new Set<string>(),nowMs=options.nowMs??Date.now(),quietMs=options.quietMs??Infinity;
 const look=async(name:string):Promise<void>=>{
  const file=join(directory,name),from=offsets.get(name)??0;let info:Awaited<ReturnType<typeof stat>>;try{info=await stat(file);}catch{return;}
  const size=info.size;
  if(size<=from){if(size<from)offsets.delete(name);else if(nowMs-info.mtimeMs>=quietMs)settled.add(name);return;}
  let text:string;try{text=(await readFile(file)).subarray(from,size).toString("utf8");}catch{return;}
  const end=text.lastIndexOf("\n");if(end<0)return;
  offsets.set(name,from+Buffer.byteLength(text.slice(0,end+1)));
  const folded=new Map<string,number>();for(const line of text.slice(0,end).split("\n")){try{const event=JSON.parse(line) as {d?:unknown;t?:unknown};if(typeof event.d==="string"&&Number.isFinite(event.t)&&(event.t as number)>(folded.get(event.d)??-1))folded.set(event.d,event.t as number);}catch{}}
  for(const [digest,at] of folded)if(at>(latest.get(digest)??-1))latest.set(digest,at);
  if(nowMs-info.mtimeMs>=quietMs)settled.add(name);
 };
 const refresh=async():Promise<void>=>{
  let names:string[];try{names=(await readdir(directory)).filter(name=>SEGMENT.test(name)&&!settled.has(name));}catch{return;}
  for(let index=0;index<names.length;index+=SEGMENT_READS)await Promise.all(names.slice(index,index+SEGMENT_READS).map(look));
 };
 await refresh();
 return{latest:digest=>latest.get(digest)??0,snapshot:()=>latest,refresh};
}

/** 📒️ Synchronous form of `appendCasAccessV1` for callers that cannot await (build scripts, compiler wrappers). */
export function appendCasAccessSyncV1(root:string,shard:string,writer:string,digest:string,atMs:number):void{
 const directory=casLedgerShardV1(root,shard),line=JSON.stringify({d:digest,t:atMs})+"\n";
 try{appendFileSync(join(directory,writer+".jsonl"),line);}
 catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;mkdirSync(directory,{recursive:true});appendFileSync(join(directory,writer+".jsonl"),line);}
}
