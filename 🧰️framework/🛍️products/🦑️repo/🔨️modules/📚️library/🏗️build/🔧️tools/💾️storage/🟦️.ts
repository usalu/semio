import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{requireIssuedResourceControlInvocationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🏦️ Admits each engine's finite physical stores before immutable graph reader custody begins; the selected backend owns what that means for it. */
export async function prepareBuildToolStorageV1(root:string,tool:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,snapshotBytes:number):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);if(!Number.isSafeInteger(snapshotBytes)||snapshotBytes<0||snapshotBytes>536870912)throw Error("Bounded exact graph snapshot bytes required");
 const {loadBuildBackendV1}=await import("../../🔌️backends/🟦️.ts");
 await (await loadBuildBackendV1(tool)).prepareStorage({root,invocation,phase:phaseControl,args:[],tool},snapshotBytes);
 checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
}
