import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🛠️ Names the build tools that implement the neutral operations. */
export type BuildToolNameV1=string;
/** 📈️ Counts what a tool's builds did since its cache was last cleared: builds, scheduled actions, actions that ran, and actions restored from a cache. */
export type BuildCacheActivityV1=Readonly<{builds:number;actions:number;executed:number;cacheHits:number}>;
/** 🗄️ Reports one build tool's local task cache without exposing the tool's own layout; `activity` is present for tools that account for their builds. */
export type BuildCacheStatusV1=Readonly<{version:1;tool:BuildToolNameV1;available:boolean;directory:string|null;entries:number;bytes:number;activity?:BuildCacheActivityV1}>;
/** 🧹️ Reports how much a clear removed, measured by the same status before and after. */
export type BuildCacheClearV1=Readonly<{version:1;tool:BuildToolNameV1;cleared:boolean;removedEntries:number;removedBytes:number}>;
/** 🎛️ Carries the issued phase, so every tool implementation stops when the caller cancels and reports progress. */
export type BuildCacheContextV1=Readonly<{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;progress:(stage:string)=>Promise<void>}>;
/** 🔌️ The build tool's side of the cache operation. */
export interface BuildCacheToolV1 {
 readonly tool:BuildToolNameV1;
 status(context:BuildCacheContextV1):Promise<BuildCacheStatusV1>;
 clear(context:BuildCacheContextV1):Promise<void>;
}
/** 🧪️ One input of a cache probe target, spelled without any tool's syntax. */
export type CacheProbeInputV1=Readonly<{file:string}>|Readonly<{env:string}>|Readonly<{named:string}>;
/** 🎯️ One probe target with the inputs and outputs the tool must hash and restore. */
export type CacheProbeTargetV1=Readonly<{name:string;cache:boolean;inputs?:readonly CacheProbeInputV1[];outputs:readonly string[];dependsOn?:readonly string[];command:string}>;
/** 🧬️ Tool-neutral description of the isolated workspace that proves cache restoration. */
export type CacheProbeSpecV1=Readonly<{version:1;project:string;maxCacheSize:string;namedInputs:Readonly<Record<string,readonly CacheProbeInputV1[]>>;ignore:readonly string[];targets:readonly CacheProbeTargetV1[]}>;
/** 🔬️ The build tool's side of a cache probe: renders its workspace files and its task invocation. */
export interface CacheProbeToolV1 {
 readonly tool:BuildToolNameV1;
 readonly label:string;
 version():string;
 files(specification:CacheProbeSpecV1):Readonly<Record<string,string|object>>;
 invocation(specification:CacheProbeSpecV1,task:string,fixture:string,environment:Readonly<Record<string,string|undefined>>):Readonly<{command:string;arguments:readonly string[];environment:Readonly<Record<string,string|undefined>>}>;
}
