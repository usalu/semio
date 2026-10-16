import{requireNodeGraphRuntimeV1,type NodeGraphRuntimeV1,type NodeGraphSessionV1}from"../../📋️runtime/🟦️.ts";
import{installPageRuntimeAssets}from"@semio-tech/framework/canvas/runtime-assets";
export type{NodeGraphSessionV1}from"../../📋️runtime/🟦️.ts";
let pending:Promise<NodeGraphRuntimeV1>|undefined;
async function loadRuntimeV1():Promise<NodeGraphRuntimeV1>{
 if(pending)return pending;
 const attempt=(async()=>{const namespace:unknown=await import("@semio-tech/framework-os-node-graph-rs/framework_os_node_graph.js"),runtime=requireNodeGraphRuntimeV1(namespace);await runtime.initialize();await installPageRuntimeAssets(runtime.assets);return runtime;})();pending=attempt;void attempt.catch(()=>{if(pending===attempt)pending=undefined;});return attempt;
}
/** 🌐️ Initializes the actual statically mapped managed package and validates every current session method. */
export async function openNodeGraphSessionV1():Promise<NodeGraphSessionV1>{return(await loadRuntimeV1()).create();}
