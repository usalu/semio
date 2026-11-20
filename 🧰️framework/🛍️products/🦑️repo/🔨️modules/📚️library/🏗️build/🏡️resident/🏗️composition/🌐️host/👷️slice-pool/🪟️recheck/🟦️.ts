import {withNativeRecheckWindowV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📡️roster/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,withIssuedResourceOperationV1,type ScriptInvocation} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🪟️ Restores the Native recheck window lost at the fork for this issued graph load only. */
export function withResidentSliceNativeRecheckV1<T>(invocation:ScriptInvocation,control:ResourceLeaseControl,body:()=>Promise<T>):Promise<T>{
 requireIssuedResourceControlInvocationV1(control,invocation);
 return withIssuedResourceOperationV1(control,()=>withNativeRecheckWindowV1(body));
}
