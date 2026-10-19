import type {ValueError,ValueErrorRetainedProgress} from "../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import type {Fault} from "@semio-tech/framework-diagnostic-ts/retained";
import type {IntrinsicValue} from "@semio-tech/value-ts/schema/intrinsic";
/** 🧬️ The original domain lends its complete typed fields and distinct native receipt. */
export interface OriginalDomainFailure {readonly code:string;readonly message:string;readonly retainedProgress:ValueErrorRetainedProgress;readonly original:Extract<IntrinsicValue,{kind:"object"}>}
import type {OriginalReceivingPrimary} from "../../🌱️value/🛬️decode/🏭️construction/🧾️result/🛬️receiving/🟦️.ts";
export type {OriginalReceivingPrimary} from "../../🌱️value/🛬️decode/🏭️construction/🧾️result/🛬️receiving/🟦️.ts";
/** 🧳️ Original typed refusals and diagnostics keep their complete first-party representation. */
export type OriginalTurnPrimary = {readonly kind:"value";readonly value:ValueError}|{readonly kind:"fault";readonly fault:Fault}|{readonly kind:"domain";readonly domain:OriginalDomainFailure}|{readonly kind:"receiving";readonly receiving:OriginalReceivingPrimary};
/** 🪢️ The original semantic owner and complete receipt refusal remain independent. */
export interface OriginalTurnFailure {readonly effects:ValueErrorRetainedProgress;readonly primary:OriginalTurnPrimary;readonly receiptRefusal?:ValueError;readonly contextRefusal?:ValueError}

/** 🤝️ Complete original failure owners remain distinct through source retirement. */
export interface OriginalClosureFailure {readonly initial:OriginalTurnFailure;readonly closure:OriginalTurnFailure}

/** 📬️ Transports the original semantic cause and the complete distinct receipt refusal. */
export function faultFromOriginalTurnFailure(error:OriginalTurnFailure):Fault {
 const primary=error.primary,secondary=[error.receiptRefusal,error.contextRefusal].flatMap(cause=>cause?[{message:cause.message,code:cause.kind,retainedProgress:cause.retainedProgress}]:[]);
 const fault:Fault=primary.kind==="fault"?primary.fault:primary.kind==="receiving"?faultFromOriginalReceiving(primary.receiving):primary.kind==="domain"?{origin:"framework",code:primary.domain.code,severity:"error",message:primary.domain.message,scope:{},retryable:false,retainedProgress:primary.domain.retainedProgress,domain:primary.domain.original}:{origin:"framework",code:primary.value.kind,severity:"error",message:primary.value.message,scope:{},retryable:false,retainedProgress:primary.value.retainedProgress};
 const span=error.effects,own=fault.retainedProgress,same=span===own||(span.copiedItems===own.copiedItems&&span.copiedBytes===own.copiedBytes&&span.retainedCapacityBytes===own.retainedCapacityBytes&&span.releasedBytes===own.releasedBytes);
 return secondary.length?{...fault,retainedProgress:span,causes:[...(fault.causes??[]),...secondary]}:same?fault:{...fault,retainedProgress:span};
}

/** 🛬️ Receiving roles retain their exact typed causes and independent physical effects. */
function faultFromOriginalReceiving(original:import("../../🧵️job/⚠️failure/🟦️.ts").OriginalReceivingPrimary):Fault {
 const progress=(value:typeof original.retainedProgress):import("../../🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts").IntrinsicValue=>({kind:"object",members:Object.entries(value).map(([name,number])=>({name,value:{kind:"unsigned" as const,value:BigInt(number)}}))});
 const causes=(["primary","quoteRefusal","allocationRefusal"] as const).flatMap(name=>{const cause=original[name];return cause?[{name,value:{kind:"object" as const,members:[{name:"kind",value:{kind:"text" as const,value:cause.kind}},{name:"message",value:{kind:"text" as const,value:cause.message}},{name:"retainedProgress",value:progress(cause.retainedProgress)}]}}]:[]});
 const first=original.primary??original.quoteRefusal??original.allocationRefusal;
 return {origin:"framework",code:"original.receiving",severity:"error",message:first?.message??"Original receiving receipt refused",scope:{},retryable:false,retainedProgress:original.retainedProgress,domain:{kind:"object",members:[...causes,{name:"retainedProgress",value:progress(original.retainedProgress)}]}};
}
