import {lstat,opendir,readlink} from "node:fs/promises";
import {createHash} from "node:crypto";
import {join,resolve} from "node:path";
import {readPhysicalFileV1,observePhysicalFileV1,type PhysicalFileClaimV1,type FileObservationControlV1} from "../🟦️.ts";

/** 🧵️ Operations in flight at once under one current physical authority: enough to overlap descriptor round trips with hashing, few enough to hold at most this many chunk buffers. */
export const CURRENT_PHYSICAL_CONCURRENCY_V1=8;

/** 🪜️ Runs `run` over `items` with at most `limit` calls in flight and returns the results in item order.
 *
 * Items start in order. After the first failure no further item starts, the calls already in flight settle, and the first failure is thrown, so nothing runs on past a refusal.
 * @see ../../../🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🧪️tests/🟦️.ts
 */
export async function mapBoundedV1<T,R>(items:readonly T[],limit:number,run:(item:T,index:number)=>Promise<R>):Promise<R[]>{
 if(!Number.isSafeInteger(limit)||limit<1)throw Error("Positive concurrency limit required");
 const results=new Array<R>(items.length);let next=0,failed=false,failure:unknown;
 const worker=async():Promise<void>=>{while(!failed&&next<items.length){const index=next++;try{results[index]=await run(items[index]!,index);await Promise.resolve();}catch(error){if(!failed){failed=true;failure=error;}return;}}};
 await Promise.all(Array.from({length:Math.min(limit,items.length)},worker));
 if(failed)throw failure;
 return results;
}

/** 🌲️ One exact physical child identity independent of compiler or application schemas. */
export type CurrentPhysicalEntryV1=Readonly<{path:string;kind:"file"|"directory"|"symlink"|"other";symlinkTarget:string|null}>;

/** 🚪️ Complete current-byte authority supplied by the physical owning composition. */
export interface CurrentPhysicalPortV1 {readonly control:FileObservationControlV1;read(path:string):Promise<string|Uint8Array|undefined>;digest(path:string):Promise<string|undefined>;measure(path:string):Promise<Readonly<{sha256:string;byteLength:number}>|undefined>;directoryEntries(path:string):Promise<readonly CurrentPhysicalEntryV1[]|undefined>;checkpoint():Promise<void>;recheck():Promise<void>;}

/** 🧮️ Body bytes one file custody read: a total and, per file, how often and how many bytes. In a build session every observed file is read exactly twice
 * (first observation and the final recheck) however many invocations name it, which is the hash-once proof.
 * @see ../../../🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🧪️tests/🟦️.ts */
export type CurrentPhysicalReadsV1=Readonly<{bytes:number;files:ReadonlyMap<string,Readonly<{reads:number;bytes:number}>>}>;

/** 🔐️ File claims shared by every owner of one build session: each file is hashed once while its identity (device, inode, size, modification and change time)
 * is unchanged, and rechecked once by {@link CurrentPhysicalFilesV1.recheck}. */
export class CurrentPhysicalFilesV1 {
 readonly claims=new Map<string,PhysicalFileClaimV1>();readonly identities=new Map<string,readonly bigint[]>();private bodyBytes=0;private readonly bodies=new Map<string,{reads:number;bytes:number}>();
 /** ➕️ Counts one completed body read. */
 counted(claim:PhysicalFileClaimV1):PhysicalFileClaimV1{const row=this.bodies.get(claim.path)??{reads:0,bytes:0};row.reads++;row.bytes+=claim.byteLength;this.bodies.set(claim.path,row);this.bodyBytes+=claim.byteLength;return claim;}
 /** 🧮️ The body bytes read so far, in total and per file. */
 observedReads():CurrentPhysicalReadsV1{return Object.freeze({bytes:this.bodyBytes,files:new Map([...this.bodies].map(([path,row])=>[path,Object.freeze({...row})]))});}
 /** ✅️ Re-hashes every claimed file once, each under a fresh control from `control`, and refuses any byte or identity change. */
 async recheck(control:()=>FileObservationControlV1):Promise<void>{await mapBoundedV1([...this.claims],CURRENT_PHYSICAL_CONCURRENCY_V1,async([path,claim])=>{const observation=control();if(observation.cancelled())throw Error("Current observation cancelled");const before=await lstat(path,{bigint:true}).catch(()=>undefined),identity=this.identities.get(path);if(!before||!identity||[before.dev,before.ino,before.size,before.mtimeNs,before.ctimeNs].some((value,index)=>value!==identity[index]))throw Error("Current physical identity changed before transfer");const current=this.counted(await observePhysicalFileV1(path,observation));if(current.sha256!==claim.sha256||current.byteLength!==claim.byteLength)throw Error("Current physical bytes changed before transfer");});}
}

/** 📁️ Owns cumulative asynchronous file and directory observations without retaining source bodies. File claims live in a {@link CurrentPhysicalFilesV1}
 * custody that several owners of one build session may share; a digest of an already claimed, identity-unchanged file is answered from the claim. Directory
 * rosters and absent paths are this owner's own structural observations. */
export class CurrentPhysicalOwnerV1 implements CurrentPhysicalPortV1 {
 private work=0;private bytes=0;private readonly directories=new Map<string,string>();private readonly missing=new Set<string>();
 private readonly root:string;readonly control:FileObservationControlV1;readonly custody:CurrentPhysicalFilesV1;
 constructor(root:string,control:FileObservationControlV1,custody:CurrentPhysicalFilesV1=new CurrentPhysicalFilesV1()){this.root=root;this.control=control;this.custody=custody;if(!control||![control.maxBytes,control.maxWork,control.chunkBytes].every(Number.isSafeInteger)||control.maxBytes<0||control.maxWork<1||control.chunkBytes<1||control.chunkBytes>1048576||typeof control.cancelled!=="function"||typeof control.remainingMs!=="function"||typeof control.onProgress!=="function")throw Error("Current physical authority refused");this.check();}
 private check():void{if(this.control.cancelled())throw Error("Current observation cancelled");const time=this.control.remainingMs();if(!Number.isFinite(time)||time<=0)throw Error("Current observation deadline");if(this.work>this.control.maxWork||this.bytes>this.control.maxBytes)throw Error("Current observation budget");}
 private admit(units:number,bytes=0):void{this.check();if(!Number.isSafeInteger(units)||units<0||!Number.isSafeInteger(bytes)||bytes<0||units>this.control.maxWork-this.work||bytes>this.control.maxBytes-this.bytes)throw Error("Current observation budget");this.work+=units;this.bytes+=bytes;}
 private progress():void{this.control.onProgress({phase:"read",bytes:this.bytes,totalBytes:this.bytes,work:this.work});this.check();}
 async checkpoint():Promise<void>{this.admit(1);if(this.work%128===0){this.progress();await new Promise<void>(done=>setImmediate(done));this.check();}}
 private path(path:string):string{const value=resolve(this.root,path);if(value.length>256||value.includes("\0"))throw Error("Current physical path refused");return value;}
 private async kind(path:string):Promise<"file"|"directory"|undefined>{await this.checkpoint();try{const stat=await lstat(path);this.check();if(this.missing.has(path))throw Error("Current physical absent input appeared");return stat.isFile()&&!stat.isSymbolicLink()?"file":stat.isDirectory()&&!stat.isSymbolicLink()?"directory":undefined;}catch(error){this.check();if((error as {code?:string}).code==="ENOENT"|| (error as {code?:string}).code==="ENOTDIR"){if(!this.missing.has(path)){this.admit(1,64+path.length*2);this.missing.add(path);}return;}throw error;}}
 private observation():FileObservationControlV1{let work=0,bytes=0;return{maxBytes:this.control.maxBytes-this.bytes,maxWork:this.control.maxWork-this.work,chunkBytes:this.control.chunkBytes,cancelled:()=>this.control.cancelled(),remainingMs:()=>this.control.remainingMs(),onProgress:event=>{this.admit(event.work-work,event.bytes-bytes);work=event.work;bytes=event.bytes;this.progress();}};}
 private async file<T>(path:string,observe:(control:FileObservationControlV1)=>Promise<T>):Promise<T>{await this.checkpoint();const before=await lstat(path,{bigint:true});this.check();const identity=[before.dev,before.ino,before.size,before.mtimeNs,before.ctimeNs],previous=this.custody.identities.get(path);if(!before.isFile()||before.isSymbolicLink()||previous&&previous.some((value,index)=>value!==identity[index]))throw Error("Current physical identity changed");const value=await observe(this.observation());await this.checkpoint();const after=await lstat(path,{bigint:true});this.check();if(!after.isFile()||after.isSymbolicLink()||[after.dev,after.ino,after.size,after.mtimeNs,after.ctimeNs].some((value,index)=>value!==identity[index]))throw Error("Current physical identity changed during read");if(!previous)this.admit(1,80);this.custody.identities.set(path,Object.freeze(identity));return value;}
 private retain(claim:PhysicalFileClaimV1):void{this.custody.counted(claim);const previous=this.custody.claims.get(claim.path);if(previous&&(previous.sha256!==claim.sha256||previous.byteLength!==claim.byteLength))throw Error("Current physical bytes changed");if(!previous)this.admit(1,128+claim.path.length*2);this.custody.claims.set(claim.path,claim);}
 private async claimed(path:string):Promise<PhysicalFileClaimV1|undefined>{const claim=this.custody.claims.get(path),identity=this.custody.identities.get(path);if(!claim||!identity)return;await this.checkpoint();const stat=await lstat(path,{bigint:true}).catch(()=>undefined);this.check();return stat&&stat.isFile()&&!stat.isSymbolicLink()&&[stat.dev,stat.ino,stat.size,stat.mtimeNs,stat.ctimeNs].every((value,index)=>value===identity[index])?claim:undefined;}
 /** 🧮️ The body bytes this owner's file custody read so far, in total and per file. */
 observedReads():CurrentPhysicalReadsV1{return this.custody.observedReads();}
 async read(path:string):Promise<Uint8Array|undefined>{const absolute=this.path(path);if(await this.kind(absolute)!=="file")return;const read=await this.file(absolute,control=>readPhysicalFileV1(absolute,control));this.check();this.retain(read.claim);return read.bytes;}
 /** 📏️ A regular file's digest and byte length from one streamed read, answered from the session claim while the file's identity is unchanged; `undefined` for anything but a regular file. */
 async measure(path:string):Promise<Readonly<{sha256:string;byteLength:number}>|undefined>{const absolute=this.path(path),known=await this.claimed(absolute);if(known)return{sha256:known.sha256,byteLength:known.byteLength};if(await this.kind(absolute)!=="file")return;const claim=await this.file(absolute,control=>observePhysicalFileV1(absolute,control));this.check();this.retain(claim);return{sha256:claim.sha256,byteLength:claim.byteLength};}
 async digest(path:string):Promise<string|undefined>{const absolute=this.path(path),known=await this.claimed(absolute);if(known)return known.sha256;const kind=await this.kind(absolute);if(kind==="file"){const claim=await this.file(absolute,control=>observePhysicalFileV1(absolute,control));this.check();this.retain(claim);return claim.sha256;}if(kind==="directory"){const entries=await this.directoryEntries(absolute);if(!entries)return;const source=JSON.stringify(entries.map(entry=>[entry.path.slice(absolute.length+1),entry.kind,entry.symlinkTarget]));this.admit(1,Buffer.byteLength(source));return createHash("sha256").update(source).digest("hex");}return;}
 async directoryEntries(path:string):Promise<readonly CurrentPhysicalEntryV1[]|undefined>{
  const absolute=this.path(path);if(await this.kind(absolute)!=="directory")return;await this.checkpoint();const before=await lstat(absolute,{bigint:true});this.check();await this.checkpoint();const handle=await opendir(absolute,{bufferSize:1}),rows:CurrentPhysicalEntryV1[]=[];
  try{for(;;){await this.checkpoint();const entry=await handle.read();this.check();if(!entry)break;const child=join(absolute,entry.name);await this.checkpoint();const stat=await lstat(child);this.check();let symlinkTarget:string|null=null;if(stat.isSymbolicLink()){await this.checkpoint();symlinkTarget=await readlink(child);this.check();}this.admit(1,64+child.length*2+(symlinkTarget?.length??0)*2);rows.push({path:child,kind:stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":"other",symlinkTarget});}
   this.admit(1,rows.length*16);for(let width=1;width<rows.length;width*=2){const sorted:CurrentPhysicalEntryV1[]=[];for(let start=0;start<rows.length;start+=width*2){let left=start,right=Math.min(start+width,rows.length),middle=right,end=Math.min(start+width*2,rows.length);while(left<middle||right<end){await this.checkpoint();sorted.push(right>=end||left<middle&&Buffer.compare(Buffer.from(rows[left]!.path),Buffer.from(rows[right]!.path))<=0?rows[left++]!:rows[right++]!);}}for(let index=0;index<rows.length;index++){await this.checkpoint();rows[index]=sorted[index]!;}}
   await this.checkpoint();const after=await lstat(absolute,{bigint:true});this.check();if(!after.isDirectory()||before.dev!==after.dev||before.ino!==after.ino||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Current directory changed");const source=JSON.stringify(rows.map(entry=>[entry.path.slice(absolute.length+1),entry.kind,entry.symlinkTarget]));this.admit(1,Buffer.byteLength(source));const sha256=createHash("sha256").update(source).digest("hex"),previous=this.directories.get(absolute);if(previous&&previous!==sha256)throw Error("Current directory roster changed");this.directories.set(absolute,sha256);return rows;
  }finally{await handle.close();}
 }
 /** 📐️ Rechecks this owner's own structural observations: every directory roster and every absent path. */
 async recheckStructure():Promise<void>{for(const [path,sha256]of this.directories){await this.checkpoint();const entries=await this.directoryEntries(path);if(!entries||createHash("sha256").update(JSON.stringify(entries.map(entry=>[entry.path.slice(path.length+1),entry.kind,entry.symlinkTarget]))).digest("hex")!==sha256)throw Error("Current directory changed before transfer");}for(const path of this.missing){await this.checkpoint();try{await lstat(path);this.check();throw Error("Current physical absent input appeared before transfer");}catch(error){this.check();if((error as {code?:string}).code!=="ENOENT"&&(error as {code?:string}).code!=="ENOTDIR")throw error;}}this.progress();}
 async recheck():Promise<void>{await this.custody.recheck(()=>this.observation());await this.recheckStructure();}
}
