export type ExactKeyGrant={maximumItems:number;maximumCopyBytes:number;maximumDepth:number};
export type ExactKeyReceipt={copiedItems:number;copiedBytes:number};
/** 🔑️ Complete original UTF8 equality advances by explicitly funded pages. */
export class ExactKeyCursor{
 private position=0;
 private offset=0;
 private done=false;
 private match:number|undefined;
 readonly fixedCopyBytes:number;
 constructor(wordBytes:4|8){this.fixedCopyBytes=12*wordBytes;}
 get nextIndex():number{return this.position;}
 get ready():boolean{return this.done;}
 get found():number|undefined{return this.done?this.match:undefined;}
 demand(key:Uint8Array,candidate:Uint8Array|undefined,body:number):number{
  if(this.done)return 0;
  if(this.offset>key.length||this.offset>0&&candidate?.length!==key.length)throw new Error("exact lookup original candidate changed");
  const remaining=candidate?.length===key.length?key.length-this.offset:0;
  const page=Math.min(remaining,Math.max(1,Math.floor((body-this.fixedCopyBytes)/2)),32768);
  return this.fixedCopyBytes+2*page;
 }
 advance(key:Uint8Array,candidate:Uint8Array|undefined,grant:ExactKeyGrant):ExactKeyReceipt{
  if(this.done)return{copiedItems:0,copiedBytes:0};
  const demand=this.demand(key,candidate,grant.maximumCopyBytes);
  if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<demand)return{copiedItems:0,copiedBytes:0};
  const count=(demand-this.fixedCopyBytes)/2;let equal=candidate?.length===key.length;
  if(equal)for(let index=this.offset;index<this.offset+count;index++)if(key[index]!==candidate![index]){equal=false;break;}
  if(candidate===undefined)this.done=true;
  else if(!equal){if(!Number.isSafeInteger(this.position+1))throw new RangeError("exact lookup source ordinal overflow");this.position++;this.offset=0;}
  else{this.offset+=count;if(this.offset===key.length){this.match=this.position;this.done=true;}}
  return{copiedItems:1,copiedBytes:demand};
 }
}
