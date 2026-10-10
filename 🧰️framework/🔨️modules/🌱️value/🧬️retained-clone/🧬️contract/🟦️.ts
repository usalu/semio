/** 🎟️ Portable wire contracts for original retained clone currencies. */
export interface RetainedCloneGrant{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}
export interface RetainedCloneProgress{copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number}
export interface RetirementDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
