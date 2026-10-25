import type { BuildCacheClearV1, BuildCacheContextV1, BuildCacheStatusV1, BuildCacheToolV1, BuildToolNameV1 } from "./📋️contract/🟦️.ts";
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

/** 🔨️ Runs `cache status|clear` against the tool the caller resolves, so this neutral module never imports an adapter, and returns the portable result. */
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

/** 🗓️ Plans `exec` without running anything: one `project:repo-exec` task per selected project, shaped like every other dry run. */
export function buildExecPlanV1(projects:readonly string[]):BuildDryRunPlanV1{
 const ids=projects.map(project=>project+":repo-exec");
 return Object.freeze({tasks:Object.freeze({roots:ids,tasks:Object.fromEntries(projects.map((project,index)=>[ids[index]!,{id:ids[index]!,target:{project,target:"repo-exec"}}])),dependencies:Object.fromEntries(ids.map(id=>[id,[] as string[]]))})});
}
