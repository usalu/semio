import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {formatChangeCursorV1,mergeChangeRowsV1,parseChangeCursorV1,type ChangeCursorV1,type ChangeRowV1} from "../🟦️.ts";
import type {ChangeJournalClientV1} from "../🧭️client/🟦️.ts";

/** 🪪️ The declared constants of the vouch records; the schema file is their single source. */
export const VOUCH_POLICY_V1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as Readonly<{version:1;directory:string;fileSuffix:string;maximumRecordBytes:number}>);
/** 🎚️ How a vouch was obtained: `journal` proves against the cursor an earlier verification sealed, `cursor` only records the point a verification in progress starts from, `walk` has no journal answer and proves nothing. */
export type VouchModeV1="journal"|"cursor"|"walk";
/** 📜️ One sealed record: what was verified (`key`) and the journal position at which the world matched it. */
export type VouchRecordV1=Readonly<{version:1;consumer:string;key:string;cursor:string;linked:readonly string[]}>;
/** 🔏️ What a seal keeps for a later look: the cursor and the symbolic-link directories above the verified paths. */
export type VouchSealV1=Readonly<{cursor:ChangeCursorV1;linked:readonly string[]}>;

/** 🤝️ A verification's view of the change journal: it only ever narrows the set of paths a walk must ask the file system about, and answers nothing (an empty set) whenever the journal cannot vouch. */
export interface VouchV1{
 readonly mode:VouchModeV1;
 readonly refusal?:string;
 /** 🔎️ The workspace-relative paths of `paths` that no change touched since the sealed point; empty unless the mode is `journal`. */
 skippable(paths:Iterable<string>):ReadonlySet<string>;
 /** 🔁️ Asks the journal again and answers the paths of `paths` that stayed untouched since the anchor (the sealed point, or the start of the verification when none exists); empty when the journal cannot vouch any more. */
 settled(paths:Iterable<string>,signal?:AbortSignal):Promise<ReadonlySet<string>>;
 /** 🔏️ Stores the point at which the world matched: the cursor of the FIRST fence the final pass asked for (`settled` is called once per group of rows with a fresh fence; a row proven unchanged at an earlier fence and not looked at again must not be covered by a later one), else the start cursor; call it only after every row of the verification held. */
 seal():Promise<void>;
}

/** 🗂️ The record file of one consumer inside a vouch directory. */
export const vouchFileV1=(consumer:string):string=>createHash("sha256").update(consumer).digest("hex").slice(0,20)+VOUCH_POLICY_V1.fileSuffix;

/** 📖️ Reads the seal kept for `consumer` when it names `key`; anything else (absent, torn, foreign, another key, no link list) is no record. */
export function readVouchSealV1(directory:string,consumer:string,key:string):VouchSealV1|undefined{
 try{
  const text=readFileSync(join(directory,vouchFileV1(consumer)),"utf8");
  if(text.length>VOUCH_POLICY_V1.maximumRecordBytes)return undefined;
  const value=JSON.parse(text) as Partial<VouchRecordV1>;
  const cursor=value.version===1&&value.consumer===consumer&&value.key===key&&Array.isArray(value.linked)&&value.linked.every(link=>typeof link==="string")?parseChangeCursorV1(value.cursor):undefined;
  return cursor===undefined?undefined:{cursor,linked:value.linked!};
 }catch{return undefined;}
}

/** 📖️ Reads the cursor sealed for `consumer` when it names `key`. */
export function readVouchRecordV1(directory:string,consumer:string,key:string):ChangeCursorV1|undefined{return readVouchSealV1(directory,consumer,key)?.cursor;}

/** 💾️ Stores a record atomically; a newer cursor of the same epoch is kept over an older one, a failure costs only the next walk. */
export function writeVouchRecordV1(directory:string,consumer:string,key:string,cursor:ChangeCursorV1,linked:readonly string[]=[]):void{
 const file=join(directory,vouchFileV1(consumer)),temporary=file+"."+process.pid+"."+Date.now()+".tmp";
 try{
  const kept=readVouchRecordV1(directory,consumer,key);
  if(kept&&kept.epoch===cursor.epoch&&kept.seq>=cursor.seq)return;
  mkdirSync(directory,{recursive:true,mode:0o700});
  writeFileSync(temporary,JSON.stringify({version:1,consumer,key,cursor:formatChangeCursorV1(cursor),linked:[...linked]} satisfies VouchRecordV1),{mode:0o600});
  renameSync(temporary,file);
 }catch{try{rmSync(temporary,{force:true});}catch{}}
}

const foldKey=(path:string):string=>/^[\u0000-\u007f]*$/u.test(path)?path.toLowerCase():path.normalize("NFC").toLowerCase();
/** 🧮️ The paths of `paths` that the changes `rows` did not touch, among those a journal can report at all (inside the workspace, not in an excluded region, not below a symbolic link). A path is touched by a row that names it or one of its descendants, and, unless the row only modifies a directory, by a row that names one of its ancestors; the cost is one set lookup per path plus the depth of the path while some row is structural. */
export function provenUnchangedV1(paths:Iterable<string>,rows:readonly ChangeRowV1[],excluded:(path:string)=>boolean,insensitive:boolean,linked:readonly string[]=[]):ReadonlySet<string>{
 const key=insensitive?foldKey:(path:string)=>path,touched=new Set<string>(),structural=new Set<string>(),links=linked.map(link=>link+"/"),kept=new Set<string>();
 for(const row of rows){
  const id=key(row[2]);touched.add(id);
  for(let cut=id.lastIndexOf("/");cut>0;cut=id.lastIndexOf("/",cut-1))touched.add(id.slice(0,cut));
  if(!(row[1]===1&&row[0]==="m"))structural.add(id);
 }
 for(const path of paths){
  if(path===""||path==="."||path.startsWith("/")||path.startsWith("../")||path.includes("\\")||excluded(path)||linked.includes(path)||links.some(link=>path.startsWith(link)))continue;
  const id=key(path);
  if(touched.has(id))continue;
  let below=false;
  if(structural.size)for(let cut=id.lastIndexOf("/");cut>0;cut=id.lastIndexOf("/",cut-1))if(structural.has(id.slice(0,cut))){below=true;break;}
  if(!below)kept.add(path);
 }
 return kept;
}

/** 🚪️ Opens the vouch of one verification: with the cursor sealed for `key` the journal answers which paths changed since; without it (first run, another key, restarted service, gap, overflow) only the start cursor is taken, so that a second look of the same verification can be answered by the journal instead of a second walk. */
export async function openVouchV1(options:Readonly<{client:ChangeJournalClientV1;directory:string;consumer:string;key:string;insensitive?:boolean;linked?:readonly string[];discoverLinked?:(paths:ReadonlySet<string>)=>Promise<readonly string[]>;signal?:AbortSignal}>):Promise<VouchV1>{
 const {client,directory,consumer,key,signal,discoverLinked}=options,insensitive=options.insensitive??(process.platform==="darwin"||process.platform==="win32"),sealed=readVouchSealV1(directory,consumer,key),stored=sealed?.cursor;
 let linked:readonly string[]=[...new Set([...(options.linked??[]),...(sealed?.linked??[])])];
 let mode:VouchModeV1="walk",cursor:ChangeCursorV1|undefined,anchor:ChangeCursorV1|undefined,rows:ChangeRowV1[]=[],refusal:string|undefined,broken=false,excluded:(path:string)=>boolean=()=>true;
 try{
  if(stored){const answer=await client.since(stored,[""],signal);if(answer.exact){mode="journal";cursor=answer.cursor;rows=[...answer.rows];}else refusal=answer.reason;}
  if(mode!=="journal"){const taken=await client.cursor(signal);if("exact" in taken)refusal??=taken.reason;else{mode="cursor";cursor=taken;}}
  if(mode!=="walk")excluded=client.excluder();
 }catch(error){signal?.throwIfAborted();mode="walk";cursor=undefined;refusal="error";}
 return{
  mode,
  ...(refusal===undefined?{}:{refusal}),
  skippable:paths=>mode==="journal"&&!broken?provenUnchangedV1(paths,rows,excluded,insensitive,linked):new Set<string>(),
  async settled(paths,abort){
   if(mode==="walk"||cursor===undefined||broken)return new Set<string>();
   try{
    const answer=await client.since(cursor,[""],abort??signal);
    if(!answer.exact){broken=true;return new Set<string>();}
    cursor=answer.cursor;anchor??=answer.cursor;rows=mergeChangeRowsV1([...rows,...answer.rows]);
    if(mode==="cursor"&&discoverLinked){const asked=new Set(paths);linked=[...new Set([...linked,...await discoverLinked(asked)])];return provenUnchangedV1(asked,rows,excluded,insensitive,linked);}
    return provenUnchangedV1(paths,rows,excluded,insensitive,linked);
   }catch(error){(abort??signal)?.throwIfAborted();broken=true;return new Set<string>();}
  },
  async seal(){const point=anchor??cursor;if(mode!=="walk"&&point!==undefined&&!broken)writeVouchRecordV1(directory,consumer,key,point,linked);},
 };
}
