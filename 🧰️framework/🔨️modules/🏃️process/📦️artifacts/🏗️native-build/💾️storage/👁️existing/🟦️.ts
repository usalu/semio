import{createHash,randomBytes}from"node:crypto";
import{lstat}from"node:fs/promises";
import{dirname,join}from"node:path";
import{requireNativePlatformImageV1,requireNativePlatformImageBindingV1}from"../🌐️platform/🟦️.ts";
import type{NativeImageStorageV1}from"../🌐️platform/📋️contract/🟦️.ts";
import{nativeStorageSchemaV1,readNativeStorageDataV1,nativeStorageStateV1,nativeStorageEventV1,snapshotNativeStorageStateV1,type NativeStoragePolicyV1,type NativeStorageStateV1}from"../📜️journal/🟦️.ts";
import{acquireQueuedResourceLease,checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{requireIssuedResourceControlV1}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{readPhysicalFileV1}from"../../../../../📁️filesystem/🧾️observation/🟦️.ts";
import schema from"./🔣️schema.json"with{type:"json"};

type ProgressV1=Readonly<{phase:"read"|"replay";bytes:number;events:number}>;
export type NativeStorageExistingSnapshotV1=Readonly<{version:1;effectiveCapacityBytes:number;journal:Readonly<{bytes:number;sha256:string;events:number}>;state:NativeStorageStateV1}>;
const protocol=nativeStorageSchemaV1.definitions.CleanupProtocol.const;
const same=(left:Awaited<ReturnType<typeof lstat>>,right:Awaited<ReturnType<typeof lstat>>)=>left.dev===right.dev&&left.ino===right.ino&&left.nlink===right.nlink&&left.size===right.size&&left.mtimeMs===right.mtimeMs&&left.ctimeMs===right.ctimeMs;

/** 📜️ Replays complete existing bytes afresh; no receipt or journal projection cache participates. */
export async function replayExistingNativeStorageJournalV1(text:string,capacityBytes:number,control:ResourceLeaseControl,progress?:(value:ProgressV1)=>void|Promise<void>):Promise<Readonly<{state:NativeStorageStateV1;events:number}>>{
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);if(!text||!text.endsWith("\n")||Buffer.byteLength(text)>protocol.journalMaximumBytes)throw Error("Complete existing Native journal required");
 let state=nativeStorageStateV1(),events=0,bytes=0;for(const line of text.slice(0,-1).split("\n")){checkResourceLeaseControl(control);if(!line)throw Error("Unknown existing Native journal bytes");state=nativeStorageEventV1(state,JSON.parse(line),capacityBytes);bytes+=Buffer.byteLength(line)+1;events++;if(events%schema.definitions.Protocol.const.progressEvents===0){await progress?.({phase:"replay",bytes,events});await control.yieldContinuation();checkResourceLeaseControl(control);}}
 return Object.freeze({state:snapshotNativeStorageStateV1(state),events});
}

/** 👁️ Reads only an initialized canonical image journal under its actual shared writer-excluding lease. */
export function observeExistingNativeStorageStateV1(input:Readonly<{image:NativeImageStorageV1;policy:NativeStoragePolicyV1;control:ResourceLeaseControl;onProgress?:(value:ProgressV1)=>void|Promise<void>}>):Promise<NativeStorageExistingSnapshotV1>{
 const image=input.image,control=input.control,policy=Object.freeze(readNativeStorageDataV1<NativeStoragePolicyV1>("Policy",{...input.policy})),onProgress=input.onProgress;
 requireIssuedResourceControlV1(control);requireNativePlatformImageV1(image,control);requireNativePlatformImageBindingV1(image,control,{directory:dirname(image.image),leaseDirectory:policy.leaseDirectory,filesystemBytes:image.filesystemBytes,maximumPhysicalBytes:image.maximumPhysicalBytes});
 if(policy.root!==image.root||policy.maximumWorkspaceBytes>policy.capacityBytes||policy.maximumArtifactBytes>policy.capacityBytes)throw Error("Exact existing Native observation policy required");
 const notify=async(actual:ResourceLeaseControl,value:ProgressV1)=>{checkResourceLeaseControl(actual);await actual.publish({resource:"native:existing-journal:"+value.phase+":"+value.bytes+":"+value.events,mode:"shared",elapsedMs:0});await onProgress?.(Object.freeze(value));checkResourceLeaseControl(actual);};
 return image.port.withOperation(control,actual=>image.port.withJournal(actual,async current=>{
  checkResourceLeaseControl(current);requireNativePlatformImageV1(image,current);const backing=await image.port.observe(policy,current);if(!backing||backing.root!==image.root||backing.maximumPhysicalBytes!==image.maximumPhysicalBytes||backing.maximumPhysicalBytes>policy.capacityBytes||typeof backing.identity!=="string"||!backing.identity)throw Error("Exact existing Native backing required");
  const capacityBytes=Math.min(policy.capacityBytes,backing.maximumPhysicalBytes),lease=await acquireQueuedResourceLease({directory:policy.leaseDirectory,resource:"native-storage:"+backing.identity,mode:"shared",sharedCohort:true,owner:randomBytes(16).toString("base64url"),control:current,signal:current.signal});
  try{await image.port.recheck(backing,current);const file=join(image.root,"storage-events.jsonl");let before;try{before=await lstat(file);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")throw Error("Existing initialized Native journal required");throw error;}checkResourceLeaseControl(current);
   if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1||before.size<1||before.size>protocol.journalMaximumBytes)throw Error("Bounded regular existing Native journal required");
   const captured=await readPhysicalFileV1(file,{maxBytes:before.size,maxWork:protocol.maximumReadWork,chunkBytes:protocol.chunkBytes,cancelled:()=>{checkResourceLeaseControl(current);return false;},remainingMs:()=>current.remainingMilliseconds()??Number.MAX_SAFE_INTEGER,onProgress:()=>checkResourceLeaseControl(current)});checkResourceLeaseControl(current);
   const after=await lstat(file);if(!same(before,after)||!after.isFile()||after.isSymbolicLink())throw Error("Existing Native journal changed during shared observation");await notify(current,{phase:"read",bytes:captured.bytes.byteLength,events:0});
   const text=new TextDecoder("utf-8",{fatal:true}).decode(captured.bytes),replayed=await replayExistingNativeStorageJournalV1(text,capacityBytes,current,value=>notify(current,value));await image.port.recheck(backing,current);requireNativePlatformImageV1(image,current);
   if(!same(before,await lstat(file)))throw Error("Existing Native journal changed before shared handoff");checkResourceLeaseControl(current);return Object.freeze({version:1,effectiveCapacityBytes:capacityBytes,journal:Object.freeze({bytes:captured.bytes.byteLength,sha256:createHash("sha256").update(captured.bytes).digest("hex"),events:replayed.events}),state:replayed.state});
  }finally{lease.release();}
 }));
}
