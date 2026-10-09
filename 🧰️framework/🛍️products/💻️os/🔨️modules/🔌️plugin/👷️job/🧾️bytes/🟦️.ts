/** 👷️ Reserved payload work and backing release have independent declared authorities. */
export type ReservedJobByteOwner={length:number;capacity:number};
export type ReservedJobByteGrant={items:number;copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number};
export function reservedJobByteDemand(owner:ReservedJobByteOwner){
 if(!Number.isSafeInteger(owner.length)||!Number.isSafeInteger(owner.capacity)||owner.length<0||owner.capacity<owner.length)throw Error("invalid original reserved byte owner");
 return {copyBytes:Number(owner.length>0),capacityBytes:0,releaseBytes:owner.length===0?owner.capacity:0,depth:Number(owner.capacity>0)};
}
export function reservedJobByteTurn(owner:ReservedJobByteOwner,grant:ReservedJobByteGrant){
 const progress={copiedItems:0,copiedBytes:0,capacityBytes:0,releaseBytes:0};
 const demand=reservedJobByteDemand(owner);
 if(!Object.values(grant).every(value=>Number.isSafeInteger(value)&&value>=0))throw Error("invalid original reserved byte grant");
 if(demand.depth===0)return {owner,progress,complete:true};
 if(grant.items===0)return {owner,progress,complete:false};
 if(grant.depth<demand.depth)return {owner,progress,complete:false,refusal:"depthLimit"};
 if(grant.copyBytes<demand.copyBytes||grant.releaseBytes<demand.releaseBytes)return {owner,progress,complete:false};
 if(owner.length>0){const copiedBytes=Math.min(owner.length,grant.copyBytes);return {owner:{length:owner.length-copiedBytes,capacity:owner.capacity},progress:{...progress,copiedItems:1,copiedBytes},complete:false};}
 return {owner:{length:0,capacity:0},progress:{...progress,copiedItems:1,releaseBytes:owner.capacity},complete:false};
}
