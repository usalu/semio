import {projectBuildOrchestrationCapabilitiesV1} from "./📤️orchestration/🟨️.mjs";
import nxCallerSchema from"../../🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json";
import authoringSchema from "../🧬️schema/📥️authoring.json";
import schema from "../🧬️schema/📥️caller.json";
import nativeSchema from "../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json";
import executionPolicy from "../../⚡️caching/📦️artifacts/📋️native-orchestration/🔣️policy.json";
import { canonicalArchitectureEnvironment } from "../../⚡️caching/🦀️cargo/🌿️environment/🟦️.ts";
import { readNativeOwnerCapabilities } from "../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import type { NativeOwnerCapabilities } from "../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { checkScriptInvocation, readIssuedScriptProcessEnvelopeV1, scriptProcessEnvironment, readScriptProcessEnvelope, type ScriptProcessEnvelope, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { BuildExecutionV1 } from "../🕸️graph/🟦️.ts";
import { resolve } from "node:path";
import {issueRepositoryCargoExecutionBackendV1} from "../../🏃️process/🦀️cargo/🏃️execution/🟦️.ts";

/** 🏛️ Declares actual native resources granted to an owned build invocation. */
export interface NativeBuildCallerCapabilitiesV1 { readonly tool:"nx"|"bazel"; readonly workspaceRoot:string; readonly arguments:readonly string[]; readonly selection:readonly string[]; readonly buildMode:"prototype"|"release"; readonly native:Readonly<{artifactDirectory:string;transport:NativeOwnerCapabilities["transport"];child:NativeOwnerCapabilities["child"];network:NativeOwnerCapabilities["network"]}>; readonly launches:readonly (readonly string[])[] }

/** 🎟️ Creates the configured native grant before the build process envelope is issued. */
export function createNativeBuildCallerCapabilitiesV1(root:string,args:readonly string[],tool:"nx"|"bazel",environment:NodeJS.ProcessEnv):NativeBuildCallerCapabilitiesV1 {
  if(validateJsonSchemaSubset(nativeSchema.$defs.NativeOwnerExecutionPolicyV1,executionPolicy).length)throw Error("Build native execution policy refused");
  const canonical=canonicalArchitectureEnvironment(root,environment),offline=canonical.CARGO_NET_OFFLINE??"false";
  if(!["true","false"].includes(offline))throw Error("Build native network selection refused");
  const capabilities={tool,workspaceRoot:resolve(root),arguments:["repo",...args],selection:["repo",...args],buildMode:(environment.SEMIO_BUILD_MODE??"prototype") as "prototype"|"release",native:{artifactDirectory:canonical.CARGO_TARGET_DIR!,transport:executionPolicy.transport,child:executionPolicy.child,network:{offline:offline==="true"}},launches:[["repo",...args]]};
  if(validateJsonSchemaSubset(schema,capabilities).length)throw Error("Build native caller capabilities refused");
  return capabilities;
}

/** 📝️ Admits explicit editor resource grants before an original process envelope is created. */
export function readBuildAuthoringCapabilitiesV1(root:string,args:readonly string[],tool:"nx"|"bazel",value:unknown):NativeBuildCallerCapabilitiesV1 {
  if(validateJsonSchemaSubset(authoringSchema,value).length)throw Error("Build authoring capabilities refused");
  const declared=value as NativeBuildCallerCapabilitiesV1,request=["repo",...args],workspaceRoot=resolve(root);
  if(resolve(workspaceRoot,declared.workspaceRoot)!==workspaceRoot||declared.tool!==undefined&&declared.tool!==tool||JSON.stringify(declared.arguments)!==JSON.stringify(request)||JSON.stringify(declared.selection)!==JSON.stringify(request)||!declared.launches.some(launch=>JSON.stringify(launch)===JSON.stringify(request)))throw Error("Original authored build request changed");
  const capabilities={...declared,tool,workspaceRoot,native:{...declared.native,artifactDirectory:resolve(workspaceRoot,declared.native.artifactDirectory)}};
  if(validateJsonSchemaSubset(schema,capabilities).length)throw Error("Initial build caller capabilities refused");
  return capabilities;
}

/** 📤️ Combines declared literal defaults with this caller's original task arguments. */
export function buildExecutionArgumentsV1(execution:BuildExecutionV1,args:readonly string[]):string[] {
  if(execution.forwardAllArgs===false)return [];
  const defaults=execution.args??[];
  if(!Array.isArray(defaults)||defaults.some(argument=>typeof argument!=="string"))throw Error("Build execution arguments must be literal strings");
  return [...defaults,...args];
}

/** 🔑️ Issues one inferred native command from the selected graph's original orchestration resources. */
export function issueBuildTaskEnvironmentV1(root:string,invocation:ScriptInvocation,execution:BuildExecutionV1,args:readonly string[],environment:NodeJS.ProcessEnv,orchestration?:object):NodeJS.ProcessEnv {
  readIssuedScriptProcessEnvelopeV1(invocation);
  const capabilities=invocation.capabilities as NativeBuildCallerCapabilitiesV1;
  if(validateJsonSchemaSubset(schema,capabilities).length||resolve(capabilities.workspaceRoot)!==resolve(root))throw Error("Original build caller authority required");
  const canonical=canonicalArchitectureEnvironment(root,{...environment,SEMIO_BUILD_MODE:capabilities.buildMode,CARGO_NET_OFFLINE:String(capabilities.native.network.offline)});
  if(resolve(capabilities.native.artifactDirectory)!==canonical.CARGO_TARGET_DIR)throw Error("Build native artifact authority changed");
  const request=execution.nativeOwnerCommand,effective=buildExecutionArgumentsV1(execution,args);
  let selected:object=capabilities;
  if(orchestration!==undefined){if(request!==undefined)throw Error("Native commands cannot receive orchestration authority");selected=projectOwnedBuildOrchestrationProcessEnvelopeV1(root,invocation,orchestration).capabilities;}
  if(request!==undefined){
    if(validateJsonSchemaSubset(nativeSchema.properties.command,request).length)throw Error("Discovered native build command required");
    const declared=request as NativeOwnerCapabilities["command"],command={...declared,arguments:[...declared.arguments,...effective]},cargoExecutionBackend=issueRepositoryCargoExecutionBackendV1(root,command,capabilities.native.transport);
    selected=readNativeOwnerCapabilities({version:1,buildMode:capabilities.buildMode,repositoryRoot:root,artifactDirectory:capabilities.native.artifactDirectory,command,transport:capabilities.native.transport,child:capabilities.native.child,network:capabilities.native.network,...(cargoExecutionBackend?{cargoExecutionBackend}:{})});
  }
  const result=scriptProcessEnvironment({...readIssuedScriptProcessEnvelopeV1(invocation),capabilities:selected},{...canonical,SEMIO_BUILD_TOOL:capabilities.tool,SEMIO_WORKSPACE_ROOT:root});
  delete result.SEMIO_SCRIPT_CAPABILITIES;
  if(execution.ownerArguments)result.SEMIO_OWNER_ARGUMENTS=JSON.stringify({version:1,arguments:effective});
  return result;
}

/** 🪪️ Admits only an already declared Repo request beneath its privately live original. */
export function readOwnedBuildInvocationV1(root:string,invocation:ScriptInvocation,args:readonly string[]):NativeBuildCallerCapabilitiesV1{
 const original=readIssuedScriptProcessEnvelopeV1(invocation),capabilities=original.capabilities as NativeBuildCallerCapabilitiesV1;if(validateJsonSchemaSubset(schema,capabilities).length||resolve(capabilities.workspaceRoot)!==resolve(root)||capabilities.arguments[0]!=="repo")throw Error("Original Repo build caller required");
 const request:string[]=["repo"];let tool=capabilities.tool;const selected:string[]=[];for(let index=0;index<args.length;index++){const argument=args[index]!;if(argument==="--"){selected.push(...args.slice(index));break;}if(argument==="--tool"){tool=args[++index]as typeof tool;continue;}if(argument.startsWith("--tool=")){tool=argument.slice(7)as typeof tool;continue;}selected.push(argument);}request.push(...selected);if(tool!==capabilities.tool||!capabilities.launches.some(launch=>JSON.stringify(launch)===JSON.stringify(request)))throw Error("Undeclared original Repo build selection");return capabilities;
}

/** 📨️ Serializes only predeclared Nx child data while retaining the exact private parent invocation. */
export function projectOwnedNxProcessEnvelopeV1(root:string,invocation:ScriptInvocation,selected:readonly string[]):ScriptProcessEnvelope{
 const original=readIssuedScriptProcessEnvelopeV1(invocation),role=original.capabilities as NativeBuildCallerCapabilitiesV1,selection=["nx",...selected];let capabilities:object;
 if(role.tool!==undefined){if(validateJsonSchemaSubset(schema,role).length||resolve(role.workspaceRoot)!==resolve(root)||role.arguments[0]!=="repo")throw Error("Original Repo-to-Nx caller required");const launches=role.launches.filter(launch=>launch[0]==="nx");if(!launches.length||!launches.some(launch=>JSON.stringify(launch)===JSON.stringify(selection)))throw Error("Undeclared Repo-to-Nx child selection");capabilities={workspaceRoot:role.workspaceRoot,arguments:launches[0],selection,buildMode:role.buildMode,native:role.native,launches};}
 else{const role=original.capabilities as import("../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts").NativeNxCallerCapabilitiesV1;if(validateJsonSchemaSubset(nxCallerSchema,role).length||resolve(role.workspaceRoot)!==resolve(root)||role.arguments[0]!=="nx"||!role.launches?.some(launch=>JSON.stringify(launch)===JSON.stringify(selection)))throw Error("Undeclared original Nx child selection");capabilities={...role,selection};}
 if(validateJsonSchemaSubset(nxCallerSchema,capabilities).length)throw Error("Closed original Nx child data required");return readScriptProcessEnvelope({...original,capabilities},Date.now());
}

/** 🧭️ Projects serialized task data only beneath the exact live canonical process issuer. */
export function projectOwnedBuildOrchestrationProcessEnvelopeV1(root:string,invocation:ScriptInvocation,context:object):ScriptProcessEnvelope{
 const original=readIssuedScriptProcessEnvelopeV1(invocation),caller=original.capabilities as NativeBuildCallerCapabilitiesV1,capabilities=projectBuildOrchestrationCapabilitiesV1(original,resolve(root),caller.tool,context,Date.now());return readScriptProcessEnvelope({...original,capabilities},Date.now());
}
