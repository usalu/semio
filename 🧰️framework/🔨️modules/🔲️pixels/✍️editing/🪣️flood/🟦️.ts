export type FloodSource=Uint8Array|readonly[number,number,number,number];
export interface FloodGrant{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}
export interface FloodDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface FloodReceipt{copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number}
export interface FloodProgress{work:number;completed:number;total:number;done:boolean;progress:FloodReceipt}
export interface FloodCloseProgress{done:boolean;progress:FloodReceipt}
export interface FloodError{kind:"invalidValue"|"canceled"|"ownershipLimit"|"allocationFailed";message:string;progress:FloodReceipt}
export interface FloodOutcome<T>{work:number;result:{ok:true;value:T}|{ok:false;error:FloodError}}
const metadataBytes=512;
const receipt=():FloodReceipt=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
const emptyDemand=():FloodDemand=>({copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0});
const admits=(grant:FloodGrant,demand:FloodDemand):boolean=>Object.values(grant).every(value=>Number.isSafeInteger(value)&&value>=0)&&grant.maximumItems>=1&&grant.maximumCopyBytes>=demand.copyBytes&&grant.maximumCapacityBytes>=demand.capacityBytes&&grant.maximumReleaseBytes>=demand.releaseBytes&&grant.maximumDepth>=demand.depth;

export interface FloodMask{readonly length:number;at(index:number):number;[Symbol.iterator]():IterableIterator<number>}
export const FLOOD_PAGE_PIXELS=4096;
const referenceBytes=8;
class FloodPages{
 private pages?:Array<Uint8Array|Uint32Array|undefined>;
 private allocated=0;
 readonly pageCount:number;
 constructor(private readonly count:number,private readonly wordBytes:1|4){this.pageCount=Math.ceil(count/FLOOD_PAGE_PIXELS);}
 get directoryHeld():boolean{return this.pages!==undefined;}
 get complete():boolean{return this.directoryHeld&&this.allocated===this.pageCount;}
 get held():boolean{return this.directoryHeld;}
 get allocationBytes():number{return !this.pages?this.pageCount*referenceBytes:Math.min(FLOOD_PAGE_PIXELS,this.count-this.allocated*FLOOD_PAGE_PIXELS)*this.wordBytes;}
 allocate():number{
  if(!this.pages){this.pages=new Array(this.pageCount);return this.pageCount*referenceBytes;}
  const length=Math.min(FLOOD_PAGE_PIXELS,this.count-this.allocated*FLOOD_PAGE_PIXELS);
  const page=this.wordBytes===1?new Uint8Array(length):new Uint32Array(length);
  this.pages[this.allocated++]=page;
  return page.byteLength;
 }
 get(index:number):number{return this.pages![Math.floor(index/FLOOD_PAGE_PIXELS)]![index%FLOOD_PAGE_PIXELS]!;}
 set(index:number,value:number):void{this.pages![Math.floor(index/FLOOD_PAGE_PIXELS)]![index%FLOOD_PAGE_PIXELS]=value;}
 get releaseBytes():number{return this.allocated?this.pages![this.allocated-1]!.byteLength:this.pages?this.pageCount*referenceBytes:0;}
 release():number{
  const bytes=this.releaseBytes;
  if(this.allocated)this.pages![--this.allocated]=undefined;
  else this.pages=undefined;
  return bytes;
 }
}

/** 🪣️ Bounded four-connected RGBA workspace uses logical managed-memory receipts, not VM heap or GC measurements. */
export class FloodRegionCursor{
 private readonly count:number;
 private readonly seed:number;
 private sourceBuffer?:ArrayBufferLike;
 private sourceOffset=0;
 private sourceLength=0;
 private sourceBound=false;
 private readonly seen:FloodPages;
 private readonly mask:FloodPages;
 private readonly queue:FloodPages;
 private red=0;
 private green=0;
 private blue=0;
 private alpha=0;
 private uniformIdentity?:number;
 private initialized=0;
 private head=0;
 private tail=0;
 private phase:"allocate"|"initialize"|"flood"|"done"|"close"|"closed"="allocate";
 private cancelled=false;
 constructor(private readonly width:number,private readonly height:number,x:number,y:number){
  this.count=width*height;this.seed=y*width+x;
  if(![width,height,x,y].every(Number.isSafeInteger)||width<1||height<1||width>16384||height>16384||this.count>16*1024*1024||x<0||y<0||x>=width||y>=height)throw Error("Invalid flood extent or seed");
  this.seen=new FloodPages(this.count,1);this.mask=new FloodPages(this.count,1);this.queue=new FloodPages(this.count,4);
 }
 get done():boolean{return this.phase==="done"&&!this.cancelled;}
 get closed():boolean{return this.phase==="closed";}
 get workspaceBytes():number{return this.count*6+this.seen.pageCount*referenceBytes*3;}
 private allocationTarget():FloodPages{
  if(!this.seen.directoryHeld)return this.seen;if(!this.mask.directoryHeld)return this.mask;if(!this.queue.directoryHeld)return this.queue;
  if(!this.seen.complete)return this.seen;if(!this.mask.complete)return this.mask;return this.queue;
 }
 demands(maximumWork:number):FloodDemand{
  if(!Number.isSafeInteger(maximumWork)||maximumWork<1||maximumWork>65536)throw Error("Invalid flood work budget");
  if(this.phase==="allocate")return{copyBytes:metadataBytes+referenceBytes,capacityBytes:this.allocationTarget().allocationBytes,releaseBytes:0,depth:1};
  if(this.phase==="initialize")return{copyBytes:metadataBytes+Math.min(maximumWork,this.count-this.initialized)*2+5,capacityBytes:0,releaseBytes:0,depth:1};
  if(this.phase==="flood")return{copyBytes:metadataBytes+Math.min(maximumWork,this.count-this.head)*32,capacityBytes:0,releaseBytes:0,depth:1};
  return emptyDemand();
 }
 private failure(kind:FloodError["kind"],message:string,progress:FloodReceipt,work=0):FloodOutcome<never>{return{work,result:{ok:false,error:{kind,message,progress}}};}
 private step(work:number,progress:FloodReceipt):FloodOutcome<FloodProgress>{return{work,result:{ok:true,value:{work,completed:this.initialized+this.head,total:this.count*2,done:this.done,progress}}};}
 private enqueue(index:number,progress:FloodReceipt):void{
  if(!this.seen.get(index)){this.seen.set(index,1);this.queue.set(this.tail++,index);progress.copiedBytes+=5;}
 }
 advance(source:FloodSource,maximumWork:number,grant:FloodGrant,cancelled=false):FloodOutcome<FloodProgress>{
  const progress=receipt();if(cancelled)this.cancelled=true;
  if(this.cancelled)return this.failure("canceled","Flood cancelled",progress);
  if(this.phase==="close"||this.closed)return this.failure("invalidValue","Flood is closing",progress);
  if(!Number.isSafeInteger(maximumWork)||maximumWork<1||maximumWork>65536)return this.failure("invalidValue","Invalid flood work budget",progress);
  const demand=this.demands(maximumWork);if(!admits(grant,demand))return this.step(0,progress);
  const rgba=source instanceof Uint8Array;
  if(rgba?source.byteLength!==this.count*4:source.length!==4||!source.every(value=>Number.isInteger(value)&&value>=0&&value<=255))return this.failure("invalidValue","Invalid flood source extent",progress);
  const uniform=rgba?undefined:(source[0]|source[1]<<8|source[2]<<16|source[3]<<24)>>>0;
  if(this.sourceBound&&(rgba?source.buffer!==this.sourceBuffer||source.byteOffset!==this.sourceOffset||source.byteLength!==this.sourceLength:this.sourceBuffer!==undefined||uniform!==this.uniformIdentity))return this.failure("invalidValue","Original flood source changed",progress);
  let work=0;
  if(this.phase==="allocate"){
   const offset=rgba?this.seed*4:0;this.sourceBuffer=rgba?source.buffer:undefined;this.sourceOffset=rgba?source.byteOffset:0;this.sourceLength=rgba?source.byteLength:0;this.uniformIdentity=uniform;this.red=source[offset]!;this.green=source[offset+1]!;this.blue=source[offset+2]!;this.alpha=source[offset+3]!;this.sourceBound=true;work=1;progress.copiedItems=1;progress.copiedBytes=metadataBytes+referenceBytes;
   try{
    progress.retainedCapacityBytes=this.allocationTarget().allocate();
    if(this.queue.complete)this.phase="initialize";
   }catch{return this.failure("allocationFailed","Flood allocation refused",progress,work);}
   if(progress.retainedCapacityBytes>demand.capacityBytes)return this.failure("ownershipLimit","Flood allocation exceeds its quoted capacity",progress,work);
  }else if(this.phase==="initialize"){
   const end=Math.min(this.count,this.initialized+maximumWork);
   while(this.initialized<end){this.seen.set(this.initialized,0);this.mask.set(this.initialized++,0);work++;progress.copiedBytes+=2;}
   if(this.initialized===this.count){this.queue.set(this.tail++,this.seed);this.seen.set(this.seed,1);this.phase="flood";progress.copiedBytes+=5;}
  }else if(this.phase==="flood"){
   while(work<maximumWork&&this.head<this.tail){
    const index=this.queue.get(this.head++),offset=rgba?index*4:0;work++;progress.copiedBytes+=8;
    if(source[offset]!==this.red||source[offset+1]!==this.green||source[offset+2]!==this.blue||source[offset+3]!==this.alpha)continue;
    this.mask.set(index,255);progress.copiedBytes++;const x=index%this.width,y=Math.floor(index/this.width);
    if(x>0)this.enqueue(index-1,progress);if(x+1<this.width)this.enqueue(index+1,progress);if(y>0)this.enqueue(index-this.width,progress);if(y+1<this.height)this.enqueue(index+this.width,progress);
   }
   if(this.head===this.tail)this.phase="done";
  }
  if(work&&progress.copiedItems===0){progress.copiedItems=1;progress.copiedBytes+=metadataBytes;}
  return this.step(work,progress);
 }
 result():FloodMask{if(!this.done)throw Error("Flood result is unpublished");return this;}
 get length():number{if(!this.done)throw Error("Flood result is unpublished");return this.count;}
 at(index:number):number{if(!this.done)throw Error("Flood result is unpublished");if(!Number.isSafeInteger(index)||index<0||index>=this.count)throw Error("Invalid flood result index");return this.mask.get(index);}
 *[Symbol.iterator]():IterableIterator<number>{for(let index=0;index<this.length;index++)yield this.at(index);}
 beginClose():void{if(!this.closed)this.phase="close";}
 closeDemands(maximumCopyBytes:number):FloodDemand{
  if(this.phase!=="close"&&!this.closed)throw Error("Flood close has not begun");
  if(!Number.isSafeInteger(maximumCopyBytes)||maximumCopyBytes<0)throw Error("Invalid flood close budget");
  const storage=this.queue.held?this.queue:this.mask.held?this.mask:this.seen.held?this.seen:undefined;
  return storage?{copyBytes:metadataBytes+referenceBytes,capacityBytes:0,releaseBytes:storage.releaseBytes,depth:1}:emptyDemand();
 }
 close(grant:FloodGrant):FloodOutcome<FloodCloseProgress>{
  const progress=receipt();
  if(this.phase!=="close"&&!this.closed)return this.failure("invalidValue","Flood close has not begun",progress);
  if(this.closed)return{work:0,result:{ok:true,value:{done:true,progress}}};
  const demand=this.closeDemands(grant.maximumCopyBytes);
  if(!admits(grant,demand))return{work:0,result:{ok:true,value:{done:false,progress}}};
  if(this.queue.held)progress.releasedBytes=this.queue.release();
  else if(this.mask.held)progress.releasedBytes=this.mask.release();
  else if(this.seen.held)progress.releasedBytes=this.seen.release();
  const work=progress.releasedBytes?1:0;if(work){progress.copiedItems=1;progress.copiedBytes=metadataBytes+referenceBytes;}
  if(!this.queue.held&&!this.mask.held&&!this.seen.held){this.sourceBuffer=undefined;this.sourceBound=false;this.uniformIdentity=undefined;this.phase="closed";}
  return{work,result:{ok:true,value:{done:this.closed,progress}}};
 }
}
