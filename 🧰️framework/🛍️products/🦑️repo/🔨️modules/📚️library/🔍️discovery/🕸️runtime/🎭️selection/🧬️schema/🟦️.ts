import type { FileObservationControlV1, PhysicalFileClaimV1 } from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import type { RuntimeCargoObservationV1 } from "../../🧬️schema/🟦️.ts";
import type { RuntimeDynamicImportOwnerV1, RuntimeGraphEvidenceV1 } from "../../🟦️.ts";
import schema from "./🔣️.json";

/** 🎭️ Relative selected output custody, independent of publication storage or an application. */
export interface SelectedRuntimeRelativeClaimV1 { readonly relativePath:string; readonly sha256:string; readonly byteLength:number }
/** 🏭️ Complete original actor factory inputs and exact same-package outputs. */
export interface SelectedRuntimeActorProducerV1 { readonly schema:"semio.os.closed-browser-actor-producer/v1"; readonly actor:SelectedRuntimeRelativeClaimV1; readonly component:SelectedRuntimeRelativeClaimV1; readonly descriptor:SelectedRuntimeRelativeClaimV1; readonly policyCanonical:string; readonly policySha256:string; readonly runtime:PhysicalFileClaimV1; readonly compiler:PhysicalFileClaimV1; readonly inputs:readonly (PhysicalFileClaimV1 & {readonly logicalPath:string})[] }
/** 📦️ One selected production package and its original native invocation owners. */
export interface SelectedRuntimePackageProducerV1 { readonly pluginId:string; readonly packageId:string; readonly cargoPackage:string; readonly component:SelectedRuntimeRelativeClaimV1; readonly descriptor:SelectedRuntimeRelativeClaimV1; readonly cargoInvocations:readonly PhysicalFileClaimV1[]; readonly browserActor:PhysicalFileClaimV1|null }
/** 🧾️ Typed original publication generation with no storage-specific API. */
export interface SelectedRuntimeGenerationV1 { readonly schema:string; readonly profileId:string; readonly generationId:string; readonly bundle:SelectedRuntimeRelativeClaimV1; readonly packages:readonly SelectedRuntimePackageProducerV1[] }
/** 📍️ Exact live pointer, bundle and generation producer custody. */
export interface SelectedRuntimePublicationRecordV1 { readonly schema:string; readonly dataRoot:string; readonly profileId:string; readonly generationId:string; readonly publicationRevision:string; readonly pointer:PhysicalFileClaimV1; readonly bundle:PhysicalFileClaimV1; readonly producer:PhysicalFileClaimV1 }
/** 📥️ Current publication observation supplied by its owning composition. */
export interface SelectedRuntimePublicationV1 { readonly path:string; readonly record:SelectedRuntimePublicationRecordV1; readonly generationPath:string; readonly generation:SelectedRuntimeGenerationV1 }
/** 🔒️ A closed first-party browser actor with its exact selected component and descriptor. */
export interface SelectedRuntimeClosedActorV1 { readonly kind:"closed-browser-actor"; readonly schema:"semio.os.closed-browser-actor.v1"; readonly codegenPolicy:"semio.os.browser-jco-1.34.0-jspi.v1"; readonly sha256:string; readonly byteLength:number; readonly path:string; readonly sourceComponentSha256:string; readonly sourceDescriptorByteSha256:string; readonly policySha256:string; readonly importInterfaces:readonly string[] }
/** 🔗️ Complete native, factory and closed-graph witness for one real selected actor. */
export interface SelectedRuntimeActorWitnessV1 { readonly pluginId:string; readonly packageId:string; readonly actor:SelectedRuntimeClosedActorV1; readonly producer:SelectedRuntimeActorProducerV1; readonly componentInvocation:RuntimeCargoObservationV1; readonly descriptorInvocation:RuntimeCargoObservationV1; readonly owner:RuntimeDynamicImportOwnerV1; readonly closed:RuntimeGraphEvidenceV1 }
/** 🧬️ Serializable selected production evidence admitted by the portable contract. */
export interface SelectedRuntimeActorEvidenceV1 { readonly version:1; readonly dataRoot:string; readonly requested:string; readonly profile:string; readonly published:SelectedRuntimePublicationV1; readonly dynamicImports:Readonly<Record<string,readonly RuntimeDynamicImportOwnerV1[]>>; readonly observations:readonly RuntimeCargoObservationV1[]; readonly witnesses:readonly SelectedRuntimeActorWitnessV1[] }
/** 🔄️ Evidence retains its actual owning recheck operation under explicit authority. */
export interface SelectedRuntimeActorsV1 extends SelectedRuntimeActorEvidenceV1 { recheck(control:FileObservationControlV1):Promise<void> }
/** 🚪️ Required observation and explicit acquisition operations supplied by application composition. */
export interface SelectedRuntimeActorPublicationPortV1 { readonly diagnosticOwner:string; observeCurrent(control:FileObservationControlV1):Promise<SelectedRuntimeActorsV1>; acquireCurrent(control:FileObservationControlV1):Promise<void> }
const refuse:(reason:string)=>never=(reason)=>{throw Error("Selected runtime actor evidence: "+reason);};
function check(control:FileObservationControlV1):void{
 if(!control||!Number.isSafeInteger(control.maxBytes)||control.maxBytes<0||control.maxBytes>134217728||!Number.isSafeInteger(control.maxWork)||control.maxWork<1||control.maxWork>65536||!Number.isSafeInteger(control.chunkBytes)||control.chunkBytes<1||control.chunkBytes>1048576||typeof control.cancelled!=="function"||typeof control.remainingMs!=="function"||typeof control.onProgress!=="function")refuse("invalid observation authority");
 if(control.cancelled())refuse("cancelled");const remaining=control.remainingMs();if(!Number.isFinite(remaining)||remaining<=0)refuse("deadline");
}

/** 🛂️ Admits complete JSON evidence with finite cumulative work, bytes, depth and cancellable yields.
 * @see ./🔣️.json
 * @see ../🧫️fixtures/🔣️.json
 */
export async function parseSelectedRuntimeActorEvidenceV1(input:unknown,control:FileObservationControlV1):Promise<SelectedRuntimeActorEvidenceV1>{
 check(control);let bytes=0,work=0;const seen=new Set<object>(),encoder=new TextEncoder(),stack:{value:unknown;depth:number;leave?:object}[]=[{value:input,depth:0}];
 const progress=()=>{control.onProgress({phase:"read",bytes,totalBytes:bytes,work});check(control);};
 const text=async(value:string)=>{for(let offset=0;offset<value.length;offset+=Math.max(1,Math.floor(control.chunkBytes/4))){check(control);bytes+=encoder.encode(value.slice(offset,offset+Math.max(1,Math.floor(control.chunkBytes/4)))).byteLength;if(bytes>control.maxBytes)refuse("byte-capacity");if(++work>control.maxWork)refuse("work-capacity");if(work%128===0){progress();await new Promise<void>(done=>setTimeout(done,0));check(control);}}};
 while(stack.length){check(control);if(++work>control.maxWork)refuse("work-capacity");const {value,depth,leave}=stack.pop()!;if(leave){seen.delete(leave);continue;}if(depth>64)refuse("depth-capacity");bytes+=2;if(bytes>control.maxBytes)refuse("byte-capacity");
  if(typeof value==="string")await text(value);else if(typeof value==="number"){if(!Number.isFinite(value))refuse("non-JSON number");bytes+=String(value).length;}else if(typeof value==="boolean"||value===null)bytes+=5;
  else if(typeof value==="object"){
   if(seen.has(value))refuse("cyclic JSON identity");seen.add(value);stack.push({value:null,depth,leave:value});if(!Array.isArray(value)&&Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null)refuse("non-JSON prototype");
   const keys=Reflect.ownKeys(value);if(keys.length>control.maxWork-work)refuse("work-capacity");for(const key of keys){if(Array.isArray(value)&&key==="length")continue;if(typeof key!=="string")throw Error("Selected runtime actor evidence: non-JSON key");const descriptor=Object.getOwnPropertyDescriptor(value,key)!;if(!("value" in descriptor))refuse("accessor");if(!descriptor.enumerable)refuse("non-enumerable JSON member");if(Array.isArray(value)&&(!/^(0|[1-9][0-9]*)$/u.test(key)||Number(key)>=value.length))refuse("non-JSON array member");await text(key);stack.push({value:descriptor.value,depth:depth+1});}if(Array.isArray(value)&&keys.length!==value.length+1)refuse("sparse JSON array");
  }else refuse("non-JSON value");
  if(bytes>control.maxBytes)refuse("byte-capacity");if(work%128===0){progress();await new Promise<void>(done=>setTimeout(done,0));}
 }
 progress();const errors=validateJsonSchemaSubset(schema,input);check(control);if(errors.length)refuse(errors[0]!);control.onProgress({phase:"complete",bytes,totalBytes:bytes,work});check(control);return input as SelectedRuntimeActorEvidenceV1;
}
function admitPort(port:SelectedRuntimeActorPublicationPortV1,control:FileObservationControlV1):void{check(control);if(!port||typeof port.diagnosticOwner!=="string"||!port.diagnosticOwner||port.diagnosticOwner.length>4096||typeof port.observeCurrent!=="function"||typeof port.acquireCurrent!=="function")refuse("required publication port");}
/** 🔍️ Validates the owning current observation without acquiring production evidence. */
export async function observeSelectedRuntimeActorsV1(port:SelectedRuntimeActorPublicationPortV1,control:FileObservationControlV1):Promise<SelectedRuntimeActorsV1>{
 admitPort(port,control);const current=await port.observeCurrent(control);check(control);if(!current||typeof current.recheck!=="function")refuse("required owning recheck");const {recheck,...input}=current,evidence=await parseSelectedRuntimeActorEvidenceV1(input,control);return{...evidence,recheck:async authority=>{check(authority);await recheck.call(current,authority);check(authority);}};
}
/** 🏗️ Executes only the explicitly supplied acquisition operation under the caller's authority. */
export async function acquireSelectedRuntimeActorsV1(port:SelectedRuntimeActorPublicationPortV1,control:FileObservationControlV1):Promise<void>{admitPort(port,control);await port.acquireCurrent(control);check(control);}

/** 🗺️ Required finite relative package roots selected by the owning workspace composition. */
export type SelectedRuntimeGraphPackageRootsV1=readonly string[];
/** 🛂️ Admits explicit coverage without an application inventory or a fallback root. */
export function parseSelectedRuntimeGraphPackageRootsV1(input:unknown):SelectedRuntimeGraphPackageRootsV1{
 if(!Array.isArray(input)||Object.getPrototypeOf(input)!==Array.prototype||input.length<1||input.length>128)refuse("required explicit graph package roots");
 for(const key of Reflect.ownKeys(input)){if(key==="length")continue;const descriptor=Object.getOwnPropertyDescriptor(input,key)!;if(typeof key!=="string"||! /^(0|[1-9][0-9]*)$/u.test(key)||!("value" in descriptor)||typeof descriptor.value!=="string")refuse("graph package root member");}
 const errors=validateJsonSchemaSubset(schema.$defs.SelectedRuntimeGraphPackageRootsV1,input,schema);if(errors.length)refuse(errors[0]!);return input as SelectedRuntimeGraphPackageRootsV1;
}
