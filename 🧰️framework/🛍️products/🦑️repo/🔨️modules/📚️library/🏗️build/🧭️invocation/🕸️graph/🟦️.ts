import{readBuildGraphAncestorSelectionV1}from"../🟦️.ts";
import{buildInvocationGraphSelectionV1,type BuildGraphSelectionV1}from"../../🕸️graph/🚪️admission/🟦️.ts";
import type{BuildGraphV1}from"../../🕸️graph/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{checkScriptInvocation,advanceScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🕸️ Loads only the selected graph through the exact current phase and Original. */
export async function loadBuildInvocationGraphV1(root:string,args:readonly string[],original:ScriptInvocation,current:ScriptInvocation,phaseControl:ResourceLeaseControl,reuse:boolean,read?:(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse:boolean;graph?:{policy?:"fresh"|"current";compact?:boolean}})=>Promise<BuildGraphV1>,graphPolicy?:"fresh"|"current",compact?:boolean):Promise<BuildGraphV1>{
 requireIssuedResourceControlInvocationV1(phaseControl,current);checkResourceLeaseControl(phaseControl);checkScriptInvocation(original);if(current.control!==original.control||current.policy!==original.policy)throw Error("Original nested build process control changed");
 const selection=buildInvocationGraphSelectionV1(args),load=read??(await import("../../🕸️graph/🟦️.ts")).loadBuildGraphV1,ancestor=reuse?readBuildGraphAncestorSelectionV1(root,original.capabilities):undefined;
 if(ancestor&&JSON.stringify(ancestor)!==JSON.stringify(selection)){
  if((current.capabilities as {native?:unknown}).native!==(original.capabilities as {native?:unknown}).native)throw Error("Original nested build native resources changed");
  await load(root,original,phaseControl,{selection:ancestor,reuse:true});checkResourceLeaseControl(phaseControl);checkScriptInvocation(original);await advanceScriptInvocation(original,"build:nested-selection","running");checkResourceLeaseControl(phaseControl);checkScriptInvocation(original);
  const graph=await load(root,current,phaseControl,{selection,reuse:false,...(graphPolicy||compact?{graph:{...(graphPolicy?{policy:graphPolicy}:{}),...(compact?{compact}:{})}}:{})});checkResourceLeaseControl(phaseControl);return graph;
 }
 const graph=await load(root,current,phaseControl,{selection,reuse,...(graphPolicy||compact?{graph:{...(graphPolicy?{policy:graphPolicy}:{}),...(compact?{compact}:{})}}:{})});checkResourceLeaseControl(phaseControl);return graph;
}
