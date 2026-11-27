import {readFileSync,renameSync,writeFileSync} from "node:fs";
import {lstat} from "node:fs/promises";
import {provenUnchangedV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";

/** 🗞️ One recorded fact of an admission memo: the part the journal predicate reads. */
export type JournalRowV1=Readonly<{kind:string;path:string;fact?:unknown}>;
export type JournalCursorV1=Readonly<{epoch:string;seq:number}>;
export type JournalRefusedV1=Readonly<{exact:false}>;
type ChangeRowsV1=readonly (readonly [string,0|1,string])[];
/** 🗞️ The exact, fenced change feed of the workspace; the journal client of the repository satisfies it structurally.
 * @see ../../../../🏡️resident/📜️journal/🟦️.ts */
export interface JournalFeedV1{
  excluder():(path:string)=>boolean;
  cursor(signal?:AbortSignal):Promise<JournalCursorV1|JournalRefusedV1>;
  since(from:JournalCursorV1,roots:readonly string[],signal?:AbortSignal):Promise<Readonly<{exact:true;cursor:JournalCursorV1;rows:ChangeRowsV1}>|JournalRefusedV1>;
}
/** 🔗️ The directories above the vouched paths that were probed: `checked` are no symbolic links, `linked` are. */
export type LinkStateV1=Readonly<{checked:readonly string[];linked:readonly string[]}>;
/** 🗞️ The persisted statement that every row of the memo `memo` held at the cursor, with the link state of its directories. */
export type JournalAttestationV1=Readonly<{version:1;memo:string;epoch:string;seq:number;links?:LinkStateV1}>;
export type JournalCleanPortsV1=Readonly<{
  feed:JournalFeedV1;root:string;read:()=>JournalAttestationV1|undefined;write:(value:JournalAttestationV1)=>void;
  insensitive?:boolean;timeoutMilliseconds?:number;signal?:AbortSignal;
}>;
export type JournalCleanV1=Readonly<{
  clean:(row:JournalRowV1)=>boolean;
  seal:(memo:string,rows?:readonly JournalRowV1[])=>Promise<void>;
  counts:()=>Readonly<{skipped:number;checked:number;changed:number}|{skipped:number;checked:number;changed:undefined}>;
}>;

const PATH_KINDS_V1:ReadonlySet<string>=new Set(["file","exists","stat","lstat","fileExists","directoryExists","readdir","readdirNames"]);
const NONE_V1:JournalCleanV1=Object.freeze({clean:()=>false,seal:async()=>undefined,counts:()=>({skipped:0,checked:0,changed:undefined})});
const strings=(value:unknown):value is string[]=>Array.isArray(value)&&value.every(item=>typeof item==="string");
const linkStateOf=(value:unknown):LinkStateV1|undefined=>{
  if(typeof value!=="object"||value===null)return undefined;
  const {checked,linked}=value as Record<string,unknown>;
  return strings(checked)&&strings(linked)?{checked,linked}:undefined;
};

/** 🔗️ Which directories above paths are symbolic links: a change below a link target is reported at the target, never at the link path, so nothing under a link may be vouched for. Directories proven no link by an earlier state stay so unless a change touched them (a link is made by a change at its own path), directories that were links stay links; every other directory is probed once, in parallel batches, so the loop yields. */
export function linkIndexV1(root:string,stored:LinkStateV1|undefined,changes?:Readonly<{rows:ChangeRowsV1;excluded:(path:string)=>boolean;insensitive:boolean}>){
  const known=new Map<string,boolean>(),queried=new Set<string>();
  if(stored!==undefined&&changes!==undefined){
    for(const directory of provenUnchangedV1(stored.checked,changes.rows as never,changes.excluded,changes.insensitive,[]))known.set(directory,false);
    for(const directory of stored.linked)known.set(directory,true);
  }
  return {
    async among(paths:Iterable<string>):Promise<readonly string[]>{
      const directories=new Set<string>();
      for(const path of paths)for(let cut=path.lastIndexOf("/");cut>0;cut=path.lastIndexOf("/",cut-1)){const directory=path.slice(0,cut);if(directories.has(directory))break;directories.add(directory);}
      const unknown=[...directories].filter(directory=>!known.has(directory));
      for(let at=0;at<unknown.length;at+=128)await Promise.all(unknown.slice(at,at+128).map(async directory=>{
        known.set(directory,await lstat(root+"/"+directory).then(status=>status.isSymbolicLink(),()=>true));
      }));
      for(const directory of directories)queried.add(directory);
      return [...directories].filter(directory=>known.get(directory)===true);
    },
    state():LinkStateV1{
      const checked:string[]=[],linked:string[]=[];
      for(const directory of queried)(known.get(directory)===true?linked:checked).push(directory);
      return {checked,linked};
    },
  };
}

/** 🗞️ Reads the attestation file; anything unreadable or foreign is no attestation. */
export function readJournalAttestationV1(path:string):JournalAttestationV1|undefined{
  try{
    const value=JSON.parse(readFileSync(path,"utf8")) as Record<string,unknown>,keys=Object.keys(value).length,links=value.links===undefined?undefined:linkStateOf(value.links);
    if(value.links!==undefined&&links===undefined)return undefined;
    return value.version===1&&typeof value.memo==="string"&&value.memo!==""&&typeof value.epoch==="string"&&value.epoch!==""&&Number.isSafeInteger(value.seq)&&(value.seq as number)>=0&&keys===(links===undefined?4:5)?{version:1,memo:value.memo,epoch:value.epoch,seq:value.seq as number,...(links===undefined?{}:{links})}:undefined;
  }catch{return undefined;}
}

/** 🗞️ Writes the attestation atomically (temporary file, then rename); a failure leaves the previous attestation. */
export function writeJournalAttestationV1(path:string,value:JournalAttestationV1):void{
  try{const temporary=path+"."+process.pid+".tmp";writeFileSync(temporary,JSON.stringify(value));renameSync(temporary,path);}catch{}
}

const within=<T>(work:Promise<T>,milliseconds:number):Promise<T|undefined>=>new Promise(resolve=>{
  const timer=setTimeout(()=>resolve(undefined),milliseconds);
  work.then(value=>{clearTimeout(timer);resolve(value);},()=>{clearTimeout(timer);resolve(undefined);});
});

/** 🗞️ Which rows of a memo may skip their probe: a row that is a path fact under the workspace, outside the journal exclusions and outside symbolic-link directories, with no change reported at, above or below its path since the cursor of the memo's attestation, and equal to the memo's own row. The cursor is taken before any row is probed, so a change during the probe is reported by the next question. Without an exact feed, an attestation of another memo or a refusal, no row is clean (every row is probed as before). A directory row of kind `m` changes only its own entries; any other row stands for its subtree. `seal` stores the cursor with the link state, for the rows given (default: the rows of this call).
 * @see ./🧪️tests/🟦️.ts */
export async function journalCleanRowsV1(ports:JournalCleanPortsV1,memo:string|undefined,rows:readonly JournalRowV1[]):Promise<JournalCleanV1>{
  const {feed,root,timeoutMilliseconds=5000,signal}=ports,insensitive=ports.insensitive??(process.platform==="darwin"||process.platform==="win32");
  const fold=(path:string)=>insensitive?path.toLowerCase():path,prefix=fold(root.replaceAll("\\","/").replace(/\/+$/,""))+"/";
  const at=await within(feed.cursor(signal),timeoutMilliseconds);
  if(at===undefined||(at as JournalRefusedV1).exact===false)return NONE_V1;
  const cursor=at as JournalCursorV1,state={skipped:0,checked:0,changed:undefined as number|undefined};
  const attested=memo===undefined?undefined:ports.read();
  const candidatesOf=(source:readonly JournalRowV1[])=>{
    const found:{row:JournalRowV1;relative:string;folded:string}[]=[];
    for(const row of source){
      if(!PATH_KINDS_V1.has(row.kind))continue;
      const original=row.path.replaceAll("\\","/"),folded=fold(original);
      if(!folded.startsWith(prefix))continue;
      const relative=original.slice(prefix.length);
      if(relative!=="")found.push({row,relative,folded});
    }
    return found;
  };
  let answer=memo===undefined||attested===undefined||attested.memo!==memo?undefined:await within(feed.since({epoch:attested.epoch,seq:attested.seq},[""],signal),timeoutMilliseconds);
  if(answer!==undefined&&answer.exact!==true)answer=undefined;
  const excluded=feed.excluder(),links=linkIndexV1(root,attested?.links,answer===undefined?undefined:{rows:answer.rows,excluded,insensitive});
  const seal=async(sealed:string,sealRows:readonly JournalRowV1[]=rows)=>{
    try{await links.among(candidatesOf(sealRows).map(candidate=>candidate.relative));}catch{return;}
    ports.write({version:1,memo:sealed,epoch:cursor.epoch,seq:cursor.seq,links:links.state()});
  };
  if(answer===undefined)return Object.freeze({clean:()=>false,seal,counts:()=>state});
  state.changed=answer.rows.length;
  const candidates=candidatesOf(rows),linked=await links.among(candidates.map(candidate=>candidate.relative)),kept=provenUnchangedV1(candidates.map(candidate=>candidate.relative),answer.rows as never,excluded,insensitive,linked);
  const eligible=new Map<string,JournalRowV1>();
  for(const {row,relative,folded} of candidates)if(kept.has(relative))eligible.set(row.kind+"\0"+folded,row);
  const clean=(row:JournalRowV1):boolean=>{
    state.checked++;
    const known=eligible.get(row.kind+"\0"+fold(row.path.replaceAll("\\","/")));
    if(known===undefined)return false;
    if(known!==row&&JSON.stringify(known.fact)!==JSON.stringify(row.fact))return false;
    state.skipped++;return true;
  };
  return Object.freeze({clean,seal,counts:()=>state});
}

/** 🧾️ The membership records a hash may reuse: `cached` answers the record of an entry the journal vouches for (no probe), `learn` keeps a probed record, `done` stores every kept record with the cursor taken before the first probe. */
export type MembershipRecordsV1=Readonly<{
  cached:(entry:string)=>string|undefined;
  learn:(entry:string,record:string)=>void;
  done:()=>Promise<void>;
  counts:()=>Readonly<{reused:number;probed:number}>;
}>;
export type JournalMembershipPortsV1=Readonly<{feed:JournalFeedV1;root:string;file:string;insensitive?:boolean;timeoutMilliseconds?:number;signal?:AbortSignal}>;
type MembershipFileV1=Readonly<{version:1;epoch:string;seq:number;records:readonly string[];links?:LinkStateV1}>;

const NO_RECORDS_V1:MembershipRecordsV1=Object.freeze({cached:()=>undefined,learn:()=>undefined,done:async()=>undefined,counts:()=>({reused:0,probed:0})});

function readMembershipFileV1(file:string):MembershipFileV1|undefined{
  try{
    const value=JSON.parse(readFileSync(file,"utf8")) as Record<string,unknown>,links=value.links===undefined?undefined:linkStateOf(value.links);
    if(value.links!==undefined&&links===undefined)return undefined;
    return value.version===1&&typeof value.epoch==="string"&&value.epoch!==""&&Number.isSafeInteger(value.seq)&&(value.seq as number)>=0&&strings(value.records)&&Object.keys(value).length===(links===undefined?4:5)?{version:1,epoch:value.epoch,seq:value.seq as number,records:value.records,...(links===undefined?{}:{links})}:undefined;
  }catch{return undefined;}
}

/** 🧾️ Journal-vouched membership records: an entry (a file or directory path relative to the workspace, as the hash names it) whose path no change touched since the cursor of the stored records keeps its stored record; every other entry is probed as before and the hash stays the hash of the same records. Without an exact journal, a stored file of another epoch, a gap or a refusal every entry is probed.
 * @see ./🧪️tests/🟦️.ts */
export async function journalMembershipRecordsV1(ports:JournalMembershipPortsV1):Promise<MembershipRecordsV1>{
  const {feed,root,file,timeoutMilliseconds=5000,signal}=ports,insensitive=ports.insensitive??(process.platform==="darwin"||process.platform==="win32");
  const at=await within(feed.cursor(signal),timeoutMilliseconds);
  if(at===undefined||(at as JournalRefusedV1).exact===false)return NO_RECORDS_V1;
  const cursor=at as JournalCursorV1,next=new Map<string,string>(),usable=new Map<string,string>(),state={reused:0,probed:0},stored=readMembershipFileV1(file);
  let answer=stored===undefined?undefined:await within(feed.since({epoch:stored.epoch,seq:stored.seq},[""],signal),timeoutMilliseconds);
  if(answer!==undefined&&answer.exact!==true)answer=undefined;
  const excluder=feed.excluder(),links=linkIndexV1(root,stored?.links,answer===undefined?undefined:{rows:answer.rows,excluded:excluder,insensitive});
  if(stored!==undefined&&answer!==undefined){
    const entries=new Map<string,string>();
    for(const record of stored.records)try{const entry=(JSON.parse(record) as unknown[])[0];if(typeof entry==="string")entries.set(entry,record);}catch{}
    const names=[...entries.keys()],kept=provenUnchangedV1(names,answer.rows as never,excluder,insensitive,await links.among(names));
    for(const entry of kept)usable.set(entry,entries.get(entry)!);
  }
  return Object.freeze({
    cached:entry=>{const record=usable.get(entry);if(record!==undefined){state.reused++;next.set(entry,record);}return record;},
    learn:(entry,record)=>{state.probed++;next.set(entry,record);},
    done:async()=>{
      try{await links.among(next.keys());const temporary=file+"."+process.pid+".tmp";writeFileSync(temporary,JSON.stringify({version:1,epoch:cursor.epoch,seq:cursor.seq,records:[...next.values()],links:links.state()}));renameSync(temporary,file);}catch{}
    },
    counts:()=>state,
  });
}
