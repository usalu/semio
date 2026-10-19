import type {ValueError,ValueErrorRetainedProgress} from "../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import type {Fault} from "@semio-tech/framework-diagnostic-ts/retained";
import type {IntrinsicValue} from "@semio-tech/value-ts/schema/intrinsic";
/** 🧬️ The original domain lends its complete typed fields and distinct native receipt. */
export interface OriginalDomainFailure {readonly code:string;readonly message:string;readonly retainedProgress:ValueErrorRetainedProgress;readonly original:Extract<IntrinsicValue,{kind:"object"}>}
/** 🧳️ Original typed refusals and diagnostics keep their complete first-party representation. */
export type OriginalTurnPrimary = {readonly kind:"value";readonly value:ValueError}|{readonly kind:"fault";readonly fault:Fault}|{readonly kind:"domain";readonly domain:OriginalDomainFailure};
/** 🪢️ The original semantic owner and complete receipt refusal remain independent. */
export interface OriginalTurnFailure {readonly primary:OriginalTurnPrimary;readonly receiptRefusal?:ValueError;readonly contextRefusal?:ValueError}

/** 🤝️ Complete original failure owners remain distinct through source retirement. */
export interface OriginalClosureFailure {readonly initial:OriginalTurnFailure;readonly closure:OriginalTurnFailure}

/** 📬️ Transports the original semantic cause and the complete distinct receipt refusal. */
export function faultFromOriginalTurnFailure(error:OriginalTurnFailure):Fault {
 const primary=error.primary,secondary=[error.receiptRefusal,error.contextRefusal].flatMap(cause=>cause?[{message:cause.message,code:cause.kind,retainedProgress:cause.retainedProgress}]:[]);
 const fault:Fault=primary.kind==="fault"?primary.fault:primary.kind==="domain"?{origin:"framework",code:primary.domain.code,severity:"error",message:primary.domain.message,scope:{},retryable:false,retainedProgress:primary.domain.retainedProgress,domain:primary.domain.original}:{origin:"framework",code:primary.value.kind,severity:"error",message:primary.value.message,scope:{},retryable:false,retainedProgress:primary.value.retainedProgress};
 return secondary.length?{...fault,causes:[...(fault.causes??[]),...secondary]}:fault;
}
