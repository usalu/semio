import type {RetainedCloneGrant} from "../../🧬️retained-clone/🌐️wire/🟦️.ts";
export type SliceEqualityGrant=RetainedCloneGrant;
export type SliceEqualityReceipt={copiedItems:number;copiedBytes:number;work:number};
/** 🔬️ Comparisons read only the fixed inline representation without allocating or traversing referenced payloads. */
export interface RetainedEqualityElement<T>{readonly byteWidth:number;readonly work:number;inlineEqual(left:T,right:T):boolean}
export type RetainedEqualitySlice<T>={readonly length:number;readonly [index:number]:T};

/** 🟰️ Borrowed scalar buffers advance only funded pages without cloning source arrays. */
export class RetainedSliceEquality<T>{
 private offset=0;
 private result:boolean|undefined;
 readonly fixedCopyBytes:number;
 constructor(readonly element:RetainedEqualityElement<T>,wordBytes:4|8=8){if(!Number.isSafeInteger(element.byteWidth)||element.byteWidth<0||!Number.isSafeInteger(element.work)||element.work<1)throw Error("inline element width and work must be exact");this.fixedCopyBytes=12*wordBytes;}
 get ready():boolean{return this.result!==undefined;}
 get equal():boolean|undefined{return this.result;}
 private count(left:number,right:number,body:number,maximumWork:number):number{
  if(this.ready)throw Error("slice comparison already settled");if(left!==right)return 0;const remaining=left-this.offset;if(remaining<0)throw Error("original slice bounds changed");const work=Math.floor(Math.max(0,maximumWork-1)/this.element.work);if(this.element.byteWidth===0)return Math.min(remaining,1,work);return Math.min(remaining,Math.max(1,Math.floor(32768/(2*this.element.byteWidth))),Math.max(0,Math.floor((body-this.fixedCopyBytes)/(2*this.element.byteWidth))),work);
 }
 demand(left:number,right:number,body:number,maximumWork:number):number{if(![left,right,body,maximumWork].every(n=>Number.isSafeInteger(n)&&n>=0))throw Error("slice extents and budgets must be exact");const count=this.count(left,right,body,maximumWork);if(maximumWork===0||(left===right&&this.offset<left&&count===0))return 0;const bytes=this.fixedCopyBytes+count*2*this.element.byteWidth;if(!Number.isSafeInteger(bytes))throw Error("slice copy extent overflow");return bytes;}
 workDemand(left:number,right:number,body:number,maximumWork:number):number{this.demand(left,right,body,maximumWork);if(maximumWork===0)return 0;if(left!==right||this.offset===left)return 1;const count=this.count(left,right,body,maximumWork);return count===0?0:1+count*this.element.work;}
 advance(left:RetainedEqualitySlice<T>,right:RetainedEqualitySlice<T>,body:number,maximumWork:number,grant:SliceEqualityGrant):SliceEqualityReceipt{
  if(!Number.isSafeInteger(grant.maximumItems)||grant.maximumItems<0||!Number.isSafeInteger(grant.maximumCopyBytes)||grant.maximumCopyBytes<0||!Number.isSafeInteger(grant.maximumCapacityBytes)||grant.maximumCapacityBytes<0||!Number.isSafeInteger(grant.maximumReleaseBytes)||grant.maximumReleaseBytes<0||!Number.isSafeInteger(grant.maximumDepth)||grant.maximumDepth<0)throw Error("slice comparison requires complete canonical integer grant axes");
  const selected=this.workDemand(left.length,right.length,body,maximumWork),demand=this.demand(left.length,right.length,body,maximumWork);if(selected===0||grant.maximumItems<1||grant.maximumCopyBytes<demand||grant.maximumDepth<1)return{copiedItems:0,copiedBytes:0,work:0};
  let work=1,read=0;
  if(left.length!==right.length)this.result=false;
  else{const end=this.offset+this.count(left.length,right.length,body,maximumWork);for(let index=this.offset;index<end;index++){work+=this.element.work;read+=2*this.element.byteWidth;if(!this.element.inlineEqual(left[index]!,right[index]!)){this.result=false;break;}}this.offset=end;if(this.result===undefined&&end===left.length)this.result=true;}
  return{copiedItems:1,copiedBytes:this.fixedCopyBytes+read,work};
 }
}
