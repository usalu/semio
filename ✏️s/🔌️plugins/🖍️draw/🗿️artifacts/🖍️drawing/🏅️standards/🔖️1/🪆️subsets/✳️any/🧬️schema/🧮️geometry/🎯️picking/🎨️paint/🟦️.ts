/** 🎨️ Borrowed path admission and painted winding use the renderer's geometry kernels. */
import {PathFlattenJob,type FlatContour} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🟦️.ts";
import {StrokeOutlineJob,type StrokeGeometryStyle} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧹️retire/🟦️.ts";
import type {PathGeometrySegment} from "../../../🟦️.ts";
import type {DocumentSceneNode} from "../../../🎬️scene/📋️prepare/🟦️.ts";
type Point=readonly [number,number];
export type PaintedPathStroke={width:number;cap:"butt"|"round"|"square";join:"miter"|"round"|"bevel";dash:readonly number[]};
export type PaintedPathQuery={point:Point;transform:readonly number[];tolerance:number;flatness:number;fill:boolean;fillRule:"nonzero"|"evenodd";stroke:PaintedPathStroke|null};
export type PaintedPathHit={contains:boolean;bounds:[number,number,number,number]|null};
export type PaintedPathProgress={phase:"admitting"|"flattening"|"flattenCleanup"|"fill"|"contours"|"stroke"|"strokeCleanup"|"polygons"|"cleanup"|"complete";work:number;done:boolean};
const valid=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
const invalid=(message:string):never=>{throw new RangeError(message);};
const map=(p:Point,m:readonly number[]):Point=>[m[0]!*p[0]+m[2]!*p[1]+m[4]!,m[1]!*p[0]+m[3]!*p[1]+m[5]!];
function retireContours(contours:(FlatContour|Point[])[]):boolean{const contour=contours.at(-1);if(!contour)return true;const points=Array.isArray(contour)?contour:contour.points;if(points.length)points.pop();else contours.pop();return false;}
function distance(p:Point,a:Point,b:Point):number{const dx=b[0]-a[0],dy=b[1]-a[1],length=Math.hypot(dx,dy),t=length?Math.max(0,Math.min(length,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length)):0;return Math.hypot(p[0]-a[0]-(length?dx*t/length:0),p[1]-a[1]-(length?dy*t/length:0));}
function own(segment:PathGeometrySegment):PathGeometrySegment{switch(segment.kind){case "close":return{kind:"close"};case "move":case "line":return{kind:segment.kind,to:[...segment.to]};case "quad":return{...segment,ctrl:[...segment.ctrl],to:[...segment.to]};case "cubic":return{...segment,ctrl1:[...segment.ctrl1],ctrl2:[...segment.ctrl2],to:[...segment.to]};case "arc":return{...segment,to:[...segment.to]};}}
/** 🧱️ A completed scalar result carries no borrowed path or paint buffers. */
export class PaintedPathHitJob{
 /** 🎬️ Copies bounded query attributes from the actual resolved leaf while borrowing its source separately. */
 static fromPrepared(node:DocumentSceneNode,point:Point,tolerance:number,flatness:number):PaintedPathHitJob{const c=node.content;if(c.kind!=="path")throw new RangeError("Painted query requires a resolved path leaf");const s=c.stroke;return new PaintedPathHitJob({point,transform:node.transform,tolerance,flatness,fill:c.fill!==null,fillRule:c.fillRule,stroke:s?{width:s.width,cap:s.cap,join:s.join,dash:s.dash??[]}:null});}
 private query:PaintedPathQuery;private style:StrokeGeometryStyle|null;private segments:PathGeometrySegment[]=[];private flat:FlatContour[]=[];private contours:FlatContour[]=[];private polygons:Point[][]=[];
 private flatten:PathFlattenJob|null=null;private flattenRetirement:WorkRetirement|null=null;private outline:StrokeOutlineJob|null=null;private outlineRetirement:WorkRetirement|null=null;
 private phase:PaintedPathProgress["phase"]="admitting";private next=0;private at=0;private edge=0;private work=0;private winding=0;private boundary=false;private strokeHit=false;private bounds:[number,number,number,number]=[Infinity,Infinity,-Infinity,-Infinity];private output:PaintedPathHit|null=null;private cancelled=false;private failure:unknown=null;private cleanup=0;private transferred=false;
 constructor(query:PaintedPathQuery){
  if(query.point.length!==2||query.transform.length!==6||![...query.point,...query.transform,query.tolerance].every(valid)||query.tolerance<0||!Number.isFinite(query.flatness)||query.flatness<1e-6||query.flatness>16||typeof query.fill!=="boolean"||!["nonzero","evenodd"].includes(query.fillRule))invalid("Invalid painted query contract");
  const s=query.stroke;if(s&&(!valid(s.width)||s.width<0||!["butt","round","square"].includes(s.cap)||!["miter","round","bevel"].includes(s.join)||s.dash.length>1024||!s.dash.every(v=>valid(v)&&v>=0)))invalid("Invalid painted query stroke");
  this.query={...query,point:[...query.point],transform:[...query.transform],stroke:s?{...s,dash:[...s.dash]}:null};this.style=s?{...s,dash:[...s.dash],miterLimit:4,dashOffset:0}:null;
 }
 private line(a:Point,b:Point):void{
  if(![...a,...b].every(valid))invalid("Painted query exceeds coordinate limit");
  for(const p of [a,b]){this.bounds[0]=Math.min(this.bounds[0],p[0]);this.bounds[1]=Math.min(this.bounds[1],p[1]);this.bounds[2]=Math.max(this.bounds[2],p[0]);this.bounds[3]=Math.max(this.bounds[3],p[1]);}
  const rounding=Math.max(1,...a.map(Math.abs),...b.map(Math.abs),...this.query.point.map(Math.abs))*Number.EPSILON*8;this.boundary ||=distance(this.query.point,a,b)<=this.query.tolerance+rounding;
  const[x,y]=this.query.point;if((a[1]<=y&&b[1]>y)||(b[1]<=y&&a[1]>y)){const t=(y-a[1])/(b[1]-a[1]);if(a[0]*(1-t)+b[0]*t>x)this.winding+=b[1]>a[1]?1:-1;}
 }
 private publish():void{this.output={contains:this.query.fill&&(this.boundary||(this.query.fillRule==="evenodd"?this.winding%2!==0:this.winding!==0)),bounds:Number.isFinite(this.bounds[0])?[...this.bounds]:null};}
 private retireOne():boolean{
  if(this.cleanup===8)return true;let complete=true;
  switch(this.cleanup){
   case 0:if(this.flattenRetirement){if(!this.flattenRetirement.terminalIsEmpty()){this.flattenRetirement.advance(1);return false;}this.flattenRetirement=null;}break;
   case 1:if(this.outlineRetirement){if(!this.outlineRetirement.terminalIsEmpty()){this.outlineRetirement.advance(1);return false;}this.outlineRetirement=null;}break;
   case 2:complete=retireContours(this.flat);break;case 3:complete=retireContours(this.contours);break;case 4:complete=retireContours(this.polygons);break;
   case 5:if(this.segments.length){this.segments.pop();complete=false;}else this.segments=[];break;
   case 6:this.style=null;this.query={...this.query,stroke:null};break;case 7:this.failure=null;break;
  }if(complete)this.cleanup++;return this.cleanup===8;
 }
 private step(source:(index:number)=>PathGeometrySegment|undefined):void{
  switch(this.phase){
   case "admitting":{const segment=source(this.next);if(segment){if(this.next>=65536)invalid("Painted query exceeds segment limit");this.segments.push(own(segment));this.next++;}else{this.flatten=new PathFlattenJob({segments:this.segments,transform:this.query.transform,tolerance:this.query.flatness});this.phase="flattening";}break;}
   case "flattening":if(this.flatten!.advance(1).done){const moved=this.flatten!.intoRetirement();this.flattenRetirement=moved.job;this.flat=moved.output!;this.flatten=null;this.phase="flattenCleanup";}break;
   case "flattenCleanup":if(!this.flattenRetirement!.terminalIsEmpty())this.flattenRetirement!.advance(1);else{this.flattenRetirement=null;this.phase="fill";}break;
   case "fill":{const c=this.flat[this.at];if(c){if(!this.query.fill||c.points.length<2){this.at++;this.edge=0;}else if(this.edge<c.points.length){const a=map(c.points[this.edge]!,this.query.transform),b=map(c.points[(this.edge+1)%c.points.length]!,this.query.transform);this.edge++;this.line(a,b);}else{this.at++;this.edge=0;}}else this.phase="contours";break;}
   case "contours":{const c=this.flat.at(-1);if(c){if(this.style){this.flat.pop();this.contours.push(c);}else if(c.points.length)c.points.pop();else this.flat.pop();}else if(this.style){this.outline=new StrokeOutlineJob({contours:this.contours,transform:this.query.transform,tolerance:this.query.flatness,style:this.style});this.phase="stroke";}else{this.publish();this.phase="cleanup";}break;}
   case "stroke":if(this.outline!.advance(1).done){const moved=this.outline!.intoRetirement();this.outlineRetirement=moved.job;this.polygons=moved.output!;this.outline=null;this.phase="strokeCleanup";}break;
   case "strokeCleanup":if(!this.outlineRetirement!.terminalIsEmpty())this.outlineRetirement!.advance(1);else{this.outlineRetirement=null;this.publish();this.winding=0;this.boundary=false;this.at=0;this.edge=0;this.phase="polygons";}break;
   case "polygons":{const polygon=this.polygons[this.at];if(polygon){if(this.edge<polygon.length){const a=map(polygon[this.edge]!,this.query.transform),b=map(polygon[(this.edge+1)%polygon.length]!,this.query.transform);this.edge++;this.line(a,b);}else{this.strokeHit ||=this.boundary||this.winding!==0;this.winding=0;this.boundary=false;this.at++;this.edge=0;}}else{this.output={contains:this.output!.contains||this.strokeHit,bounds:Number.isFinite(this.bounds[0])?[...this.bounds]:null};this.phase="cleanup";}break;}
   case "cleanup":if(this.retireOne())this.phase="complete";break;case "complete":break;
  }
 }
 advance(grant:number,source:(index:number)=>PathGeometrySegment|undefined):PaintedPathProgress{
  if(!Number.isSafeInteger(grant)||grant<1)invalid("Invalid painted query work grant");if(this.cancelled)throw new DOMException("Painted query cancelled","AbortError");if(this.failure)throw this.failure;
  try{for(let at=0;at<grant&&this.phase!=="complete";at++){this.step(source);this.work++;}}catch(error){this.failure=error;throw error;}return{phase:this.phase,work:this.work,done:this.phase==="complete"};
 }
 result():PaintedPathHit{if(this.cancelled)throw new DOMException("Painted query cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Painted query incomplete");return this.output!;}
 cancel():void{this.cancelled=true;}
 /** 🧹️ Adopts active children and releases its actual private owners under caller grants. */
 intoRetirement():{job:WorkRetirement;output:PaintedPathHit|null}{
  if(this.transferred)invalid("Painted query ownership already transferred");this.transferred=true;const output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.output:null;this.cancelled=true;
  if(this.flatten){this.flatten.cancel();this.flattenRetirement=this.flatten.intoRetirement().job;this.flatten=null;}if(this.outline){this.outline.cancel();this.outlineRetirement=this.outline.intoRetirement().job;this.outline=null;}
  return{job:new UnitRetirement(()=>this.retireOne()),output};
 }
}
