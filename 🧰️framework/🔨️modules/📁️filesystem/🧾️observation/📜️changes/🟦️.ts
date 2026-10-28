import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {closeSync,existsSync,fstatSync,mkdirSync,openSync,readSync,readdirSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync,writeSync} from "node:fs";
import {join} from "node:path";

/** 📜️ Segment sizes, retention, liveness bounds, fence timing and the EN/DE refusal texts, exactly as the schema declares them. */
export type ChangeJournalPolicyV1=Readonly<{version:1;segmentBytes:number;segmentsKept:number;maximumRowsPerRecord:number;maximumRowsPerAnswer:number;heartbeatMilliseconds:number;heartbeatStaleMilliseconds:number;activityMilliseconds:number;idleShutdownMilliseconds:number;fenceTimeoutMilliseconds:number;fencePollMilliseconds:number;startTimeoutMilliseconds:number;startPollMilliseconds:number;unavailableMilliseconds:number;stopWaitMilliseconds:number;watcherLatencyMilliseconds:number;maximumResidentBytes:number;tailChunkBytes:number;texts:Readonly<Record<string,Readonly<Record<string,string>>>>}>;
/** 📐️ The declared constants of the change journal, read from its schema and frozen. */
export const CHANGE_JOURNAL_POLICY_V1:ChangeJournalPolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as ChangeJournalPolicyV1);

/** 🔔️ One change: kind (added, modified, removed, renamed; advisory), whether the path was a directory, and its workspace-relative `/` path. Every row means "this path may have changed"; a directory row of kind a, r or n also means its subtree. */
export type ChangeRowV1=readonly["a"|"m"|"r"|"n",0|1,string];
/** 🧾️ One log record: a batch of rows (`rows`) or a gap (`lost`: reason and scope; empty scope means everywhere). */
export type ChangeRecordV1=Readonly<{seq:number;at:number;rows?:readonly ChangeRowV1[];lost?:readonly[string,string]}>;
/** 🧭️ A position in the log: the epoch of the service run and the last sequence number the holder has accounted for. */
export type ChangeCursorV1=Readonly<{epoch:string;seq:number}>;
/** 🚫️ Why the journal could not vouch; the consumer walks the tree itself. */
export type ChangeRefusalV1="disabled"|"no-service"|"starting"|"stale"|"untrusted"|"excluded"|"epoch"|"floor"|"gap"|"lost"|"fence-timeout"|"overflow"|"remote-file-system"|"cancelled";
/** 🙅️ A refusal with the fields its texts name. */
export type ChangeRefusedV1=Readonly<{exact:false;reason:ChangeRefusalV1;path?:string;detail?:string;count?:number}>;
/** 🗂️ The service state document (`state.json`). */
export type ChangeStateV1=Readonly<{version:1;epoch:string;instance:string;pid:number;startedAt:number;root:string;fence:string;exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>;backend:"fsevents"|"inotify"|"rdcw"|"portable"|"poll"|"none";trust:"exact"|"verify"|"none";ready:boolean;reason?:string}>;

const PATH_REFUSED=/^\/|(^|\/)\.\.?(\/|$)|\u0000|\\/u,KINDS=new Set(["a","m","r","n"]);
const validPath=(path:unknown):path is string=>typeof path==="string"&&path.length>0&&path.length<=4096&&!PATH_REFUSED.test(path);
const STATE_FILE="state.json",HEARTBEAT_FILE="heartbeat",ACTIVITY_FILE="activity",UNAVAILABLE_FILE="unavailable.json";
const SEGMENT=/^([0-9a-f]{16})\.(\d{16})\.log$/u;
/** 📁️ The file names of one journal directory. */
export const CHANGE_JOURNAL_FILES_V1=Object.freeze({state:STATE_FILE,heartbeat:HEARTBEAT_FILE,activity:ACTIVITY_FILE,unavailable:UNAVAILABLE_FILE,lock:"lock"});

/** 🧱️ The text of one record, one line with the final newline. */
export function encodeChangeRecordV1(record:ChangeRecordV1):string{
 return record.lost?`{"s":${record.seq},"t":${record.at},"l":${JSON.stringify(record.lost)}}\n`:`{"s":${record.seq},"t":${record.at},"r":${JSON.stringify(record.rows??[])}}\n`;
}

/** 🔍️ Reads one line strictly (closed keys, declared kinds, admitted paths); undefined for anything else. */
export function decodeChangeRecordV1(line:string):ChangeRecordV1|undefined{
 let value:unknown;
 try{value=JSON.parse(line);}catch{return undefined;}
 if(!value||typeof value!=="object"||Array.isArray(value))return undefined;
 const {s,t,r,l,...rest}=value as Record<string,unknown>;
 if(Object.keys(rest).length||!Number.isSafeInteger(s)||(s as number)<1||!Number.isSafeInteger(t)||(t as number)<0||(r===undefined)===(l===undefined))return undefined;
 if(l!==undefined){
  if(!Array.isArray(l)||l.length!==2||typeof l[0]!=="string"||!l[0]||l[0].length>64||typeof l[1]!=="string"||l[1].length>4096)return undefined;
  return Object.freeze({seq:s as number,at:t as number,lost:Object.freeze([l[0],l[1]]) as readonly[string,string]});
 }
 if(!Array.isArray(r))return undefined;
 for(const row of r)if(!Array.isArray(row)||row.length!==3||!KINDS.has(row[0])||(row[1]!==0&&row[1]!==1)||!validPath(row[2]))return undefined;
 return Object.freeze({seq:s as number,at:t as number,rows:r as readonly ChangeRowV1[]});
}

/** 🧬️ The text form of a cursor, stable for memos: `epoch:seq`. */
export const formatChangeCursorV1=(cursor:ChangeCursorV1):string=>cursor.epoch+":"+cursor.seq;
/** 🔡️ The cursor of its text form; undefined for anything else. */
export function parseChangeCursorV1(text:unknown):ChangeCursorV1|undefined{
 const match=typeof text==="string"?/^([0-9a-f]{16}):(\d{1,15})$/u.exec(text):null;
 return match?Object.freeze({epoch:match[1]!,seq:Number(match[2])}):undefined;
}

const fold=(path:string):string=>/^[\u0000-\u007f]*$/u.test(path)?path.toLowerCase():path.normalize("NFC").toLowerCase();
/** 🎯️ Whether a row can have changed something at or below `root` ("" is the whole workspace): the row names the root, lies below it, or is a structural directory row (added, removed, renamed) above it. `insensitive` compares the way a case- and normalization-insensitive file system does. */
export function changeAffectsV1(row:ChangeRowV1,root:string,insensitive:boolean):boolean{
 if(root==="")return true;
 const path=insensitive?fold(row[2]):row[2],base=insensitive?fold(root):root;
 if(path===base||path.startsWith(base+"/"))return true;
 return base.startsWith(path+"/")&&!(row[1]===1&&row[0]==="m");
}

/** 🧮️ The distinct paths of many rows, one row per path: the directory flag is the union, the kind the first structural kind (else `m`). */
export function mergeChangeRowsV1(rows:Iterable<ChangeRowV1>):ChangeRowV1[]{
 const merged=new Map<string,ChangeRowV1>();
 for(const row of rows){
  const old=merged.get(row[2]);
  merged.set(row[2],old?[old[0]!=="m"?old[0]:row[0],(old[1]|row[1]) as 0|1,row[2]]:row);
 }
 return[...merged.values()].sort((a,b)=>a[2]<b[2]?-1:a[2]>b[2]?1:0);
}

/** 🌐️ Renders a refusal in one declared language. */
export function renderChangeRefusalV1(refused:ChangeRefusedV1,language:string):string{
 const table=CHANGE_JOURNAL_POLICY_V1.texts[language];
 if(!table||typeof table[refused.reason]!=="string")throw Error("Declared change journal text required: "+language+"."+refused.reason);
 return table[refused.reason]!.replace(/\{([a-z]+)\}/gu,(_,name:string)=>String((refused as Record<string,unknown>)[name]??"?"));
}

/** 📥️ Reads and validates `state.json`; undefined when missing, torn or foreign. */
export function readChangeStateV1(directory:string):ChangeStateV1|undefined{
 try{
  const value=JSON.parse(readFileSync(join(directory,STATE_FILE),"utf8")) as Partial<ChangeStateV1>;
  const exclude=value.exclude;
  if(value.version!==1||typeof value.epoch!=="string"||!/^[0-9a-f]{16}$/u.test(value.epoch)||!Number.isSafeInteger(value.pid)||typeof value.root!=="string"||!validPath(value.fence)||!exclude||!Array.isArray(exclude.rooted)||!Array.isArray(exclude.names)||typeof value.ready!=="boolean"||!["exact","verify","none"].includes(String(value.trust)))return undefined;
  return value as ChangeStateV1;
 }catch{return undefined;}
}

/** 💓️ The age of the heartbeat in milliseconds; infinite when there is none. */
export function changeHeartbeatAgeV1(directory:string,now:number=Date.now()):number{
 try{return now-statSync(join(directory,HEARTBEAT_FILE)).mtimeMs;}catch{return Number.POSITIVE_INFINITY;}
}

/** 📋️ The segments of one epoch, oldest first: first sequence number and file. */
export function listChangeSegmentsV1(directory:string,epoch:string):readonly Readonly<{first:number;file:string}>[]{
 let names:string[];
 try{names=readdirSync(directory);}catch{return[];}
 const rows:{first:number;file:string}[]=[];
 for(const name of names){const match=SEGMENT.exec(name);if(match&&match[1]===epoch)rows.push({first:Number(match[2]),file:join(directory,name)});}
 return rows.sort((a,b)=>a.first-b.first);
}

const SEQUENCE=/^\{"s":(\d+),/u;
function readTail(file:string,after:number,chunk:number):readonly string[]|undefined{
 let descriptor:number;
 try{descriptor=openSync(file,"r");}catch{return undefined;}
 try{
  let start=fstatSync(descriptor).size,buffer=Buffer.alloc(0);
  for(;;){
   const from=Math.max(0,start-chunk),part=Buffer.allocUnsafe(start-from);
   for(let got=0;got<part.length;){const n=readSync(descriptor,part,got,part.length-got,from+got);if(!n)return undefined;got+=n;}
   buffer=Buffer.concat([part,buffer]);start=from;
   const lines=buffer.toString("utf8").split("\n");lines.pop();
   if(start>0)lines.shift();
   const first=lines[0]===undefined?undefined:SEQUENCE.exec(lines[0])?.[1];
   if(start===0||(first!==undefined&&Number(first)<=after))return lines;
  }
 }catch{return undefined;}finally{try{closeSync(descriptor);}catch{}}
}

/** 📖️ What the log holds after a sequence number: the records and the highest sequence number the log holds. */
export type ChangeLogReadV1=Readonly<{ok:true;records:readonly ChangeRecordV1[];head:number}>|Readonly<{ok:false;reason:"gap"|"floor"}>;
/** 📚️ Reads every record of an epoch with a sequence number above `after`, verifying that they are contiguous from `after + 1`, that the log reaches back to `after` and that `after` is not beyond the head. An incomplete last line is a writer in progress and is not an error. */
export function readChangeRecordsAfterV1(directory:string,epoch:string,after:number,chunk:number=CHANGE_JOURNAL_POLICY_V1.tailChunkBytes):ChangeLogReadV1{
 const segments=listChangeSegmentsV1(directory,epoch);
 if(!segments.length)return after===0?{ok:true,records:[],head:0}:{ok:false,reason:"gap"};
 const collected:ChangeRecordV1[][]=[];
 let seen=segments[segments.length-1]!.first-1;
 for(let at=segments.length-1;at>=0;at--){
  const segment=segments[at]!,lines=readTail(segment.file,after,chunk);
  if(!lines)return{ok:false,reason:"gap"};
  const records:ChangeRecordV1[]=[];
  for(const line of lines){
   const sequence=SEQUENCE.exec(line)?.[1];
   if(sequence===undefined)return{ok:false,reason:"gap"};
   seen=Math.max(seen,Number(sequence));
   if(Number(sequence)<=after)continue;
   const record=decodeChangeRecordV1(line);
   if(!record)return{ok:false,reason:"gap"};
   records.push(record);
  }
  collected.unshift(records);
  if(segment.first<=after+1)break;
  if(at===0)return{ok:false,reason:"floor"};
 }
 const all=collected.flat();
 for(let index=0;index<all.length;index++)if(all[index]!.seq!==after+1+index)return{ok:false,reason:"gap"};
 const head=all.length?all[all.length-1]!.seq:seen;
 return head<after?{ok:false,reason:"gap"}:{ok:true,records:all,head};
}

/** 🔝️ The highest sequence number of an epoch's log (0 when empty); undefined when the log is unreadable. */
export function changeHeadV1(directory:string,epoch:string,chunk:number=CHANGE_JOURNAL_POLICY_V1.tailChunkBytes):number|undefined{
 const segments=listChangeSegmentsV1(directory,epoch);
 if(!segments.length)return 0;
 const last=segments[segments.length-1]!,lines=readTail(last.file,Number.MAX_SAFE_INTEGER,chunk);
 if(!lines)return undefined;
 let head=last.first-1;
 for(const line of lines){const sequence=SEQUENCE.exec(line)?.[1];if(sequence===undefined)return undefined;head=Math.max(head,Number(sequence));}
 return head;
}

/** ✍️ The single writer of one epoch. */
export interface ChangeLogWriterV1{
 readonly epoch:string;
 seq():number;
 append(rows:readonly ChangeRowV1[],at:number):number;
 lost(reason:string,scope:string,at:number):number;
 close():void;
}

/** 🆕️ Opens the log of a new epoch for appending: removes every segment of other epochs, rotates at the declared size and keeps the declared number of segments. Records are written with one `write` each, so a crash leaves at most an incomplete last line, which readers ignore. */
export function createChangeLogWriterV1(options:Readonly<{directory:string;epoch:string;policy?:ChangeJournalPolicyV1}>):ChangeLogWriterV1{
 const {directory,epoch,policy=CHANGE_JOURNAL_POLICY_V1}=options;
 mkdirSync(directory,{recursive:true,mode:0o700});
 for(const name of readdirSync(directory)){const match=SEGMENT.exec(name);if(match&&match[1]!==epoch)rmSync(join(directory,name),{force:true});}
 let sequence=0,descriptor=-1,bytes=0;
 const name=(first:number)=>join(directory,epoch+"."+String(first).padStart(16,"0")+".log");
 const rotate=()=>{
  if(descriptor>=0)closeSync(descriptor);
  descriptor=openSync(name(sequence+1),"a",0o600);bytes=0;
  const kept=listChangeSegmentsV1(directory,epoch);
  for(const old of kept.slice(0,Math.max(0,kept.length-policy.segmentsKept)))rmSync(old.file,{force:true});
 };
 const write=(record:ChangeRecordV1):number=>{
  if(descriptor<0||bytes>=policy.segmentBytes)rotate();
  const text=Buffer.from(encodeChangeRecordV1(record));
  writeSync(descriptor,text);bytes+=text.length;sequence=record.seq;
  return record.seq;
 };
 return{
  epoch,
  seq:()=>sequence,
  append(rows,at){
   if(rows.length>policy.maximumRowsPerRecord)return write({seq:sequence+1,at,lost:["overflow",""]});
   for(const row of rows)if(!KINDS.has(row[0])||!validPath(row[2]))return write({seq:sequence+1,at,lost:["unreadable-row",""]});
   return write({seq:sequence+1,at,rows});
  },
  lost:(reason,scope,at)=>write({seq:sequence+1,at,lost:[reason,scope]}),
  close(){if(descriptor>=0){closeSync(descriptor);descriptor=-1;}},
 };
}

/** 💾️ Writes `state.json` atomically with owner-only permissions. */
export function writeChangeStateV1(directory:string,state:ChangeStateV1):void{
 mkdirSync(directory,{recursive:true,mode:0o700});
 const temporary=join(directory,STATE_FILE+"."+process.pid+".tmp");
 writeFileSync(temporary,JSON.stringify(state),{mode:0o600});
 renameSync(temporary,join(directory,STATE_FILE));
}

/** 🫀️ Touches a file of the journal directory (creating it when missing); a failure is only a missed beat. */
export function touchChangeFileV1(directory:string,name:string,now:number=Date.now()):void{
 const file=join(directory,name);
 try{if(!existsSync(file))writeFileSync(file,"",{mode:0o600});utimesSync(file,now/1000,now/1000);}catch{}
}

/** 🕳️ Whether a lost record's scope can hide changes under any of the roots. */
export function lostAffectsV1(scope:string,roots:readonly string[],insensitive:boolean):boolean{
 if(scope===""||roots.includes(""))return true;
 return roots.some(root=>changeAffectsV1(["a",1,scope],root,insensitive));
}

/** ⛔️ Whether a root lies where the watcher never reports: inside a rooted exclusion or below an excluded name. */
export function changeRootExcludedV1(state:ChangeStateV1,root:string,insensitive:boolean):boolean{
 if(root==="")return false;
 const path=insensitive?fold(root):root;
 for(const prefix of state.exclude.rooted){const base=insensitive?fold(prefix):prefix;if(path===base||path.startsWith(base+"/"))return true;}
 const names=new Set(insensitive?state.exclude.names.map(fold):state.exclude.names);
 return path.split("/").some(part=>names.has(part));
}

/** 🔦️ Which of many watched paths (files or directories, workspace-relative) a set of rows can have changed: exactly the paths `changeAffectsV1` accepts one by one, found in time proportional to the rows (plus one pass over the watched paths when a structural directory row asks for the paths below it). */
export function changedWatchedPathsV1(watched:Iterable<string>,rows:readonly ChangeRowV1[],insensitive:boolean):Set<string>{
 const key=insensitive?fold:(path:string)=>path,subjects=new Map<string,string[]>();
 for(const path of watched){const id=key(path),list=subjects.get(id);if(list)list.push(path);else subjects.set(id,[path]);}
 const hit=new Set<string>(),take=(id:string)=>{for(const path of subjects.get(id)??[])hit.add(path);};
 const structural=new Set<string>();
 for(const row of rows){
  const id=key(row[2]);
  take(id);
  for(let cut=id.lastIndexOf("/");cut>0;cut=id.lastIndexOf("/",cut-1))take(id.slice(0,cut));
  if(!(row[1]===1&&row[0]==="m"))structural.add(id);
 }
 if(structural.size)for(const[id,paths]of subjects){
  for(let cut=id.lastIndexOf("/");cut>0;cut=id.lastIndexOf("/",cut-1))if(structural.has(id.slice(0,cut))){for(const path of paths)hit.add(path);break;}
 }
 return hit;
}
