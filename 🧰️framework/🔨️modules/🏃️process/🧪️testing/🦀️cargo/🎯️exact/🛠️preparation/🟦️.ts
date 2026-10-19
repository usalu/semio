import{projectExactCargoPreparationV1,type ExactCargoLawPreparationV1}from"./📋️request/🟦️.ts";
import {isAbsolute} from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {runOwnedCommand} from "../../../../🎛️owned-execution/🟦️.ts";
import {captureOwnedProcess} from "../../../../📥️capture/🟦️.ts";
import{requireIssuedResourceControlInvocationV1,withIssuedResourceOperationV1,scriptResourceProcessEnvironmentV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{compilerAdmissionBudgetV1}from"../../../../🏗️build/🎟️capacity/🟦️.ts";
import {advanceScriptInvocation} from "../../../../🧭️routing/📥️invocation/🟦️.ts";
import {exactExecutableFingerprint,originalNativeLibraryDirectory,type ExactCargoLawPort} from "../🟦️.ts";

/** 📋️ Declares the caller-owned preparation executable and finite transport independently of Cargo discovery. */
export type {ExactCargoLawPreparationV1};
/** 🔐️ Admits an explicit preparation descriptor before any compiler acquisition. */
export function readExactCargoLawPreparationV1(value:unknown):ExactCargoLawPreparationV1{if(validateJsonSchemaSubset(schema,value).length)throw Error("Complete exact Cargo preparation descriptor required");const descriptor=value as ExactCargoLawPreparationV1;if(!isAbsolute(descriptor.cwd)||[descriptor.command,...descriptor.args].some(value=>value.includes("\0")))throw Error("Exact Cargo preparation paths or arguments refused");return descriptor;}
/** 🔌️ Preserves the original build grant across caller preparation and compiler capture. */
export function exactCargoLawPreparationPortV1(input:ExactCargoLawPreparationV1,host:Readonly<{execute:typeof runOwnedCommand;capture:ExactCargoLawPort["probe"];fingerprint:ExactCargoLawPort["fingerprint"];nativeLibraryDirectory?:ExactCargoLawPort["nativeLibraryDirectory"]}>={execute:runOwnedCommand,capture:captureOwnedProcess,fingerprint:exactExecutableFingerprint,nativeLibraryDirectory:originalNativeLibraryDirectory}):ExactCargoLawPort{
 const captured=readExactCargoLawPreparationV1(input),descriptor=Object.freeze({...captured,args:Object.freeze([...captured.args]),environment:Object.freeze({...captured.environment})}),execute=host.execute;
 return {fingerprint:host.fingerprint,nativeLibraryDirectory:host.nativeLibraryDirectory,probe:host.capture,async prepare(command,args,capture){
  if(command!=="cargo")return;
  const original=capture.invocation,control=capture.control;if(!original)throw Error("Original exact Cargo preparation invocation required");requireIssuedResourceControlInvocationV1(control,original);compilerAdmissionBudgetV1(control);
  const request=projectExactCargoPreparationV1(descriptor,args,capture.env),maximumOutputBytes=capture.maxOutputBytes;
  return withIssuedResourceOperationV1(control,async()=>{
   compilerAdmissionBudgetV1(control);await advanceScriptInvocation(original,"exact-cargo:preparation","running");compilerAdmissionBudgetV1(control);
   await execute(request.command,[...request.args],request.cwd,"exact-cargo:preparation",compilerAdmissionBudgetV1(control),{env:scriptResourceProcessEnvironmentV1(control,request.environment),signal:control.signal,stdout:"stderr",output:{maximumBytes:maximumOutputBytes,maximumLines:request.maximumOutputLines},onProgress:async()=>{compilerAdmissionBudgetV1(control);await advanceScriptInvocation(original,"exact-cargo:preparation","running");compilerAdmissionBudgetV1(control);}});
   compilerAdmissionBudgetV1(control);
  });
 }};
}
/** 📥️ Receives preparation from the calling launcher without discovering any product owner. */
export function configuredExactCargoLawPreparationPortV1(environment:Readonly<Record<string,string|undefined>>):ExactCargoLawPort{const serialized=environment.SEMIO_EXACT_CARGO_PREPARATION;if(!serialized)throw Error("Explicit exact Cargo preparation descriptor required");return exactCargoLawPreparationPortV1(readExactCargoLawPreparationV1(JSON.parse(serialized)));}
