/** 🔍️ Pixel-cell contours with bounded grants, topology checks and private publication. */
import {UnitRetirement,type WorkRetirement,type WorkRetirementProgress} from "../🧹️retire/🟦️.ts";
import type {PathSegment,Vec2} from "../🟦️.ts";
export type BitmapTraceInput={width:number;height:number;mask:readonly number[]|Uint8Array;threshold:number;simplifyEpsilon:number;maxPixels:number;maxEdges:number;maxSegments:number;maxWork:number};
export type BitmapTraceProgress={phase:"scan"|"contours"|"compact"|"simplify"|"topology"|"coverage"|"emit"|"complete";scanned:number;pixels:number;edges:number;contours:number;segments:number;simplified:boolean;work:number;done:boolean};
export type BitmapTraceOptions={workBudget?:number;signal?:AbortSignal;onProgress?:(progress:BitmapTraceProgress)=>void};
export type BitmapTraceRetirementProgress=WorkRetirementProgress;
export type BitmapTraceRetirement=WorkRetirement;

type Edge={a:Vec2;b:Vec2;direction:number;used:boolean};
type Range={a:number;b:number;at:number;distance:number;index:number};
const invalid=(message:string):never=>{throw new RangeError(message);};
const integer=(n:number,max:number)=>Number.isSafeInteger(n)&&n>=1&&n<=max;
const cross=(a:Vec2,b:Vec2,c:Vec2)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
const equal=(a:Vec2,b:Vec2)=>a[0]===b[0]&&a[1]===b[1];
const between=(a:Vec2,b:Vec2,p:Vec2)=>cross(a,b,p)===0&&p[0]>=Math.min(a[0],b[0])&&p[0]<=Math.max(a[0],b[0])&&p[1]>=Math.min(a[1],b[1])&&p[1]<=Math.max(a[1],b[1]);
function overlap(a:Vec2,b:Vec2,c:Vec2,d:Vec2):boolean {
 const p=cross(a,b,c),q=cross(a,b,d),r=cross(c,d,a),s=cross(c,d,b);
 if(p*q<0&&r*s<0)return true;
 if(p===0&&q===0){const axis=Math.abs(b[0]-a[0])>=Math.abs(b[1]-a[1])?0:1;return Math.min(Math.max(a[axis],b[axis]),Math.max(c[axis],d[axis]))>Math.max(Math.min(a[axis],b[axis]),Math.min(c[axis],d[axis]));}
 return (p===0&&between(a,b,c)&&!equal(c,a)&&!equal(c,b))||(q===0&&between(a,b,d)&&!equal(d,a)&&!equal(d,b))||(r===0&&between(c,d,a)&&!equal(a,c)&&!equal(a,d))||(s===0&&between(c,d,b)&&!equal(b,c)&&!equal(b,d));
}
function distance(p:Vec2,a:Vec2,b:Vec2):number {
 const dx=b[0]-a[0],dy=b[1]-a[1],length=dx*dx+dy*dy,t=length===0?0:Math.max(0,Math.min(1,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length));
 return Math.hypot(p[0]-a[0]-t*dx,p[1]-a[1]-t*dy);
}
/** ⏱️ Trace a stable byte snapshot; each grant visits one pixel, edge, vertex or validation pair. */
export class BitmapTraceJob {
 private phase:BitmapTraceProgress["phase"]="scan";private scanned=0;private work=0;private cancelled=false;private transferred=false;private failed:unknown=null;private simplified=false;
 private edges:Edge[]=[];private outgoing=new Map<number,number[]>();private raw:Vec2[][]=[];private base:Vec2[][]=[];private candidate:Vec2[][]=[];private flat:{a:Vec2;b:Vec2}[]=[];
 private first=0;private current=-1;private ring:Vec2[]=[];private positions=new Map<number,number>();private split:Vec2[]|null=null;private splitAt=0;private splitStop=0;private splitCopy=true;private start:Vec2=[0,0];private contour=0;private at=0;private anchor=0;private farthest=0;
 private mode:"anchor"|"ranges"|"build"|"edges"="anchor";private kept=new Set<number>();private ranges:Range[]=[];private baseArea=0;private candidateArea=0;private previous:Vec2|null=null;private changed=false;
 private left=0;private right=1;private pixel=0;private edge=0;private winding=0;private changes=new Map<number,number>();private covering=false;private output:PathSegment[]=[];private pixels:number;private threshold:number;private edgeCount=0;
 constructor(private input:BitmapTraceInput) {
  if(!integer(input.width,8192)||!integer(input.height,8192)||!integer(input.maxPixels,16777216)||!integer(input.maxEdges,65536)||!integer(input.maxSegments,65536)||!integer(input.maxWork,1000000000)||!Number.isFinite(input.threshold)||input.threshold<0||input.threshold>1||!Number.isFinite(input.simplifyEpsilon)||input.simplifyEpsilon<0||input.simplifyEpsilon>8192)invalid("Invalid bitmap trace contract");
  this.input={...input};this.pixels=input.width*input.height;this.threshold=Math.round(input.threshold*255);
  if(this.pixels>input.maxPixels||input.mask.length!==this.pixels)invalid("Bitmap trace expects exact dimensions within the pixel budget");
 }
 private on(x:number,y:number):boolean {
  if(x<0||y<0||x>=this.input.width||y>=this.input.height)return false;
  const value=this.input.mask[y*this.input.width+x]!;if(!Number.isInteger(value)||value<0||value>255)invalid("Invalid bitmap trace byte");return value>=this.threshold;
 }
 private key(p:Vec2):number{return p[1]*(this.input.width+1)+p[0];}
 private appendEdge(a:Vec2,b:Vec2,direction:number):void {
  if(this.edges.length>=this.input.maxEdges)invalid("Bitmap trace exceeds edge budget");const index=this.edges.length;this.edges.push({a,b,direction,used:false});this.edgeCount++;const key=this.key(a),next=this.outgoing.get(key)??[];next.push(index);this.outgoing.set(key,next);
 }
 private fallback():void {this.simplified=false;this.phase="emit";this.contour=0;this.at=0;}
 private step():void {
  if(this.phase==="scan"){
   if(this.scanned===this.pixels){this.phase="contours";return;}
   const x=this.scanned%this.input.width,y=Math.floor(this.scanned/this.input.width);this.scanned++;
   if(!this.on(x,y))return;
   if(!this.on(x,y-1))this.appendEdge([x,y],[x+1,y],0);if(!this.on(x+1,y))this.appendEdge([x+1,y],[x+1,y+1],1);if(!this.on(x,y+1))this.appendEdge([x+1,y+1],[x,y+1],2);if(!this.on(x-1,y))this.appendEdge([x,y+1],[x,y],3);return;
  }
  if(this.phase==="contours"){
   if(this.split){
    if(this.splitCopy){if(this.splitAt<this.ring.length){this.split.push(this.ring[this.splitAt++]!);return;}this.splitCopy=false;return;}
    if(this.ring.length>this.splitStop){this.positions.delete(this.key(this.ring.pop()!));return;}
    if(this.split.length<3)invalid("Bitmap trace split contour is degenerate");this.raw.push(this.split);this.split=null;return;
   }
   if(this.current<0){
    if(this.first===this.edges.length){this.phase="compact";this.contour=0;this.at=0;return;}
    const index=this.first++;if(this.edges[index]!.used)return;this.current=index;this.start=this.edges[index]!.a;this.ring=[];this.positions.clear();return;
   }
   const edge=this.edges[this.current]!;if(edge.used)invalid("Bitmap trace contour is not manifold");const key=this.key(edge.a),repeated=this.positions.get(key);
   if(repeated!==undefined){this.split=[];this.splitAt=repeated;this.splitStop=repeated;this.splitCopy=true;return;}
   this.positions.set(key,this.ring.length);edge.used=true;this.ring.push(edge.a);
   if(equal(edge.b,this.start)){this.raw.push(this.ring);this.current=-1;return;}
   let next=-1,priority=5;for(const index of this.outgoing.get(this.key(edge.b))??[]){const candidate=this.edges[index]!;if(candidate.used)continue;const turn=(candidate.direction-edge.direction+4)%4,rank=turn===1?0:turn===0?1:turn===3?2:3;if(rank<priority){next=index;priority=rank;}}
   if(next<0)invalid("Bitmap trace contour is open");this.current=next;return;
  }
  if(this.phase==="compact"){
   if(this.contour===this.raw.length){this.contour=0;this.at=0;this.phase=this.input.simplifyEpsilon>0?"simplify":"emit";return;}
   const raw=this.raw[this.contour]!;if(this.at===0)this.base.push([]);
   const p=raw[this.at]!,previous=raw[(this.at+raw.length-1)%raw.length]!,next=raw[(this.at+1)%raw.length]!;
   if(cross(previous,p,next)!==0)this.base[this.contour]!.push(p);
   this.at++;if(this.at===raw.length){if(this.base[this.contour]!.length<3)invalid("Bitmap trace contour is degenerate");this.contour++;this.at=0;}return;
  }
  if(this.phase==="simplify"){
   if(this.mode==="edges"){
    if(this.contour===this.candidate.length){if(!this.changed){this.fallback();return;}this.simplified=true;this.phase="topology";return;}
    const ring=this.candidate[this.contour]!;this.flat.push({a:ring[this.at]!,b:ring[(this.at+1)%ring.length]!});this.at++;if(this.at===ring.length){this.contour++;this.at=0;}return;
   }
   if(this.contour===this.base.length){this.contour=0;this.at=0;this.mode="edges";return;}
   const ring=this.base[this.contour]!,n=ring.length;
   if(this.mode==="anchor"){
    const p=ring[this.at]!,d=(p[0]-ring[0]![0])**2+(p[1]-ring[0]![1])**2;if(d>this.farthest){this.farthest=d;this.anchor=this.at;}
    this.baseArea+=p[0]*ring[(this.at+1)%n]![1]-ring[(this.at+1)%n]![0]*p[1];this.at++;
    if(this.at===n){this.kept=new Set([0,this.anchor]);this.ranges=[{a:this.anchor,b:n,at:this.anchor+1,distance:0,index:0},{a:0,b:this.anchor,at:1,distance:0,index:0}];this.mode="ranges";}return;
   }
   if(this.mode==="ranges"){
    const range=this.ranges.at(-1);if(!range){this.mode="build";this.at=0;this.candidate.push([]);return;}
    if(range.at<range.b){const d=distance(ring[range.at%n]!,ring[range.a%n]!,ring[range.b%n]!);if(d>range.distance){range.distance=d;range.index=range.at;}range.at++;return;}
    this.ranges.pop();if(range.distance>this.input.simplifyEpsilon){this.kept.add(range.index%n);this.ranges.push({a:range.index,b:range.b,at:range.index+1,distance:0,index:0},{a:range.a,b:range.index,at:range.a+1,distance:0,index:0});}return;
   }
   if(this.at<n){if(this.kept.has(this.at)){const p=ring[this.at]!;if(this.previous)this.candidateArea+=this.previous[0]*p[1]-p[0]*this.previous[1];this.candidate[this.contour]!.push(p);this.previous=p;}this.at++;return;}
   const candidate=this.candidate[this.contour]!;if(this.previous&&candidate.length)this.candidateArea+=this.previous[0]*candidate[0]![1]-candidate[0]![0]*this.previous[1];
   if(candidate.length<3||Math.sign(this.candidateArea)!==Math.sign(this.baseArea)){this.fallback();return;}
   this.changed||=candidate.length!==n;this.contour++;this.at=0;this.anchor=0;this.farthest=0;this.baseArea=0;this.candidateArea=0;this.previous=null;this.mode="anchor";return;
  }
  if(this.phase==="topology"){
   if(this.left>=this.flat.length-1){this.phase="coverage";return;}
   if(this.right===this.flat.length){this.left++;this.right=this.left+1;return;}
   const a=this.flat[this.left]!,b=this.flat[this.right++]!;if(overlap(a.a,a.b,b.a,b.b))this.fallback();return;
  }
  if(this.phase==="coverage"){
   if(this.pixel===this.pixels){this.phase="emit";this.contour=0;this.at=0;return;}
   const x=this.pixel%this.input.width,y=Math.floor(this.pixel/this.input.width)+.5;
   if(!this.covering){
    if(this.edge===this.flat.length){this.covering=true;return;}
    const {a,b}=this.flat[this.edge++]!;if((a[1]>y)===(b[1]>y))return;
    const crossing=a[0]+(y-a[1])*(b[0]-a[0])/(b[1]-a[1]),end=Math.ceil(crossing-.5),sign=b[1]>a[1]?1:-1;
    if(Number.isInteger(crossing-.5)&&crossing>=.5&&crossing<this.input.width){this.fallback();return;}
    if(end>0){this.winding+=sign;if(end<this.input.width)this.changes.set(end,(this.changes.get(end)??0)-sign);}return;
   }
   this.winding+=this.changes.get(x)??0;this.changes.delete(x);if((this.winding!==0)!==this.on(x,Math.floor(y))){this.fallback();return;}
   this.pixel++;if(this.pixel%this.input.width===0){this.edge=0;this.winding=0;this.covering=false;}return;
  }
  if(this.phase==="emit"){
   const rings=this.simplified?this.candidate:this.base;if(this.contour===rings.length){this.phase="complete";return;}
   if(this.output.length>=this.input.maxSegments)invalid("Bitmap trace exceeds segment budget");const ring=rings[this.contour]!;
   if(this.at===ring.length){this.output.push({kind:"close"});this.contour++;this.at=0;}else{this.output.push({kind:this.at===0?"move":"line",to:ring[this.at]!});this.at++;}return;
  }
 }
 advance(budget:number):BitmapTraceProgress {
  if(!Number.isSafeInteger(budget)||budget<=0)invalid("Bitmap trace grant must be a positive integer");if(this.cancelled)throw new DOMException("Bitmap trace cancelled","AbortError");if(this.failed)throw this.failed;
  try{for(let at=0;at<budget&&this.phase!=="complete";at++){if(this.work>=this.input.maxWork)invalid("Bitmap trace exceeds work budget");this.step();this.work++;}}catch(error){this.failed=error;throw error;}
  return {phase:this.phase,scanned:this.scanned,pixels:this.pixels,edges:this.edgeCount,contours:this.base.length,segments:this.output.length,simplified:this.simplified,work:this.work,done:this.phase==="complete"};
 }
 result():PathSegment[]{if(this.cancelled)throw new DOMException("Bitmap trace cancelled","AbortError");if(this.failed)throw this.failed;if(this.phase!=="complete")throw Error("Bitmap trace incomplete");return this.output;}
 /** 🧹️ Transfers the mask unchanged and closes active geometry without cloning or clearing it eagerly. */
 intoRetirement():{job:BitmapTraceRetirement;mask:BitmapTraceInput["mask"]|null}{
  if(this.transferred)invalid("Bitmap trace ownership already transferred");this.transferred=true;this.cancelled=true;const mask=this.input.mask.length?this.input.mask:null;this.input={...this.input,mask:[]};let slot=0,iterator:Iterator<number>|null=null;
  const tree=(value:Map<number,unknown>|Set<number>):boolean=>{iterator??=value.keys();const entry=iterator.next();if(!entry.done){value.delete(entry.value);return false;}iterator=null;return true;};
  const step=():boolean=>{let complete=true;switch(slot){
   case 0:this.edges=[];break;
   case 1:complete=tree(this.outgoing);break;
   case 2:if(this.raw.length){this.raw.pop();complete=false;}else this.raw=[];break;
   case 3:if(this.base.length){this.base.pop();complete=false;}else this.base=[];break;
   case 4:if(this.candidate.length){this.candidate.pop();complete=false;}else this.candidate=[];break;
   case 5:this.flat=[];break;case 6:this.ring=[];break;case 7:this.ranges=[];break;
   case 8:complete=tree(this.kept);break;case 9:complete=tree(this.changes);break;case 10:complete=tree(this.positions);break;
   case 11:this.split=null;break;case 12:this.output=[];break;
  }if(complete)slot++;return slot===13;};return{job:new UnitRetirement(step),mask};
 }
 cancel():void{this.cancelled=true;}
}
/** 🕰️ Cooperative tracing with abort checks before observer calls and result publication. */
export async function traceBitmap(input:BitmapTraceInput,options:BitmapTraceOptions={}):Promise<PathSegment[]> {
 const job=new BitmapTraceJob(input),check=()=>{if(options.signal?.aborted){job.cancel();throw new DOMException("Bitmap trace cancelled","AbortError");}};
 try{check();for(;;){const p=job.advance(options.workBudget??4096);check();options.onProgress?.(p);check();if(p.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();throw error;}finally{const close=job.intoRetirement().job;while(!close.advance(4096).done)await new Promise<void>(resolve=>setTimeout(resolve,0));}
}
