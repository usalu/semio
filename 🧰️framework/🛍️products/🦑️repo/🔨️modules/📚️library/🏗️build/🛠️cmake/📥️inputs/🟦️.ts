/** 📥️ Inputs: module exporting `CmakeInputsV1`, `withCmakeInputsV1`, `cmakeInputBytesV1`, `cmakeInputFactsV1` and more. */
import{withArtifactSourceSetV1,artifactSourceEntriesV1,readArtifactSourceChunkV1,recheckArtifactSourceSetV1,type ArtifactSourceSetV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/📥️sources/🟦️.ts";
import{requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withRequestedIssuedResourceControlV1,withIssuedResourceOperationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import schema from"../🧬️schema/🔣️.json" with { type: "json" };
declare const inputBrand:unique symbol;
/** 📚️ Identifies live pinned CMake inputs without lending descriptor or Native authority. */
export type CmakeInputsV1=Readonly<{readonly[inputBrand]:true}>;
type Scope={sources:ArtifactSourceSetV1;control:ResourceLeaseControl;live:boolean;bytes:Map<string,Uint8Array>};
const scopes=new WeakMap<CmakeInputsV1,Scope>(),policy=schema.definitions.Policy.const;
function scopeFor(token:CmakeInputsV1){const scope=scopes.get(token);if(!scope?.live)throw Error("Live retained CMake inputs required");requireIssuedResourceControlV1(scope.control);return scope;}
/** 🧷️ Pins actual Source and executable files through accepted child drainage, capturing only finite Source bytes. */
export function withCmakeInputsV1<T>(files:ReadonlyMap<string,string>,control:ResourceLeaseControl,body:(inputs:CmakeInputsV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{requireIssuedResourceControlV1(control);return withIssuedResourceOperationV1(control,()=>withArtifactSourceSetV1(files,{maximumFiles:policy.maximumInputFiles,maximumBytes:policy.maximumInputBytes,control,signal:control.signal},async sources=>{let retained:Scope|undefined;try{return await withRequestedIssuedResourceControlV1(control,0,async actual=>{const token=Object.freeze({})as CmakeInputsV1,bytes=new Map<string,Uint8Array>(),scope={sources,control:actual,live:true,bytes};scopes.set(token,scope);retained=scope;let total=0;for(const row of artifactSourceEntriesV1(sources)){if(!row.name.startsWith("source/"))continue;total+=row.byteLength;if(total>policy.maximumSourceBytes)throw Error("Finite CMake Source image required");const data=new Uint8Array(row.byteLength);for(let offset=0;offset<data.length;offset+=65536)data.set(await readArtifactSourceChunkV1(sources,row.name,offset,Math.min(65536,data.length-offset)),offset);bytes.set(row.name,data);}return body(token,actual);});}finally{if(retained)retained.live=false;}}));}
/** 📖️ Copies captured Source bytes rather than rereading mutable compiler paths. */
export function cmakeInputBytesV1(token:CmakeInputsV1,name:string):Uint8Array{const data=scopeFor(token).bytes.get(name);if(!data)throw Error("Captured CMake Source name required");return Uint8Array.from(data);}
/** 🔎️ Reads immutable exact retained input facts without exporting handles. */
export function cmakeInputFactsV1(token:CmakeInputsV1){return artifactSourceEntriesV1(scopeFor(token).sources);}
/** 🔏️ Rechecks pinned executable and Source extents before completed command consumption. */
export function recheckCmakeInputsV1(token:CmakeInputsV1):Promise<void>{return recheckArtifactSourceSetV1(scopeFor(token).sources);}

/** 🧬️ Requires every command to remain beneath its exact captured input control. */
export function requireCmakeInputsControlV1(token:CmakeInputsV1,control:ResourceLeaseControl):void{requireIssuedResourceControlRefinementV1(scopeFor(token).control,control);}
