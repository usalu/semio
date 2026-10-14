import {randomBytes} from "node:crypto";
import {existsSync,lstatSync,readFileSync,appendFileSync,openSync,fsyncSync,closeSync,writeFileSync,renameSync,unlinkSync,constants,fstatSync} from "node:fs";
import {join,resolve} from "node:path";
import {acquireQueuedResourceLease,checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 💾️ Describes one fixed managed allocation envelope, independently of eviction preferences. */
export type NativeStoragePolicyV1=Readonly<{version:1;root:string;leaseDirectory:string;capacityBytes:number;maximumWorkspaceBytes:number;maximumArtifactBytes:number}>;
/** 🧮️ Retains conservative charge through publication, cancellation and physical retirement. */
export type NativeStorageAllocationV1=Readonly<{id:string;owner:string;kind:"compiler-workspace"|"artifact-publication"|"artifact-restore";bytes:number;phase:"reserved"|"committed"|"retiring"}>;
/** 📚️ Projects the durable storage event stream without expiring unfinished owners. */
export type NativeStorageStateV1=Readonly<{version:1;sequence:number;allocations:readonly NativeStorageAllocationV1[]}>;
/** 📨️ Records allocation custody changes; retirement follows physical release, never a PID timeout. */
export type NativeStorageEventV1=Readonly<{type:"reserved";allocation:NativeStorageAllocationV1}>|Readonly<{type:"committed"|"retiring"|"retired";id:string}>|Readonly<{type:"checkpoint";state:NativeStorageStateV1}>;
/** 🗜️ Identifies a real enforcing backing and its complete bounded host allocation. */
export type NativeStorageBackingV1=Readonly<{identity:string;root:string;maximumPhysicalBytes:number}>;
/** 🔌️ Owns actual platform enforcement, identity revalidation and complete physical retirement. */
export interface NativeStoragePortV1{
 observe(policy:NativeStoragePolicyV1,control:ResourceLeaseControl):Promise<NativeStorageBackingV1|null>;
 recheck(backing:NativeStorageBackingV1,control:ResourceLeaseControl):Promise<void>;
 retire(allocation:NativeStorageAllocationV1,backing:NativeStorageBackingV1,control:ResourceLeaseControl):Promise<void>;
}

function admit<T>(definition:keyof typeof schema.definitions,value:unknown):T{const errors=validateJsonSchemaSubset(schema.definitions[definition],value,schema);if(errors.length)throw Error(`Invalid native storage ${definition}: ${errors.join("; ")}`);return value as T;}
/** 🌱️ Starts an empty event projection. */
export function nativeStorageStateV1():NativeStorageStateV1{return{version:1,sequence:0,allocations:[]};}
/** ⚖️ Includes reserved, committed and physically retiring allocations equally. */
export function nativeStorageUsedBytesV1(state:NativeStorageStateV1):number{return state.allocations.reduce((sum,row)=>{if(!Number.isSafeInteger(sum+row.bytes))throw Error("Native storage capacity overflow");return sum+row.bytes;},0);}
/** 🛂️ Applies one custody event only when the complete conservative allocation fits. */
export function nativeStorageEventV1(state:NativeStorageStateV1,event:NativeStorageEventV1,capacityBytes:number):NativeStorageStateV1{
 admit("State",state);admit("Event",event);if(!Number.isSafeInteger(capacityBytes)||capacityBytes<1)throw Error("Invalid native storage capacity");
 if(event.type==="checkpoint"){if(state.sequence!==0||state.allocations.length)throw Error("Native storage checkpoint must begin the journal");admit("State",event.state);if(nativeStorageUsedBytesV1(event.state)>capacityBytes||new Set(event.state.allocations.map(row=>row.id)).size!==event.state.allocations.length)throw Error("Invalid native storage checkpoint capacity");return event.state;}
 const allocations=[...state.allocations];
 if(nativeStorageUsedBytesV1(state)>capacityBytes||new Set(allocations.map(row=>row.id)).size!==allocations.length)throw Error("Invalid native storage capacity state");
 if(event.type==="reserved"){
  if(event.allocation.phase!=="reserved"||allocations.some(row=>row.id===event.allocation.id))throw Error("Native storage reservation identity already held");
  if(event.allocation.bytes>capacityBytes-nativeStorageUsedBytesV1(state))throw Error("Native storage capacity exhausted");allocations.push(event.allocation);
 }else{
  const index=allocations.findIndex(row=>row.id===event.id),allocation=allocations[index];if(!allocation)throw Error("Native storage allocation not held");
  if(event.type==="retired"){if(allocation.phase!=="retiring")throw Error("Native storage physical retirement required");allocations.splice(index,1);}
  else{if(event.type==="committed"&&allocation.phase!=="reserved"||event.type==="retiring"&&allocation.phase==="retiring")throw Error("Invalid native storage custody transition");allocations[index]={...allocation,phase:event.type};}
 }
 if(!Number.isSafeInteger(state.sequence+1))throw Error("Native storage sequence overflow");return{version:1,sequence:state.sequence+1,allocations};
}

/** 🚫️ An ordinary directory has no portable aggregate physical quota. */
export function ordinaryNativeStoragePortV1():NativeStoragePortV1{return{observe:async()=>null,recheck:async()=>{throw Error("No enforcing storage backend");},retire:async()=>{throw Error("No enforcing storage backend");}};}

/** 🤝️ Supplies the original writer and reader's awaited physical release to an enforcing backing. */
export function nativeStorageRetirementPortV1(port:NativeStoragePortV1,retire:NativeStoragePortV1["retire"]):NativeStoragePortV1{return{observe:(policy,control)=>port.observe(policy,control),recheck:(backing,control)=>port.recheck(backing,control),retire:async(allocation,backing,control)=>{checkResourceLeaseControl(control);await port.recheck(backing,control);await retire(allocation,backing,control);checkResourceLeaseControl(control);await port.recheck(backing,control);}};}

/** 🎟️ Keeps original custody and charge until an enforcing owner acknowledges physical retirement. */
export interface NativeStorageReservationV1{
 readonly allocation:NativeStorageAllocationV1;
 readonly backing:NativeStorageBackingV1;
 recheck():Promise<void>;
 commit():Promise<void>;
 retire():Promise<void>;
}

const originalReservations=new WeakMap<NativeStorageReservationV1,AbortSignal>();
/** 🪪️ Refuses invented storage receipts and receipts borrowed from another original cancellation source. */
export function requireNativeStorageReservationV1(reservation:NativeStorageReservationV1,control:ResourceLeaseControl):NativeStorageReservationV1{checkResourceLeaseControl(control);if(!reservation||originalReservations.get(reservation)!==control.signal)throw Error("Original issued native storage reservation required");return reservation;}

const journalProjections=new Map<string,Readonly<{text:string;state:NativeStorageStateV1}>>();

type StorageCustodyOptionsV1=Readonly<{policy:NativeStoragePolicyV1;control:ResourceLeaseControl;port:NativeStoragePortV1}>;

async function storageCustodyV1(options:StorageCustodyOptionsV1):Promise<Readonly<{backing:NativeStorageBackingV1;transaction:<T>(run:(state:NativeStorageStateV1,append:(event:NativeStorageEventV1)=>void,checkpoint:()=>void)=>T)=>Promise<T>}>>{
 const {control,port}=options;checkResourceLeaseControl(control);const policy=admit<NativeStoragePolicyV1>("Policy",options.policy),root=requireMaterializedPathV1(policy.root);
 if(root!==policy.root||policy.maximumWorkspaceBytes>policy.capacityBytes||policy.maximumArtifactBytes>policy.capacityBytes)throw Error("Invalid native storage envelope");
 const backing=await port.observe(policy,control);checkResourceLeaseControl(control);
 if(!backing||resolve(backing.root)!==root||!backing.identity||!Number.isSafeInteger(backing.maximumPhysicalBytes)||backing.maximumPhysicalBytes<1||backing.maximumPhysicalBytes>policy.capacityBytes)throw Error("An enforcing storage backend is required before native writes");
 const capacityBytes=Math.min(policy.capacityBytes,backing.maximumPhysicalBytes);
 const journal=requireMaterializedPathV1(join(root,"storage-events.jsonl")),leaseDirectory=requireMaterializedPathV1(policy.leaseDirectory);
 return{backing,transaction:async run=>{
  checkResourceLeaseControl(control);await port.recheck(backing,control);checkResourceLeaseControl(control);
  const lease=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"native-storage:"+backing.identity,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:control.signal,control});
  try{
   await port.recheck(backing,control);checkResourceLeaseControl(control);if(existsSync(journal)){const file=lstatSync(journal);if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size>1048576)throw Error("Native storage journal needs owned recovery");}let state=nativeStorageStateV1(),text=existsSync(journal)?readFileSync(journal,"utf8"):"";
   if(Buffer.byteLength(text)>1048576||text&&!text.endsWith("\n"))throw Error("Native storage journal needs owned recovery");const previous=journalProjections.get(journal);if(previous?.text===text){state=previous.state;if(nativeStorageUsedBytesV1(state)>capacityBytes)throw Error("Native storage checkpoint exceeds actual backing capacity");}else for(const line of text.split("\n").filter(Boolean))state=nativeStorageEventV1(state,JSON.parse(line),capacityBytes);
   const protocol=schema.definitions.ImageProtocol.const,checkpoint=()=>{
    checkResourceLeaseControl(control);const event:NativeStorageEventV1={type:"checkpoint",state},encoded=JSON.stringify(event)+"\n";nativeStorageEventV1(nativeStorageStateV1(),event,capacityBytes);if(Buffer.byteLength(encoded)>protocol.journalCheckpointBytes)throw Error("Native storage held state exceeds checkpoint capacity");const temporary=requireMaterializedPathV1(join(root,protocol.journalCheckpointFile));if(existsSync(temporary)){const status=lstatSync(temporary);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>protocol.journalMaximumBytes)throw Error("Original checkpoint transaction file required");unlinkSync(temporary);}
    const fd=openSync(temporary,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|(constants.O_NOFOLLOW??0),0o600);try{writeFileSync(fd,encoded);fsyncSync(fd);checkResourceLeaseControl(control);}finally{closeSync(fd);}renameSync(temporary,journal);const directory=openSync(root,constants.O_RDONLY);try{fsyncSync(directory);}finally{closeSync(directory);}text=encoded;
   };
   const value=run(state,event=>{const next=nativeStorageEventV1(state,event,capacityBytes),encoded=JSON.stringify(event)+"\n";if(Buffer.byteLength(text)+Buffer.byteLength(encoded)>protocol.journalCheckpointBytes)checkpoint();if(Buffer.byteLength(text)+Buffer.byteLength(encoded)>protocol.journalMaximumBytes)throw Error("Native storage journal capacity exhausted");checkResourceLeaseControl(control);const fd=openSync(journal,constants.O_WRONLY|constants.O_APPEND|constants.O_CREAT|(constants.O_NOFOLLOW??0),0o600);try{const status=fstatSync(fd);if(!status.isFile()||status.nlink!==1||status.size!==Buffer.byteLength(text))throw Error("Original journal append custody changed");appendFileSync(fd,encoded);fsyncSync(fd);}finally{closeSync(fd);}if(!text){const directory=openSync(root,constants.O_RDONLY);try{fsyncSync(directory);}finally{closeSync(directory);}}state=next;text+=encoded;},checkpoint);journalProjections.delete(journal);journalProjections.set(journal,{text,state:Object.freeze({...state,allocations:Object.freeze(state.allocations.map(row=>Object.freeze({...row})))})});while(journalProjections.size>protocol.journalProjectionCacheEntries)journalProjections.delete(journalProjections.keys().next().value!);return value;
  }finally{lease.release();}
 }};
}

function heldReservationV1(options:StorageCustodyOptionsV1,allocation:NativeStorageAllocationV1,custody:Awaited<ReturnType<typeof storageCustodyV1>>):NativeStorageReservationV1{
 const {control,port}=options,backing=Object.freeze({...custody.backing});allocation=Object.freeze({...allocation});let phase:NativeStorageAllocationV1["phase"]|"retired"=allocation.phase;
 const transition=(event:NativeStorageEventV1)=>custody.transaction((_,append)=>append(event));
 const reservation:NativeStorageReservationV1={allocation,backing,recheck:async()=>{checkResourceLeaseControl(control);if(phase==="retired")throw Error("Native storage reservation retired");await custody.transaction(state=>{const held=state.allocations.find(row=>row.id===allocation.id);if(!held||held.owner!==allocation.owner||held.kind!==allocation.kind||held.bytes!==allocation.bytes)throw Error("Original durable native storage allocation no longer held");if(held.phase==="retiring")throw Error("Original native storage allocation is retiring");});checkResourceLeaseControl(control);},commit:async()=>{if(phase!=="reserved")throw Error("Native storage reservation already settled");await transition({type:"committed",id:allocation.id});phase="committed";},retire:async()=>{if(phase==="retired")return;if(phase!=="retiring"){await transition({type:"retiring",id:allocation.id});phase="retiring";}await port.retire({...allocation,phase:"retiring"},backing,control);checkResourceLeaseControl(control);await transition({type:"retired",id:allocation.id});phase="retired";}};
 originalReservations.set(reservation,control.signal);return Object.freeze(reservation);
}

/** 🏪️ Reserves before any managed write under a shared physical owner's serialized durable journal. */
export async function acquireNativeStorageV1(options:Readonly<{policy:NativeStoragePolicyV1;control:ResourceLeaseControl;port:NativeStoragePortV1;owner:string;kind:NativeStorageAllocationV1["kind"];bytes:number}>):Promise<NativeStorageReservationV1>{
 const custody=await storageCustodyV1(options),policy=options.policy;
 const allocation=admit<NativeStorageAllocationV1>("Allocation",{id:randomBytes(16).toString("hex"),owner:options.owner,kind:options.kind,bytes:options.bytes,phase:"reserved"});
 if(allocation.bytes>(allocation.kind==="compiler-workspace"?policy.maximumWorkspaceBytes:policy.maximumArtifactBytes))throw Error("Native storage allocation exceeds kind capacity");
 await custody.transaction((_,append)=>append({type:"reserved",allocation}));return heldReservationV1(options,allocation,custody);
}

/** 🔄️ Reacquires exact durable custody after restart; no age, PID death or missing file releases its charge. */
export async function recoverNativeStorageReservationV1(options:StorageCustodyOptionsV1&Readonly<{id:string}>):Promise<NativeStorageReservationV1>{
 const custody=await storageCustodyV1(options),allocation=await custody.transaction(state=>{const allocation=state.allocations.find(row=>row.id===options.id);if(!allocation)throw Error("Original durable native storage allocation required");return allocation;});return heldReservationV1(options,allocation,custody);
}

/** 🧾️ Atomically checkpoints only the exact serialized durable state; held and retiring charges remain intact. */
export async function checkpointNativeStorageV1(options:StorageCustodyOptionsV1):Promise<NativeStorageStateV1>{const custody=await storageCustodyV1(options);return custody.transaction((state,_,checkpoint)=>{checkpoint();return state;});}

/** 👁️ Reads the complete durable projection only under actual original backing and journal transaction custody. */
export async function observeNativeStorageStateV1(options:StorageCustodyOptionsV1):Promise<NativeStorageStateV1>{const custody=await storageCustodyV1(options);return custody.transaction(state=>Object.freeze({...state,allocations:Object.freeze(state.allocations.map(row=>Object.freeze({...row})))}));}
