/** ⌨️ Closed native frontend recovery before JavaScript image or graph admission. */
import {captureBoundedSourceRevisionV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

const protocol=schema.$defs.Protocol.const;
export type NativeRepoRecoveryPlanV1=Readonly<{kind:"frontend";target:"build"|"install";owner:string}>|Readonly<{kind:"maintenance";target:"preview"|"retire";owner:string}>|Readonly<{kind:"cache-reset";target:"graph"|"nx";owner:string}>;
export type NativeRepoRecoveryContextV1=Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>;
export type NativeRepoRecoveryPortsV1=Readonly<{build?:(context:NativeRepoRecoveryContextV1)=>Promise<void>;install?:(context:NativeRepoRecoveryContextV1)=>Promise<void>;child?:(root:string,args:readonly string[],original:ScriptInvocation,phase:ResourceLeaseControl)=>Promise<void>;maintenance?:(context:NativeRepoRecoveryContextV1,operation:"preview"|"retire")=>Promise<void>;reset?:(context:NativeRepoRecoveryContextV1,profile:"graph"|"nx")=>Promise<void>}>;

/** 🎯️ Selects only fixed authored recovery commands, retaining ordinary graph semantics otherwise. */
export function readNativeRepoRecoveryPlanV1(root:string,args:readonly string[]):NativeRepoRecoveryPlanV1|undefined {
 if(args[0]!=="run")return undefined;const target=args[1]?.split(":");if(target?.length!==2)return undefined;
 const maintenance=target[0]===protocol.maintenance.project&&Object.hasOwn(protocol.maintenance.targets,target[1]!),frontend=target[0]===protocol.project&&protocol.targets.includes(target[1]!),reset=target[0]===protocol.reset.project&&Object.hasOwn(protocol.reset.targets,target[1]!);if(!maintenance&&!frontend&&!reset)return undefined;
 let parallel=false;for(let index=2;index<args.length;index++){if(!parallel&&/^--parallel=[1-9][0-9]*$/u.test(args[index]!)&&!validateJsonSchemaSubset(schema.$defs.CapacityParallelJobs,Number(args[index]!.slice(11))).length){parallel=true;continue;}if(args[index]==="--output"&&args[index+1]==="stream"){index++;continue;}if(!protocol.options.includes(args[index]!)&&!(frontend&&target[1]==="build"&&protocol.buildOnlyOptions.includes(args[index]!)))return undefined;}
 const owner=join(root,reset?protocol.reset.owner:maintenance?protocol.maintenance.owner:protocol.owner),captured=captureBoundedSourceRevisionV1(join(owner,"📋️project.json"),protocol.maximumOwnerBytes,true),manifest=JSON.parse(captured.text!);
 if(validateJsonSchemaSubset(reset?schema.$defs.ResetOwner:maintenance?schema.$defs.MaintenanceOwner:schema.$defs.Owner,manifest).length)throw Error("Exact authored native recovery owner and dependencies required");
 return reset?Object.freeze({kind:"cache-reset",target:protocol.reset.targets[target[1] as keyof typeof protocol.reset.targets] as "graph"|"nx",owner:join(root,protocol.reset.scriptRoot)}):maintenance?Object.freeze({kind:"maintenance",target:protocol.maintenance.targets[target[1] as keyof typeof protocol.maintenance.targets] as "preview"|"retire",owner:join(root,protocol.maintenance.scriptRoot)}):Object.freeze({kind:"frontend",target:target[1] as "build"|"install",owner});
}

/** 🧬️ Closes the installation's authored build dependency before the Original is issued. */
export function nativeRepoRecoveryBuildDescendantsV1(root:string,args:readonly string[]):readonly (readonly string[])[] {
 const plan=readNativeRepoRecoveryPlanV1(root,args);return plan?.kind==="frontend"&&plan.target==="install"?[protocol.buildChild]:[];
}

/** 🛡️ Executes the existing owner beneath the exact genuine incoming Original and phase. */
export async function runNativeRepoRecoveryV1(root:string,args:readonly string[],original:ScriptInvocation,phase:ResourceLeaseControl,ports?:NativeRepoRecoveryPortsV1):Promise<boolean> {
 requireIssuedResourceControlInvocationV1(phase,original);checkResourceLeaseControl(phase);const plan=readNativeRepoRecoveryPlanV1(root,args);if(!plan)return false;
 if(!ports){if(plan.kind==="cache-reset")ports={reset:async(context,profile)=>(await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts")).runInactiveRepositoryCacheResetV1(context,profile)};else if(plan.kind==="maintenance")ports={maintenance:async(context,operation)=>(await import("../../../⚡️caching/🧹️pruning/🏪️budget/📜️script.ts")).runInactiveCompilerMaintenanceV1(context,operation)};else{const builder=await import("../../../../💻️client/🔌️mcp/🟦️.ts");checkResourceLeaseControl(phase);ports={build:builder.runBuildScript,install:async context=>(await import("../../../../💻️client/🔌️mcp/📜️script.ts")).installNativeRepoV1(context),child:async(workspace,request,parent,control)=>(await import("../../📥️caller/📤️child/🟦️.ts")).runOwnedBuildChildInvocationV1(workspace,request,parent,control)};}}
 checkResourceLeaseControl(phase);const context=Object.freeze({root:plan.owner,repoRoot:root,invocation:original,control:phase});
 if(plan.kind==="cache-reset"){if(!ports.reset)throw Error("Exact inactive repository cache reset owner port required");await ports.reset(context,plan.target);}
 else if(plan.kind==="maintenance"){if(!ports.maintenance)throw Error("Exact maintenance owner port required");await ports.maintenance(context,plan.target);}
 else if(plan.target==="install"){if(!ports.child||!ports.install)throw Error("Exact native installation ports required");await ports.child(root,protocol.buildChild,original,phase);checkResourceLeaseControl(phase);await ports.install(context);}else{if(!ports.build)throw Error("Exact native build port required");await ports.build(context);}
 checkResourceLeaseControl(phase);return true;
}
