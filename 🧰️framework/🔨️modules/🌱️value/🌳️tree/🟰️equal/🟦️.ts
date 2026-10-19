/** 🟰️ Typed traversal preserves variant, record, ordered collection, and native floating-point semantics. */
export type Node={kind:"bool";value:boolean}|{kind:"u64"|"i64";value:bigint}|{kind:"f64";value:number}|{kind:"text"|"bytes";value:Uint8Array}|{kind:"sequence";items:Node[]}|{kind:"record";fields:{name:Uint8Array;value:Node}[]}|{kind:"variant";tag:Uint8Array;value:Node}|{kind:"optional";value:Node|null};
export type Grant={items:number;copy:number;capacity:number;release:number;depth:number};
export type Demand={copy:number;capacity:number;release:number;depth:number};
export type Step={equal:boolean|null;copy:number;capacity:number;release:number};
type Frame={left:Node;right:Node;phase:number;index:number;offset:number};
/** 🧳️ Sources remain caller-owned; logical cursor allocations are released one funded frame at a time. */
export class RetainedTypedEquality{
 private left:Node|null=null;private right:Node|null=null;private frames:Frame[]=[];private started=false;private closing=false;private result:boolean|null=null;
 constructor(readonly wordBytes:4|8){}
 private get fixed(){return this.wordBytes*12;}
 private get frameBytes(){return this.wordBytes*6;}
 private verify(left:Node,right:Node){if(this.left!==null&&(left!==this.left||right!==this.right))throw Error("foreign original typed source");}
 value(){return this.result;}
 demand(left:Node,right:Node,body:number):Demand{
  this.verify(left,right);if(this.closing||this.result!==null)throw Error("settled typed comparison");
  if(!this.started)return{copy:this.fixed+this.frameBytes,capacity:this.frameBytes,release:0,depth:1};
  const frame=this.frames.at(-1)!;if(frame.phase===2)return{copy:this.fixed,capacity:0,release:this.frameBytes,depth:1};
  const pair=this.pair(frame);if(pair===null)return{copy:this.fixed,capacity:0,release:0,depth:1};
  if(pair[0].kind==="text"&&pair[1].kind==="text"||pair[0].kind==="bytes"&&pair[1].kind==="bytes")return{copy:this.fixed+2,capacity:0,release:0,depth:1};
  return{copy:this.fixed+this.frameBytes,capacity:this.frameBytes,release:0,depth:1};
 }
 private pair(frame:Frame):[Node,Node]|null{
  const{left:l,right:r}=frame;if(l.kind!==r.kind)return null;
  if(l.kind==="text"||l.kind==="bytes")return[l,r];
  if(l.kind==="sequence"&&r.kind==="sequence")return l.items.length===r.items.length&&frame.index<l.items.length?[l.items[frame.index]!,r.items[frame.index]!]:null;
  if(l.kind==="record"&&r.kind==="record"){if(l.fields.length!==r.fields.length||frame.index>=l.fields.length*2)return null;const a=l.fields[frame.index>>1]!,b=r.fields[frame.index>>1]!;return frame.index%2===0?[{kind:"text",value:a.name},{kind:"text",value:b.name}]:[a.value,b.value];}
  if(l.kind==="variant"&&r.kind==="variant")return frame.index===0?[{kind:"text",value:l.tag},{kind:"text",value:r.tag}]:frame.index===1?[l.value,r.value]:null;
  if(l.kind==="optional"&&r.kind==="optional")return frame.index===0&&l.value!==null&&r.value!==null?[l.value,r.value]:null;
  return null;
 }
 advance(left:Node,right:Node,g:Grant):Step{
  const d=this.demand(left,right,g.copy);if(g.items<1||g.copy<d.copy||g.capacity<d.capacity||g.release<d.release||g.depth<d.depth)return{equal:null,copy:0,capacity:0,release:0};
  const p={equal:null as boolean|null,copy:d.copy,capacity:d.capacity,release:d.release};
  if(!this.started){this.left=left;this.right=right;this.started=true;this.frames.push({left,right,phase:0,index:0,offset:0});return p;}
  const frame=this.frames.at(-1)!;if(frame.phase===2){this.frames.pop();if(this.frames.length===0)this.result??=true;else this.frames.at(-1)!.index++;p.equal=this.result;return p;}
  const l=frame.left,r=frame.right;if(l.kind!==r.kind){this.result=false;return{...p,equal:false};}
  if(l.kind==="sequence"&&r.kind==="sequence"&&l.items.length!==r.items.length||l.kind==="record"&&r.kind==="record"&&l.fields.length!==r.fields.length||l.kind==="optional"&&r.kind==="optional"&&(l.value===null)!==(r.value===null)){this.result=false;return{...p,equal:false};}
  const pair=this.pair(frame);
  if(pair===null){if("value"in l&&"value"in r&&(l.kind==="bool"||l.kind==="u64"||l.kind==="i64"||l.kind==="f64")&&l.value!==r.value){this.result=false;return{...p,equal:false};}frame.phase=2;return p;}
  if((pair[0].kind==="text"&&pair[1].kind==="text")||(pair[0].kind==="bytes"&&pair[1].kind==="bytes")){
   const a=pair[0].value,b=pair[1].value;if(a.length!==b.length){this.result=false;return{...p,equal:false};}
   const count=Math.min(a.length-frame.offset,Math.floor((g.copy-this.fixed)/2),4096);let read=0;for(let at=frame.offset;at<frame.offset+count;at++){read+=2;if(a[at]!==b[at]){this.result=false;break;}}frame.offset+=count;p.copy=this.fixed+read;
   if(this.result===false)p.equal=false;else if(frame.offset===a.length){frame.offset=0;if(l.kind==="text"||l.kind==="bytes")frame.phase=2;else frame.index++;}return p;
  }
  this.frames.push({left:pair[0],right:pair[1],phase:0,index:0,offset:0});return p;
 }
 beginClose(){this.closing=true;}
 closeDemand():Demand{return{copy:this.frames.length?this.fixed:0,capacity:0,release:this.frames.length?this.frameBytes:0,depth:this.frames.length?1:0};}
 close(g:Grant):Step{if(!this.closing)throw Error("typed comparison close not begun");const d=this.closeDemand();if(this.frames.length&&(g.items<1||g.copy<d.copy||g.release<d.release||g.depth<d.depth))return{equal:null,copy:0,capacity:0,release:0};this.frames.pop();if(this.frames.length===0){this.left=null;this.right=null;}return{equal:null,copy:d.copy,capacity:0,release:d.release};}
 empty(){return this.frames.length===0&&this.left===null&&this.right===null;}
}
