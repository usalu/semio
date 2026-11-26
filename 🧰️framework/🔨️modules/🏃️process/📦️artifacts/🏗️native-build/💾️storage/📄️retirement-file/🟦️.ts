/** 📖️ Reads one bounded native image retirement file through a pinned descriptor and returns its bytes with its identity stamp; the storage core and both retirement backends read receipts and claim sidecars through it, so it sits below all of them. */

import { lstatSync, readSync, openSync, closeSync, constants, fstatSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { descriptorProtocolV1, descriptorRefusalV1 } from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🧷️descriptor/🟦️.ts";
import { isConcurrentEditV1 } from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

const retirementPause=new Int32Array(new SharedArrayBuffer(4));
/** 📄️ Reads a bounded single-owner retirement record through the same no-follow descriptor and stable identity (class `immutable` of the descriptor protocol): a record that changed while it was read is read again after each declared pause, and only a record that keeps changing is refused as a concurrent edit in both languages. */
export function readNativeImageRetirementFileV1(path:string,maximumBytes:number):Readonly<{bytes:Buffer;stamp:string}>{
 const pauses=descriptorProtocolV1().pausesMilliseconds;
 for(let attempt=0;;attempt++){
  try{return readNativeImageRetirementFileOnceV1(path,maximumBytes);}
  catch(error){
   if((error as Error).message!=="Original image retirement record changed")throw error;
   if(attempt>=pauses.length)throw descriptorRefusalV1("immutable",path,{en:"Image retirement record",de:"Abräumvermerk des Abbilds"});
   Atomics.wait(retirementPause,0,0,pauses[attempt]!);
  }
 }
}
function readNativeImageRetirementFileOnceV1(path:string,maximumBytes:number):Readonly<{bytes:Buffer;stamp:string}>{
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<=0||!Number.isSafeInteger(maximumBytes+1))throw Error("Bounded original image retirement record required");
 let ancestor=dirname(resolve(path));for(;;){const row=lstatSync(ancestor);if(!row.isDirectory()||row.isSymbolicLink())throw Error("Original image retirement ancestry required");const parent=dirname(ancestor);if(parent===ancestor)break;ancestor=parent;}const stamp=(row:NonNullable<ReturnType<typeof lstatSync>>)=>[row.dev,row.ino,row.nlink,row.size,row.mtimeMs,row.ctimeMs].join(":"),before=lstatSync(path);if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1||!Number.isSafeInteger(before.size)||before.size<0||before.size>maximumBytes)throw Error("Bounded original image retirement record required");const fd=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));try{if(stamp(fstatSync(fd))!==stamp(before))throw Error("Original image retirement record changed");const buffer=Buffer.alloc(before.size+1);let length=0;while(length<buffer.length){const count=readSync(fd,buffer,length,buffer.length-length,null);if(count===0)break;length+=count;}const bytes=buffer.subarray(0,length),after=lstatSync(path);if(bytes.length!==before.size||stamp(after)!==stamp(before)||stamp(fstatSync(fd))!==stamp(before))throw Error("Original image retirement record changed");return{bytes,stamp:stamp(before)};}finally{closeSync(fd);}
}

/** 🔁️ Re-reads a record whose metadata stamp moved under a live replicated worktree and adopts it only while its bytes stay identical to the held bytes; a bounded number of rescans converge on a quiet stamp, and changed content or an unquiet record beyond the bound is refused. */
export function rescanNativeImageRetirementFileV1(path:string,maximumBytes:number,held:Readonly<{bytes:Buffer;stamp:string}>,policy:Readonly<{rescans:number;delayMilliseconds:number}>,onRescan?:(attempt:number)=>void):Readonly<{bytes:Buffer;stamp:string}>{
 const pause=new Int32Array(new SharedArrayBuffer(4));
 for(let attempt=0;attempt<=policy.rescans;attempt++){
  if(attempt>0){onRescan?.(attempt);Atomics.wait(pause,0,0,policy.delayMilliseconds);}
  let read:Readonly<{bytes:Buffer;stamp:string}>;
  try{read=readNativeImageRetirementFileV1(path,maximumBytes);}catch(error){if(isConcurrentEditV1(error))continue;throw error;}
  if(!read.bytes.equals(held.bytes))throw Error("Original Native terminal record changed");
  return read;
 }
 throw Error("Original Native terminal record changed");
}
