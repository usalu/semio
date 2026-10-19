import {HeapSortCursor} from "../../../📶️sort/🟦️.ts";
import {IndexedEditPlan} from "../🟦️.ts";

export type NamedEditLane="base"|"removed"|"modified"|"added";
export type NamedEditShape={baseLen:number;removed:number;modified:number;added:number};
/** 🔐️ What the named plan needs from an admitted original key directory; the index owner implements it. */
export interface NamedEditAdmittedBase{readonly length:number;readonly order:readonly number[];checkSource(source:NamedEditSource):void}
export interface NamedEditSource{readonly originalBase:object;readonly keyProjection?:object;readonly shape:NamedEditShape;readonly admittedBase?:NamedEditAdmittedBase;keyDemand(lane:NamedEditLane,ordinal:number):NamedEditDemand;observeKey(lane:NamedEditLane,ordinal:number,grant:NamedEditGrant):{key:Uint8Array;receipt:NamedEditReceipt}|undefined;addedIndex(ordinal:number):number}
export type NamedEditGrant={maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumDepth:number};
export type NamedEditDemand={copyBytes:number;capacityBytes:number;depth:number};
export type NamedEditReceipt={copiedItems:number;copiedBytes:number;retainedCapacityBytes:number};
const zero=():NamedEditReceipt=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0});
/** 👁️ A contiguous original header grant precedes its borrowed key selector. */
export function observeContiguousNamedKey(key:()=>Uint8Array,grant:NamedEditGrant):{key:Uint8Array;receipt:NamedEditReceipt}|undefined{if(grant.maximumItems<1||grant.maximumCopyBytes<64||grant.maximumDepth<1)return;return{key:key(),receipt:{copiedItems:1,copiedBytes:64,retainedCapacityBytes:0}};}

/** 🔑️ Exact UTF8 key admission retains only coordinate directories and borrows original key bytes. */
export class NamedEditPlan{
 readonly fixedCopyBytes=1024;
 private readonly directories:[number[],number[],number[],number[]]=[[],[],[],[]];
 private readonly coordinates:number[]=[];
 private indexed:IndexedEditPlan|undefined;
 private phase=0;
 private cursor=0;
 private low=0;
 private high=0;
 private searching=false;
 private closing=false;
 private offset=0;
 private sort=new HeapSortCursor(0);
 private baseActions=0;
 fault:string|null=null;
 constructor(readonly shape:NamedEditShape,private readonly baseIndex?:NamedEditAdmittedBase){if(Object.values(shape).some(n=>!Number.isSafeInteger(n)||n<0))throw new RangeError("named cardinalities must be exact and nonnegative");if(baseIndex){if(baseIndex.length!==shape.baseLen)throw Error("admitted original named cardinality changed");this.phase=4;}}
 get baseAdmissionActions():number{return this.baseActions;}
 private get baseOrder():readonly number[]{return this.baseIndex?.order??this.directories[0];}
 takeBaseOrder():number[]{if(!this.ready||this.baseIndex||this.shape.removed+this.shape.modified+this.shape.added!==0)throw Error("original base directory is not transferable");const order=this.directories[0];this.directories[0]=[];return order;}
 get settled():boolean{return this.phase===15;}
 get ready():boolean{return this.settled&&this.fault===null&&!this.closing;}
 get plan():IndexedEditPlan{if(!this.ready||!this.indexed?.ready)throw Error("named plan is not admitted");return this.indexed;}
 modifiedIndex(ordinal:number):number{if(!this.ready||this.directories[3][ordinal]===undefined)throw Error("modified ordinal is not admitted");return this.directories[3][ordinal]!;}
 /** 📐️ Quotes an original ordinal and fixed owned coordinate directory observation. */
 modifiedIndexDemand(ordinal:number):NamedEditDemand{if(!this.ready||!Number.isSafeInteger(ordinal)||ordinal<0||ordinal>=this.shape.modified||this.directories[3][ordinal]===undefined)throw Error("modified ordinal is not admitted");return{copyBytes:74,capacityBytes:0,depth:1};}
 /** 👁️ Reveals an admitted original coordinate only under its observation currencies. */
 observeModifiedIndex(ordinal:number,grant:NamedEditGrant):{index:number;receipt:NamedEditReceipt}|undefined{const demand=this.modifiedIndexDemand(ordinal);if(grant.maximumItems<1||grant.maximumCopyBytes<demand.copyBytes||grant.maximumDepth<demand.depth)return;return{index:this.modifiedIndex(ordinal),receipt:{copiedItems:1,copiedBytes:demand.copyBytes,retainedCapacityBytes:0}};}
 private keyCoordinates():readonly [readonly [NamedEditLane,number],readonly [NamedEditLane,number]]|undefined{
  if(this.phase===2||this.phase===12){const work=this.sort.next();if(work.kind!=="compare")return;const lane=this.phase===2?"base":"added",order=this.phase===2?this.baseOrder:this.directories[1];return[[lane,order[work.left]!],[lane,order[work.right]!]];}
  if(this.phase===3||this.phase===13){const lane=this.phase===3?"base":"added",order=this.phase===3?this.baseOrder:this.directories[1],count=this.phase===3?this.shape.baseLen:this.shape.added;if(this.cursor<count)return[[lane,order[this.cursor-1]!],[lane,order[this.cursor]!]];}
  if((this.phase===5||this.phase===7||this.phase===14)&&this.searching&&this.low<this.high)return[[this.phase===5?"removed":this.phase===7?"modified":"added",this.cursor],["base",this.baseOrder[this.low+Math.floor((this.high-this.low)/2)]!]];
 }
 demand(source:NamedEditSource,body:number):NamedEditDemand{
  if(this.closing||this.settled||(["baseLen","removed","modified","added"]as const).some(key=>source.shape[key]!==this.shape[key]))throw Error("named original shape changed");
  if(this.baseIndex){this.baseIndex.checkSource(source);if(source.admittedBase!==this.baseIndex)throw Error("original admitted key directory changed");}
  const count=this.phase===0?this.shape.baseLen:this.phase===4?this.shape.removed:this.phase===6?this.shape.modified:this.phase===10||this.phase===8?this.shape.added:0;
  const pair=this.keyCoordinates();let copyBytes=this.fixedCopyBytes,depth=1;
  if(pair){const a=source.keyDemand(...pair[0]),b=source.keyDemand(...pair[1]);copyBytes+=a.copyBytes+b.copyBytes+64;copyBytes+=2*Math.min(32768,Math.max(2,body-copyBytes)/2|0);depth=Math.max(a.depth,b.depth)+1;}
  if(this.phase===9){copyBytes+=128;depth=2;}
  return{copyBytes,capacityBytes:count*8,depth};
 }
 advance(source:NamedEditSource,grant:NamedEditGrant):NamedEditReceipt{
  const demand=this.demand(source,grant.maximumCopyBytes);if(grant.maximumItems<1||grant.maximumCopyBytes<demand.copyBytes||grant.maximumCapacityBytes<demand.capacityBytes||grant.maximumDepth<demand.depth)return zero();
  const receipt={copiedItems:1,copiedBytes:demand.copyBytes,retainedCapacityBytes:demand.capacityBytes};
  if(this.phase<=3)this.baseActions++;
  if(this.phase===9){this.indexed!.step();if(this.indexed!.settled){if(this.indexed!.fault)this.refuse("coordinates:"+this.indexed!.fault);else this.phase=10;}return receipt;}
  const coordinates=this.keyCoordinates();let observation:-1|0|1|undefined;
  if(coordinates){const a=source.observeKey(...coordinates[0],grant),b=source.observeKey(...coordinates[1],grant);if(!a||!b)throw Error("funded original key observation disappeared");const pair=[a.key,b.key]as const,limit=Math.min(pair[0].length,pair[1].length),page=(demand.copyBytes-this.fixedCopyBytes-64-a.receipt.copiedBytes-b.receipt.copiedBytes)/2;if(this.offset>limit)throw Error("original partial key changed");const end=Math.min(limit,this.offset+page);let order:-1|0|1=0;for(let i=this.offset;i<end;i++){if(pair[0][i]!==pair[1][i]){order=pair[0][i]!<pair[1][i]!?-1:1;break;}}receipt.copiedBytes=this.fixedCopyBytes+64+a.receipt.copiedBytes+b.receipt.copiedBytes+2*(end-this.offset);this.offset=end;if(order!==0||end===limit){observation=order===0?pair[0].length<pair[1].length?-1:pair[0].length>pair[1].length?1:0:order;this.offset=0;}}
  switch(this.phase){
   case 0:case 4:case 6:case 10:this.cursor=0;this.searching=false;this.phase++;break;
   case 1:case 11:{const count=this.phase===1?this.shape.baseLen:this.shape.added;if(this.cursor===count){this.sort=new HeapSortCursor(count);this.phase++;}else this.directories[this.phase===1?0:1].push(this.cursor++);break;}
   case 2:case 12:{const work=this.sort.next();if(work.kind==="complete"){this.cursor=1;this.phase++;}else if(work.kind==="compare"){if(observation!==undefined)this.sort.advance(observation);}else{if(work.kind==="swap"){const rows=this.directories[this.phase===2?0:1];[rows[work.left],rows[work.right]]=[rows[work.right]!,rows[work.left]!];}this.sort.advance();}break;}
   case 3:case 13:{const count=this.phase===3?this.shape.baseLen:this.shape.added;if(this.cursor>=count){this.cursor=0;this.searching=false;this.phase++;}else if(observation!==undefined){if(observation===0)this.refuse(this.phase===3?"duplicate-base":"duplicate-added");else this.cursor++;}break;}
   case 5:case 7:case 14:{const count=this.phase===5?this.shape.removed:this.phase===7?this.shape.modified:this.shape.added;if(this.cursor===count){this.cursor=0;this.phase++;}else if(!this.searching){this.low=0;this.high=this.shape.baseLen;this.searching=true;}else if(this.low===this.high){if(this.phase!==14)this.refuse(this.phase===5?"missing-remove":"missing-modify");else{this.cursor++;this.searching=false;}}else if(observation!==undefined){const middle=this.low+Math.floor((this.high-this.low)/2);if(observation<0)this.high=middle;else if(observation>0)this.low=middle+1;else{const index=this.baseOrder[middle]!;if(this.phase===5)this.directories[2].push(index);else if(this.phase===7)this.directories[3].push(index);else{const removed=this.indexed!.entries(0);let low=0,high=removed.length;while(low<high){const middle=low+Math.floor((high-low)/2);if(removed[middle]!.idx<index)low=middle+1;else high=middle;}if(removed[low]?.idx!==index){this.refuse("retained-added");break;}}this.cursor++;this.searching=false;}}break;}
   case 8:this.cursor=0;this.phase=16;break;
   case 16:if(this.cursor<this.shape.added)this.coordinates.push(source.addedIndex(this.cursor++));else{this.indexed=new IndexedEditPlan(this.shape.baseLen,this.directories[2],this.directories[3],this.coordinates);this.phase=9;}break;
   default:throw Error("named frontier is invalid");
  }
  return receipt;
 }
 private refuse(fault:string):void{this.fault=fault;this.phase=15;}
 beginClose():void{this.closing=true;this.indexed?.beginClose();}
 get terminalIsEmpty():boolean{return this.closing&&!this.indexed&&this.directories.every(rows=>rows.length===0)&&this.coordinates.length===0;}
 closeDemand(_body:number):NamedEditDemand{if(!this.closing)throw Error("named closure is not admitted");return{copyBytes:this.terminalIsEmpty?0:this.fixedCopyBytes+128,capacityBytes:0,depth:this.terminalIsEmpty?0:2};}
 closeStep(grant:NamedEditGrant):NamedEditReceipt{
  const demand=this.closeDemand(grant.maximumCopyBytes);if(this.terminalIsEmpty)return zero();if(grant.maximumItems<1||grant.maximumCopyBytes<demand.copyBytes||grant.maximumDepth<demand.depth)return zero();
  if(this.indexed){if(this.indexed.terminalIsEmpty)this.indexed=undefined;else this.indexed.closeStep();}
  else{const rows=this.directories.find(rows=>rows.length>0)??this.coordinates;rows.pop();}
  return{copiedItems:1,copiedBytes:demand.copyBytes,retainedCapacityBytes:0};
 }
}
