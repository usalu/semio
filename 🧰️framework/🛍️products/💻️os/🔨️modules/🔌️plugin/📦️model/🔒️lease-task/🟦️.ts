import {AsyncLocalStorage} from "node:async_hooks";
import {withLeaseTaskV1} from "../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";

/** 🪢️ Captures the product lease parent before the compiler writer is acquired. */
export function captureComponentProductLeaseTaskV1(): <T>(body:()=>Promise<T>)=>Promise<T>{
 const fromCompilerStore=AsyncLocalStorage.snapshot();
 return body=>fromCompilerStore(()=>withLeaseTaskV1(body));
}
