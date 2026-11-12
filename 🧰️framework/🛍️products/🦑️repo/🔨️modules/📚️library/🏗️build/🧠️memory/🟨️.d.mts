/** 🧠️ The declared limits of the graph memory governor. */
export interface GraphMemoryPolicyV1{readonly version:1;readonly languages:readonly ("en"|"de")[];readonly minimumBudgetMegabytes:number;readonly maximumBudgetMegabytes:number;readonly hostShare:number;readonly grantShare:number;readonly pollMilliseconds:number;readonly admitMinimumMilliseconds:number;readonly progressMilliseconds:number;readonly retainedImageBytes:number;readonly minimumInternedLength:number;readonly maximumInternedCharacters:number;readonly maximumCachedClosureEntries:number;readonly maximumParsedManifests:number;readonly childHeapMegabytes:number;readonly serviceHeapMegabytes:number;readonly serviceLimitMegabytes:number;readonly initialOwnerMegabytes:number;readonly safetyMarginBasisPoints:number;readonly budgetEnvironment:string;readonly planEnvironment:string;readonly limitEnvironment:string}
export declare const GRAPH_MEMORY_POLICY_V1:GraphMemoryPolicyV1;
export type GraphMemoryMessageKeyV1="waiting"|"serial"|"resumed"|"learning";
export declare const GRAPH_MEMORY_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<GraphMemoryMessageKeyV1,string>>>>;
export function selectGraphMemoryLanguagesV1(environment:Readonly<Record<string,string|undefined>>):("en"|"de")[];
export function renderGraphMemoryMessagesV1(key:GraphMemoryMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string[];
export function hostMemoryMegabytesV1():number;
/** 🧒️ The fixed JavaScriptCore memory size of the graph child process in bytes, independent of the capacity grant. */
export function graphChildHeapBytesV1(policy?:GraphMemoryPolicyV1):number;
export function graphServiceHeapBytesV1(policy?:GraphMemoryPolicyV1):number;
export interface MemoryPortsV1{readonly sample:()=>number;readonly collect:()=>void}
export function createMemoryPortsV1():MemoryPortsV1;
export function darwinAvailableBytesV1(text:string):number|undefined;
export function linuxAvailableBytesV1(text:string):number|undefined;
export function createAvailableMemoryV1(ports?:{platform?:string;read?:(path:string)=>string;run?:()=>string;free?:()=>number;now?:()=>number;maximumAgeMilliseconds?:number}):()=>number;
export const availableMemoryBytesV1:()=>number;
export function serviceMemoryHeadroomBytesV1(options?:{ports?:unknown;policy?:GraphMemoryPolicyV1;cgroup?:(ports?:unknown)=>{limitBytes:number;currentBytes:number}|undefined;free?:()=>number;total?:()=>number;environment?:Readonly<Record<string,string|undefined>>;usage?:()=>number}):number;
export interface MemoryGovernorOptionsV1{workers:number;headroom?:()=>number;attributable?:()=>number;estimateBytes?:number;peaks?:GraphOwnerPeaksV1;decide?:(decision:{key:string;used:number;budget:number;kind?:string})=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;signal?:AbortSignal;report?:(line:string)=>void;environment?:Readonly<Record<string,string|undefined>>;policy?:GraphMemoryPolicyV1}
export interface MemoryGovernorStatsV1{waits:number;waitedMilliseconds:number;peakOwners:number;ownerCostMegabytes:number;kinds:Readonly<Record<string,number>>}
export interface MemoryGovernorV1{pressure():boolean;run<T>(task:()=>Promise<T>|T,options?:{kind?:string}):Promise<T>;stats():MemoryGovernorStatsV1;dispose():void}
export function createMemoryGovernorV1(options:MemoryGovernorOptionsV1&{headroom:()=>number}):MemoryGovernorV1;
export function createGraphParentMemoryGovernorV1(options:MemoryGovernorOptionsV1):MemoryGovernorV1;
export interface StringInternerV1{share(text:string):string;stats():{entries:number;characters:number;hits:number;clears:number};clear():void}
export function createStringInternerV1(options?:{maximumCharacters?:number;minimumLength?:number}):StringInternerV1;
export function internJsonStringsV1<T>(value:T,interner:StringInternerV1):T;
export declare const graphStringInternerV1:StringInternerV1;
/** 💾️ The durable learned peak in bytes of every owner kind. */
export interface GraphOwnerPeaksV1{readonly read:()=>Record<string,number[]>;readonly write:(peaks:Record<string,number[]>)=>void}
/** 🧮️ The admission law of one more owner. */
export function graphOwnerAdmissionV1(state:{headroom:number;running:number;inflight:readonly {estimate:number;taken:number}[];estimate:number;learned:boolean}):boolean;
/** 💾️ The learned owner peaks kept in one file. */
export function createGraphOwnerPeaksFileV1(path:string):GraphOwnerPeaksV1;
