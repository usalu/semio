export interface DerivedNodesPolicyV1{readonly version:1;readonly kinds:readonly string[];readonly deriveConcurrency:number;readonly maximumNodes:number;readonly maximumTrace:number;readonly verifyParallelNodes:number;readonly storeDelayMilliseconds:number;readonly storeParallelWrites:number;readonly heavyMaximum:number;readonly heavyBytes:number;readonly heavyReserveBytes:number;readonly releaseRssBytes:number;readonly releaseMinimumMilliseconds:number;readonly releaseMinimumGain:number;readonly releasePauseMilliseconds:number;readonly maximumListedPaths:number;readonly retainedFileBytes:number;readonly retainedBytesPerOperation:number;readonly statIndex:{readonly directory:string;readonly racyMilliseconds:number;readonly maximumEntries:number;readonly maximumRecords:number;readonly maximumRecordBytes:number;readonly retainMilliseconds:number};readonly store:{readonly directory:string;readonly maximumRecords:number;readonly maximumRecordBytes:number;readonly retainMilliseconds:number}}
/** 🧮️ The declared constants of the derived-node DAG. */
export const DERIVED_NODES_POLICY_V1:DerivedNodesPolicyV1;
export interface NodeKeyV1{kind:string;subject:string;params:string}
export interface RowRefV1{kind:"file"|"glob"|"observed";path:string;paths?:string[];digests?:string[];rows?:unknown;patterns?:unknown[];bases?:string[];acc?:string;stamps?:Record<string,string>;count?:number;digest:string}
export interface DirtySetV1{revision?:string;paths:readonly string[];directories?:readonly string[];overflow?:boolean}
export interface DerivedNodeV1{key:NodeKeyV1;valueDigest:string;value:any;deps:{key:NodeKeyV1;digest:string}[];rawReads:RowRefV1[];computedAtRev:string}
export interface WorldPortV1{verify?(root:string,signal?:AbortSignal):Promise<string|undefined>;stamp?(root:string):string|undefined;stat?(path:string):Promise<{size:number;mtimeMs:number;ino:number}|undefined>;read(path:string):Promise<Uint8Array|undefined>;files(root:string,signal?:AbortSignal):Promise<readonly string[]>}
export interface InvalidationV1{changed:NodeKeyV1[];computed:number;cutoffHit:number;revalidated:number;trace:{kind:string;id:string}[]}
/** #️⃣ The sha256 of a text or byte string. */
export function sha256V1(data:string|Uint8Array):string;
/** #️⃣ The digest of a JSON value. */
export function digestOfV1(value:unknown):string;
/** 🌡️ The `capacity` port from the host capacity grant. */
export function capacityOfGrantV1(source:()=>{handle?:{cpu?:number;jobs?:number;reservedMegabytes?:number};limit?:{memoryMegabytes?:number;cpuShares?:number}}|undefined,ports?:{rss?:()=>number;host?:()=>{cpus:number;availableBytes:number}}):()=>{cpus:number;availableBytes:number;reserveBytes?:number};
/** 🔑 The key of one node. */
export function nodeKeyV1(kind:string,subject:string,params?:unknown):NodeKeyV1;
/** 🪪 The identity of a key as one string. */
export function keyIdV1(key:NodeKeyV1):string;
export interface MemoStoreV1{key(kind:string,subject:string,params?:unknown):NodeKeyV1;get(key:NodeKeyV1,options?:{signal?:AbortSignal;progress?:(event:{phase:"derive";done:number;total:number})=>unknown}):Promise<DerivedNodeV1>;getMany(keys:readonly NodeKeyV1[],options?:{signal?:AbortSignal;slice?:number;progress?:(event:{phase:"derive";done:number;total:number})=>unknown}):Promise<DerivedNodeV1[]>;put(node:DerivedNodeV1,params?:unknown):void;invalidate(dirty:DirtySetV1,options?:{signal?:AbortSignal;progress?:(event:{phase:"derive";done:number;total:number})=>unknown}):Promise<InvalidationV1>;stats():Record<string,number>;drainTrace():{kind:string;id:string}[];flush():Promise<void>}
/** 🧮️ The derived-node graph with early cutoff. */
export function createDerivedGraphV1(options:{rules:Readonly<Record<string,(ctx:any)=>Promise<unknown>>>;world:WorldPortV1;store?:import("./💾️store/🟨️.mjs").NodeStoreV1;revision?:string;capacity?:()=>{cpus:number;availableBytes:number;reserveBytes?:number};statIndex?:import("./🗂️stat-index/🟨️.mjs").StatIndexV1;release?:()=>void;environment?:object;environmentKey?:string;verifyObserved?:(ref:RowRefV1,options:{signal?:AbortSignal;environment?:any})=>Promise<boolean>|boolean;concurrency?:number;maximumNodes?:number;trace?:boolean}):MemoStoreV1;
