import executionPolicy from "../🔣️policy.json";
import nativeSchema from "../📥️invocation/🧬️schema/🔣️.json";
import type {NativeOwnerCapabilities} from "../📥️invocation/🟦️.ts";
import callerSchema from "../../../../🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json";
import {canonicalArchitectureEnvironment} from "../../../🦀️cargo/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {readScriptProcessEnvelope,scriptProcessEnvironment,type ScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {resolve} from "node:path";

/** 📥️ Issues resources for the actual selected Nx child while retaining the original request and deadline. */
export function nativeNxChildEnvironmentV1(root:string,envelope:ScriptProcessEnvelope,selected:readonly string[],environment:Readonly<Record<string,string|undefined>>):Record<string,string|undefined>{
 const original=readScriptProcessEnvelope(envelope,Date.now()),requested=original.capabilities as {workspaceRoot?:unknown;arguments?:unknown;native?:unknown;selection?:unknown};
 if(requested.native===undefined){const result=scriptProcessEnvironment(original,environment);delete result.SEMIO_SCRIPT_CAPABILITIES;return result;}
 if(typeof requested.workspaceRoot!=="string"||resolve(requested.workspaceRoot)!==resolve(root)||!Array.isArray(requested.arguments)||requested.arguments[0]!=="nx"||requested.arguments.some(argument=>typeof argument!=="string"))throw Error("Original Nx caller request required");
 if(validateJsonSchemaSubset(callerSchema,requested).length||JSON.stringify(requested.selection)!==JSON.stringify(["nx",...selected]))throw Error("Original selected Nx native caller required");
 const canonical=canonicalArchitectureEnvironment(root,environment),offline=canonical.CARGO_NET_OFFLINE??"false";
 if(offline!=="true"&&offline!=="false")throw Error("Original Cargo network selection refused");
 const resources=requested.native;
 if(validateJsonSchemaSubset(callerSchema.properties.native,resources).length)throw Error("Original Nx native resources refused");
 const native=resources as {artifactDirectory:string;transport:object;child:object;network:{offline:boolean}};
 if(resolve(root,native.artifactDirectory)!==canonical.CARGO_TARGET_DIR||String(native.network.offline)!==offline)throw Error("Original Nx artifact or network selection changed");
 const capabilities=requested;
 if(validateJsonSchemaSubset(callerSchema,capabilities).length)throw Error("Selected Nx native caller refused");
 const result=scriptProcessEnvironment({...original,capabilities},{...canonical,CARGO_NET_OFFLINE:offline});
 delete result.SEMIO_SCRIPT_CAPABILITIES;
 return result;
}

/** 🏛️ Declares production resources and the original bootstrap's finite child command set before launch. */
export interface NativeNxCallerCapabilitiesV1 {
 readonly workspaceRoot:string;
 readonly arguments:readonly string[];
 readonly selection:readonly string[];
 readonly native:Readonly<{artifactDirectory:string;transport:NativeOwnerCapabilities["transport"];child:NativeOwnerCapabilities["child"];network:NativeOwnerCapabilities["network"]}>;
 readonly launches?:readonly (readonly string[])[];
}

/** 🏗️ Issues genuine configured resources only for a newly owned outer Nx invocation. */
export function createNativeNxCallerCapabilitiesV1(root:string,arguments_:readonly string[],selected:readonly string[],environment:Readonly<Record<string,string|undefined>>,launches:readonly (readonly string[])[]=[selected]):NativeNxCallerCapabilitiesV1{
 if(validateJsonSchemaSubset(nativeSchema.$defs.NativeOwnerExecutionPolicyV1,executionPolicy).length)throw Error("Production native execution policy refused");
 const canonical=canonicalArchitectureEnvironment(root,environment),offline=canonical.CARGO_NET_OFFLINE??"false";
 if(offline!=="true"&&offline!=="false")throw Error("Original Cargo network selection refused");
 const capabilities={workspaceRoot:resolve(root),arguments:[...arguments_],selection:["nx",...selected],native:{artifactDirectory:canonical.CARGO_TARGET_DIR!,transport:executionPolicy.transport,child:executionPolicy.child,network:{offline:offline==="true"}},launches:launches.map(args=>["nx",...args])};
 if(arguments_[0]!=="nx"||validateJsonSchemaSubset(callerSchema,capabilities).length||!capabilities.launches.some(args=>JSON.stringify(args)===JSON.stringify(capabilities.selection)))throw Error("Original bootstrap launch declaration refused");
 return capabilities;
}

/** 🔑️ Selects only an already declared child while retaining original resources, request and deadline. */
export function issueSelectedNxCallerEnvelopeV1(root:string,envelope:ScriptProcessEnvelope,selected:readonly string[]):ScriptProcessEnvelope{
 const original=readScriptProcessEnvelope(envelope,Date.now()),capabilities=original.capabilities as unknown as NativeNxCallerCapabilitiesV1;
 if(capabilities.native===undefined)return original;
 if(validateJsonSchemaSubset(callerSchema,capabilities).length||resolve(capabilities.workspaceRoot)!==resolve(root)||capabilities.arguments[0]!=="nx")throw Error("Original Nx launch authority refused");
 const selection=["nx",...selected];
 if(JSON.stringify(capabilities.selection)===JSON.stringify(selection))return original;
 if(!capabilities.launches?.some(args=>JSON.stringify(args)===JSON.stringify(selection)))throw Error("Undeclared Nx child selection refused");
 return {...original,capabilities:{...original.capabilities,selection}};
}
