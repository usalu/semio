import type {NativeEncodeAllocationOutcome} from "@semio-tech/value-ts/encode/allocation";
import type {NativeEncodeEffectFailure} from "@semio-tech/value-ts/encode/effect";
export type {NativeEncodeAllocationOutcome} from "@semio-tech/value-ts/encode/allocation";
export type {NativeEncodeEffectFailure} from "@semio-tech/value-ts/encode/effect";
import {ValueError,type ValueErrorRetainedProgress,type ValueRefusalKind} from "@semio-tech/value-ts/refusal";
import type {NativeAllocationFailure,NativeAllocationRequest} from "../../🌱️value/🎟️control/🧬️contract/🛂️allocation/🟦️.ts";
import type {NativeDecodeAllocationOutcome} from "../../🌱️value/🛬️decode/📬️outcome/🛂️allocation/🟦️.ts";
import type {OriginalReceivingPrimary,OriginalReceivingRetirementFailure} from "../../🌱️value/🛬️decode/📬️outcome/🛬️receiving/🟦️.ts";
export type {NativeAllocationFailure,NativeAllocationRequest,NativeDecodeAllocationOutcome,OriginalReceivingPrimary,OriginalReceivingRetirementFailure,ValueErrorRetainedProgress};
/** 🧭️ Retains the original domain of each snapshot construction refusal. */
export type NativeSnapshotCause={readonly type:"value";readonly value:ValueError}|{readonly type:"receiving";readonly value:OriginalReceivingPrimary}|{readonly type:"retirement";readonly value:OriginalReceivingRetirementFailure};
/** 🛂️ Keeps the exact directional allocation owner. */
export type NativeSnapshotAllocationFailure=NativeEncodeAllocationOutcome|NativeDecodeAllocationOutcome;
/** 🧩️ Transfers a complete snapshot failure without cloning any original child. */
export interface NativeSnapshotFailureParts{readonly cause:NativeSnapshotCause|null;readonly allocation:NativeSnapshotAllocationFailure|null;readonly encodeEffect:NativeEncodeEffectFailure|null;readonly receiptRefusal:ValueError|null;readonly contextRefusal:ValueError|null;readonly workRefusal:ValueError|null;readonly settlementRefusal:ValueError|null;readonly progress:ValueErrorRetainedProgress}
const allocationKeys=["primary","admission","forwardedAdmission","scopedAdmission","forwardedQuote","scopedQuote","forwardedPost","scopedPost","forwardedCancel","scopedCancel","receipt","extent"]as const;
function allocationPrimary(value:NativeSnapshotAllocationFailure):ValueError|undefined{return value.primary??(value.type==="encode"?value.forwardedQuote??value.scopedQuote??value.extentRefusal??value.forwardedRefusal??value.scopedRefusal:value.quoteRefusal??value.extentRefusal??value.settlementRefusal)??undefined}
function retirementPrimary(value:OriginalReceivingRetirementFailure):ValueError|undefined{let node:OriginalReceivingRetirementFailure|undefined=value,fallback:ValueError|undefined;while(node!==undefined){if(node.primary!==undefined)return node.primary;fallback=node.receiptRefusal??node.workRefusal??node.allocationRefusal??fallback;node=node.child}return fallback}
function receivingPrimary(value:OriginalReceivingPrimary):ValueError|undefined{const direct=value.primary??value.quoteRefusal??value.allocationRefusal??value.workRefusal??value.workOverrun;if(direct!==undefined)return direct;if(value.allocationFailure!==undefined)for(const key of allocationKeys){const error=value.allocationFailure[key];if(error!==null)return error}return(value.nativeAllocation===undefined?undefined:allocationPrimary(value.nativeAllocation))??value.receiptRefusal??value.receiptCompletion??(value.retirementFailure===undefined?undefined:retirementPrimary(value.retirementFailure))}
function effectPrimary(value:NativeEncodeEffectFailure):ValueError|undefined{return value.primary??value.context??value.invariant??value.physicalInvariant??value.capacityInvariant??value.markerFailure??value.work??value.workOverrun??value.local??value.physical??(value.physicalReceipt===null?undefined:receivingPrimary(value.physicalReceipt))??(value.allocation===null?undefined:allocationPrimary(value.allocation))}
/** 🧳️ Owns the complete native failure and exposes prose only through borrowed projections. */
export class NativeSnapshotFailure implements NativeSnapshotFailureParts{
 readonly cause:NativeSnapshotCause|null;readonly allocation:NativeSnapshotAllocationFailure|null;readonly encodeEffect:NativeEncodeEffectFailure|null;readonly receiptRefusal:ValueError|null;readonly contextRefusal:ValueError|null;readonly workRefusal:ValueError|null;readonly settlementRefusal:ValueError|null;readonly progress:ValueErrorRetainedProgress;
 constructor(parts:NativeSnapshotFailureParts){this.cause=parts.cause;this.allocation=parts.allocation;this.encodeEffect=parts.encodeEffect;this.receiptRefusal=parts.receiptRefusal;this.contextRefusal=parts.contextRefusal;this.workRefusal=parts.workRefusal;this.settlementRefusal=parts.settlementRefusal;this.progress=parts.progress}
 /** 🚨️ Moves the original semantic error and its actual receipt into complete custody. */
 static fromValueError(value:ValueError):NativeSnapshotFailure{return new NativeSnapshotFailure({cause:{type:"value",value},allocation:null,encodeEffect:null,receiptRefusal:null,contextRefusal:null,workRefusal:null,settlementRefusal:null,progress:value.retainedProgress})}
 /** 🔎️ Borrows the same priority projection as the native owner without discarding other causes. */
 get primaryValue():ValueError|undefined{const cause=this.cause;return(cause===null?undefined:cause.type==="value"?cause.value:cause.type==="receiving"?receivingPrimary(cause.value):retirementPrimary(cause.value))??(this.encodeEffect===null?undefined:effectPrimary(this.encodeEffect))??(this.allocation===null?undefined:allocationPrimary(this.allocation))??this.receiptRefusal??this.contextRefusal??this.workRefusal??this.settlementRefusal??undefined}
 /** 🏷️ Borrows the original semantic discriminator rather than classifying prose. */
 get kind():ValueRefusalKind{return this.primaryValue?.kind??"invalidValue"}
 /** 🗣️ Projects only the selected display message while all original causes remain owned. */
 get message():string{return this.primaryValue?.message??"original native snapshot refused"}
 /** 🧾️ Borrows the original enclosing physical receipt. */
 get retainedProgress():ValueErrorRetainedProgress{return this.progress}
}
