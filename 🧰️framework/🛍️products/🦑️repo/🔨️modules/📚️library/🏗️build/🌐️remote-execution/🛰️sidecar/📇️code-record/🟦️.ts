import {mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {machinePathV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";

/** 📇️ What one sidecar process says about its code: the digest of the closure it started with (`running`), the digest of the same closure on disk now (`disk`) and since when they differ. */
export type SidecarCodeRecordV1=Readonly<{version:1;role:string;pid:number;startedMs:number;atMs:number;running:string;disk:string;current:boolean;staleSinceMs:number|null}>;

/** 📇️ One role in `sidecar-code.json` of the fleet store: the record with its age, digests cut to 12 hex digits. */
export type SidecarCodeRowV1=Readonly<{pid:number;startedMs:number;running:string;disk:string;current:boolean;staleSinceMs:number|null;ageSeconds:number}>;

/** 📇️ `v1/origins/<origin>/sidecar-code.json`: the code every role of one machine runs against the code on its disk, the truth behind "the fleet runs current code". */
export type SidecarCodeMirrorV1=Readonly<{version:1;origin:string;atMs:number;current:boolean;roles:Readonly<Record<string,SidecarCodeRowV1>>}>;

const SHORT=12,FRESH_MILLISECONDS=120_000,REWRITE_MILLISECONDS=60_000,STORE_REWRITE_MILLISECONDS=60_000,READ_MILLISECONDS=10_000;

/** 📍️ The machine file of one role's record, below `.cache/f/sidecars`. */
export const sidecarCodeFileV1=(root:string,role:string):string=>machinePathV1(root,".cache","f","sidecars",`code-${role}.json`);

/** 📍️ The file of one origin in the fleet store. */
export const sidecarCodeStoreFileV1=(store:string,origin:string):string=>join(store,"v1","origins",origin,"sidecar-code.json");

const writeAtomicV1=(file:string,text:string):void=>{
 mkdirSync(dirname(file),{recursive:true});
 const temporary=`${file}.${process.pid}.tmp`;
 writeFileSync(temporary,text);renameSync(temporary,file);
};

/** 🖋️ Keeps the record of one role: rewritten when `current` flips, when the disk digest moves, and at least every minute (inside the freshness bound of the table); `staleSinceMs` is the first look that saw a disk digest other than the running one (kept while it stays different). */
export function createSidecarCodeRecorderV1(input:Readonly<{root:string;role:string;pid?:number;startedMs?:number;now?:()=>number}>):Readonly<{record(running:string,disk:string):SidecarCodeRecordV1}>{
 const now=input.now??Date.now,pid=input.pid??process.pid,startedMs=input.startedMs??now();
 let staleSince:number|null=null,lastDisk:string|undefined,lastCurrent:boolean|undefined,lastWrite=0;
 return Object.freeze({record(running:string,disk:string):SidecarCodeRecordV1{
  const at=now(),current=running===disk;
  staleSince=current?null:staleSince??at;
  const row:SidecarCodeRecordV1={version:1,role:input.role,pid,startedMs,atMs:at,running:running.slice(0,SHORT),disk:disk.slice(0,SHORT),current,staleSinceMs:staleSince};
  if(lastDisk!==row.disk||lastCurrent!==current||at-lastWrite>=REWRITE_MILLISECONDS){writeAtomicV1(sidecarCodeFileV1(input.root,input.role),JSON.stringify(row));lastDisk=row.disk;lastCurrent=current;lastWrite=at;}
  return row;
 }});
}

/** 📖️ The record of one role, `undefined` when there is none or it cannot be read. */
export function readSidecarCodeRecordV1(root:string,role:string):SidecarCodeRecordV1|undefined{
 try{
  const row=JSON.parse(readFileSync(sidecarCodeFileV1(root,role),"utf8")) as Partial<SidecarCodeRecordV1>;
  return row.version===1&&row.role===role&&typeof row.pid==="number"&&typeof row.running==="string"&&typeof row.disk==="string"?row as SidecarCodeRecordV1:undefined;
 }catch{return undefined;}
}

/** 🪞️ The pure table of the roles of one machine: a record older than two minutes means that process is gone (its row says `current: false` with its age, so a dead role never counts as current). */
export function sidecarCodeMirrorOfV1(input:Readonly<{origin:string;rows:Readonly<Record<string,SidecarCodeRecordV1|undefined>>;now:number}>):SidecarCodeMirrorV1{
 const roles:Record<string,SidecarCodeRowV1>={};
 for(const [role,row] of Object.entries(input.rows)){
  if(row===undefined)continue;
  const ageSeconds=Math.max(0,Math.round((input.now-row.atMs)/1000)),fresh=input.now-row.atMs<FRESH_MILLISECONDS;
  roles[role]={pid:row.pid,startedMs:row.startedMs,running:row.running,disk:row.disk,current:fresh&&row.current,staleSinceMs:row.staleSinceMs,ageSeconds};
 }
 return Object.freeze({version:1,origin:input.origin,atMs:input.now,current:Object.values(roles).length>0&&Object.values(roles).every(row=>row.current),roles:Object.freeze(roles)});
}

/** 📡️ Publishes the table of this machine into the fleet store (rewritten when a role flips or every minute), so `repo fleet status` and the convergence table read one file per origin. A missing store says nothing. */
export function createSidecarCodeMirrorV1(input:Readonly<{root:string;store:string|undefined;origin:string;roles:readonly string[];now?:()=>number}>):Readonly<{tick():SidecarCodeMirrorV1|undefined}>{
 const now=input.now??Date.now;
 let lastKey="",lastWrite=0,lastRead=-Infinity,last:SidecarCodeMirrorV1|undefined;
 return Object.freeze({tick():SidecarCodeMirrorV1|undefined{
  if(input.store===undefined||input.store==="")return undefined;
  if(now()-lastRead<READ_MILLISECONDS&&last!==undefined)return last;
  lastRead=now();
  const rows:Record<string,SidecarCodeRecordV1|undefined>={};
  for(const role of input.roles)rows[role]=readSidecarCodeRecordV1(input.root,role);
  const at=now(),table=sidecarCodeMirrorOfV1({origin:input.origin,rows,now:at}),key=JSON.stringify(Object.entries(table.roles).map(([role,row])=>[role,row.pid,row.running,row.disk,row.current]));
  last=table;
  if(key===lastKey&&at-lastWrite<STORE_REWRITE_MILLISECONDS)return table;
  try{writeAtomicV1(sidecarCodeStoreFileV1(input.store,input.origin),JSON.stringify(table));lastKey=key;lastWrite=at;}catch{}
  return table;
 }});
}
