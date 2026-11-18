/** 🦶️ The declared numbers of the footprint gate. */
export interface FootprintPolicyV1{readonly version:1;readonly softPercent:number;readonly hardPercent:number;readonly ttlMilliseconds:number;readonly settleMinimumMilliseconds:number;readonly waitStepMilliseconds:number;readonly waitBoundMilliseconds:number;readonly waitCooldownMilliseconds:number;readonly watchMilliseconds:number;readonly censusTypes:number;readonly traceBytes:number;readonly heartbeatMilliseconds:number;readonly grantTtlMilliseconds:number;readonly sampleMilliseconds:number;readonly sampleTypes:number;readonly sampleGrowthBytes:number;readonly sampleQuietMilliseconds:number;readonly sampleLagMilliseconds:number;readonly pulseMilliseconds:number;readonly pulseStarvedMilliseconds:number;readonly pulseFrames:number;readonly releaseGrowthBytes:number;readonly takeReleaseGrowthBytes:number;readonly releaseDutyPercent:number;readonly censusDutyPercent:number;readonly commandTimeoutMilliseconds:number;readonly languages:readonly string[]}
export declare const FOOTPRINT_POLICY_V1:FootprintPolicyV1;
export type FootprintMessageKeyV1="waveSettled"|"relieved"|"slowed"|"census"|"sampled"|"starved";
export declare const FOOTPRINT_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<FootprintMessageKeyV1,string>>>>;
export function renderFootprintMessageV1(key:FootprintMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment?:Readonly<Record<string,string|undefined>>):string[];
export type FootprintDecisionV1="run"|"settle"|"wait";
export function decideFootprintV1(input:{footprintBytes:number;grantBytes:number|undefined;policy?:FootprintPolicyV1}):FootprintDecisionV1;
export function footprintHeadroomBytesV1(input:{footprintBytes:number;grantBytes:number|undefined;policy?:FootprintPolicyV1}):number;
export function readProcessFootprintBytesV1(pid:number,ports?:{platform?:NodeJS.Platform;run?:(command:string,args:readonly string[])=>Promise<string>;read?:(path:string)=>Promise<string|null>;rss?:()=>number}):Promise<number|undefined>;
export interface FootprintMeterV1{bytes():number;refresh():Promise<number>;fresh():Promise<number>;measured():boolean;peakBytes():number}
/** 🪫️ The bytes the runtime of this process holds (heap in use plus external memory). */
export const managedBytesV1:()=>number;
/** 🧱️ A footprint reading of this process raised to what its own runtime holds; a failed reading (undefined) stays one. */
export function footprintAtLeastManagedV1<T extends number|undefined>(bytes:T,managed?:()=>number):T extends number?number:undefined;
export function createFootprintMeterV1(options?:{pid?:number;read?:()=>Promise<number|undefined>;rss?:()=>number;now?:()=>number;ttlMilliseconds?:number}):FootprintMeterV1;
export function releaseRuntimeMemoryV1(ports?:{collect?:()=>void;shrink?:()=>void}):void;
export interface HeapCensusV1{readonly heapMegabytes:number;readonly extraMegabytes:number;readonly objects:number;readonly types:string}
export function heapCensusV1(options?:{stats?:()=>Promise<{heapSize?:number;extraMemorySize?:number;objectCount?:number;objectTypeCounts?:Record<string,number>}>|{heapSize?:number;extraMemorySize?:number;objectCount?:number;objectTypeCounts?:Record<string,number>};types?:number}):Promise<HeapCensusV1|undefined>;
export function createFootprintTraceV1(options?:{directory?:string;pid?:number;maximumBytes?:number;now?:()=>string}):((line:string)=>void)&{readonly path:string};
export interface FootprintStatsV1{readonly admitted:number;readonly settled:number;readonly waits:number;readonly waitedMilliseconds:number;readonly peakBytes:number}
export interface FootprintGateV1{admit(signal?:AbortSignal):Promise<FootprintDecisionV1>;relieve(reliever:()=>unknown):()=>boolean;watch():void;stop():void;settle(relieve?:boolean):Promise<number>;headroomBytes():number;footprintBytes():number;grantBytes():number|undefined;stats():FootprintStatsV1}
export function createFootprintGateV1(options:{meter?:FootprintMeterV1;grantBytes:()=>number|undefined;release?:()=>void;heapCensus?:()=>Promise<HeapCensusV1|undefined>;heartbeat?:(row:{footprintBytes:number;grantBytes:number;rssBytes:number})=>void;sample?:(line:string)=>void;phase?:()=>string|undefined;setTimer?:(callback:()=>unknown,milliseconds:number)=>{unref?:()=>unknown};clearTimer?:(timer:unknown)=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;report?:(line:string)=>void;environment?:Readonly<Record<string,string|undefined>>;policy?:FootprintPolicyV1}):FootprintGateV1;
export function createHeapSamplerV1(options:{meter:{bytes:()=>number};stats?:()=>unknown;usage?:()=>{rss:number;arrayBuffers?:number};now?:()=>number;phase?:()=>string|undefined;statsSync?:()=>unknown;stack?:()=>string;emit:(fields:Readonly<Record<string,string|number>>,key?:"sampled"|"starved")=>void;policy?:FootprintPolicyV1}):Readonly<{tick():Promise<void>;starved(at:number):void}>;
export function footprintPulseV1():void;
export function noteTraceV1(line:string,trace?:(line:string)=>void):void;
export function createStageClockV1(options?:{now?:()=>number}):Readonly<{add:(name:string,milliseconds:number)=>void;time:<T>(name:string,work:()=>Promise<T>|T)=>Promise<T>;totals:()=>Record<string,number>;text:()=>string}>;
export function createOwnerClockV1(options?:{now?:()=>number;trace?:(line:string)=>void;slowMilliseconds?:number;rollupOwners?:number}):Readonly<{begin:(name:string)=>void;add:(name:string,step:string,milliseconds:number)=>void;get:(name:string,step:string)=>number;finish:(name:string,state?:{running?:number})=>void}>;
export function noteSlowV1(label:string,startedAt:number,thresholdMilliseconds?:number,trace?:(line:string)=>void):number;
export function releaseIfGrownV1(options?:{usage?:()=>number;release?:()=>void;threshold?:number;every?:number;now?:()=>number;decision?:()=>FootprintDecisionV1|undefined;state?:{quietUntil:number;releasedAt:number}}):boolean;
export function noteFootprintStepV1(label:string|undefined):void;
export function footprintStepV1():string;
export function bindProcessGateV1(grant:(()=>unknown)|undefined,ports?:{trace?:(line:string)=>void;write?:(line:string)=>void;create?:typeof createFootprintGateV1}):FootprintGateV1|undefined;
export function readTreeFootprintBytesV1(pid:number,ports?:{observe?:(pid:number)=>Promise<number|null>}):Promise<number|undefined>;
export function planCeilingBytesV1(environment?:Readonly<Record<string,string|undefined>>,policy?:{readonly planCeilingEnvironment:string}):number|undefined;
export function useSliceCeilingGateV1(environment?:Readonly<Record<string,string|undefined>>,ports?:{bind?:typeof bindProcessGateV1;pid?:number;read?:(pid:number)=>Promise<number|undefined>}):FootprintGateV1|undefined;
export function useEnvironmentGateV1(environment?:Readonly<Record<string,string|undefined>>,ports?:{read?:()=>number|undefined;bind?:(grant:()=>unknown)=>FootprintGateV1|undefined;now?:()=>number}):FootprintGateV1|undefined;
export function useFootprintGateV1(gate:FootprintGateV1|undefined):void;
export function footprintGateV1():FootprintGateV1|undefined;
export function grantBytesOfV1(source:(()=>unknown)|undefined):number|undefined;
export interface WaveSettleResultV1{readonly beforeMegabytes:number;readonly afterMegabytes:number;readonly managedMegabytes:number;readonly grantMegabytes:number}
export function settleWaveMemoryV1(options:{generation:number;release?:()=>unknown;meter?:FootprintMeterV1;managed?:()=>number;grantBytes?:()=>number|undefined;collect?:()=>void;heapCensus?:()=>Promise<HeapCensusV1|undefined>;report?:(line:string)=>void;environment?:Readonly<Record<string,string|undefined>>}):Promise<WaveSettleResultV1|undefined>;
