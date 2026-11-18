import type{GraphOwnerScopeV1}from"./🎯️owner-scope/🟨️.mjs";
/** ⛔️ An owner the governor stopped: its key and kind, what it had grown by, the limit it passed and the seconds it ran. */
export interface GraphOwnerStopRowV1{readonly key:string;readonly kind:string;readonly takenBytes:number;readonly limitBytes:number;readonly seconds:number}
/** 🪙️ Where the governor records what one owner took, stopped or not. */
export interface GraphOwnerCostPortV1{record(row:{key:string;kind:string;takenBytes:number;seconds:number;stopped:boolean}):void}
/** 🧠️ The declared limits of the graph memory governor. */
export interface GraphMemoryPolicyV1{readonly version:1;readonly languages:readonly ("en"|"de")[];readonly minimumBudgetMegabytes:number;readonly maximumBudgetMegabytes:number;readonly hostShare:number;readonly grantShare:number;readonly pollMilliseconds:number;readonly admitMinimumMilliseconds:number;readonly progressMilliseconds:number;readonly retainedImageBytes:number;readonly minimumInternedLength:number;readonly maximumInternedCharacters:number;readonly maximumCachedClosureEntries:number;readonly maximumParsedManifests:number;readonly childHeapMegabytes:number;readonly serviceHeapMegabytes:number;readonly serviceHostShare:number;readonly serviceMinimumMegabytes:number;readonly initialOwnerMegabytes:number;readonly floorOwnerMegabytes:number;readonly safetyMarginBasisPoints:number;readonly parentManagedMegabytes:number;readonly parentHostShare:number;readonly serveHostMegabytes:number;readonly serveHostShare:number;readonly collectEveryProjects:number;readonly collectAboveMegabytes:number;readonly collectGrowthMegabytes:number;readonly collectMinimumIntervalMilliseconds:number;readonly collectDutyPercent:number;readonly budgetEnvironment:string;readonly heapEnvironment:string;readonly planEnvironment:string;readonly limitEnvironment:string;readonly planCeilingEnvironment:string;readonly stopCollectMinimumMilliseconds:number;readonly waitBoundMilliseconds:number;readonly fleetFallbackMilliseconds:number;readonly fleetAskMilliseconds:number;readonly grantMegabytes:number;readonly refusalExitCode:number}
export declare const GRAPH_MEMORY_POLICY_V1:GraphMemoryPolicyV1;
export type GraphMemoryMessageKeyV1="waiting"|"serial"|"resumed"|"learning"|"stopped"|"admittedAlone"|"refused"|"fleet";
export declare const GRAPH_MEMORY_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<GraphMemoryMessageKeyV1,string>>>>;
export function selectGraphMemoryLanguagesV1(environment:Readonly<Record<string,string|undefined>>):("en"|"de")[];
export function renderGraphMemoryMessagesV1(key:GraphMemoryMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string[];
export function hostMemoryMegabytesV1():number;
export function graphServiceLimitMegabytesV1(hostMegabytes:number,policy?:GraphMemoryPolicyV1):number;
export type FleetHostReadingV1=Readonly<{physicalMegabytes:number;limitMegabytes:number|null;hostMegabytes:number;availableMegabytes:number|null}>;
export function fleetHostReadingV1(ports?:Readonly<{ports?:unknown;total?:()=>number;environment?:Readonly<Record<string,string|undefined>>;limit?:(ports?:unknown)=>number|undefined;available?:()=>number|undefined}>):FleetHostReadingV1;
/** 🧒️ The fixed JavaScriptCore memory size of the graph child process in bytes, independent of the capacity grant. */
export function graphChildHeapBytesV1(policy?:GraphMemoryPolicyV1):number;
export function graphServiceHeapBytesV1(policy?:GraphMemoryPolicyV1):number;
export interface MemoryPortsV1{readonly sample:()=>number;readonly collect:()=>void}
export function createMemoryPortsV1():MemoryPortsV1;
export function serviceMemoryHeadroomBytesV1(options?:{ports?:unknown;policy?:GraphMemoryPolicyV1;cgroup?:(ports?:unknown)=>{limitBytes:number;currentBytes:number}|undefined;free?:()=>number|undefined;total?:()=>number;environment?:Readonly<Record<string,string|undefined>>;usage?:()=>number}):number;
export interface MemoryGovernorOptionsV1{estimate?:(key:string,kind:string)=>number|undefined;budgetBytes?:number;baselineBytes?:number;ports?:MemoryPortsV1;workers:number;headroom?:()=>number;attributable?:()=>number;measure?:()=>number;collect?:()=>void;hold?:()=>boolean;ask?:{set(row:{key:string;needMegabytes:number;alone:boolean}):void;clear():void};ownerLimitBytes?:number;onStop?:(stop:GraphOwnerStopRowV1)=>void;costs?:GraphOwnerCostPortV1;fleet?:GraphFleetPortV1;estimateBytes?:number;peaks?:GraphOwnerPeaksV1;decide?:(decision:{key:string;used:number;budget:number;kind?:string})=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;signal?:AbortSignal;report?:(line:string)=>void;environment?:Readonly<Record<string,string|undefined>>;policy?:GraphMemoryPolicyV1}
export interface MemoryGovernorStatsV1{stops:number;waits:number;waitedMilliseconds:number;peakOwners:number;ownerCostMegabytes:number;kinds:Readonly<Record<string,number>>}
export interface MemoryGovernorV1{pressure():boolean;run<T>(task:(owner:{away:<U>(work:()=>Promise<U>|U)=>Promise<U>;scope:GraphOwnerScopeV1})=>Promise<T>|T,options?:{kind?:string;key?:string;limitBytes?:number}):Promise<T>;stats():MemoryGovernorStatsV1;dispose():void}
export function createMemoryGovernorV1(options:MemoryGovernorOptionsV1&{headroom:()=>number}):MemoryGovernorV1;
export function graphHostAskV1(environment?:Record<string,string|undefined>):{set(row:{key:string;needMegabytes:number;alone:boolean}):void;clear():void}|undefined;
export function createGraphParentMemoryGovernorV1(options:MemoryGovernorOptionsV1):MemoryGovernorV1;
export function psProcessRowsV1(text:string):{pid:number;ppid:number;rssBytes:number}[];
export function processTreeBytesV1(rows:readonly {pid:number;ppid:number;rssBytes:number}[],pid:number):number;
export function createProcessTreeMemoryV1(options?:{pid?:number;platform?:string;run?:()=>string;own?:()=>number;now?:()=>number;maximumAgeMilliseconds?:number}):()=>number;
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
/** 📐️ The managed bytes the parent of a graph read may hold. */
export function graphParentManagedBytesV1(environment?:Readonly<Record<string,string|undefined>>,policy?:GraphMemoryPolicyV1):number;
/** 🧹️ What one wave of the parent held: peak and surviving managed bytes, collections run and the milliseconds they took. */
export interface ParentWaveMemoryV1{readonly peakBytes:number;readonly liveBytes:number;readonly collections:number;readonly collectMilliseconds:number}
export interface ParentCollectorV1{afterProject():boolean;finish():ParentWaveMemoryV1;headroomBytes():number}
export function createParentCollectorV1(options?:{ports?:MemoryPortsV1;policy?:GraphMemoryPolicyV1;budgetBytes?:number;baselineBytes?:number;now?:()=>number}):ParentCollectorV1;
/** 🛰️ Asks the fleet to infer the pending projects `keys`; answers the keys it takes over. */
export type GraphFleetPortV1=(ask:{keys:readonly string[];seconds:number})=>Promise<Iterable<string>|undefined>|Iterable<string>|undefined;
/** ⛔️ The typed refusal (exit code 75) of a graph read whose next project needs more than one project is granted. */
export class GraphMemoryRefusedErrorV1 extends Error{readonly code:"BUILD_GRAPH_MEMORY_REFUSED";readonly exitCode:75;readonly detail:Readonly<{key:string;needBytes:number;availableBytes:number;freeBytes:number;seconds:number}>;constructor(detail:{key:string;needBytes:number;availableBytes:number;freeBytes:number;seconds:number;message:string})}
/** 🛰️ A project the fleet took over while it waited for headroom. */
export class GraphOwnerFleetErrorV1 extends Error{readonly code:"BUILD_GRAPH_OWNER_FLEET";readonly detail:Readonly<{key:string}>;constructor(key:string)}/** 🔎️ The typed refusal inside `error` or its causes. */
export function graphMemoryRefusalOfV1(error:unknown,depth?:number):GraphMemoryRefusedErrorV1|undefined;