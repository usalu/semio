import type {ValueError} from "../../🌱️value/⚠️refusal/🟦️.ts";
import type {Fault} from "../../⚠️diagnostic/🧾️retained/🟦️.ts";
/** 🧳️ Original typed refusals and diagnostics keep their complete first-party representation. */
export type OriginalTurnPrimary = {readonly kind:"value";readonly value:ValueError}|{readonly kind:"fault";readonly fault:Fault};
/** 🪢️ The original semantic owner and complete receipt refusal remain independent. */
export interface OriginalTurnFailure {readonly primary:OriginalTurnPrimary;readonly receiptRefusal?:ValueError;readonly contextRefusal?:ValueError}
