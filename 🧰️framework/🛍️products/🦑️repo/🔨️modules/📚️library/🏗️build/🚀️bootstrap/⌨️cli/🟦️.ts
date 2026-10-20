/** ⌨️ Closed native frontend recovery before JavaScript image or graph admission. */
import {readFileSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

const protocol=schema.$defs.Protocol.const;
export type NativeRepoRecoveryPlanV1=Readonly<{target:"build"|"install";owner:string}>;
export type NativeRepoRecoveryContextV1=Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>;
export type NativeRepoRecoveryPortsV1=Readonly<{build:(context:NativeRepoRecoveryContextV1)=>Promise<void>;install:(context:NativeRepoRecoveryContextV1)=>Promise<void>;child:(root:string,args:readonly string[],original:ScriptInvocation,phase:ResourceLeaseControl)=>Promise<void>}>;

/** 🎯️ Selects only fixed authored recovery commands, retaining ordinary graph semantics otherwise. */
export function readNativeRepoRecoveryPlanV1(root:string,args:readonly string[]):NativeRepoRecoveryPlanV1|undefined {
 if(args[0]!=="run")return undefined;const target=args[1]?.split(":");if(target?.length!==2||target[0]!==protocol.project||!protocol.targets.includes(target[1]!))return undefined;
 for(let index=2;index<args.length;index++){if(args[index]==="--output"&&args[index+1]==="stream"){index++;continue;}if(!protocol.options.includes(args[index]!)&&!(target[1]==="build"&&protocol.buildOnlyOptions.includes(args[index]!)))return undefined;}
 const owner=join(root,protocol.owner),manifest=JSON.parse(readFileSync(join(owner,"📋️project.json"),"utf8"));
 if(validateJsonSchemaSubset(schema.$defs.Owner,manifest).length)throw Error("Exact authored native recovery owner and dependencies required");
 return Object.freeze({target:target[1] as "build"|"install",owner});
}

/** 🧬️ Closes the installation's authored build dependency before the Original is issued. */
export function nativeRepoRecoveryBuildDescendantsV1(root:string,args:readonly string[]):readonly (readonly string[])[] {
 return readNativeRepoRecoveryPlanV1(root,args)?.target==="install"?[protocol.buildChild]:[];
}

/** 🛡️ Executes the existing owner beneath the exact genuine incoming Original and phase. */
export async function runNativeRepoRecoveryV1(root:string,args:readonly string[],original:ScriptInvocation,phase:ResourceLeaseControl,ports?:NativeRepoRecoveryPortsV1):Promise<boolean> {
 requireIssuedResourceControlInvocationV1(phase,original);checkResourceLeaseControl(phase);const plan=readNativeRepoRecoveryPlanV1(root,args);if(!plan)return false;
 if(!ports){const builder=await import("../../../../💻️client/🔌️mcp/🟦️.ts");checkResourceLeaseControl(phase);ports={build:builder.runBuildScript,install:async context=>(await import("../../../../💻️client/🔌️mcp/📜️script.ts")).installNativeRepoV1(context),child:async(workspace,request,parent,control)=>(await import("../../📥️caller/📤️child/🟦️.ts")).runOwnedBuildChildInvocationV1(workspace,request,parent,control)};}
 checkResourceLeaseControl(phase);const context=Object.freeze({root:plan.owner,repoRoot:root,invocation:original,control:phase});
 if(plan.target==="install"){await ports.child(root,protocol.buildChild,original,phase);checkResourceLeaseControl(phase);await ports.install(context);}else await ports.build(context);
 checkResourceLeaseControl(phase);return true;
}
