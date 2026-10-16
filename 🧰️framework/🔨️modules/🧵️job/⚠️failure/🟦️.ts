import type {ValueError,ValueErrorRetainedProgress} from "../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import type {Fault} from "../../⚠️diagnostic/🧾️retained/🟦️.ts";
import type {IntrinsicValue} from "@semio-tech/value-ts/schema/intrinsic";
/** 🧬️ The original domain lends its complete typed fields and distinct native receipt. */
export interface OriginalDomainFailure {readonly code:string;readonly message:string;readonly retainedProgress:ValueErrorRetainedProgress;readonly original:Extract<IntrinsicValue,{kind:"object"}>}
/** 🧳️ Original typed refusals and diagnostics keep their complete first-party representation. */
export type OriginalTurnPrimary = {readonly kind:"value";readonly value:ValueError}|{readonly kind:"fault";readonly fault:Fault}|{readonly kind:"domain";readonly domain:OriginalDomainFailure};
/** 🪢️ The original semantic owner and complete receipt refusal remain independent. */
export interface OriginalTurnFailure {readonly primary:OriginalTurnPrimary;readonly receiptRefusal?:ValueError;readonly contextRefusal?:ValueError}

/** 🤝️ Complete original failure owners remain distinct through source retirement. */
export interface OriginalClosureFailure {readonly initial:OriginalTurnFailure;readonly closure:OriginalTurnFailure}
