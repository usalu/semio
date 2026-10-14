/** 🗂️ Names closed graph fact namespaces in the authored fixed partition. */
export function repositoryBuildGraphDataDirectoryV1(workspace:string,namespace:"build-input-closures"|"build-lock-closures"|"inference"|"admission"):string;
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {RepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
/** 🎟️ Runs producers within original fixed-volume custody. */
export function withRepositoryBuildGraphFactStoreV1<T>(workspace:string,invocation:ScriptInvocation,body:(store:BuildGraphFactStoreV1)=>Promise<T>):Promise<T>;
/** 🎯️ Holds one selected inference namespace within the original graph writer. */
export function withRepositoryBuildGraphSelectionV1<T>(workspace:string,invocation:ScriptInvocation,identity:string,body:(store:BuildGraphFactStoreV1)=>Promise<T>):Promise<T>;
/** 🔒️ Checks live writer custody before synchronous publication. */
export function requireRepositoryBuildGraphWriterV1(workspace:string):void;
/** 📁️ Admits one closed directory on the original writer device. */
export function repositoryBuildGraphWriterDirectoryV1(workspace:string,namespace:"build-input-closures"|"build-lock-closures"|"inference"|"admission"):string;

/** 🧬️ Declares the sole logical Source root admitted to one bounded dataset. */
export interface BuildInferenceDatasetV1{readonly version:1;readonly identity:string;readonly sourceRoot:string}
/** 🧬️ Refuses undeclared dataset transport fields and ambient paths. */
export function readBuildInferenceDatasetV1(value:unknown):BuildInferenceDatasetV1;
/** 🧷️ Runs one logical dataset in original real repository graph custody. */
export function withRepositoryBuildInferenceDatasetV1<T>(workspace:string,invocation:ScriptInvocation,value:unknown,body:(scope:Readonly<{root:string;retire:()=>void}>)=>T|Promise<T>):Promise<T>;

/** 🧮️ Models exact optional cache peaks independently of physical custody. */
export interface BuildGraphOptionalQuotaStateV1{readonly maximumBytes:number;readonly usedBytes:number;readonly availableBytes:number;readonly minimumAvailableBytes:number}
export interface BuildGraphOptionalPublicationV1{readonly peakBytes:number;readonly retainedDeltaBytes:number}
export interface BuildGraphOptionalQuotaV1{readonly usedBytes:number;readonly pendingBytes:number;reserve(request:BuildGraphOptionalPublicationV1):Readonly<{commit():void;release():void}>|undefined}
export interface BuildGraphFactStoreV1 extends RepositoryCacheStoreV1{readonly optionalQuota:BuildGraphOptionalQuotaV1}
export function createBuildGraphOptionalQuotaV1(state:BuildGraphOptionalQuotaStateV1,check?:()=>void):BuildGraphOptionalQuotaV1;
/** 🏷️ Refuses mandatory files and foreign optional cache names. */
export function isBuildGraphOptionalCachePathV1(path:unknown):boolean;
/** 📦️ Reserves optional peak bytes under the original serial writer. */
export function withBuildGraphOptionalPublicationV1(path:string,plan:BuildGraphOptionalPublicationV1,write:()=>void):boolean;
/** 🔌️ Closes cache handles before the original writer leaves image custody. */
export function registerBuildGraphOptionalHandleV1(close:()=>void):void;
/** 🚰️ Decides whether a writer session must first retire optional leaves down to the low-water mark. */
export function buildGraphOptionalRetirementDueV1(usedBytes:number,availableBytes:number):boolean;
/** 🧹️ Defers optional pressure recovery when original exclusive image custody is unavailable. */
export function maintainBuildGraphOptionalCachesV1(image:import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts").RepositoryCacheStoreV1["image"],control:import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts").ResourceLeaseControl):Promise<boolean>;
