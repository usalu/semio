export interface RetainedCloseDemand {copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface RetainedCloseDemandSource {readonly closeStarted:boolean;closeDemand(maximumBodyBytes:number):RetainedCloseDemand}

/** 🌿️ Quotes the original child once and adds its enclosing structural depth. */
export function nestedRetainedCloseDemand(child:RetainedCloseDemandSource,maximumBodyBytes:number):RetainedCloseDemand {
 if(!Number.isSafeInteger(maximumBodyBytes)||maximumBodyBytes<0)throw new RangeError("close body must be a nonnegative safe integer");
 if(!child.closeStarted)return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 const demand=child.closeDemand(maximumBodyBytes);
 if(!Number.isSafeInteger(demand.copyBytes)||demand.copyBytes<0||!Number.isSafeInteger(demand.capacityBytes)||demand.capacityBytes<0||!Number.isSafeInteger(demand.releaseBytes)||demand.releaseBytes<0||!Number.isSafeInteger(demand.depth)||demand.depth<0||demand.depth===Number.MAX_SAFE_INTEGER)throw new RangeError("invalid original close demand");
 return{copyBytes:demand.copyBytes,capacityBytes:demand.capacityBytes,releaseBytes:demand.releaseBytes,depth:demand.depth+1};
}
