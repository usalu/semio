import type {ValueError} from "@semio-tech/value-ts/refusal";
import type {NativeAllocationRequest} from "../../🎟️control/🧬️contract/🛂️allocation/🟦️.ts";
export type {NativeAllocationRequest} from "../../🎟️control/🧬️contract/🛂️allocation/🟦️.ts";
/** 📦️ Preserves the original independent encode allocation verdicts. */
export interface NativeEncodeAllocationOutcome{readonly type:"encode";readonly request:NativeAllocationRequest|null;readonly admitted:boolean;readonly primary:ValueError|null;readonly forwardedQuote:ValueError|null;readonly scopedQuote:ValueError|null;readonly extentRefusal:ValueError|null;readonly forwardedRefusal:ValueError|null;readonly scopedRefusal:ValueError|null;readonly actual:number|null}
