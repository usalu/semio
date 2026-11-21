import {lstat,rm} from "node:fs/promises";
import policy from "./🔣️.json" with {type:"json"};

type Put=Readonly<{hlc:string;sha256:string;bytes:number;executable:boolean}>;
type Proof=string;
type Stage={put:Put;file:string;source:string;original:Proof;image:Proof;started:number;invalid:boolean;removing?:Promise<void>};
type Current=(path:string,put:Put)=>boolean;
const exact=(a:Put,b:Put)=>a.hlc===b.hlc&&a.sha256===b.sha256&&a.bytes===b.bytes&&a.executable===b.executable;

/** 🔏️ Reads exact no-follow identity for a regular, single-link staged or CAS file. */
export async function stagedPutStampV1(path:string,bytes:number):Promise<Proof|undefined>{
 try{
  const row=await lstat(path,{bigint:true});
  if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1n||row.ino===0n||row.size!==BigInt(bytes))return undefined;
  return [row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");
 }catch{return undefined;}
}

/** 📥️ Keeps verified private inputs briefly when an ordinary worktree destination rejects replacement. */
export function createStagedPutsV1(){
 const rows=new Map<string,Stage>(),owners=new Set<string>();
 let allocated=0,closed=false,epoch=0;
 const leave=(path:string,row:Stage)=>{if(rows.get(path)===row){rows.delete(path);allocated-=row.put.bytes;}};
 const drop=(path:string,row:Stage)=>{
  row.invalid=true;
  return row.removing??=(async()=>{try{await rm(row.file,{force:true});leave(path,row);}catch{}finally{row.removing=undefined;}})();
 };
 const drain=async(stale:readonly [string,Stage][])=>{
  for(let offset=0;offset<stale.length;offset+=16)await Promise.all(stale.slice(offset,offset+16).map(([path,row])=>drop(path,row)));
 };
 const live=(row:Stage,now:number)=>!row.invalid&&now>=row.started&&now-row.started<policy.maximumAgeMilliseconds;
 const admits=(put:Put)=>!closed&&Number.isSafeInteger(put.bytes)&&put.bytes>=0&&put.bytes<=policy.maximumFileBytes&&rows.size<policy.maximumFiles&&allocated+put.bytes<=policy.maximumBytes;
 return {
  begin(path:string){if(closed||owners.has(path))return false;owners.add(path);return true;},
  async end(path:string){const row=rows.get(path);if(row?.invalid)await drop(path,row);owners.delete(path);},
  admits,
  async find(path:string,put:Put,source:string,now:number):Promise<Stage|undefined>{
   const row=rows.get(path);if(!row)return undefined;
   if(!live(row,now)||!exact(row.put,put)||row.source!==source){await drop(path,row);return undefined;}
   const [original,image]=await Promise.all([stagedPutStampV1(source,put.bytes),stagedPutStampV1(row.file,put.bytes)]);
   if(rows.get(path)===row&&!row.invalid&&original===row.original&&image===row.image)return row;
   await drop(path,row);return undefined;
  },
  async retain(path:string,put:Put,file:string,source:string,original:Proof,image:Proof,now:number,current:Current,signal?:AbortSignal):Promise<boolean>{
   if(rows.has(path)||!admits(put))return false;
   const turn=epoch,checked=await stagedPutStampV1(source,put.bytes);
   if(checked!==original||turn!==epoch||signal?.aborted||rows.has(path)||!admits(put)||!current(path,put))return false;
   rows.set(path,{put,file,source,original,image,started:now,invalid:false});allocated+=put.bytes;
   return true;
  },
  async invalidate(path:string){const row=rows.get(path);if(row)await drop(path,row);},
  applied(path:string,row:Stage){leave(path,row);},
  async sweep(now:number,current:Current){
   await drain([...rows].filter(([path,row])=>!owners.has(path)&&(!live(row,now)||!current(path,row.put))));
  },
  async clear(close=false){
   if(close)closed=true;epoch++;
   const all=[...rows];
   for(const [,row] of all)row.invalid=true;
   await drain(all.filter(([path])=>!owners.has(path)));
  },
 };
}
