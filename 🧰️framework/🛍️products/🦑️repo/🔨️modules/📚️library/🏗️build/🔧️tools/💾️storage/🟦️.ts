import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🏦️ Admits each engine's finite physical stores before immutable graph reader custody begins. */
export async function prepareBuildToolStorageV1(root:string,tool:"nx"|"bazel",invocation:ScriptInvocation):Promise<void>{
 checkScriptInvocation(invocation);if(tool==="nx"){const {prepareNxGraphCacheStorageV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts");await prepareNxGraphCacheStorageV1(root,invocation);}else if(tool==="bazel"){const {withBazelStorageV1}=await import("../🏗️bazel/💾️storage/🟦️.ts");await withBazelStorageV1(root,invocation,async storage=>{await storage.tools.recheck();await storage.dependencies.recheck();await storage.products.recheck();});}else throw Error("Declared build tool storage owner required");checkScriptInvocation(invocation);
}
