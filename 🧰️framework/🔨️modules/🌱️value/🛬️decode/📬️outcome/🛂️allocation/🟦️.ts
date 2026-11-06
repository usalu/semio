import type {NativeAllocationRequest,ValueError} from "../../../🎟️control/🧬️contract/🛂️allocation/🟦️.ts";
/** 🧾️ The declared native decode outcome exposes only returned owned refusal causes. */
export interface NativeDecodeAllocationOutcome{readonly type:"decode";readonly request:NativeAllocationRequest|null;readonly admitted:boolean;readonly primary:ValueError|null;readonly quoteRefusal:ValueError|null;readonly extentRefusal:ValueError|null;readonly settlementRefusal:ValueError|null;readonly actual:number|null}
