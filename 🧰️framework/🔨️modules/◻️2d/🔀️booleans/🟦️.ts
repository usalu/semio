/** 🔀️ Filled planar arrangements with spatial queries, bounded grants and private outputs. */
import type {PathSegment,Vec2} from "../🟦️.ts";
import {UnitRetirement,type WorkRetirement,type WorkRetirementProgress} from "../🧹️retire/🟦️.ts";
export type BooleanRetirement=WorkRetirement;
export type BooleanRetirementProgress=WorkRetirementProgress;
export type BooleanOperand={contours:readonly (readonly Vec2[])[];fillRule:"nonzero"|"evenodd"};
export type BooleanInput={operation:"union"|"difference"|"intersection"|"xor";operands:readonly BooleanOperand[];epsilon:number;maxEdges:number;maxParameters:number;maxAtomicEdges:number;maxSegments:number;maxWork:number};
export type BooleanProgress={phase:"preparing"|"indexing"|"intersections"|"splitting"|"classifying"|"contours"|"compacting"|"emitting"|"complete";operands:number;vertices:number;edges:number;parameters:number;pairs:number;atomicEdges:number;boundaryEdges:number;contours:number;segments:number;work:number;done:boolean};
export type BooleanOptions={workBudget?:number;signal?:AbortSignal;onProgress?:(progress:BooleanProgress)=>void};
type Box=[number,number,number,number];
type Edge={a:Vec2;b:Vec2;operand:number;box:Box;parameters:Set<number>};
type Node={box:Box;left:number;right:number;edge:number;maximum:number};
type Boundary={from:number;to:number;used:boolean};
type Ring={points:Vec2[];anchor:number;area:number;angle:number};
const bad=(message:string):never=>{throw new RangeError(message);};
const integer=(n:number,cap:number)=>Number.isSafeInteger(n)&&n>=1&&n<=cap;
const point=(p:Vec2):Vec2=>{if(!Array.isArray(p)||p.length!==2||!p.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e12))bad("Invalid boolean point");return[p[0]===0?0:p[0],p[1]===0?0:p[1]];};
const cross=(a:Vec2,b:Vec2,c:Vec2)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
const length=(a:Vec2,b:Vec2)=>Math.hypot(b[0]-a[0],b[1]-a[1]);
const box=(a:Vec2,b:Vec2):Box=>[Math.min(a[0],b[0]),Math.min(a[1],b[1]),Math.max(a[0],b[0]),Math.max(a[1],b[1])];
const merge=(a:Box,b:Box):Box=>[Math.min(a[0],b[0]),Math.min(a[1],b[1]),Math.max(a[2],b[2]),Math.max(a[3],b[3])];
const intersects=(a:Box,b:Box,e:number)=>a[0]<=b[2]+e&&a[2]+e>=b[0]&&a[1]<=b[3]+e&&a[3]+e>=b[1];
const boxDistance=(b:Box,p:Vec2)=>Math.hypot(Math.max(b[0]-p[0],0,p[0]-b[2]),Math.max(b[1]-p[1],0,p[1]-b[3]));
const ray=(b:Box,p:Vec2,axis:0|1,direction:number)=>((direction>0?b[axis+2]!>=p[axis]:b[axis]!<=p[axis])&&b[1-axis]!<=p[1-axis]!&&b[3-axis]!>p[1-axis]!);
const interpolate=(e:Edge,t:number):Vec2=>t===0?e.a:t===1?e.b:point([e.a[0]+(e.b[0]-e.a[0])*t,e.a[1]+(e.b[1]-e.a[1])*t]);
function parameter(e:Edge,p:Vec2):number {const dx=e.b[0]-e.a[0],dy=e.b[1]-e.a[1];return Math.abs(dx)>=Math.abs(dy)?(p[0]-e.a[0])/dx:(p[1]-e.a[1])/dy;}
function endpoint(e:Edge,p:Vec2,epsilon:number,len:number):number|null{const t=parameter(e,p),tolerance=epsilon/len;return t>=-tolerance&&t<=1+tolerance&&Math.abs(cross(e.a,e.b,p))<=epsilon*len?Math.max(0,Math.min(1,t)):null;}
function distance(e:Edge,p:Vec2):number {const dx=e.b[0]-e.a[0],dy=e.b[1]-e.a[1],len=Math.hypot(dx,dy),t=Math.max(0,Math.min(1,((p[0]-e.a[0])*(dx/len)+(p[1]-e.a[1])*(dy/len))/len));return length(p,interpolate(e,t));}
function winding(e:Edge,p:Vec2,axis:0|1,direction:number):number {const other=1-axis,ax=e.a[axis]*direction,bx=e.b[axis]*direction,px=p[axis]*direction,ay=e.a[other]!,by=e.b[other]!,py=p[other]!,c=(bx-ax)*(py-ay)-(by-ay)*(px-ax);return ay<=py&&by>py&&c>0?1:ay>py&&by<=py&&c<0?-1:0;}
const apply=(operation:BooleanInput["operation"],a:boolean,b:boolean)=>operation==="union"?a||b:operation==="difference"?a&&!b:operation==="intersection"?a&&b:a!==b;
const spread=(value:number)=>{let n=value;n=(n|(n<<8))&0x00ff00ff;n=(n|(n<<4))&0x0f0f0f0f;n=(n|(n<<2))&0x33333333;return(n|(n<<1))&0x55555555;};
function morton(e:Edge,b:Box):number {const x=b[2]===b[0]?0:Math.max(0,Math.min(65535,Math.floor(((e.a[0]+e.b[0])/2-b[0])/(b[2]-b[0])*65535))),y=b[3]===b[1]?0:Math.max(0,Math.min(65535,Math.floor(((e.a[1]+e.b[1])/2-b[1])/(b[3]-b[1])*65535)));return(spread(x)|(spread(y)<<1))>>>0;}
class Heap<T> {
 private values:T[]=[];
 constructor(private compare:(a:T,b:T)=>number){}
 get size():number{return this.values.length;}
 release():void{this.values=[];}
 push(value:T):void {let at=this.values.length;this.values.push(value);while(at>0){const parent=(at-1)>>1;if(this.compare(this.values[parent]!,value)<=0)break;this.values[at]=this.values[parent]!;at=parent;}this.values[at]=value;}
 pop():T|undefined {const first=this.values[0],last=this.values.pop();if(last===undefined||!this.values.length)return first;let at=0;while(at*2+1<this.values.length){let child=at*2+1;if(child+1<this.values.length&&this.compare(this.values[child+1]!,this.values[child]!)<0)child++;if(this.compare(last,this.values[child]!)<=0)break;this.values[at]=this.values[child]!;at=child;}this.values[at]=last;return first;}
}
/** ⏱️ Prepare, intersect, classify and emit a stable region snapshot under explicit work grants. */
const retainedAdmission=Symbol("retained Boolean admission");
export class BooleanJob {
 private phase:BooleanProgress["phase"]="preparing";private work=0;private cancelled=false;private transferred=false;private failure:unknown=null;private prepared=0;private vertices=0;private parameters=0;private pairs=0;
 private operand=0;private contour=0;private at=0;private entering=true;private first:Vec2|null=null;private previous:Vec2|null=null;private bounds:Box=[Infinity,Infinity,-Infinity,-Infinity];private rules:BooleanOperand["fillRule"][]=[];private source:Edge[]=[];
 private indexMode:"push"|"leaves"|"levels"="push";private indexAt=0;private indexHeap=new Heap<{key:number;edge:number}>((a,b)=>a.key-b.key||a.edge-b.edge);private tree:Node[]=[];private level:number[]=[];private nextLevel:number[]=[];private root=-1;private query:number[]=[];private pivot=0;
 private splitAt=0;private splitValues:Iterator<number>|null=null;private splitHeap=new Heap<number>((a,b)=>a-b);private splitBuild=true;private splitPrevious:number|null=null;private nodes:Vec2[]=[];private grid=new Map<string,number[]>();private atomic:[number,number][]=[];private atomicIds=new Set<string>();
 private classifyAt=0;private classifyMode:"start"|"nearest"|"ray"|"fold"="start";private classifyAxis:0|1=0;private classifyDirection=1;private midpoint:Vec2=[0,0];private leftPoint:Vec2=[0,0];private rightPoint:Vec2=[0,0];private nearest=Infinity;private leftWinding:number[]=[];private rightWinding:number[]=[];private foldAt=0;private leftFilled=false;private rightFilled=false;
 private boundary:Boundary[]=[];private outgoing=new Map<number,number[]>();private boundaryAt=0;private current=-1;private start=0;private raw:number[][]=[];private ring:number[]=[];private positions=new Map<number,number>();private selecting=false;private choiceAt=0;private choiceNode=0;private reverseAngle=0;private best=-1;private bestAngle=Infinity;
 private splitRing:number[]|null=null;private ringSplitAt=0;private ringSplitStop=0;private ringSplitCopy=true;
 private compactAt=0;private compactVertex=0;private compactMode:"scan"|"measure"="scan";private compactPoints:Vec2[]=[];private area=0;private lower=0;private upper=0;private rings:Ring[]=[];
 private ringHeap=new Heap<{key:[number,number,number,number];ring:number}>((a,b)=>a.key[0]-b.key[0]||a.key[1]-b.key[1]||a.key[2]-b.key[2]||a.key[3]-b.key[3]);private emitting:number|null=null;private emitAt=0;private output:PathSegment[]=[];
 /** 📥️ Refused contracts keep their actual input under retained admission. */
 static admit(input:BooleanInput):BooleanJob{return new BooleanJob(input,retainedAdmission);}
 constructor(private input:BooleanInput,admission?:typeof retainedAdmission) {
  try{
  if(!["union","difference","intersection","xor"].includes(input.operation)||!Array.isArray(input.operands)||!input.operands.length||input.operands.length>1024||!Number.isFinite(input.epsilon)||input.epsilon<1e-12||input.epsilon>16||!integer(input.maxEdges,262144)||!integer(input.maxParameters,1048576)||!integer(input.maxAtomicEdges,262144)||!integer(input.maxSegments,327680)||!integer(input.maxWork,1000000000))bad("Invalid boolean contract");this.input={...input};}catch(error){if(admission!==retainedAdmission)throw error;this.failure=error;}
 }
 private addEdge(a:Vec2,b:Vec2):void {
  if(length(a,b)===0)return;if(this.source.length>=this.input.maxEdges)bad("Boolean exceeds edge budget");if(this.parameters+2>this.input.maxParameters)bad("Boolean exceeds parameter budget");
  this.source.push({a,b,operand:this.operand,box:box(a,b),parameters:new Set([0,1])});this.parameters+=2;
 }
 private addParameter(e:Edge,value:number):void {const v=Math.max(0,Math.min(1,value));if(!e.parameters.has(v)){if(this.parameters>=this.input.maxParameters)bad("Boolean exceeds parameter budget");e.parameters.add(v);this.parameters++;}}
 private intersect(a:Edge,b:Edge):void {
  const rx=a.b[0]-a.a[0],ry=a.b[1]-a.a[1],sx=b.b[0]-b.a[0],sy=b.b[1]-b.a[1],ox=b.a[0]-a.a[0],oy=b.a[1]-a.a[1],denominator=rx*sy-ry*sx,la=length(a.a,a.b),lb=length(b.a,b.b),ta=this.input.epsilon/la,tb=this.input.epsilon/lb;
  if(Math.abs(denominator)>Number.EPSILON*16*la*lb){let snap=endpoint(b,a.a,this.input.epsilon,lb);if(snap!==null){this.addParameter(a,0);this.addParameter(b,snap);return;}snap=endpoint(b,a.b,this.input.epsilon,lb);if(snap!==null){this.addParameter(a,1);this.addParameter(b,snap);return;}snap=endpoint(a,b.a,this.input.epsilon,la);if(snap!==null){this.addParameter(a,snap);this.addParameter(b,0);return;}snap=endpoint(a,b.b,this.input.epsilon,la);if(snap!==null){this.addParameter(a,snap);this.addParameter(b,1);return;}const t=(ox*sy-oy*sx)/denominator,u=(ox*ry-oy*rx)/denominator;if(t>=-ta&&t<=1+ta&&u>=-tb&&u<=1+tb){this.addParameter(a,t);this.addParameter(b,u);}return;}
  if(Math.abs(ox*ry-oy*rx)>this.input.epsilon*la)return;
  for(const p of [b.a,b.b]){const t=parameter(a,p);if(t>=-ta&&t<=1+ta)this.addParameter(a,t);}for(const p of [a.a,a.b]){const t=parameter(b,p);if(t>=-tb&&t<=1+tb)this.addParameter(b,t);}
 }
 private intern(p:Vec2):number {
  const epsilon=this.input.epsilon,x=Math.floor(p[0]/epsilon),y=Math.floor(p[1]/epsilon);let found=-1;
  for(let dy=-1;dy<=1;dy++)for(let dx=-1;dx<=1;dx++)for(const index of this.grid.get((x+dx)+":"+(y+dy))??[])if(length(this.nodes[index]!,p)<=epsilon&&(found<0||index<found))found=index;
  if(found>=0)return found;if(this.nodes.length>=this.input.maxAtomicEdges*2)bad("Boolean exceeds vertex budget");
  const index=this.nodes.length;this.nodes.push(p);const key=x+":"+y,cell=this.grid.get(key)??[];if(cell.length>=16)bad("Boolean spatial precision exceeded");cell.push(index);this.grid.set(key,cell);return index;
 }
 private prepare():void {
  if(this.operand===this.input.operands.length){const magnitude=Math.max(Math.abs(this.bounds[0]),Math.abs(this.bounds[1]),Math.abs(this.bounds[2]),Math.abs(this.bounds[3]));if(this.source.length&&this.input.epsilon<magnitude*Number.EPSILON*16)bad("Boolean epsilon is below coordinate precision");this.phase="indexing";return;}
  const operand=this.input.operands[this.operand]!;
  if(this.entering){if(!operand||!Array.isArray(operand.contours)||operand.contours.length>65536||!["nonzero","evenodd"].includes(operand.fillRule))bad("Invalid boolean operand");this.rules.push(operand.fillRule);this.prepared++;this.entering=false;return;}
  if(this.contour===operand.contours.length){this.operand++;this.contour=0;this.entering=true;return;}
  const points=operand.contours[this.contour]!;if(!Array.isArray(points)||points.length>262144)bad("Invalid boolean contour");
  if(this.at<points.length){if(this.vertices>=this.input.maxEdges)bad("Boolean exceeds input vertex budget");const p=point(points[this.at++]!);this.vertices++;this.bounds=merge(this.bounds,[p[0],p[1],p[0],p[1]]);if(this.previous)this.addEdge(this.previous,p);else this.first=p;this.previous=p;return;}
  if(this.previous&&this.first)this.addEdge(this.previous,this.first);this.contour++;this.at=0;this.previous=null;this.first=null;
 }
 private indexing():void {
  if(!this.source.length){this.phase="complete";return;}
  if(this.indexMode==="push"){if(this.indexAt<this.source.length){this.indexHeap.push({key:morton(this.source[this.indexAt]!,this.bounds),edge:this.indexAt++});return;}this.indexMode="leaves";return;}
  if(this.indexMode==="leaves"){const entry=this.indexHeap.pop();if(entry){this.level.push(this.tree.length);this.tree.push({box:this.source[entry.edge]!.box,left:-1,right:-1,edge:entry.edge,maximum:entry.edge});return;}this.indexMode="levels";this.indexAt=0;return;}
  if(this.level.length===1){this.root=this.level[0]!;this.query=[this.root];this.phase="intersections";return;}
  if(this.indexAt===this.level.length){this.level=this.nextLevel;this.nextLevel=[];this.indexAt=0;return;}
  const left=this.level[this.indexAt++]!,right=this.level[this.indexAt++];if(right===undefined){this.indexAt=this.level.length;this.nextLevel.push(left);return;}
  const a=this.tree[left]!,b=this.tree[right]!;this.nextLevel.push(this.tree.length);this.tree.push({box:merge(a.box,b.box),left,right,edge:-1,maximum:Math.max(a.maximum,b.maximum)});
 }
 private intersections():void {
  if(this.pivot===this.source.length){this.phase="splitting";return;}
  const index=this.query.pop();if(index===undefined){this.pivot++;if(this.pivot<this.source.length)this.query.push(this.root);return;}
  const node=this.tree[index]!,edge=this.source[this.pivot]!;if(node.maximum<=this.pivot||!intersects(node.box,edge.box,this.input.epsilon))return;
  if(node.edge>=0){this.pairs++;this.intersect(edge,this.source[node.edge]!);}else{const l=this.tree[node.left]!,r=this.tree[node.right]!;if(r.maximum>this.pivot&&intersects(r.box,edge.box,this.input.epsilon))this.query.push(node.right);if(l.maximum>this.pivot&&intersects(l.box,edge.box,this.input.epsilon))this.query.push(node.left);}
 }
 private splitting():void {
  if(this.splitAt===this.source.length){this.phase="classifying";return;}
  const edge=this.source[this.splitAt]!;
  if(!this.splitValues){this.splitValues=edge.parameters.values();this.splitBuild=true;this.splitPrevious=null;return;}
  if(this.splitBuild){const next=this.splitValues.next();if(next.done)this.splitBuild=false;else{edge.parameters.delete(next.value);this.splitHeap.push(next.value);}return;}
  const t=this.splitHeap.pop();if(t===undefined){this.splitValues=null;this.splitAt++;return;}
  const previous=this.splitPrevious;this.splitPrevious=t;if(previous===null||previous===t)return;
  const p=interpolate(edge,previous),q=interpolate(edge,t);if(length(p,q)<=this.input.epsilon)return;const from=this.intern(p),to=this.intern(q);if(from===to)return;const a=Math.min(from,to),b=Math.max(from,to),key=a+":"+b;
  if(!this.atomicIds.has(key)){if(this.atomic.length>=this.input.maxAtomicEdges)bad("Boolean exceeds atomic edge budget");this.atomicIds.add(key);this.atomic.push([a,b]);}
 }
 private classify():void {
  if(this.classifyAt===this.atomic.length){this.query=[];this.phase="contours";return;}
  const [from,to]=this.atomic[this.classifyAt]!,a=this.nodes[from]!,b=this.nodes[to]!;
  if(this.classifyMode==="start"){this.midpoint=[(a[0]+b[0])/2,(a[1]+b[1])/2];let distance=Infinity;for(const axis of [0,1]as const)for(const direction of [-1,1]){const next=direction>0?this.bounds[axis+2]!-this.midpoint[axis]:this.midpoint[axis]-this.bounds[axis]!;if(next<distance){distance=next;this.classifyAxis=axis;this.classifyDirection=direction;}}const len=length(a,b);this.nearest=4*Math.min(len*.25,Math.max(this.input.epsilon*2,len*1e-7));this.query=[this.root];this.classifyMode="nearest";return;}
  if(this.classifyMode==="nearest"){
   const index=this.query.pop();if(index!==undefined){const n=this.tree[index]!;if(boxDistance(n.box,this.midpoint)>this.nearest)return;if(n.edge>=0){const d=distance(this.source[n.edge]!,this.midpoint);if(d>this.input.epsilon)this.nearest=Math.min(this.nearest,d);}else{const l=boxDistance(this.tree[n.left]!.box,this.midpoint),r=boxDistance(this.tree[n.right]!.box,this.midpoint);if(l<=r){if(r<=this.nearest)this.query.push(n.right);if(l<=this.nearest)this.query.push(n.left);}else{if(l<=this.nearest)this.query.push(n.left);if(r<=this.nearest)this.query.push(n.right);}}return;}
   const len=length(a,b),offset=Math.min(len*.25,this.nearest*.25,Math.max(this.input.epsilon*2,len*1e-7)),nx=-(b[1]-a[1])/len,ny=(b[0]-a[0])/len;
   this.leftPoint=[this.midpoint[0]+nx*offset,this.midpoint[1]+ny*offset];this.rightPoint=[this.midpoint[0]-nx*offset,this.midpoint[1]-ny*offset];if(length(this.leftPoint,this.rightPoint)===0)bad("Boolean probes exceed coordinate precision");
   this.leftWinding=Array(this.rules.length).fill(0);this.rightWinding=Array(this.rules.length).fill(0);this.query=[this.root];this.classifyMode="ray";return;
  }
  if(this.classifyMode==="ray"){const index=this.query.pop();if(index!==undefined){const n=this.tree[index]!;if(!ray(n.box,this.leftPoint,this.classifyAxis,this.classifyDirection)&&!ray(n.box,this.rightPoint,this.classifyAxis,this.classifyDirection))return;if(n.edge>=0){const e=this.source[n.edge]!;this.leftWinding[e.operand]!+=winding(e,this.leftPoint,this.classifyAxis,this.classifyDirection);this.rightWinding[e.operand]!+=winding(e,this.rightPoint,this.classifyAxis,this.classifyDirection);}else{const l=this.tree[n.left]!.box,r=this.tree[n.right]!.box;if(ray(r,this.leftPoint,this.classifyAxis,this.classifyDirection)||ray(r,this.rightPoint,this.classifyAxis,this.classifyDirection))this.query.push(n.right);if(ray(l,this.leftPoint,this.classifyAxis,this.classifyDirection)||ray(l,this.rightPoint,this.classifyAxis,this.classifyDirection))this.query.push(n.left);}return;}this.foldAt=0;this.classifyMode="fold";return;}
  if(this.foldAt<this.rules.length){const rule=this.rules[this.foldAt]!,inside=(n:number)=>rule==="evenodd"?n%2!==0:n!==0,l=inside(this.leftWinding[this.foldAt]!),r=inside(this.rightWinding[this.foldAt]!);this.leftFilled=this.foldAt?apply(this.input.operation,this.leftFilled,l):l;this.rightFilled=this.foldAt?apply(this.input.operation,this.rightFilled,r):r;this.foldAt++;return;}
  if(this.leftFilled!==this.rightFilled){const edge:Boundary=this.leftFilled?{from,to,used:false}:{from:to,to:from,used:false},index=this.boundary.length;this.boundary.push(edge);const outgoing=this.outgoing.get(edge.from)??[];outgoing.push(index);this.outgoing.set(edge.from,outgoing);}
  this.classifyAt++;this.classifyMode="start";
 }
 private contours():void {
  if(this.splitRing){if(this.ringSplitCopy){if(this.ringSplitAt<this.ring.length){this.splitRing.push(this.ring[this.ringSplitAt++]!);return;}this.ringSplitCopy=false;return;}if(this.ring.length>this.ringSplitStop){this.positions.delete(this.ring.pop()!);return;}if(this.splitRing.length<3)bad("Boolean split contour is degenerate");this.raw.push(this.splitRing);this.splitRing=null;return;}
  if(this.selecting){const choices=this.outgoing.get(this.choiceNode)??[],index=choices[this.choiceAt++];if(index!==undefined){const e=this.boundary[index]!;if(!e.used){const a=this.nodes[e.from]!,b=this.nodes[e.to]!,angle=(this.reverseAngle-Math.atan2(b[1]-a[1],b[0]-a[0])+Math.PI*2)%(Math.PI*2);if(angle<this.bestAngle){this.best=index;this.bestAngle=angle;}}return;}if(this.best<0)bad("Boolean boundary is open");this.current=this.best;this.selecting=false;return;}
  if(this.current<0){if(this.boundaryAt===this.boundary.length){this.phase="compacting";return;}const index=this.boundaryAt++;if(this.boundary[index]!.used)return;this.current=index;this.start=this.boundary[index]!.from;this.ring=[];this.positions.clear();return;}
  const e=this.boundary[this.current]!;if(e.used)bad("Boolean boundary is not manifold");const repeated=this.positions.get(e.from);if(repeated!==undefined){this.splitRing=[];this.ringSplitAt=repeated;this.ringSplitStop=repeated;this.ringSplitCopy=true;return;}
  this.positions.set(e.from,this.ring.length);this.ring.push(e.from);e.used=true;if(e.to===this.start){this.raw.push(this.ring);this.current=-1;return;}
  this.choiceNode=e.to;this.choiceAt=0;this.best=-1;this.bestAngle=Infinity;const a=this.nodes[e.from]!,b=this.nodes[e.to]!;this.reverseAngle=Math.atan2(a[1]-b[1],a[0]-b[0]);this.selecting=true;
 }
 private compact():void {
  if(this.compactAt===this.raw.length){this.phase="emitting";return;}
  const raw=this.raw[this.compactAt]!;
  if(this.compactMode==="scan"){if(this.compactVertex<raw.length){const at=this.compactVertex++,p=this.nodes[raw[at]!]!,a=this.nodes[raw[(at+raw.length-1)%raw.length]!]!,b=this.nodes[raw[(at+1)%raw.length]!]!;if(Math.abs(cross(a,p,b))>this.input.epsilon*(length(a,p)+length(p,b)))this.compactPoints.push(p);return;}this.compactMode="measure";this.compactVertex=0;this.area=0;this.lower=0;this.upper=0;return;}
  const points=this.compactPoints,at=this.compactVertex++;if(at<points.length){const p=points[at]!,l=points[this.lower]!,u=points[this.upper]!;if(p[0]<l[0]||p[0]===l[0]&&p[1]<l[1])this.lower=at;if(p[0]<u[0]||p[0]===u[0]&&p[1]>u[1])this.upper=at;if(at>0&&at<points.length-1)this.area+=cross(points[0]!,p,points[at+1]!)/2;return;}
  if(points.length>=3&&Math.abs(this.area)>this.input.epsilon*this.input.epsilon){const anchor=this.area>0?this.lower:this.upper,a=points[anchor]!,b=points[(anchor+1)%points.length]!,ring={points,anchor,area:this.area,angle:Math.atan2(b[1]-a[1],b[0]-a[0])};const index=this.rings.length;this.rings.push(ring);this.ringHeap.push({key:[a[0],a[1],ring.angle,ring.area],ring:index});}
  this.compactAt++;this.compactVertex=0;this.compactPoints=[];this.compactMode="scan";
 }
 private emit():void {
  if(this.emitting===null){const item=this.ringHeap.pop();if(!item){this.phase="complete";return;}this.emitting=item.ring;this.emitAt=0;return;}
  if(this.output.length>=this.input.maxSegments)bad("Boolean exceeds output segment budget");const ring=this.rings[this.emitting]!;
  if(this.emitAt===ring.points.length){this.output.push({kind:"close"});this.emitting=null;return;}this.output.push({kind:this.emitAt===0?"move":"line",to:ring.points[(ring.anchor+this.emitAt++)%ring.points.length]!});
 }
 private step():void {switch(this.phase){case"preparing":this.prepare();break;case"indexing":this.indexing();break;case"intersections":this.intersections();break;case"splitting":this.splitting();break;case"classifying":this.classify();break;case"contours":this.contours();break;case"compacting":this.compact();break;case"emitting":this.emit();break;case"complete":break;}}
 /** 🧭️ Nested owners advance bounded work without allocating observer snapshots per unit. */
 advanceWork(grant:number):boolean {
  if(!Number.isSafeInteger(grant)||grant<=0)bad("Boolean work grant must be a positive integer");if(this.cancelled)throw new DOMException("Boolean cancelled","AbortError");if(this.failure)throw this.failure;
  try{for(let at=0;at<grant&&this.phase!=="complete";at++){if(this.work>=this.input.maxWork)bad("Boolean exceeds work budget");this.step();this.work++;}}catch(error){this.failure=error;throw error;}
  return this.phase==="complete";
 }
 advance(grant:number):BooleanProgress {
  this.advanceWork(grant);
  return{phase:this.phase,operands:this.prepared,vertices:this.vertices,edges:this.source.length,parameters:this.parameters,pairs:this.pairs,atomicEdges:this.atomic.length,boundaryEdges:this.boundary.length,contours:this.rings.length,segments:this.output.length,work:this.work,done:this.phase==="complete"};
 }
 result():PathSegment[]{if(this.cancelled)throw new DOMException("Boolean cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Boolean incomplete");return this.output;}
 /** 🧹️ Move caller source and completed output; retain every private nested owner until granted cleanup. */
 intoRetirement():{job:BooleanRetirement;operands:readonly BooleanOperand[];output:PathSegment[]|null}{
  if(this.transferred)throw Error("Boolean ownership already transferred");this.transferred=true;
  const operands=this.input.operands,output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.output:null;
  this.input={...this.input,operands:[]};if(output)this.output=[];this.cancelled=true;this.splitValues=null;this.emitting=null;
  let slot=0,edge:Edge|null=null,entries:Iterator<unknown>|null=null;
  const remove=(collection:Map<unknown,unknown>|Set<unknown>)=>{entries??=collection.keys();const next=entries.next();if(next.done){entries=null;return false;}collection.delete(next.value);return true;};
  const job=new UnitRetirement(()=>{
   switch(slot){
    case 0:this.input.operands=[];break;
    case 1:this.rules=[];break;
    case 2:if(edge){if(remove(edge.parameters))return false;edge=null;return false;}edge=this.source.pop()??null;if(edge)return false;this.source=[];break;
    case 3:this.indexHeap.release();break;
    case 4:this.tree=[];break;
    case 5:this.level=[];break;
    case 6:this.nextLevel=[];break;
    case 7:this.query=[];break;
    case 8:this.splitHeap.release();break;
    case 9:this.nodes=[];break;
    case 10:if(remove(this.grid))return false;this.grid.clear();break;
    case 11:this.atomic=[];break;
    case 12:if(remove(this.atomicIds))return false;this.atomicIds.clear();break;
    case 13:this.leftWinding=[];break;
    case 14:this.rightWinding=[];break;
    case 15:this.boundary=[];break;
    case 16:if(remove(this.outgoing))return false;this.outgoing.clear();break;
    case 17:if(this.raw.pop())return false;this.raw=[];break;
    case 18:this.ring=[];break;
    case 19:if(remove(this.positions))return false;this.positions.clear();break;
    case 20:this.splitRing=null;break;
    case 21:this.compactPoints=[];break;
    case 22:if(this.rings.pop())return false;this.rings=[];break;
    case 23:this.ringHeap.release();break;
    case 24:this.output=[];break;
   }
   return ++slot===25;
  });
  return{job,operands,output};
 }
 cancel():void{this.cancelled=true;}
}
/** 🕰️ Yield between grants and recheck aborts after complete observers before publication. */
export async function booleanRegions(input:BooleanInput,options:BooleanOptions={}):Promise<PathSegment[]> {
 const job=new BooleanJob(input),check=()=>{if(options.signal?.aborted){job.cancel();throw new DOMException("Boolean cancelled","AbortError");}};
 try{check();for(;;){const progress=job.advance(options.workBudget??4096);check();options.onProgress?.(progress);check();if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();throw error;}finally{const close=job.intoRetirement().job;while(!close.advance(4096).done)await new Promise<void>(resolve=>setTimeout(resolve,0));}
}
