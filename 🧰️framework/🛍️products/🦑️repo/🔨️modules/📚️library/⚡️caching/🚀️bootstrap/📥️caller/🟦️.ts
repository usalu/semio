import{createNativeWindowsSystemGrantV1}from"../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import callerSchema from"../../../🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json";
import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";
import {resolveNxBuildMode} from "../🎚️mode/🟦️.ts";
import {resolveBuildSelectionV1} from "../../../🏗️build/🧭️invocation/🟦️.ts";
import {createNativeNxCallerCapabilitiesV1} from "../../📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts";

export const nxBootstrapJavascriptEnvironmentV1=schema.const.javascriptEnvironment;

/** 🧭️ Declares resolved Nx descendants before birth without loading CLI execution. */
export function createNxBootstrapCapabilitiesV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv):ReturnType<typeof createNativeNxCallerCapabilitiesV1>{
 const{invocation,launches}=resolveNxBootstrapLaunchesV1(root,args,environment);return createNativeNxCallerCapabilitiesV1(root,args,invocation.args,{...environment,...invocation.env},launches);
}

/** 🧭️ Resolves declared Nx request data without creating a native capability grant. */
export function resolveNxBootstrapLaunchesV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv){
 if(args[0]!=="nx")throw Error("Original bootstrap Nx request required");
 const invocation=resolveNxBuildMode(resolveBuildSelectionV1(root,args.slice(1)),environment),launches=[invocation.args],graphConsumer=!invocation.args.some(argument=>["--version","-v","--help","-h"].includes(argument))&&!["reset","daemon"].includes(invocation.args[0]??"");if(graphConsumer)launches.push(["run",nxBootstrapJavascriptEnvironmentV1,"--output-style=stream"]);
 if(invocation.watch)launches.push(["watch","--all","--includeGlobalWorkspaceFiles","--verbose","--","bun","nx","run",invocation.watch,"--output-style=stream"],["run",invocation.watch,"--output-style=stream"],["daemon","--start"]);
 return{invocation,launches};
}

/** 🪟️ Checks editor facts against the actual fresh OS before issuing an Nx Original. */
export function readNxBootstrapAuthoringCapabilitiesV1(value:unknown):ReturnType<typeof createNativeNxCallerCapabilitiesV1>{
 if(validateJsonSchemaSubset(callerSchema,value).length)throw Error("Authored Nx caller capabilities refused");const declared=value as ReturnType<typeof createNativeNxCallerCapabilitiesV1>,windowsSystem=createNativeWindowsSystemGrantV1(declared.native.windowsSystem);return{...declared,native:{...declared.native,...(windowsSystem?{windowsSystem}:{})}};
}
