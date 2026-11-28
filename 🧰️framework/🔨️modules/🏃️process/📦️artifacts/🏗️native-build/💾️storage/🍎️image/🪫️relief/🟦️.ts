import{closeSync,fsyncSync,openSync,statfsSync,unlinkSync,writeSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import{isDiskFullV1}from"../../../../../🆓️free-space/💽️disk-full/🟦️.ts";
const protocol=schema.definitions.Protocol.const;
/** 🪫️ The authored protocol of the image relief: the clamp slack, the usable floor, the attempts and backoff of a writer, the host watermarks and the texts.
 * @see ./🧬️schema/🔣️.json */
export const NATIVE_IMAGE_RELIEF_V1=protocol;
/** 🔭️ What a mounted image says about its space: what its filesystem holds free inside, what it reports as available (clamped by the host), the part of the inside it does not report (`phantomBytes`), whether that part is a clamp (`clamped`) and whether the image is starved: room inside, but less usable than the floor. */
export type NativeImageClampV1=Readonly<{capacityBytes:number;usedBytes:number;availableBytes:number;innerFreeBytes:number;phantomBytes:number;clamped:boolean;starved:boolean}>;
/** 🔭️ Reads the clamp of one mounted image from its capacity, the bytes it really holds (`diskutil`, not `statfs`) and the bytes `statfs` reports as available. Pure and language-agnostic. */
export function nativeImageClampV1(facts:Readonly<{capacityBytes:number;usedBytes:number;availableBytes:number}>):NativeImageClampV1{
 const innerFreeBytes=Math.max(0,facts.capacityBytes-facts.usedBytes),phantomBytes=Math.max(0,innerFreeBytes-facts.availableBytes);
 return Object.freeze({...facts,innerFreeBytes,phantomBytes,clamped:phantomBytes>protocol.slackBytes,starved:facts.availableBytes<protocol.floorFreeBytes&&innerFreeBytes>=protocol.floorFreeBytes+protocol.slackBytes});
}
/** 📐️ Whether an image is re-created smaller: it is clamped and its declared capacity exceeds the capacity it is sized for by at least one alignment step, so the host can back its whole declaration again. A busy image stays until it is idle. Pure and language-agnostic. */
export function nativeImageRightSizeV1(facts:Readonly<{capacityBytes:number;sizedBytes:number;clamped:boolean}>):Readonly<{retire:boolean;targetBytes:number}>{
 return Object.freeze({retire:facts.clamped&&facts.capacityBytes-facts.sizedBytes>=protocol.alignBytes,targetBytes:Math.min(facts.capacityBytes,facts.sizedBytes)});
}
/** 📉️ What a clamped image is shrunk in place to: the larger of the capacity it is sized for and twice what it holds (aligned up), never above its declaration; it shrinks only when it is clamped and the host-backed capacity falls short by at least one alignment step. The content stays, unlike the whole re-creation of `nativeImageRightSizeV1`. Pure and language-agnostic. */
export type NativeImageShrinkV1=Readonly<{capacityBytes:number;usedBytes:number;sizedBytes:number;clamped:boolean;shrink:boolean;targetBytes:number}>;
export function nativeImageShrinkV1(facts:Readonly<{capacityBytes:number;usedBytes:number;sizedBytes:number;clamped:boolean}>):NativeImageShrinkV1{
 const keepBytes=Math.ceil(facts.usedBytes*2/protocol.alignBytes)*protocol.alignBytes,targetBytes=Math.min(facts.capacityBytes,Math.max(facts.sizedBytes,keepBytes));
 return Object.freeze({...facts,shrink:facts.clamped&&facts.capacityBytes-targetBytes>=protocol.alignBytes,targetBytes});
}
/** 🔌️ What a write probe touches: the bytes a statfs of the mount reports as available, one synced write of the given size and its removal. Replaceable in a test. */
export type NativeImageWritesIoV1=Readonly<{available(mountPoint:string):number;write(file:string,bytes:number):void;remove(file:string):void}>;
const PROBE_BYTES=65536;
const REAL_WRITES_IO:NativeImageWritesIoV1=Object.freeze({available:mountPoint=>{const row=statfsSync(mountPoint);return Number(row.bavail)*row.bsize;},write:(file,bytes)=>{const descriptor=openSync(file,"w");try{writeSync(descriptor,Buffer.alloc(bytes));fsyncSync(descriptor);}finally{closeSync(descriptor);}},remove:file=>{try{unlinkSync(file);}catch{}}});
/** 🔬️ What one mounted image answered to a small write: whether the write was taken, what `statfs` reports as available and, when refused, the error code. */
export type NativeImageWritesProbeV1=Readonly<{mountPoint:string;writable:boolean;availableBytes:number;reason?:string}>;
/** 🔬️ Writes and syncs one small file below the mount point of an image and removes it again, so an image that answers every write with `ENOSPC` (a clamp of the host: nearly empty inside, nothing backed) is found before a build starts instead of 30 minutes into it. An error that is no full disk is answered by its code, never thrown. */
export function probeNativeImageWritesV1(mountPoint:string,io:NativeImageWritesIoV1=REAL_WRITES_IO):NativeImageWritesProbeV1{
 let availableBytes=0;try{availableBytes=io.available(mountPoint);}catch{}
 const file=join(mountPoint,".semio-write-probe-"+process.pid);
 try{io.write(file,PROBE_BYTES);return Object.freeze({mountPoint,writable:true,availableBytes});}
 catch(error){return Object.freeze({mountPoint,writable:false,availableBytes,reason:String((error as NodeJS.ErrnoException)?.code??(error as Error)?.message??error)});}
 finally{io.remove(file);}
}
/** 🧾️ What a writer needs to know of an image that refuses writes: the image, why, what is usable, the declared capacity, what is free inside, what the host holds and the capacity it is shrunk to. */
export type NativeImageWritesRefusedFactsV1=Readonly<{image:string;reason:string;capacityBytes:number;availableBytes:number;innerFreeBytes:number;hostFreeBytes:number;targetBytes:number}>;
/** 🛑️ An image that refuses writes before a build and could not be repaired: the typed refusal (`ENOSPC` family, both texts), raised before the build starts. */
export class NativeImageWritesRefusedErrorV1 extends Error{
 readonly code="ENOSPC";readonly facts:NativeImageWritesRefusedFactsV1;
 constructor(facts:NativeImageWritesRefusedFactsV1){super(nativeImageWritesRefusedTextV1(facts,"en")+" / "+nativeImageWritesRefusedTextV1(facts,"de"));this.name="NativeImageWritesRefusedErrorV1";this.facts=facts;}
}
/** 💬️ The refusal of an image that does not take writes, in one language. */
export function nativeImageWritesRefusedTextV1(facts:NativeImageWritesRefusedFactsV1,language:"en"|"de"):string{
 return nativeImageReliefTextV1("writesRefused",language,{image:facts.image,reason:facts.reason,available:gib(facts.availableBytes),capacity:gib(facts.capacityBytes),inner:gib(facts.innerFreeBytes),host:gib(facts.hostFreeBytes),target:gib(facts.targetBytes)});
}
/** 🌡️ How much the host and the images need collecting now: `critical` below the critical host watermark (nothing new is written), `hard` below the hard host watermark or when an image is starved, `soft` below the soft watermark, else `none`; `demandBytes` is what the host lacks up to the soft watermark. Pure and language-agnostic. */
export function nativeImagePressureV1(facts:Readonly<{hostFreeBytes:number;images:readonly Readonly<{id:string;capacityBytes:number;usedBytes:number;availableBytes:number}>[]}>):Readonly<{level:"none"|"soft"|"hard"|"critical";demandBytes:number;starved:readonly string[]}>{
 const starved=facts.images.filter(image=>nativeImageClampV1(image).starved).map(image=>image.id),level=facts.hostFreeBytes<protocol.criticalHostFreeBytes?"critical":facts.hostFreeBytes<protocol.hardHostFreeBytes||starved.length>0?"hard":facts.hostFreeBytes<protocol.softHostFreeBytes?"soft":"none";
 return Object.freeze({level,demandBytes:level==="none"?0:Math.max(0,protocol.softHostFreeBytes-facts.hostFreeBytes),starved:Object.freeze(starved)});
}
/** ⏳️ How long a writer waits before its attempt: the last backoff step holds for every later attempt. Pure and language-agnostic. */
export function nativeImageBackoffMillisecondsV1(attempt:number):number{return protocol.backoffMilliseconds[Math.min(Math.max(1,attempt),protocol.backoffMilliseconds.length)-1]!;}
/** 💬️ One text of the relief with its placeholders filled. */
export function nativeImageReliefTextV1(kind:keyof typeof protocol.messages,language:"en"|"de",values:Readonly<Record<string,string|number>>):string{return protocol.messages[kind][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]??""));}

/** 🧾️ What a writer that met a full image reports: the image, its declared capacity, what is usable and free inside, what the host holds, the partition and what was reclaimed over the attempts. */
export type NativeImageFullFactsV1=Readonly<{image:string;partition:string;capacityBytes:number;availableBytes:number;innerFreeBytes:number;hostFreeBytes:number;reclaimedBytes:number;attempts:number}>;
/** 🛑️ A write that stayed refused after every relief attempt: the typed refusal of a full image, a disk-full error (`ENOSPC`) that carries the facts and both texts. */
export class NativeImageFullErrorV1 extends Error{
 readonly code="ENOSPC";readonly facts:NativeImageFullFactsV1;
 constructor(facts:NativeImageFullFactsV1,cause?:unknown){super(nativeImageFullTextV1(facts,"en")+" / "+nativeImageFullTextV1(facts,"de"),{cause});this.name="NativeImageFullErrorV1";this.facts=facts;}
}
const gib=(bytes:number):string=>(bytes/1073741824).toFixed(1)+" GiB";
/** 💬️ The refusal of a full image in one language. */
export function nativeImageFullTextV1(facts:NativeImageFullFactsV1,language:"en"|"de"):string{
 return nativeImageReliefTextV1("full",language,{image:facts.image,available:gib(facts.availableBytes),capacity:gib(facts.capacityBytes),inner:gib(facts.innerFreeBytes),host:gib(facts.hostFreeBytes),reclaimed:gib(facts.reclaimedBytes),attempts:facts.attempts,command:protocol.command});
}
/** 🔌️ What a writer needs from outside to relieve a full image: its facts, the release of its reserve file (`true` when bytes were freed), a reclaim for a demand (the bytes freed), a wait and a progress line. */
export type NativeImageReliefPortsV1=Readonly<{image:string;facts():Omit<NativeImageFullFactsV1,"reclaimedBytes"|"attempts">|undefined;release?():boolean;reclaim(demandBytes:number):Promise<number>;sleep(milliseconds:number):Promise<void>;say?(kind:"retrying",values:Readonly<Record<string,string|number>>):void;check?():void}>;
/** 🆘️ Runs a write and, when it meets a full image (`ENOSPC`, `EDQUOT`, a runtime that lost the code), relieves the image and writes again instead of failing the task: first the reserve file of the image is released, then the image is reclaimed for the demand and the writer waits for the host (`backoffMilliseconds`), at most `attempts` times; other errors pass through untouched; a write that stays refused ends in `NativeImageFullErrorV1` with the facts of the image. Cancellation (`check`) is honoured between attempts. */
export async function retryOnImageFullV1<T>(write:()=>T|Promise<T>,ports:NativeImageReliefPortsV1):Promise<T>{
 let failure:unknown,reclaimed=0;
 try{return await write();}catch(error){if(!isDiskFullV1(error))throw error;failure=error;}
 for(let attempt=1;attempt<=protocol.attempts;attempt++){
  ports.check?.();
  const facts=ports.facts(),released=ports.release?.()===true,demand=Math.max(0,protocol.softHostFreeBytes-(facts?.hostFreeBytes??0));
  reclaimed+=await ports.reclaim(released?0:Math.max(demand,protocol.floorFreeBytes)).catch(()=>0);
  ports.say?.("retrying",{image:ports.image,available:gib(facts?.availableBytes??0),reclaimed:gib(reclaimed),attempt,attempts:protocol.attempts});
  await ports.sleep(nativeImageBackoffMillisecondsV1(attempt));ports.check?.();
  try{return await write();}catch(error){if(!isDiskFullV1(error))throw error;failure=error;}
 }
 const facts=ports.facts();
 throw new NativeImageFullErrorV1({image:ports.image,partition:facts?.partition??"",capacityBytes:facts?.capacityBytes??0,availableBytes:facts?.availableBytes??0,innerFreeBytes:facts?.innerFreeBytes??0,hostFreeBytes:facts?.hostFreeBytes??0,reclaimedBytes:reclaimed,attempts:protocol.attempts},failure);
}

/** 🛑️ The host volume is below its critical watermark and stays there after one collection: no lane or image growth starts, a typed refusal (`ENOSPC` family, both texts) instead of writing until the disk is full. */
export class NativeImageHostCriticalErrorV1 extends Error{
 readonly code="ENOSPC";readonly partition:string;readonly freeBytes:number;
 constructor(partition:string,freeBytes:number){const values={partition,free:gib(freeBytes),critical:gib(protocol.criticalHostFreeBytes),command:protocol.command};super(nativeImageReliefTextV1("critical","en",values)+" / "+nativeImageReliefTextV1("critical","de",values));this.name="NativeImageHostCriticalErrorV1";this.partition=partition;this.freeBytes=freeBytes;}
}
/** 🚧️ Lets a lane birth or an image growth start only while the host volume has its critical free bytes: below them it collects once (`collect`, bounded and cancellable by the caller) and measures again, then refuses with `NativeImageHostCriticalErrorV1`. Answers the free bytes it saw. */
export async function requireHostAboveCriticalV1(ports:Readonly<{partition:string;hostFreeBytes():number;collect():Promise<unknown>}>):Promise<number>{
 let free=ports.hostFreeBytes();
 if(free>=protocol.criticalHostFreeBytes)return free;
 await ports.collect().catch(()=>undefined);free=ports.hostFreeBytes();
 if(free<protocol.criticalHostFreeBytes)throw new NativeImageHostCriticalErrorV1(ports.partition,free);
 return free;
}
