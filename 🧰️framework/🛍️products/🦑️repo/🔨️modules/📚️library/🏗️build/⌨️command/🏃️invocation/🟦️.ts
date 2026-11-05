import {relative} from "node:path";
import {repositoryCommandKernelV1} from "../🧭️selection/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🏃️ Runs the selected owned command kernel under its Original phase without importing root CLI dispatch. */
export async function runOwnedBuildInvocationV1(root:string,typed:readonly string[],parent:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {
 const{normalizeBuildOptionsV1}=await import("../../⚙️operations/🔤️options/🟦️.ts"),args=normalizeBuildOptionsV1(typed),{withBuildCapacityV1}=await import("../../🌡️host-capacity/🟦️.ts");
 const{withImplementationPinV1}=await import("../../📥️inputs/🔏️revision/🟨️.mjs");
 await withBuildCapacityV1(args,process.env,granted=>withImplementationPinV1(()=>{const kernel=repositoryCommandKernelV1(granted.args,process.env);return import("../../🏛️sealed/📥️execution/🟦️.ts").then(({executeImplementationV1})=>executeImplementationV1(root,relative(root,kernel.entry).split("\\").join("/"),granted.args,parent,phaseControl));}),{control:phaseControl});
}
