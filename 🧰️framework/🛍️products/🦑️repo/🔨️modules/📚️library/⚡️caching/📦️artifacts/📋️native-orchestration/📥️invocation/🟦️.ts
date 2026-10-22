import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import {cargoExecutionSchemaV1,type CargoExecutionBackendV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import serviceSchema from "../../../🕸️graph/🏁️startup/🧬️lifetime/🧬️schema/🔣️.json" with { type: "json" };
import callerSchema from "../../../🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json" with { type: "json" };
import buildCallerSchema from "../../../../🏗️build/🧬️schema/📥️caller.json" with { type: "json" };
import {resolve} from "node:path";
import {checkScriptInvocation,advanceScriptInvocation,scriptResourceProcessEnvironmentV1,requireIssuedResourceControlInvocationV1,createRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {OwnedCommandOptions} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 📥️ Declares the actual native caller's selected command and finite child authority. */
export interface NativeOwnerCapabilities {
  readonly version: 1;
  readonly buildMode:"prototype"|"release";
  readonly repositoryRoot: string;
  readonly artifactDirectory: string;
  readonly command: Readonly<{ kind: "native-owner-command"; manifest: string; workingDirectory: string; program: string; arguments: readonly string[] }>;
  readonly transport: Readonly<{ maximumBytes: number; maximumLines: number }>;
  readonly child: Readonly<{ maximumElapsedMilliseconds: number }>;
  readonly network: Readonly<{ offline: boolean }>;
  readonly cargoExecutionBackend?:CargoExecutionBackendV1;
}

/** 🔐️ Refuses missing caller grants before native input or child acquisition. */
export function readNativeOwnerCapabilities(value: unknown): NativeOwnerCapabilities {
  if (validateJsonSchemaSubset(schema, value,schema,[cargoExecutionSchemaV1]).length) throw Error("Original native caller capabilities required");
  return value as NativeOwnerCapabilities;
}

/** 🧭️ Receives the existing closed build orchestration caller without issuing replacement authority. */
export interface NativeOrchestrationCallerV1 {
 readonly tool?:"nx"|"bazel";
 readonly workspaceRoot:string;
 readonly buildMode:"prototype"|"release";
 readonly arguments:readonly string[];
 readonly selection:readonly string[];
 readonly launches?:readonly (readonly string[])[];
 readonly native:Readonly<{artifactDirectory:string;transport:NativeOwnerCapabilities["transport"];child:NativeOwnerCapabilities["child"];network:NativeOwnerCapabilities["network"]}>;
}
/** 🔐️ Projects either current native role while retaining its exact declared capabilities. */
export function readNativeCallerAuthorityV1(value:unknown):Readonly<{role:"native-owner-command"|"build-orchestration";repositoryRoot:string;artifactDirectory:string;transport:NativeOwnerCapabilities["transport"];child:NativeOwnerCapabilities["child"];network:NativeOwnerCapabilities["network"];original:NativeOwnerCapabilities|NativeOrchestrationCallerV1}>{
 if(value&&typeof value==="object"&&"native" in value){
  const refusals=validateJsonSchemaSubset(value&&typeof value==="object"&&"tool" in value?buildCallerSchema:callerSchema,value);if(refusals.length)throw Error("Original native caller capabilities required: "+refusals.slice(0,4).join("; "));
  const caller=value as NativeOrchestrationCallerV1;return{role:"build-orchestration",repositoryRoot:caller.workspaceRoot,...caller.native,original:caller};
 }
 const caller=readNativeOwnerCapabilities(value);return{role:"native-owner-command",repositoryRoot:caller.repositoryRoot,artifactDirectory:caller.artifactDirectory,transport:caller.transport,child:caller.child,network:caller.network,original:caller};
}
/** 🏠️ Reads the repository root of any admitted caller principal: a native owner command, the build orchestration, or the finite Nx daemon service that names its repository itself and holds no native grants. */
export function callerRepositoryRootV1(value:unknown):string{
 if(value&&typeof value==="object"&&(value as{kind?:unknown}).kind==="nx-daemon-service"){
  const refusals=validateJsonSchemaSubset(serviceSchema.$defs.ServiceCapabilities,value);if(refusals.length)throw Error("Original Nx daemon service capabilities required: "+refusals.slice(0,4).join("; "));
  return (value as{repositoryRoot:string}).repositoryRoot;
 }
 return readNativeCallerAuthorityV1(value).repositoryRoot;
}
/** 🎛️ Carries the sole original native operation through bounded children and awaited queued custody. */
export interface NativeCallerPhaseV1 {
 readonly authority:ReturnType<typeof readNativeCallerAuthorityV1>;
 readonly leaseControl:ResourceLeaseControl;
 budget():number;
 advance(command:string):Promise<void>;
 options(environment:Readonly<Record<string,string|undefined>>):OwnedCommandOptions;
}
/** ⏳️ Intersects the explicitly declared finite native child grant with the unchanged original caller. */
export function nativeCallerPhaseV1(invocation:ScriptInvocation,root:string,parent:ResourceLeaseControl):NativeCallerPhaseV1{
 requireIssuedResourceControlInvocationV1(parent,invocation);checkScriptInvocation(invocation);const authority=readNativeCallerAuthorityV1(invocation.capabilities);
 if(resolve(authority.repositoryRoot)!==resolve(root))throw Error("Original native caller repository root refused");
 const leaseControl=createRequestedIssuedResourceControlV1(parent,authority.child.maximumElapsedMilliseconds),budget=():number=>{requireIssuedResourceControlInvocationV1(leaseControl,invocation);const remaining=leaseControl.remainingMilliseconds();if(remaining===null||remaining<=0)throw Error("Original finite native child deadline exhausted");return Math.ceil(remaining);};
 const advance=async(command:string):Promise<void>=>{budget();await advanceScriptInvocation(invocation,command,"running");budget();};
 return Object.freeze({authority,leaseControl,budget,advance,options:(environment:Readonly<Record<string,string|undefined>>)=>{budget();return{signal:leaseControl.signal,output:authority.transport,env:scriptResourceProcessEnvironmentV1(leaseControl,{...environment,CARGO_NET_OFFLINE:String(authority.network.offline),CARGO_TARGET_DIR:authority.artifactDirectory}),onProgress:advance};}});
}
