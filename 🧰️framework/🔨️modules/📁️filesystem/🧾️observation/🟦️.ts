import {constants} from "node:fs";
import {lstat,open} from "node:fs/promises";
import {createHash} from "node:crypto";
import {isAbsolute} from "node:path";

/** 🧾️ One regular physical file observation, independent of any catalog or application. */
export type PhysicalFileClaimV1=Readonly<{path:string;sha256:string;byteLength:number}>;
/** 📥️ The admitted bytes transferred to the caller with their same-descriptor claim. */
export type PhysicalFileReadV1=Readonly<{claim:PhysicalFileClaimV1;bytes:Uint8Array}>;
/** 📊️ Filesystem operations and exact bytes observed under the supplied authority. */
export type FileObservationProgressV1=Readonly<{phase:"admit"|"opened"|"read"|"complete";bytes:number;totalBytes:number;work:number}>;
/** 🎛️ Required finite byte, operation, chunk, cancellation, deadline and progress authority. */
export type FileObservationControlV1=Readonly<{maxBytes:number;maxWork:number;chunkBytes:number;cancelled():boolean;remainingMs():number;onProgress(progress:FileObservationProgressV1):void}>;
/** 🛑️ First-party refusal categories with no backend type in the public interface. */
export type FileObservationRefusalV1="invalid-control"|"not-regular"|"byte-capacity"|"work-capacity"|"cancelled"|"deadline"|"changed"|"io";
/** ⚠️ Retains the owned refusal identity across filesystem backends. */
export class FileObservationErrorV1 extends Error{constructor(readonly code:FileObservationRefusalV1,readonly path:string){super("Physical file observation: "+code+": "+path);this.name="FileObservationErrorV1";}}
const integer=(value:number,minimum:number)=>Number.isSafeInteger(value)&&value>=minimum;

async function physicalFileObservationV1(path:string,control:FileObservationControlV1,retain:boolean):Promise<{claim:PhysicalFileClaimV1;bytes:Uint8Array|null}>{
 const refuse=(code:FileObservationRefusalV1):never=>{throw new FileObservationErrorV1(code,path);};
 if(typeof path!=="string"||!isAbsolute(path)||!control||!integer(control.maxBytes,0)||!integer(control.maxWork,1)||!integer(control.chunkBytes,1)||control.chunkBytes>1048576||typeof control.cancelled!=="function"||typeof control.remainingMs!=="function"||typeof control.onProgress!=="function")refuse("invalid-control");
 let work=0,length=0,total=0,file:Awaited<ReturnType<typeof open>>|undefined,chunk:Buffer|undefined,retained:Uint8Array|undefined;
 const check=()=>{if(control.cancelled())refuse("cancelled");const remaining=control.remainingMs();if(!Number.isFinite(remaining))refuse("invalid-control");if(remaining<=0)refuse("deadline");};
 const admit=()=>{check();if(work>=control.maxWork)refuse("work-capacity");work++;};
 const progress=(phase:FileObservationProgressV1["phase"])=>{control.onProgress(Object.freeze({phase,bytes:length,totalBytes:total,work}));check();};
 try{
  admit();const before=await lstat(path);check();
  if(!before.isFile()||before.isSymbolicLink()||!integer(before.size,0))refuse("not-regular");total=before.size;if(total>control.maxBytes)refuse("byte-capacity");if(Math.ceil(total/control.chunkBytes)+5>control.maxWork)refuse("work-capacity");progress("admit");
  const same=(stat:typeof before)=>stat.isFile()&&!stat.isSymbolicLink()&&stat.dev===before.dev&&stat.ino===before.ino&&stat.size===before.size&&stat.mtimeMs===before.mtimeMs&&stat.ctimeMs===before.ctimeMs;
  admit();file=await open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));check();admit();const opened=await file.stat();check();if(!same(opened))refuse("changed");progress("opened");
  const hash=createHash("sha256");chunk=Buffer.alloc(Math.min(control.chunkBytes,Math.max(1,total)));if(retain)retained=new Uint8Array(total);
  while(length<total){admit();const read=await file.read(chunk,0,Math.min(chunk.length,total-length),length);check();if(read.bytesRead===0)refuse("changed");const bytes=chunk.subarray(0,read.bytesRead);retained?.set(bytes,length);length+=read.bytesRead;hash.update(bytes);progress("read");}
  admit();const after=await file.stat();check();admit();const linked=await lstat(path);check();if(length!==total||!same(after)||!same(linked))refuse("changed");const claim=Object.freeze({path,sha256:hash.digest("hex"),byteLength:length});await file.close();file=undefined;progress("complete");const bytes=retained??null;retained=undefined;return{claim,bytes};
 }catch(error){if(error instanceof FileObservationErrorV1)throw error;throw new FileObservationErrorV1("io",path);}
 finally{chunk?.fill(0);retained?.fill(0);if(file)try{await file.close();}catch{throw new FileObservationErrorV1("io",path);}}
}

/** 🔍️ Hashes bounded asynchronous chunks while joining descriptor and path identity before and after reading.
 * @see ./🧬️schema/🔣️.json
 */
export async function observePhysicalFileV1(path:string,control:FileObservationControlV1):Promise<PhysicalFileClaimV1>{return(await physicalFileObservationV1(path,control,false)).claim;}

/** 📖️ Reads only after exact byte/work admission and transfers one private, custody-checked output.
 * @see ./🧫️fixtures/🔣️.json
 */
export async function readPhysicalFileV1(path:string,control:FileObservationControlV1):Promise<PhysicalFileReadV1>{const observed=await physicalFileObservationV1(path,control,true);if(observed.bytes===null)throw new FileObservationErrorV1("io",path);return Object.freeze({claim:observed.claim,bytes:observed.bytes});}
