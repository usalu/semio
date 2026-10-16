import{type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../📦️budget/📥️opening/🏪️store/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🛠️ Keeps the complete conservative toolkit store under one actual finite backing. */
export type RepositoryToolingStoreV1=RepositoryCacheStoreV1;
/** 🎟️ Holds the original tools partition through installer and loaded child settlement. */
export function withRepositoryToolingStoreV1<T>(workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(store:RepositoryToolingStoreV1,actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(workspace,"tools",invocation,phaseControl,body);}
