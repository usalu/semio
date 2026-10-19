import type {RetainedCloneGrant} from "../../🧬️retained-clone/🌐️wire/🟦️.ts";
export type SliceEqualityGrant=RetainedCloneGrant;
export type SliceEqualityReceipt={copiedItems:number;copiedBytes:number};
/** 🔬️ Comparisons read only the fixed inline representation without allocating or traversing referenced payloads. */
export interface RetainedEqualityElement<T>{readonly byteWidth:number;inlineEqual(left:T,right:T):boolean}
export type RetainedEqualitySlice<T>={readonly length:number;readonly [index:number]:T};

/** 🟰️ Borrowed scalar buffers advance only funded pages without cloning source arrays. */
export class RetainedSliceEquality<T>{
 private offset=0;
 private result:boolean|undefined;
 readonly fixedCopyBytes:number;
 constructor(readonly element:RetainedEqualityElement<T>,wordBytes:4|8=8){if(!Number.isSafeInteger(element.byteWidth)||element.byteWidth<0)throw Error("inline element width must be exact");this.fixedCopyBytes=12*wordBytes;}
 get ready():boolean{return this.result!==undefined;}
 get equal():boolean|undefined{return this.result;}
 private count(left:number,right:number,body:number):number{
  if(this.ready)throw Error("slice comparison already settled");if(left!==right)return 0;const remaining=left-this.offset;if(remaining<0)throw Error("original slice bounds changed");if(this.element.byteWidth===0)return Math.min(remaining,1);return Math.min(remaining,Math.max(1,Math.floor(32768/(2*this.element.byteWidth))),Math.max(1,Math.floor((body-this.fixedCopyBytes)/(2*this.element.byteWidth))));
 }
 demand(left:number,right:number,body:number):number{if(![left,right,body].every(n=>Number.isSafeInteger(n)&&n>=0))throw Error("slice extents and copy budget must be exact");const bytes=this.fixedCopyBytes+this.count(left,right,body)*2*this.element.byteWidth;if(!Number.isSafeInteger(bytes))throw Error("slice copy extent overflow");return bytes;}
 advance(left:RetainedEqualitySlice<T>,right:RetainedEqualitySlice<T>,grant:SliceEqualityGrant):SliceEqualityReceipt{
  if(!Number.isSafeInteger(grant.maximumItems)||grant.maximumItems<0||!Number.isSafeInteger(grant.maximumCopyBytes)||grant.maximumCopyBytes<0||!Number.isSafeInteger(grant.maximumCapacityBytes)||grant.maximumCapacityBytes<0||!Number.isSafeInteger(grant.maximumReleaseBytes)||grant.maximumReleaseBytes<0||!Number.isSafeInteger(grant.maximumDepth)||grant.maximumDepth<0)throw Error("slice comparison requires complete canonical integer grant axes");
  const demand=this.demand(left.length,right.length,grant.maximumCopyBytes);if(grant.maximumItems<1||grant.maximumCopyBytes<demand||grant.maximumDepth<1)return{copiedItems:0,copiedBytes:0};
  if(left.length!==right.length)this.result=false;
  else{const end=this.offset+this.count(left.length,right.length,grant.maximumCopyBytes);for(let index=this.offset;index<end;index++)if(!this.element.inlineEqual(left[index]!,right[index]!)){this.result=false;break;}this.offset=end;if(this.result===undefined&&end===left.length)this.result=true;}
  return{copiedItems:1,copiedBytes:demand};
 }
}
