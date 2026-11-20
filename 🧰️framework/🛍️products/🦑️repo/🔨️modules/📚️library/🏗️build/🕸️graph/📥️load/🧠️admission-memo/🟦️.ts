import{randomUUID}from"node:crypto";
import{mkdir,readFile,rename,rm,writeFile}from"node:fs/promises";
import{dirname}from"node:path";
import{readBuildOwnerAdmissionV1,type BuildOwnerAdmissionV1}from"../../🚪️admission/🟦️.ts";

/** 🧠️ What a selected read remembers on the machine: the closed owner admission of one scope, every observation it was derived from and the admission facts of each owner with the rows (indexes into `observations`) that owner was observed through, under the admission format that derived them. @see ../🟦️.ts */
export type AdmissionMemoV1={version:2;format:string;scope:string;admission:BuildOwnerAdmissionV1;observations:readonly unknown[];owners:readonly {suffix:string;facts:unknown;rows:readonly number[]}[]};

/** 📦️ The admission facts of one owner as the process remembers them. */
export type OwnerFactsEntryV1={suffix:string;facts:unknown;rows:readonly unknown[]};

/** 📖️ The remembered admission of a scope, or nothing: a missing, corrupt or foreign (scope, shape) memo, or one of another admission format (`accept`), is a miss, never an error. Whether its observations still hold is asked by the caller, and a memo that does not hold still lends the facts of its owners (`ownersOfAdmissionMemoV1`), each of which revalidates its own rows. */
export async function readAdmissionMemoV1(path:string,scope:string,accept:(value:AdmissionMemoV1)=>boolean):Promise<AdmissionMemoV1|undefined>{
 let value:AdmissionMemoV1;
 try{value=JSON.parse(await readFile(path,"utf8"));if(value?.version!==2||value.scope!==scope||!Array.isArray(value.observations)||!Array.isArray(value.owners))return undefined;readBuildOwnerAdmissionV1(value.admission);}catch{return undefined;}
 return accept(value)?value:undefined;
}

/** 🗜️ A memo of the observations and of the owners, the rows of each owner stored once as indexes into the observations (an owner row that the capture did not retain is appended). */
export function packAdmissionMemoV1(input:{format:string;scope:string;admission:BuildOwnerAdmissionV1;observations:readonly unknown[];owners:readonly OwnerFactsEntryV1[]}):AdmissionMemoV1{
 const observations=[...input.observations],index=new Map<string,number>();
 observations.forEach((row,at)=>{const key=JSON.stringify(row);if(!index.has(key))index.set(key,at);});
 const at=(row:unknown)=>{const key=JSON.stringify(row);let found=index.get(key);if(found===undefined){found=observations.push(row)-1;index.set(key,found);}return found;};
 return {version:2,format:input.format,scope:input.scope,admission:input.admission,observations,owners:input.owners.map(owner=>({suffix:owner.suffix,facts:owner.facts,rows:owner.rows.map(at)}))};
}

/** 📤️ Resolves the rows of owners missing from the current admission cache. Malformed owner entries are skipped. */
export function ownersOfAdmissionMemoV1(memo:AdmissionMemoV1,known?:ReadonlySet<string>):OwnerFactsEntryV1[]{
 const owners:OwnerFactsEntryV1[]=[];
 for(const owner of memo.owners){
  if(typeof owner?.suffix!=="string"||known?.has(owner.suffix)||!Object.hasOwn(owner,"facts")||!Array.isArray(owner.rows))continue;
  if(!owner.rows.every(at=>Number.isSafeInteger(at)&&at>=0&&at<memo.observations.length))continue;
  const rows=owner.rows.map(at=>memo.observations[at]);
  if(rows.every(row=>row!==undefined))owners.push({suffix:owner.suffix,facts:owner.facts,rows});
 }
 return owners;
}

/** 💾️ Remembers an admission by writing a temporary file and renaming it, so a reader sees a whole memo or none; a memo above `maximumBytes` is not written (answers false). */
export async function writeAdmissionMemoV1(path:string,value:AdmissionMemoV1,maximumBytes:number):Promise<boolean>{
 const text=JSON.stringify(value);if(Buffer.byteLength(text)>maximumBytes)return false;
 const temporary=path+"."+randomUUID();
 try{await mkdir(dirname(path),{recursive:true});await writeFile(temporary,text,{flag:"wx"});await rename(temporary,path);return true;}
 finally{await rm(temporary,{force:true});}
}
