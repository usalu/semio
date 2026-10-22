import {buildBackendIdsV1,buildBackendRowV1} from "../🔌️backends/🟦️.ts";
import catalogue from "./🔣️catalog.json" with {type:"json"};
import {advanceScriptInvocation,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🛗️ What an operation needs before it starts: `none` runs on a fresh clone with only Bun, `dependencies` needs the locked dependencies, which the CLI ensures first; an unknown name needs them, so a typo never skips the provisioning. */
export function buildOperationRequiresV1(name:string):"none"|"dependencies"{return (catalogue.operations as readonly {name:string;requires:"none"|"dependencies"}[]).find(row=>row.name===name)?.requires??"dependencies";}

/** 🧭️ A dependency-free neutral operation owns its bootstrap and never needs another build tool's task graph. */
export function buildOperationHasNeutralBootstrapV1(name:string):boolean{
 const operation=catalogue.operations.find(row=>row.name===name);
 return operation?.requires==="none"&&buildBackendIdsV1().every(id=>(buildBackendRowV1(id).operations[name]??"neutral")==="neutral");
}

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

/** 🧭️ Runs `cache status|clear` against the tool the caller resolves, so this neutral module never imports an adapter, and returns the portable result. */
export async function runBuildCacheOperationV1(tool:BuildToolNameV1,args:readonly string[],context:Omit<BuildCacheContextV1,"progress">,resolveImplementation:(tool:BuildToolNameV1)=>Promise<BuildCacheToolV1>):Promise<BuildCacheStatusV1|BuildCacheClearV1>{
 const operation=args[0];
 if(operation!=="status"&&operation!=="clear"||args.length!==1&&!(operation==="status"&&args.length===2&&args[1]==="--tasks"))throw Error("Expected repo cache status|clear (status accepts --tasks)");
 const progress=async(stage:string)=>{checkResourceLeaseControl(context.phaseControl);await advanceScriptInvocation(context.invocation,"build:cache:"+stage,"running");checkResourceLeaseControl(context.phaseControl);},actual={...context,progress},implementation=await resolveImplementation(tool);
 checkResourceLeaseControl(context.phaseControl);
 if(operation==="status")return implementation.status(actual);
 const before=await implementation.status(actual);await progress("clearing");await implementation.clear(actual);const after=await implementation.status(actual);
 return Object.freeze({version:1,tool,cleared:after.entries===0,removedEntries:Math.max(0,before.entries-after.entries),removedBytes:Math.max(0,before.bytes-after.bytes)});
}

/** 🕸️ Tool-neutral task graph of a dry run: ids, their targets, ordering dependencies, and dependencies that must only be running. */
export type BuildTaskGraphV1=Readonly<{roots:readonly string[];tasks:Readonly<Record<string,Readonly<{id:string;target:Readonly<{project:string;target:string;configuration?:string}>}>>>;dependencies:Readonly<Record<string,readonly string[]>>;continuousDependencies?:Readonly<Record<string,readonly string[]>>}>;

/** 📋️ What every tool prints for `--dry-run`: the task graph it would execute. */
export type BuildDryRunPlanV1=Readonly<{tasks:BuildTaskGraphV1}>;

/** 📋️ Plans `exec` without running anything: one `project:repo-exec` task per selected project, shaped like every other dry run. */
export function buildExecPlanV1(projects:readonly string[]):BuildDryRunPlanV1{
 const ids=projects.map(project=>project+":repo-exec");
 return Object.freeze({tasks:Object.freeze({roots:ids,tasks:Object.fromEntries(projects.map((project,index)=>[ids[index]!,{id:ids[index]!,target:{project,target:"repo-exec"}}])),dependencies:Object.fromEntries(ids.map(id=>[id,[] as string[]]))})});
}
