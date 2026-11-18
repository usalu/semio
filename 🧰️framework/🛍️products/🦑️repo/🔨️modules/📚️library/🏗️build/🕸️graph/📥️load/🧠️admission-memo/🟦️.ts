import{randomUUID}from"node:crypto";
import{mkdir,readFile,rename,rm,writeFile}from"node:fs/promises";
import{dirname}from"node:path";
import{readBuildOwnerAdmissionV1,type BuildOwnerAdmissionV1}from"../../🚪️admission/🟦️.ts";

/** 🧠️ What a selected read remembers on the machine: the closed owner admission of one scope and every observation it was derived from, under the producer that derived it. @see ../🟦️.ts */
export type AdmissionMemoV1={version:1;module:{implementation:string;publication:string};scope:string;admission:BuildOwnerAdmissionV1;observations:readonly unknown[]};

/** 📖️ The remembered admission of a scope, or nothing: a missing, corrupt, foreign (scope, producer, shape) or stale memo (`holds` answers whether every observation still holds) is a miss, never an error, so the admission is derived again. */
export async function readAdmissionMemoV1(path:string,scope:string,accept:(value:AdmissionMemoV1)=>boolean,holds:(observations:readonly unknown[])=>Promise<boolean>):Promise<AdmissionMemoV1|undefined>{
 let value:AdmissionMemoV1;
 try{value=JSON.parse(await readFile(path,"utf8"));if(value?.version!==1||value.scope!==scope||!Array.isArray(value.observations))return undefined;readBuildOwnerAdmissionV1(value.admission);}catch{return undefined;}
 return accept(value)&&await holds(value.observations)?value:undefined;
}

/** 💾️ Remembers an admission by writing a temporary file and renaming it, so a reader sees a whole memo or none; a memo above `maximumBytes` is not written (answers false). */
export async function writeAdmissionMemoV1(path:string,value:AdmissionMemoV1,maximumBytes:number):Promise<boolean>{
 const text=JSON.stringify(value);if(Buffer.byteLength(text)>maximumBytes)return false;
 const temporary=path+"."+randomUUID();
 try{await mkdir(dirname(path),{recursive:true});await writeFile(temporary,text,{flag:"wx"});await rename(temporary,path);return true;}
 finally{await rm(temporary,{force:true});}
}
