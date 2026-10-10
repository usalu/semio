import type {RetainedCloneGrant,RetainedCloneProgress,RetirementDemand} from "../../../🌱️value/🧬️retained-clone/🧬️contract/🟦️.ts";
export type RetainedWorkGrant=RetainedCloneGrant;
export type RetainedWorkDemand=RetirementDemand;
export type RetainedWorkReceipt=RetainedCloneProgress;
/** 🎟️ Original caller funding remains independent of CPU fuel and is charged once per physical receipt. */
export class RetainedWorkBudget{
 private used:RetainedWorkReceipt={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};
 constructor(private readonly original:RetainedWorkGrant){for(const value of Object.values(original))if(!Number.isSafeInteger(value)||value<0)throw Error("Invalid original retained work grant");this.original={...original};}
 remaining():RetainedWorkGrant{return {maximumItems:this.original.maximumItems-this.used.copiedItems,maximumCopyBytes:this.original.maximumCopyBytes-this.used.copiedBytes,maximumCapacityBytes:this.original.maximumCapacityBytes-this.used.retainedCapacityBytes,maximumReleaseBytes:this.original.maximumReleaseBytes-this.used.releasedBytes,maximumDepth:this.original.maximumDepth};}
 canEnter(demand:RetainedWorkDemand):boolean{for(const value of Object.values(demand))if(!Number.isSafeInteger(value)||value<0)throw Error("Invalid retained work demand");const grant=this.remaining();return grant.maximumItems>0&&grant.maximumCopyBytes>=demand.copyBytes&&grant.maximumCapacityBytes>=demand.capacityBytes&&grant.maximumReleaseBytes>=demand.releaseBytes&&grant.maximumDepth>=demand.depth;}
 charge(receipt:RetainedWorkReceipt):void{for(const value of Object.values(receipt))if(!Number.isSafeInteger(value)||value<0)throw Error("Invalid retained work receipt");const grant=this.remaining();if(receipt.copiedItems>grant.maximumItems||receipt.copiedBytes>grant.maximumCopyBytes||receipt.retainedCapacityBytes>grant.maximumCapacityBytes||receipt.releasedBytes>grant.maximumReleaseBytes)throw Error("Retained work receipt exceeds original funding");this.used.copiedItems+=receipt.copiedItems;this.used.copiedBytes+=receipt.copiedBytes;this.used.retainedCapacityBytes+=receipt.retainedCapacityBytes;this.used.releasedBytes+=receipt.releasedBytes;}
 receipt():RetainedWorkReceipt{return {...this.used};}
}
