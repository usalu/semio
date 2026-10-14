import {isAbsolute} from "node:path";
import schema from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {runOwnedCommand} from "../../../../🎛️owned-execution/🟦️.ts";
import {captureOwnedProcess} from "../../../../📥️capture/🟦️.ts";
import {scriptInvocationEnvironment} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {checkScriptInvocation,advanceScriptInvocation} from "../../../../🧭️routing/📥️invocation/🟦️.ts";
import {exactExecutableFingerprint,originalNativeLibraryDirectory,type ExactCargoLawPort} from "../🟦️.ts";

/** 📋️ Declares the caller-owned preparation executable and finite transport independently of Cargo discovery. */
export type ExactCargoLawPreparationV1=Readonly<{version:1;command:string;args:readonly string[];cwd:string;environment:Readonly<Record<string,string>>;maximumOutputLines:number}>;
/** 🔐️ Admits an explicit preparation descriptor before any compiler acquisition. */
export function readExactCargoLawPreparationV1(value:unknown):ExactCargoLawPreparationV1{if(validateJsonSchemaSubset(schema,value).length)throw Error("Complete exact Cargo preparation descriptor required");const descriptor=value as ExactCargoLawPreparationV1;if(!isAbsolute(descriptor.cwd)||[descriptor.command,...descriptor.args].some(value=>value.includes("\0")))throw Error("Exact Cargo preparation paths or arguments refused");return descriptor;}
/** 🔌️ Preserves the original build grant across caller preparation and compiler capture. */
export function exactCargoLawPreparationPortV1(input:ExactCargoLawPreparationV1,host:Readonly<{execute:typeof runOwnedCommand;capture:ExactCargoLawPort["probe"];fingerprint:ExactCargoLawPort["fingerprint"];nativeLibraryDirectory?:ExactCargoLawPort["nativeLibraryDirectory"]}>={execute:runOwnedCommand,capture:captureOwnedProcess,fingerprint:exactExecutableFingerprint,nativeLibraryDirectory:originalNativeLibraryDirectory}):ExactCargoLawPort{
 const descriptor=readExactCargoLawPreparationV1(input);
 return {fingerprint:host.fingerprint,nativeLibraryDirectory:host.nativeLibraryDirectory,async probe(command,args,capture){
  if(command!=="cargo")return await host.capture(command,args,capture);
  const original=capture.invocation;if(!original)throw Error("Original exact Cargo preparation invocation required");const started=performance.now();
  const budget=()=>{checkScriptInvocation(original);const remaining=original.control.remainingMilliseconds(),phase=capture.budgetMs===0?null:capture.budgetMs-(performance.now()-started),left=remaining===null?phase:phase===null?remaining:Math.min(remaining,phase);if(left!==null&&left<=0)throw Error("Original exact Cargo preparation deadline exhausted");return left??0;};
  budget();await advanceScriptInvocation(original,"exact-cargo:preparation","running");
  await host.execute(descriptor.command,[...descriptor.args,...args],descriptor.cwd,"exact-cargo:preparation",budget(),{env:scriptInvocationEnvironment(original,{...capture.env,...descriptor.environment}),signal:original.control.signal,stdout:"stderr",output:{maximumBytes:capture.maxOutputBytes,maximumLines:descriptor.maximumOutputLines},onProgress:async()=>{await advanceScriptInvocation(original,"exact-cargo:preparation","running");}});
  return await host.capture(command,args,{...capture,budgetMs:budget()});
 }};
}
/** 📥️ Receives preparation from the calling launcher without discovering any product owner. */
export function configuredExactCargoLawPreparationPortV1(environment:Readonly<Record<string,string|undefined>>):ExactCargoLawPort{const serialized=environment.SEMIO_EXACT_CARGO_PREPARATION;if(!serialized)throw Error("Explicit exact Cargo preparation descriptor required");return exactCargoLawPreparationPortV1(readExactCargoLawPreparationV1(JSON.parse(serialized)));}
