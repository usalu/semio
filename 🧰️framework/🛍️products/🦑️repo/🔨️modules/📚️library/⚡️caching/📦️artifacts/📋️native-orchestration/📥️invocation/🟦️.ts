import schema from "./🧬️schema/🔣️.json";
import {cargoExecutionSchemaV1,type CargoExecutionBackendV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import callerSchema from "../../../../🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json";
import buildCallerSchema from "../../../../🏗️build/🧬️schema/📥️caller.json";
import {resolve} from "node:path";
import {checkScriptInvocation,advanceScriptInvocation,scriptInvocationEnvironment,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
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

/** 🧭️ Receives the existing closed Nx orchestration caller without issuing replacement authority. */
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
export function readNativeCallerAuthorityV1(value:unknown):Readonly<{role:"native-owner-command"|"nx-orchestration";repositoryRoot:string;artifactDirectory:string;transport:NativeOwnerCapabilities["transport"];child:NativeOwnerCapabilities["child"];network:NativeOwnerCapabilities["network"];original:NativeOwnerCapabilities|NativeOrchestrationCallerV1}>{
 if(value&&typeof value==="object"&&"native" in value){
  if(validateJsonSchemaSubset(value&&typeof value==="object"&&"tool" in value?buildCallerSchema:callerSchema,value).length)throw Error("Original native caller capabilities required");
  const caller=value as NativeOrchestrationCallerV1;return{role:"nx-orchestration",repositoryRoot:caller.workspaceRoot,...caller.native,original:caller};
 }
 const caller=readNativeOwnerCapabilities(value);return{role:"native-owner-command",repositoryRoot:caller.repositoryRoot,artifactDirectory:caller.artifactDirectory,transport:caller.transport,child:caller.child,network:caller.network,original:caller};
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
export function nativeCallerPhaseV1(invocation:ScriptInvocation,root:string):NativeCallerPhaseV1{
 checkScriptInvocation(invocation);const authority=readNativeCallerAuthorityV1(invocation.capabilities),started=performance.now();
 if(resolve(authority.repositoryRoot)!==resolve(root))throw Error("Original native caller repository root refused");
 const budget=():number=>{checkScriptInvocation(invocation);const parent=invocation.control.remainingMilliseconds(),child=authority.child.maximumElapsedMilliseconds-(performance.now()-started),remaining=parent===null?child:Math.min(parent,child);if(!Number.isFinite(remaining)||remaining<=0)throw Error("Original native child deadline exhausted");return remaining;};
 const advance=async(command:string):Promise<void>=>{budget();await advanceScriptInvocation(invocation,command,"running");budget();};
 const leaseControl:ResourceLeaseControl={signal:invocation.control.signal,remainingMilliseconds:budget,async publish(progress){budget();await invocation.control.publish({owner:invocation.policy.owner,command:"native:lease:"+progress.resource,stage:"running"});budget();},async yieldContinuation(){budget();await invocation.control.yieldContinuation();budget();}};
 return{authority,leaseControl,budget,advance,options:environment=>({signal:invocation.control.signal,output:authority.transport,env:scriptInvocationEnvironment(invocation,{...environment,CARGO_NET_OFFLINE:String(authority.network.offline),CARGO_TARGET_DIR:authority.artifactDirectory}),onProgress:advance})};
}
