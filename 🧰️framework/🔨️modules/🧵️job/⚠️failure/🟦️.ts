import type {ValueError,ValueErrorRetainedProgress} from "../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import type {Fault} from "@semio-tech/framework-diagnostic-ts/retained";
import type {IntrinsicValue} from "@semio-tech/value-ts/schema/intrinsic";
/** 🧬️ The original domain lends its complete typed fields and distinct native receipt. */
export interface OriginalDomainFailure {readonly code:string;readonly message:string;readonly retainedProgress:ValueErrorRetainedProgress;readonly original:Extract<IntrinsicValue,{kind:"object"}>}
import type {OriginalReceivingPrimary} from "../../🌱️value/🛬️decode/🏭️construction/🧾️result/🛬️receiving/🟦️.ts";
export type {OriginalReceivingPrimary} from "../../🌱️value/🛬️decode/🏭️construction/🧾️result/🛬️receiving/🟦️.ts";
/** ♻️ Original retirement causes keep their own receipts and node-local enclosing progress. */
export interface OriginalRetirementFailure {readonly primary?:ValueError;readonly workRefusal?:ValueError;readonly receiptRefusal?:ValueError;readonly child?:OriginalRetirementFailure;readonly progressOverride:ValueErrorRetainedProgress|null;readonly retainedProgress:ValueErrorRetainedProgress}
/** 🪢️ A failed retirement and its complete independent receiving refusal remain separate owners. */
export interface OriginalRetirementReceivingPrimary {readonly retirement:OriginalRetirementFailure;readonly receivingPrimary:ValueError;readonly receivingEffects:ValueErrorRetainedProgress;readonly receivingReceiptRefusal?:ValueError;readonly receivingContextRefusal?:ValueError;readonly receivingWorkRefusal?:ValueError}
/** 🧳️ Original typed refusals and diagnostics keep their complete first-party representation. */
export type OriginalTurnPrimary = {readonly kind:"value";readonly value:ValueError}|{readonly kind:"fault";readonly fault:Fault}|{readonly kind:"domain";readonly domain:OriginalDomainFailure}|{readonly kind:"receiving";readonly receiving:OriginalReceivingPrimary}|{readonly kind:"retirement";readonly retirement:OriginalRetirementFailure}|{readonly kind:"retirementReceiving";readonly retirementReceiving:OriginalRetirementReceivingPrimary};
/** 🪢️ The original semantic owner and complete receipt refusal remain independent. */
export interface OriginalTurnFailure {readonly effects:ValueErrorRetainedProgress;readonly primary:OriginalTurnPrimary;readonly receiptRefusal?:ValueError;readonly contextRefusal?:ValueError;readonly workRefusal?:ValueError}

/** 🤝️ Complete original failure owners remain distinct through source retirement. */
export interface OriginalClosureFailure {readonly initial:OriginalTurnFailure;readonly closure:OriginalTurnFailure}

/** 📬️ Transports the original semantic cause and the complete distinct receipt refusal. */
export function faultFromOriginalTurnFailure(error:OriginalTurnFailure):Fault {
 const primary=error.primary,secondary=[error.receiptRefusal,error.contextRefusal,error.workRefusal].flatMap(cause=>cause?[{message:cause.message,code:cause.kind,retainedProgress:cause.retainedProgress}]:[]);
 const fault:Fault=primary.kind==="fault"?primary.fault:primary.kind==="receiving"?faultFromOriginalReceiving(primary.receiving):primary.kind==="retirement"?faultFromOriginalRetirement(primary.retirement):primary.kind==="retirementReceiving"?faultFromOriginalRetirementReceiving(primary.retirementReceiving):primary.kind==="domain"?{origin:"framework",code:primary.domain.code,severity:"error",message:primary.domain.message,scope:{},retryable:false,retainedProgress:primary.domain.retainedProgress,domain:primary.domain.original}:{origin:"framework",code:primary.value.kind,severity:"error",message:primary.value.message,scope:{},retryable:false,retainedProgress:primary.value.retainedProgress};
 const span=error.effects,own=fault.retainedProgress,same=span===own||(span.copiedItems===own.copiedItems&&span.copiedBytes===own.copiedBytes&&span.retainedCapacityBytes===own.retainedCapacityBytes&&span.releasedBytes===own.releasedBytes);
 return secondary.length?{...fault,retainedProgress:span,causes:[...(fault.causes??[]),...secondary]}:same?fault:{...fault,retainedProgress:span};
}

type IntrinsicObject=Extract<IntrinsicValue,{kind:"object"}>;
function progressValue(progress:ValueErrorRetainedProgress):IntrinsicObject{return{kind:"object",members:Object.entries(progress).map(([name,value])=>({name,value:{kind:"unsigned" as const,value:BigInt(value)}}))};}
function refusalValue(cause:ValueError):IntrinsicObject{return{kind:"object",members:[{name:"kind",value:{kind:"text",value:cause.kind}},{name:"message",value:{kind:"text",value:cause.message}},{name:"retainedProgress",value:progressValue(cause.retainedProgress)}]};}
function retirementValue(original:OriginalRetirementFailure):IntrinsicObject{
 const chain:OriginalRetirementFailure[]=[];for(let node:OriginalRetirementFailure|undefined=original;node;node=node.child)chain.push(node);
 let child:IntrinsicObject|undefined;for(let index=chain.length-1;index>=0;index--){
  const node=chain[index]!,members:IntrinsicObject["members"]=(["primary","workRefusal","receiptRefusal"]as const).flatMap(name=>node[name]?[{name,value:refusalValue(node[name]!)}]:[]);
  if(child)members.push({name:"child",value:child});members.push({name:"progressOverride",value:node.progressOverride?progressValue(node.progressOverride):{kind:"null"}},{name:"retainedProgress",value:progressValue(node.retainedProgress)});child={kind:"object",members};
 }
 return child!;
}
function faultFromOriginalRetirement(original:OriginalRetirementFailure):Fault{
 const first=original.primary??original.workRefusal??original.receiptRefusal;return{origin:"framework",code:"original.retirement",severity:"error",message:first?.message??"Original retirement refused",scope:{},retryable:false,retainedProgress:original.retainedProgress,domain:retirementValue(original)};
}
function faultFromOriginalRetirementReceiving(original:OriginalRetirementReceivingPrimary):Fault{
 const fault=faultFromOriginalRetirement(original.retirement),members:IntrinsicObject["members"]=[{name:"retirement",value:retirementValue(original.retirement)},{name:"receivingPrimary",value:refusalValue(original.receivingPrimary)},{name:"receivingEffects",value:progressValue(original.receivingEffects)}];
 for(const name of["receivingReceiptRefusal","receivingContextRefusal","receivingWorkRefusal"]as const){const cause=original[name];if(cause)members.push({name,value:refusalValue(cause)});}
 return{...fault,code:"original.retirementReceiving",domain:{kind:"object",members}};
}

/** 🛬️ Receiving roles retain their exact typed causes and independent physical effects. */
function faultFromOriginalReceiving(original:OriginalReceivingPrimary):Fault {
 const causes=(["primary","quoteRefusal","allocationRefusal","workRefusal"] as const).flatMap(name=>{const cause=original[name];return cause?[{name,value:refusalValue(cause)}]:[]});
 const first=original.primary??original.quoteRefusal??original.allocationRefusal??original.workRefusal;
 const retirement=original.retirementFailure;const nested=retirement?[{name:"retirementFailure",value:retirementValue(retirement)}]:[];
 return {origin:"framework",code:"original.receiving",severity:"error",message:first?.message??(retirement?faultFromOriginalRetirement(retirement).message:"Original receiving receipt refused"),scope:{},retryable:false,retainedProgress:original.retainedProgress,domain:{kind:"object",members:[...causes,...nested,{name:"retainedProgress",value:progressValue(original.retainedProgress)}]}};
}
