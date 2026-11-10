import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {renderChangeRefusalV1,type ChangeRefusalV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../../📜️journal/🟦️.ts";
import {GRAPH_HOOKS_POLICY_V1,renderGraphHooksNoticeV1} from "../🟨️.mjs";
import type {CursorStoreV1,DirtyCarryV1,JournalAnswerV1,JournalNoticeV1,JournalPortV1} from "../🔌️host/🟨️.mjs";

/** 📖️ Reads the watcher journal of a workspace directly: the detached journal service is started on demand and no resident repo service has to answer. */
export function directJournalPortV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env,client:ChangeJournalClientV1=connectChangeJournalV1(root,environment)):JournalPortV1{
 return{
  async cursor(signal){
   const answer=await client.cursor(signal);
   return"exact" in answer?undefined:{instance:answer.epoch,seq:answer.seq};
  },
  async since(from,signal):Promise<JournalAnswerV1>{
   const answer=await client.since({epoch:from.instance,seq:from.seq},[""],signal);
   if(!answer.exact)return{complete:false,reason:answer.reason,...(answer.detail!==undefined?{detail:answer.detail}:{}),...(answer.path!==undefined?{path:answer.path}:{}),...(answer.count!==undefined?{count:answer.count}:{})};
   return{complete:true,cursor:{instance:answer.cursor.epoch,seq:answer.cursor.seq},rows:answer.rows};
  },
  excluded:()=>client.excluder(),
 };
}

const carryValid=(value:unknown):boolean=>!!value&&typeof value==='object'&&Array.isArray((value as DirtyCarryV1).exact)&&Array.isArray((value as DirtyCarryV1).subtrees)&&[...(value as DirtyCarryV1).exact,...(value as DirtyCarryV1).subtrees].every(path=>typeof path==='string');

/** 💾️ Keeps the cursor at which each generation was verified in the journal directory of the workspace, bounded to the newest entries; a lost or foreign file only costs one verify epoch. */
export function fileCursorStoreV1(directory:string,maximum:number=GRAPH_HOOKS_POLICY_V1.maximumCursors,file:string=GRAPH_HOOKS_POLICY_V1.cursorFile):CursorStoreV1{
 const path=join(directory,file);
 const read=():[string,string,number,DirtyCarryV1|null?,string?][]=>{
  try{
   const value=JSON.parse(readFileSync(path,"utf8")) as {version?:unknown;entries?:unknown};
   if(value.version!==1||!Array.isArray(value.entries))return[];
   return value.entries.filter((row):row is [string,string,number,DirtyCarryV1|null?,string?]=>Array.isArray(row)&&row.length>=3&&row.length<=5&&typeof row[0]==="string"&&/^[0-9a-f]{16}$/u.test(String(row[1]))&&Number.isSafeInteger(row[2])&&row[2]>=0&&(row[3]===undefined||row[3]===null||carryValid(row[3]))&&(row[4]===undefined||typeof row[4]==="string"&&/^[0-9a-f]{64}$/u.test(row[4])));
  }catch{return[];}
 };
 return{
  get(key){const row=read().find(entry=>entry[0]===key);return row?{instance:row[1],seq:row[2],...(row[3]?{carry:row[3]}:{}),...(row[4]?{install:row[4]}:{})}:undefined;},
  set(key,cursor,carry){
   const install=(cursor as {install?:string}).install,withCarry=carry&&(carry.exact.length||carry.subtrees.length)?carry:null,row:[string,string,number,DirtyCarryV1|null?,string?]=install?[key,cursor.instance,cursor.seq,withCarry,install]:withCarry?[key,cursor.instance,cursor.seq,withCarry]:[key,cursor.instance,cursor.seq],entries=[...read().filter(entry=>entry[0]!==key),row].slice(-maximum),temporary=path+"."+process.pid+".tmp";
   try{mkdirSync(directory,{recursive:true,mode:0o700});writeFileSync(temporary,JSON.stringify({version:1,entries}),{mode:0o600});renameSync(temporary,path);}
   catch{try{rmSync(temporary,{force:true});}catch{}}
  },
 };
}

/** 🧾️ The identity of the dependency install a workspace resolves against: the install root `node_modules` points to (written only by `repo dependencies sync/lock`) and the digest of the lockfile. Nothing under `node_modules` changes without it changing; undefined where either is missing. */
export function installFingerprintV1(root:string,lockfile:string=GRAPH_HOOKS_POLICY_V1.lockfile):string|undefined{
 try{
  const installed=realpathSync(join(root,"node_modules")),lock=readFileSync(join(root,lockfile));
  return createHash("sha256").update(installed).update("\0").update(lock).digest("hex");
 }catch{return undefined;}
}

/** 🔌️ The cursor store of the journal of one workspace. */
export const journalCursorStoreV1=(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):CursorStoreV1=>fileCursorStoreV1(changeJournalDirectoryV1(root,environment));

/** 🌐️ Renders the announcement of a verify epoch in one declared language. */
export function renderJournalNoticeV1(notice:JournalNoticeV1,language:string):string{
 const text=notice.reason==="unrecorded"||notice.reason==="outside"?renderGraphHooksNoticeV1(notice.reason,language,notice):renderChangeRefusalV1({exact:false,reason:notice.reason as ChangeRefusalV1,...(notice.path!==undefined?{path:notice.path}:{}),...(notice.detail!==undefined?{detail:notice.detail}:{}),...(notice.count!==undefined?{count:notice.count}:{})},language);
 return"graph: "+text;
}
