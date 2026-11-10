import{readBuildGraphAncestorSelectionV1}from"../🟦️.ts";
import{buildInvocationGraphSelectionV1,type BuildGraphSelectionV1}from"../../🕸️graph/🚪️admission/🟦️.ts";
import type { BuildGraphV1 } from "../../🕸️graph/📋️model/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{checkScriptInvocation,advanceScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🕸️ The options one invocation load passes to a Graph reader. */
export type BuildInvocationGraphLoadOptionsV1={selection:BuildGraphSelectionV1;reuse:boolean;graph?:{policy?:"fresh"|"current";compact?:boolean;externalInputs?:readonly string[]}};

/** 📖️ A Graph reader as the invocation load calls it. */
export type BuildInvocationGraphReaderV1=(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildInvocationGraphLoadOptionsV1)=>Promise<BuildGraphV1>;

/** 🔨️ Loads only the selected graph through the exact current phase and Original; a reusing load reads the pinned snapshot through a Graph reader it opens itself (or the one its caller already holds), because a nested child process starts with none. */
export async function loadBuildInvocationGraphV1(root:string,args:readonly string[],original:ScriptInvocation,current:ScriptInvocation,phaseControl:ResourceLeaseControl,reuse:boolean,read?:BuildInvocationGraphReaderV1,graphPolicy?:"fresh"|"current",compact?:boolean,externalInputs?:readonly string[]):Promise<BuildGraphV1>{
 requireIssuedResourceControlInvocationV1(phaseControl,current);checkResourceLeaseControl(phaseControl);checkScriptInvocation(original);if(current.control!==original.control||current.policy!==original.policy)throw Error("Original nested build process control changed");
 const graphOptions={...(externalInputs===undefined?{}:{externalInputs}),...(graphPolicy?{policy:graphPolicy}:{}),...(compact?{compact}:{})},selection=buildInvocationGraphSelectionV1(args),loader:BuildInvocationGraphReaderV1=read??(await import("../../🕸️graph/🟦️.ts")).loadBuildGraphV1,load:BuildInvocationGraphReaderV1=async(base:string,invocation:ScriptInvocation,control:ResourceLeaseControl,options:BuildInvocationGraphLoadOptionsV1)=>options.reuse?(await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts")).withRepositoryCacheStoreV1(base,"graph",invocation,control,(_store,actual)=>loader(base,invocation,actual,options)):loader(base,invocation,control,options),ancestor=reuse?readBuildGraphAncestorSelectionV1(root,original.capabilities):undefined;
 if(ancestor&&JSON.stringify(ancestor)!==JSON.stringify(selection)){
  if((current.capabilities as {native?:unknown}).native!==(original.capabilities as {native?:unknown}).native)throw Error("Original nested build native resources changed");
  await load(root,original,phaseControl,{selection:ancestor,reuse:true,graph:graphOptions});checkResourceLeaseControl(phaseControl);checkScriptInvocation(original);await advanceScriptInvocation(original,"build:nested-selection","running");checkResourceLeaseControl(phaseControl);checkScriptInvocation(original);
  const graph=await load(root,current,phaseControl,{selection,reuse:false,graph:graphOptions});checkResourceLeaseControl(phaseControl);return graph;
 }
 const graph=await load(root,current,phaseControl,{selection,reuse,graph:graphOptions});checkResourceLeaseControl(phaseControl);return graph;
}
