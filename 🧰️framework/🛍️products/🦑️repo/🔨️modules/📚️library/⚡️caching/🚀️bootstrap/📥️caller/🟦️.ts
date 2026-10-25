import callerSchema from"../🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json" with { type: "json" };
import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {resolveNxBuildMode} from "../🎚️mode/🟦️.ts";
import {resolveBuildSelectionV1} from "../../../🏗️build/🧭️invocation/🟦️.ts";
import {createNativeNxCallerCapabilitiesV1} from "../../📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts";
import { runtimeCommandNameV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🧭️ Declares resolved Nx descendants before birth without loading CLI execution. */
export function createNxBootstrapCapabilitiesV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv):ReturnType<typeof createNativeNxCallerCapabilitiesV1>{
 const{invocation,launches}=resolveNxBootstrapLaunchesV1(root,args,environment);return createNativeNxCallerCapabilitiesV1(root,args,invocation.args,{...environment,...invocation.env},launches);
}

/** 📬️ Resolves declared Nx request data without creating a native capability grant. */
export function resolveNxBootstrapLaunchesV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv){
 if(args[0]!=="nx")throw Error("Original bootstrap Nx request required");
 const invocation=resolveNxBuildMode(resolveBuildSelectionV1(root,args.slice(1)),environment),launches=[invocation.args];
 if(invocation.watch)launches.push(["watch","--all","--includeGlobalWorkspaceFiles","--verbose","--",runtimeCommandNameV1(),"nx","run",invocation.watch,"--output-style=stream"],["run",invocation.watch,"--output-style=stream"],["daemon","--start"]);
 return{invocation,launches};
}

/** 🪟️ Checks editor facts against the actual fresh OS before issuing an Nx Original. */
export function readNxBootstrapAuthoringCapabilitiesV1(value:unknown):ReturnType<typeof createNativeNxCallerCapabilitiesV1>{
 if(validateJsonSchemaSubset(callerSchema,value).length)throw Error("Authored Nx caller capabilities refused");return value as ReturnType<typeof createNativeNxCallerCapabilitiesV1>;
}
