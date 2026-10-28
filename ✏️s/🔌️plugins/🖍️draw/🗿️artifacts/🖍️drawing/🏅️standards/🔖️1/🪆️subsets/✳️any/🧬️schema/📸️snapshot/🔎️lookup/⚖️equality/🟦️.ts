/** ⚖️ Native field readers lend actual immutable roots and constant-time schema ordinals. */
export interface DrawingFieldTreeSource {
 readonly identity:object;
 kind(path:Int32Array,depth:number):'null'|'boolean'|'number'|'text'|'array'|'object';
 scalar(path:Int32Array,depth:number):null|boolean|number|string;
 count(path:Int32Array,depth:number):number;
 key(path:Int32Array,depth:number,index:number):string;
}
export interface DrawingEqualityGrant {items:number;copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface DrawingEqualityReceipt {items:number;copyBytes:number;capacityBytes:number;releaseBytes:number}
const zero=():DrawingEqualityReceipt=>({items:0,copyBytes:0,capacityBytes:0,releaseBytes:0});
/** 🧳️ Logical typed-array custody retains the original source pair through one field quantum per turn. */
export class DrawingOriginalEquality {
 static readonly capacityBytes=64*4*3+64;
 static readonly copyBytes=DrawingOriginalEquality.capacityBytes*2+256;
 private path=new Int32Array(64);private count=new Int32Array(64);private index=new Int32Array(64);private object=new Uint8Array(64);
 private depth=0;private phase=0;private offset=0;private left?:DrawingFieldTreeSource;private right?:DrawingFieldTreeSource;private leftIdentity?:object;private rightIdentity?:object;private settled?:boolean;private closing=false;private released=false;
 private constructor(){}
 static admit(g:DrawingEqualityGrant):[DrawingOriginalEquality,DrawingEqualityReceipt]|undefined{if(g.items<1||g.depth<1||g.capacityBytes<this.capacityBytes||g.copyBytes<this.copyBytes)return;return[new DrawingOriginalEquality(),{items:1,copyBytes:this.copyBytes,capacityBytes:this.capacityBytes,releaseBytes:0}]}
 private accepts(g:DrawingEqualityGrant,copy:number,release=0):boolean{return g.items>0&&g.depth>0&&g.copyBytes>=copy&&g.releaseBytes>=release}
 demand():DrawingEqualityReceipt{return{items:1,copyBytes:DrawingOriginalEquality.copyBytes+4,capacityBytes:0,releaseBytes:0}}
 advance(l:DrawingFieldTreeSource,r:DrawingFieldTreeSource,g:DrawingEqualityGrant):DrawingEqualityReceipt{
  if(this.closing||this.released)throw Error('Drawing equality closed');if(this.left&&(this.left!==l||this.right!==r||this.leftIdentity!==l.identity||this.rightIdentity!==r.identity))throw Error('Drawing equality original source changed');if(!this.accepts(g,this.demand().copyBytes))return zero();if(!this.left){this.left=l;this.right=r;this.leftIdentity=l.identity;this.rightIdentity=r.identity;return this.demand()}if(this.settled!==undefined)return zero();
  if(this.phase===3){if(this.depth===0)this.settled=true;else{--this.depth;if(++this.index[this.depth]<this.count[this.depth]){this.path[this.depth]=this.index[this.depth];if(this.object[this.depth])this.phase=1;else{++this.depth;this.phase=0}}}return this.demand()}
  if(this.phase===1||this.phase===2){const a=this.phase===1?l.key(this.path,this.depth,this.index[this.depth]):l.scalar(this.path,this.depth);const b=this.phase===1?r.key(this.path,this.depth,this.index[this.depth]):r.scalar(this.path,this.depth);if(typeof a!=='string'||typeof b!=='string')throw Error('Drawing equality field type changed');if(this.offset===a.length||this.offset===b.length){if(a.length!==b.length)this.settled=false;else{this.offset=0;if(this.phase===1){this.path[this.depth]=this.index[this.depth];++this.depth;this.phase=0}else this.phase=3}}else if(a.charCodeAt(this.offset++)!==b.charCodeAt(this.offset-1))this.settled=false;return this.demand()}
  const kind=l.kind(this.path,this.depth);if(kind!==r.kind(this.path,this.depth)){this.settled=false;return this.demand()}if(kind==='text'){this.phase=2;return this.demand()}if(kind==='array'||kind==='object'){const count=l.count(this.path,this.depth);if(count!==r.count(this.path,this.depth)){this.settled=false;return this.demand()}if(count===0){this.phase=3;return this.demand()}if(this.depth+1>=64)throw Error('Drawing equality declared depth');this.count[this.depth]=count;this.index[this.depth]=0;this.path[this.depth]=0;this.object[this.depth]=kind==='object'?1:0;if(kind==='object')this.phase=1;else ++this.depth;return this.demand()}if(l.scalar(this.path,this.depth)!==r.scalar(this.path,this.depth))this.settled=false;else this.phase=3;return this.demand()
 }
 result():boolean|undefined{return this.settled}
 beginClose():void{this.closing=true}
 close(g:DrawingEqualityGrant):DrawingEqualityReceipt{if(!this.closing)throw Error('Drawing equality closure not started');if(this.released)return zero();if(!this.accepts(g,DrawingOriginalEquality.copyBytes,DrawingOriginalEquality.capacityBytes))return zero();this.left=undefined;this.right=undefined;this.leftIdentity=undefined;this.rightIdentity=undefined;this.path=new Int32Array(0);this.count=new Int32Array(0);this.index=new Int32Array(0);this.object=new Uint8Array(0);this.released=true;return{items:1,copyBytes:DrawingOriginalEquality.copyBytes,capacityBytes:0,releaseBytes:DrawingOriginalEquality.capacityBytes}}
 terminalIsEmpty():boolean{return this.released}
}
