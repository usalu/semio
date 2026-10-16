import schema from "./🧬️schema/🔣️.json";
import {resolveNxBuildMode} from "../🎚️mode/🟦️.ts";
import {resolveBuildSelectionV1} from "../../../🏗️build/🧭️invocation/🟦️.ts";
import {createNativeNxCallerCapabilitiesV1} from "../../📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts";

export const nxBootstrapJavascriptEnvironmentV1=schema.const.javascriptEnvironment;

/** 🧭️ Declares resolved Nx descendants before birth without loading CLI execution. */
export function createNxBootstrapCapabilitiesV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv):ReturnType<typeof createNativeNxCallerCapabilitiesV1>{
 if(args[0]!=="nx")throw Error("Original bootstrap Nx request required");
 const invocation=resolveNxBuildMode(resolveBuildSelectionV1(root,args.slice(1)),environment),launches=[invocation.args],graphConsumer=!invocation.args.some(argument=>["--version","-v","--help","-h"].includes(argument))&&!["reset","daemon"].includes(invocation.args[0]??"");if(graphConsumer)launches.push(["run",nxBootstrapJavascriptEnvironmentV1,"--output-style=stream"]);
 if(invocation.watch)launches.push(["watch","--all","--includeGlobalWorkspaceFiles","--verbose","--","bun","nx","run",invocation.watch,"--output-style=stream"],["run",invocation.watch,"--output-style=stream"],["daemon","--start"]);
 return createNativeNxCallerCapabilitiesV1(root,args,invocation.args,{...environment,...invocation.env},launches);
}
