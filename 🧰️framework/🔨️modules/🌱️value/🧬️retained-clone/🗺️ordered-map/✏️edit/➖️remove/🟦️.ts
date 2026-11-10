export interface OrderedMapRemovalGrant { comparisonItems:number;comparisonBytes:number;movedItems:number;movedBytes:number }
export interface OrderedMapRemovalProgress { comparisonItems:number;comparisonBytes:number;movedItems:number;movedBytes:number }
export type OrderedMapRemovalStep={complete:boolean;removed:boolean;progress:OrderedMapRemovalProgress};
export interface OrderedMapRemovalCustody<K,V> {pages:Array<Array<[K,V]>>|null;key:K;removed:[K,V]|null;emptyPage:Array<[K,V]>|null}
/** ➖️ Preserves actual candidate page references while yielding key comparisons and bounded compaction. */
export class OrderedMapRemoval<K extends number|string,V> {
 private lower=0;private upper:number;private ordinal:number|null=null;private shift=0;private phase=0;private leftOffset=0;private rightOffset=0;private removedEntry:[K,V]|null=null;private emptyPage:Array<[K,V]>|null=null;private closing=false;private taken=false;
 constructor(private pages:Array<Array<[K,V]>>|null,private key:K|null,private stride:number){
  if(!pages||key===null||!Number.isSafeInteger(stride)||stride<1||pages.some((page,index)=>page.length>16||page.length===0||(index+1<pages.length&&page.length!==16)))throw Error("Invalid owned ordered page layout");
  this.upper=pages.reduce((count,page)=>count+page.length,0);
 }
 advance(grant:OrderedMapRemovalGrant):OrderedMapRemovalStep{
  if(this.closing||this.taken)throw Error("Removal cursor is closing or spent");const progress:OrderedMapRemovalProgress={comparisonItems:0,comparisonBytes:0,movedItems:0,movedBytes:0};const result=()=>({complete:this.phase===4,removed:this.ordinal!==null,progress});
  const pages=this.pages!;
  if(this.phase===0){
   if(grant.comparisonItems<1)return result();if(this.lower===this.upper){this.phase=4;progress.comparisonItems=1;return result()}
   const middle=this.lower+Math.floor((this.upper-this.lower)/2);const entry=pages[Math.floor(middle/16)]![middle%16]!;const left=entry[0];let ordering:number;
   if(typeof left==="number"&&typeof this.key==="number"){if(grant.comparisonBytes<8)return result();ordering=Math.sign(left-this.key);progress.comparisonBytes=8;}
   else if(typeof left==="string"&&typeof this.key==="string"){
    if(grant.comparisonBytes<4)return result();const a=left.codePointAt(this.leftOffset);const b=this.key.codePointAt(this.rightOffset);progress.comparisonBytes=4;
    if(a!==undefined&&b!==undefined&&a===b){this.leftOffset+=a>65535?2:1;this.rightOffset+=b>65535?2:1;progress.comparisonItems=1;return result()}
    ordering=a===undefined?(b===undefined?0:-1):b===undefined?1:Math.sign(a-b);this.leftOffset=0;this.rightOffset=0;
   }else throw Error("Ordered key kinds differ");
   progress.comparisonItems=1;if(ordering===0){this.ordinal=middle;this.phase=1}else if(ordering<0)this.lower=middle+1;else this.upper=middle;return result();
  }
  if(this.phase===1){const ordinal=this.ordinal!;const page=Math.floor(ordinal/16);const slot=ordinal%16;const moved=pages[page]!.length-slot;if(grant.movedItems<moved||grant.movedBytes<moved*this.stride)return result();this.removedEntry=pages[page]!.splice(slot,1)[0]!;this.shift=page;this.phase=2;progress.movedItems=moved;progress.movedBytes=moved*this.stride;return result()}
  if(this.phase===2){if(this.shift+1===pages.length){this.phase=3;return result()}const next=pages[this.shift+1]!;const moved=next.length+1;if(grant.movedItems<moved||grant.movedBytes<moved*this.stride)return result();pages[this.shift]!.push(next.shift()!);this.shift++;progress.movedItems=moved;progress.movedBytes=moved*this.stride;return result()}
  if(this.phase===3){if(grant.movedItems<1||grant.movedBytes<this.stride)return result();if(pages.at(-1)?.length===0)this.emptyPage=pages.pop()!;this.phase=4;progress.movedItems=1;progress.movedBytes=this.stride;}
  return result();
 }
 outputReady():boolean{return this.phase===4&&this.pages!==null&&!this.closing&&!this.taken}
 take():Array<Array<[K,V]>>|null{if(this.phase!==4||this.closing||this.taken)return null;this.taken=true;const pages=this.pages;this.pages=null;return pages}
 beginClose():void{this.closing=true}
 takeRetainedCustody():OrderedMapRemovalCustody<K,V>{if(!this.closing||this.key===null)throw Error("Removal must begin close and transfer once");const custody={pages:this.pages,key:this.key,removed:this.removedEntry,emptyPage:this.emptyPage};this.pages=null;this.key=null;this.removedEntry=null;this.emptyPage=null;return custody}
}
