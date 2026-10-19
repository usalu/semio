import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {CompilerAdmissionControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts";
import { type CargoDependencyNetworkV1 } from "@semio-tech/repo-lib/cargo";
import {buildComponentClosureV1} from "../🧩️driver-stages/🟦️.ts";
/** 🔁️ Builds one iterated component through the same actual managed driver and selected-mode publication. */
export async function iterateComponentV1(workspace:string,invocation:ScriptInvocation,pluginId:string,control:CompilerAdmissionControl,network:CargoDependencyNetworkV1):Promise<void>{
 const authority=readNativeCallerAuthorityV1(invocation.capabilities);if(authority.repositoryRoot!==workspace||authority.original.buildMode!=="prototype")throw Error("Iterated component requires its actual prototype Original");
 const outcome=await buildComponentClosureV1(workspace,new Set([pluginId]),{facet:"browser",invocation,control,network});console.log(`Iterated ${pluginId}; report ${outcome.report}`);
}
