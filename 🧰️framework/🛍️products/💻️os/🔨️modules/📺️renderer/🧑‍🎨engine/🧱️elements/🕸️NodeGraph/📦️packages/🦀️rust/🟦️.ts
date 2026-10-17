import{requireNodeGraphRuntimeV1,type NodeGraphRuntimeV1,type NodeGraphSessionV1}from"../../📋️runtime/🟦️.ts";
import{installPageRuntimeAssets}from"@semio-tech/framework/canvas/runtime-assets";
export type{NodeGraphSessionV1}from"../../📋️runtime/🟦️.ts";
let pending:Promise<NodeGraphRuntimeV1>|undefined;
async function loadRuntimeV1(loadNamespace:()=>Promise<unknown>):Promise<NodeGraphRuntimeV1>{
 if(pending)return pending;
 const attempt=(async()=>{const namespace:unknown=await loadNamespace(),runtime=requireNodeGraphRuntimeV1(namespace);await runtime.initialize();await installPageRuntimeAssets(runtime.assets);return runtime;})();pending=attempt;void attempt.catch(()=>{if(pending===attempt)pending=undefined;});return attempt;
}
/** 🌐️ Initializes the managed package namespace the caller's composition root supplies and validates every current session method. */
export async function openNodeGraphSessionV1(loadNamespace:()=>Promise<unknown>):Promise<NodeGraphSessionV1>{return(await loadRuntimeV1(loadNamespace)).create();}
