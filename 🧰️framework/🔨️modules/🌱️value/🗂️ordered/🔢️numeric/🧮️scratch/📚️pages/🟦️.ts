/** 📚️ Fixed-fanout scalar custody with one original metadata block per reserve or close step. */
export interface NumericNode<V>{key:number;value:V|undefined;generation:number;countGeneration:number;count:number;height:number;left:number|undefined;right:number|undefined;parent:number|undefined}
export interface NumericPageProgress{progressed:boolean;allocatedBytes:number;releasedAllocationBytes:number}
const FANOUT=16,BRANCH_BYTES=256,LEAF_BYTES=256+FANOUT*128;
type Page<V>={level:0;rows:NumericNode<V>[]}|{level:number;children:(Page<V>|undefined)[]};
const emptyNode=<V>():NumericNode<V>=>({key:0,value:undefined,generation:0,countGeneration:0,count:0,height:0,left:undefined,right:undefined,parent:undefined});
const progress=(allocatedBytes=0,releasedAllocationBytes=0):NumericPageProgress=>({progressed:allocatedBytes+releasedAllocationBytes>0,allocatedBytes,releasedAllocationBytes});
const leaf=<V>(page:Page<V>):page is {level:0;rows:NumericNode<V>[]}=>'rows' in page;
const span=(page:Page<unknown>)=>FANOUT**(page.level+1);
const bytes=(page:Page<unknown>)=>leaf(page)?LEAF_BYTES:BRANCH_BYTES;
/** 🧱️ Accounted bytes describe fixed owned scalar capacity, independent of VM header layout or GC timing. */
export class NumericPages<V>{
 private root:Page<V>|undefined;length=0;allocatedBytes=0;
 get(index:number):NumericNode<V>|undefined{if(!Number.isSafeInteger(index)||index<0)return undefined;const root=this.root;if(!root||index>=span(root))return undefined;let page:Page<V>=root;while(!leaf(page)){const size=FANOUT**page.level;const child:Page<V>|undefined=page.children[Math.floor(index/size)%FANOUT];if(!child)return undefined;page=child;}return page.rows[index%FANOUT];}
 get hasReservedSlot():boolean{return this.get(this.length)!==undefined;}
 nextReserveDepthDemand():number{if(this.hasReservedSlot)return 0;return this.root?this.root.level+3+Number(this.length>=span(this.root)):3;}
 nextReserveCopyByteDemand():number{return 0;}
 nextAllocationBytes():number{
  const root=this.root;if(!root)return LEAF_BYTES;if(this.length>=span(root))return BRANCH_BYTES;let page:Page<V>=root;
  while(!leaf(page)){const child:Page<V>|undefined=page.children[Math.floor(this.length/FANOUT**page.level)%FANOUT];if(!child)return page.level===1?LEAF_BYTES:BRANCH_BYTES;page=child;}return 0;
 }
 reserveOne(maximumBytes:number):NumericPageProgress{
  if(!Number.isSafeInteger(maximumBytes)||maximumBytes<0)throw RangeError('numeric.pages.capacity');const quote=this.nextAllocationBytes();if(quote===0||maximumBytes<quote)return progress();
  const make=(level:number):Page<V>=>level===0?{level:0,rows:Array.from({length:FANOUT},()=>emptyNode<V>())}:{level,children:new Array<Page<V>|undefined>(FANOUT)};
  if(!this.root)this.root=make(0);
  else if(this.length>=span(this.root)){const before=this.root;this.root=make(before.level+1);if(leaf(this.root))throw Error('numeric.pages.root');this.root.children[0]=before;}
  else {let page=this.root;while(!leaf(page)){const at=Math.floor(this.length/FANOUT**page.level)%FANOUT,child=page.children[at];if(!child){page.children[at]=make(page.level-1);break;}page=child;}}
  this.allocatedBytes+=quote;return progress(quote);
 }
 pushReserved(value:NumericNode<V>):void{const node=this.get(this.length);if(!node)throw Error('numeric.pages.reserved');if(!Number.isSafeInteger(this.length+1))throw RangeError('numeric.pages.address');Object.assign(node,value);this.length++;}
 popCopy():boolean{if(this.length===0)return false;const node=this.get(--this.length)!;Object.assign(node,emptyNode<V>());return true;}
 private last(page:Page<V>,parent:Page<V>|undefined=undefined,at=0):{page:Page<V>;parent:Page<V>|undefined;at:number}{if(!leaf(page))for(let index=FANOUT-1;index>=0;index--){const child:Page<V>|undefined=page.children[index];if(child)return this.last(child,page,index);}return{page,parent,at};}
 nextReleaseBytes():number{return this.length>0||!this.root?0:bytes(this.last(this.root).page);}
 releaseOne(maximumBytes:number):NumericPageProgress{if(!Number.isSafeInteger(maximumBytes)||maximumBytes<0)throw RangeError('numeric.pages.release');if(this.length>0||!this.root)return progress();const target=this.last(this.root),quote=bytes(target.page);if(maximumBytes<quote)return progress();if(target.parent){if(leaf(target.parent))throw Error('numeric.pages.parent');target.parent.children[target.at]=undefined;}else this.root=undefined;this.allocatedBytes-=quote;return progress(0,quote);}
 terminalIsEmpty():boolean{return this.length===0&&this.root===undefined;}
}
