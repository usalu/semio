/** 🧠️ The declared limits of the graph memory governor. */
export interface GraphMemoryPolicyV1{readonly version:1;readonly languages:readonly ("en"|"de")[];readonly languageEnvironment:readonly string[];readonly minimumBudgetMegabytes:number;readonly maximumBudgetMegabytes:number;readonly hostShare:number;readonly grantShare:number;readonly resumeRatio:number;readonly pollMilliseconds:number;readonly sampleMilliseconds:number;readonly admitMinimumMilliseconds:number;readonly collectMinimumMilliseconds:number;readonly retainedMilliseconds:number;readonly progressMilliseconds:number;readonly retainedImageBytes:number;readonly minimumInternedLength:number;readonly maximumInternedCharacters:number;readonly maximumCachedClosureEntries:number;readonly maximumCachedClosureEntries:number;readonly budgetEnvironment:string;readonly planEnvironment:string}
export declare const GRAPH_MEMORY_POLICY_V1:GraphMemoryPolicyV1;
export type GraphMemoryMessageKeyV1="waiting"|"serial"|"resumed";
export declare const GRAPH_MEMORY_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<GraphMemoryMessageKeyV1,string>>>>;
export function selectGraphMemoryLanguagesV1(environment:Readonly<Record<string,string|undefined>>):("en"|"de")[];
export function renderGraphMemoryMessagesV1(key:GraphMemoryMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string[];
export function hostMemoryMegabytesV1():number;
export function graphMemoryBudgetBytesV1(environment:Readonly<Record<string,string|undefined>>,policy?:GraphMemoryPolicyV1,hostMegabytes?:number):number;
export interface MemoryAdmissionStateV1{used:number;budget:number;running:number;workers:number;blocked:boolean;collected:boolean}
export interface MemoryAdmissionVerdictV1{admit:boolean;collect:boolean;reason:"workers"|"within"|"collect"|"over"|"forced"}
export function decideMemoryAdmissionV1(policy:GraphMemoryPolicyV1,state:MemoryAdmissionStateV1):MemoryAdmissionVerdictV1;
export interface MemoryPortsV1{readonly sample:()=>number;readonly collect:()=>void}
export function createMemoryPortsV1():MemoryPortsV1;
export function measureMemoryBaselineV1(ports?:MemoryPortsV1):number;
export function measureMemoryBaselineV1(ports?:MemoryPortsV1):number;
export interface MemoryGovernorOptionsV1{budgetBytes:number;workers:number;baselineBytes?:number;sample?:()=>number;collect?:()=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;signal?:AbortSignal;report?:(line:string)=>void;environment?:Readonly<Record<string,string|undefined>>;policy?:GraphMemoryPolicyV1}
export interface MemoryGovernorStatsV1{waits:number;waitedMilliseconds:number;collections:number;peakMegabytes:number}
export interface MemoryGovernorV1{run<T>(task:()=>Promise<T>|T):Promise<T>;stats():MemoryGovernorStatsV1}
export function createMemoryGovernorV1(options:MemoryGovernorOptionsV1):MemoryGovernorV1;
export interface StringInternerV1{share(text:string):string;stats():{entries:number;characters:number;hits:number;clears:number};clear():void}
export function createStringInternerV1(options?:{maximumCharacters?:number;minimumLength?:number}):StringInternerV1;
export function internJsonStringsV1<T>(value:T,interner:StringInternerV1):T;
export declare const graphStringInternerV1:StringInternerV1;
