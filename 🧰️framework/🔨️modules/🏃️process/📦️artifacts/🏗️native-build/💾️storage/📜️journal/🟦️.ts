import{dirname}from"node:path";
import{types}from"node:util";
import{compileJsonSchemaSubset,validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import declaredSchema from"../🧬️schema/🔣️.json" with{type:"json"};
type Immutable<T>=T extends object?{readonly[K in keyof T]:Immutable<T[K]>}:T;
function freeze<T>(value:T):Immutable<T>{if(value&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value as Immutable<T>;}
/** 🔐️ Owns the single immutable loaded contract shared by journal DATA and Native orchestration. */
export const nativeStorageSchemaV1=freeze(structuredClone(declaredSchema));
const schema=nativeStorageSchemaV1;
/** 👓️ Admits closed schema DATA without allocating storage or granting Native custody. */
export function readNativeStorageDataV1<T>(definition:keyof typeof schema.definitions,value:unknown):T{const errors=validateJsonSchemaSubset(schema.definitions[definition],value,schema);if(errors.length)throw Error(`Invalid native storage ${definition}: ${errors.join("; ")}`);return value as T;}

const admit=readNativeStorageDataV1;
const namespaceProtocol=Object.freeze({...schema.definitions.NamespaceProtocol.const});
/** 💾️ Describes one fixed managed allocation envelope; its lease directory fixes the sole queue catalog. */
export type NativeStoragePolicyV1=Readonly<{version:1;root:string;leaseDirectory:string;capacityBytes:number;maximumWorkspaceBytes:number;maximumArtifactBytes:number}>;

/** 🧮️ Retains conservative charge through publication, cancellation and physical retirement. */
export type NativeStorageAllocationV1=Readonly<{namespace?:NativeStorageNamespaceReceiptV1;child?:NativeStorageChildRequestV1;id:string;owner:string;kind:"compiler-workspace"|"compiler-state"|"persistent-state"|"artifact-publication"|"artifact-restore";bytes:number;phase:"reserved"|"committed"|"retiring"}>;

/** 📂️ Records observed directory DATA inside an existing child charge; it issues no ownership. */
export type NativeStorageNamespaceIdentityV1=Readonly<{dev:string;ino:string;mode:number}>;

/** 🧾️ Binds finite namespace metadata to exact parent and created inode DATA. */
export type NativeStorageNamespaceReceiptV1=Readonly<{directory:string;bytes:65536;parent:NativeStorageNamespaceIdentityV1;entry:NativeStorageNamespaceIdentityV1}>;

/** 🌱️ Records the exact closed child request funded by this parent allocation; DATA alone issues no child. */
export type NativeStorageChildRequestV1=Readonly<{version:1;allocation:string;parentIdentity:string;directory:string;filesystemBytes:number;maximumPhysicalBytes:number;policySha256:string}>;

/** 🧱️ Captures only the four immutable child declaration fields before any parent or allocator effect. */
export type NativeStorageChildDeclarationV1=Pick<NativeStorageChildRequestV1,"directory"|"filesystemBytes"|"maximumPhysicalBytes"|"policySha256">;

/** 📚️ Projects the durable storage event stream without expiring unfinished owners. */
export type NativeStorageStateV1=Readonly<{version:1;sequence:number;allocations:readonly NativeStorageAllocationV1[]}>;

/** 🪞️ Copies validated ordinary journal DATA into detached immutable rows without granting Native custody. */
export function snapshotNativeStorageStateV1(state:NativeStorageStateV1):NativeStorageStateV1{return Object.freeze({...state,allocations:Object.freeze(state.allocations.map(row=>Object.freeze({...row,...(row.child?{child:Object.freeze({...row.child})}:{}),...(row.namespace?{namespace:Object.freeze({...row.namespace,parent:Object.freeze({...row.namespace.parent}),entry:Object.freeze({...row.namespace.entry})})}:{})})))});}

/** 📨️ Records allocation custody changes; retirement follows physical release, never a PID timeout. */
export type NativeStorageEventV1=Readonly<{type:"reserved";allocation:NativeStorageAllocationV1}>|Readonly<{type:"committed"|"retiring"|"retired";id:string}>|Readonly<{type:"checkpoint";state:NativeStorageStateV1}>|Readonly<{type:"namespace-created";id:string;namespace:NativeStorageNamespaceReceiptV1}>|Readonly<{type:"rebudgeted";expectedSequence:number;allocation:NativeStorageAllocationV1;bytes:number}>;

/** 📉️ Binds a shrink to the entire current allocation and journal sequence. */
export type NativeStorageRebudgetRequestV1=Readonly<{version:1;expectedSequence:number;allocation:NativeStorageAllocationV1;bytes:number}>;

/** 🗜️ Identifies a real enforcing backing and its complete bounded host allocation. */
export type NativeStorageBackingV1=Readonly<{identity:string;root:string;maximumPhysicalBytes:number}>;

/** 🧲️ Starts an empty event projection. */
export function nativeStorageStateV1():NativeStorageStateV1{return{version:1,sequence:0,allocations:[]};}

/** ⚖️ Includes reserved, committed and physically retiring allocations equally. */
export function nativeStorageUsedBytesV1(state:NativeStorageStateV1):number{return state.allocations.reduce((sum,row)=>{if(!Number.isSafeInteger(sum+row.bytes))throw Error("Native storage capacity overflow");return sum+row.bytes;},0);}

/** 🛂️ Applies one custody event only when the complete conservative allocation fits. */
export function nativeStorageEventV1(state:NativeStorageStateV1,event:NativeStorageEventV1,capacityBytes:number):NativeStorageStateV1{
 admit("State",state);admit("Event",event);for(const row of[...state.allocations,...(event.type==="reserved"?[event.allocation]:event.type==="checkpoint"?event.state.allocations:[])])if(row.namespace&&(!row.child||row.kind!=="compiler-state"||row.namespace.entry.dev!==row.namespace.parent.dev||dirname(row.child.directory)!==row.namespace.directory||row.bytes-row.child.maximumPhysicalBytes<row.namespace.bytes+namespaceProtocol.minimumChildMetadataBytes))throw Error("Finite original namespace allocation required");for(const row of[...state.allocations,...((event.type==="reserved"||event.type==="rebudgeted")?[event.allocation]:event.type==="checkpoint"?event.state.allocations:[])])if(row.child&&(row.kind!=="compiler-state"||row.child.allocation!==row.id||row.child.maximumPhysicalBytes>=row.bytes))throw Error("Exact child funding allocation association required");if(state.allocations.some(row=>row.kind==="persistent-state"&&row.phase==="retiring")||event.type==="checkpoint"&&event.state.allocations.some(row=>row.kind==="persistent-state"&&row.phase==="retiring"))throw Error("Persistent state cannot be retired");if(!Number.isSafeInteger(capacityBytes)||capacityBytes<1)throw Error("Invalid native storage capacity");
 if(event.type==="checkpoint"){if(state.sequence!==0||state.allocations.length)throw Error("Native storage checkpoint must begin the journal");admit("State",event.state);if(nativeStorageUsedBytesV1(event.state)>capacityBytes||new Set(event.state.allocations.map(row=>row.id)).size!==event.state.allocations.length)throw Error("Invalid native storage checkpoint capacity");return event.state;}
 const allocations=[...state.allocations];
 if(nativeStorageUsedBytesV1(state)>capacityBytes||new Set(allocations.map(row=>row.id)).size!==allocations.length)throw Error("Invalid native storage capacity state");
 if(event.type==="reserved"){
  if(event.allocation.namespace||event.allocation.phase!=="reserved"||allocations.some(row=>row.id===event.allocation.id||!!row.child&&!!event.allocation.child&&row.child.directory===event.allocation.child.directory))throw Error("Native storage reservation identity already held");
  if(event.allocation.bytes>capacityBytes-nativeStorageUsedBytesV1(state))throw Error("Native storage capacity exhausted");allocations.push(event.allocation);
 }else if(event.type==="namespace-created"){
  const index=allocations.findIndex(row=>row.id===event.id),allocation=allocations[index];if(!allocation||allocation.phase!=="reserved"||allocation.namespace||!allocation.child||allocation.kind!=="compiler-state"||dirname(allocation.child.directory)!==event.namespace.directory||event.namespace.entry.dev!==event.namespace.parent.dev||allocation.bytes-allocation.child.maximumPhysicalBytes<event.namespace.bytes+namespaceProtocol.minimumChildMetadataBytes||allocations.some(row=>row.namespace?.directory===event.namespace.directory))throw Error("Exact charged new namespace event required");allocations[index]={...allocation,namespace:Object.freeze({...event.namespace,parent:Object.freeze({...event.namespace.parent}),entry:Object.freeze({...event.namespace.entry})})};
 }else if(event.type==="rebudgeted"){
  const index=allocations.findIndex(row=>row.id===event.allocation.id),allocation=allocations[index];if(!allocation||state.sequence!==event.expectedSequence||allocation.phase!=="committed"||allocation.kind!=="artifact-publication"||event.bytes>=allocation.bytes||(["id","owner","kind","bytes","phase"]as const).some(key=>allocation[key]!==event.allocation[key]))throw Error("Exact committed native rebudget predecessor required");allocations[index]={...allocation,bytes:event.bytes};
 }else{
  const index=allocations.findIndex(row=>row.id===event.id),allocation=allocations[index];if(!allocation)throw Error("Native storage allocation not held");
  if(allocation.kind==="persistent-state"&&(event.type==="retiring"||event.type==="retired"))throw Error("Persistent state cannot be retired");
  if(event.type==="retired"){if(allocation.phase!=="retiring")throw Error("Native storage physical retirement required");allocations.splice(index,1);}
  else{if(event.type==="committed"&&allocation.phase!=="reserved"||event.type==="retiring"&&allocation.phase==="retiring")throw Error("Invalid native storage custody transition");allocations[index]={...allocation,phase:event.type};}
 }
 if(!Number.isSafeInteger(state.sequence+1))throw Error("Native storage sequence overflow");return{version:1,sequence:state.sequence+1,allocations};
}

const journalProjectionRequestContract=compileJsonSchemaSubset(schema.definitions.JournalProjectionRequest,schema);

/** 💭️ Projects closed journal DATA without storage, allocation or cleanup authority. */
export interface NativeStorageJournalProjectionV1{project(value:unknown,check:()=>void):NativeStorageStateV1}

interface NativeStorageJournalProjectionRequestV1{readonly version:1;readonly text:string;readonly capacityBytes:number}

function captureJournalProjectionRequest(value:unknown):NativeStorageJournalProjectionRequestV1{
 if(!value||typeof value!=="object"||Array.isArray(value)||types.isProxy(value)||Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null)throw Error("Plain journal projection DATA required");const keys=Reflect.ownKeys(value),fields=["version","text","capacityBytes"];if(keys.length!==fields.length||keys.some(key=>typeof key!=="string"||!fields.includes(key)))throw Error("Closed journal projection fields required");const captured:Record<string,unknown>={};for(const key of fields){const field=Object.getOwnPropertyDescriptor(value,key);if(!field||!("value"in field)||!field.enumerable)throw Error("Own primitive journal projection fields required");captured[key]=field.value;}if(captured.version!==1||typeof captured.capacityBytes!=="number"||!Number.isSafeInteger(captured.capacityBytes)||captured.capacityBytes<1||typeof captured.text!=="string"||captured.text.length>schema.definitions.ImageProtocol.const.journalMaximumBytes||journalProjectionRequestContract.validate(captured).length)throw Error("Closed bounded journal projection request required");return Object.freeze(captured)as unknown as NativeStorageJournalProjectionRequestV1;
}

/** 🧠️ Reuses exact immutable event projections only when every historical charge prefix fits. */
export function createNativeStorageJournalProjectionV1():NativeStorageJournalProjectionV1{
 const limits=Object.freeze({...schema.definitions.ImageProtocol.const}),entries=new Map<string,Readonly<{state:NativeStorageStateV1;maximumChargedBytes:number}>>();if(limits.journalMaximumBytes!==schema.definitions.CleanupProtocol.const.journalMaximumBytes||!Number.isSafeInteger(limits.journalProjectionCacheEntries)||limits.journalProjectionCacheEntries<1)throw Error("Aligned finite physical journal projection bound required");const remember=(text:string,entry:Readonly<{state:NativeStorageStateV1;maximumChargedBytes:number}>)=>{entries.delete(text);while(entries.size>=limits.journalProjectionCacheEntries)entries.delete(entries.keys().next().value!);entries.set(text,entry);};return Object.freeze({project(value:unknown,check:()=>void):NativeStorageStateV1{
 const{text,capacityBytes}=captureJournalProjectionRequest(value);if(typeof check!=="function")throw Error("Synchronous journal check required");check();if(Buffer.byteLength(text)>limits.journalMaximumBytes)throw Error("Bounded journal projection bytes required");if(text&&!text.endsWith("\n"))throw Error("Incomplete Native cleanup journal retained");const previous=entries.get(text);if(previous&&capacityBytes>=previous.maximumChargedBytes){check();remember(text,previous);return previous.state;}
 let state=nativeStorageStateV1(),maximumChargedBytes=0;for(const line of text?text.slice(0,-1).split("\n"):[]){check();if(!line)throw Error("Unknown Native cleanup journal bytes retained");state=nativeStorageEventV1(state,JSON.parse(line),capacityBytes);maximumChargedBytes=Math.max(maximumChargedBytes,nativeStorageUsedBytesV1(state));}const result=snapshotNativeStorageStateV1(state);check();remember(text,Object.freeze({state:result,maximumChargedBytes}));return result;
 }});
}
