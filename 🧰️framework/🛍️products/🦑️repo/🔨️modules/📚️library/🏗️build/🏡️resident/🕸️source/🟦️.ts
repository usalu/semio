import {loadBuildInvocationGraphV1} from "../../🧭️invocation/🕸️graph/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🕸️ Reads the actual current generation under the resident's original invocation and phase. */
export function loadResidentGraphV1(root:string,original:ScriptInvocation,control:ResourceLeaseControl,load:typeof loadBuildInvocationGraphV1=loadBuildInvocationGraphV1){return load(root,["graph","--graph-policy=current"],original,original,control,false,undefined,"current");}
