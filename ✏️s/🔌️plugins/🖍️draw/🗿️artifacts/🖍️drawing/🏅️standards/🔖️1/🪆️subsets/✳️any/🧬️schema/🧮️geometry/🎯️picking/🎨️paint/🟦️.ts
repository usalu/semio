/** 🎨️ Borrowed path admission and painted winding use the renderer's geometry kernels. */
import {PathFlattenJob,type FlatContours} from "@semio-tech/framework/2d/path/flatten";
import {StrokeOutlineJob,type StrokeGeometryStyle,type StrokeContours} from "@semio-tech/framework/2d/path/stroke";
import {advanceCursor} from "../../../🎬️scene/🎨️paint/🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkTurn} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import type {PathSegment} from "@semio-tech/framework/2d";
import type {PathGeometrySegment} from "../../../🟦️.ts";
import type {ScenePlanNode} from "../../../🎬️scene/🗺️plan-model/🎟️owned/🟦️.ts";
type Point=readonly [number,number];
export type PaintedPathStroke={width:number;cap:"butt"|"round"|"square";join:"miter"|"round"|"bevel";dash:readonly number[]};
export type PaintedPathQuery={point:Point;transform:readonly number[];tolerance:number;flatness:number;fill:boolean;fillRule:"nonzero"|"evenodd";stroke:PaintedPathStroke|null};
export type PaintedPathHit={contains:boolean;bounds:[number,number,number,number]|null};
export type PaintedPathProgress={phase:"admitting"|"flattening"|"flattenCleanup"|"fill"|"contours"|"stroke"|"strokeCleanup"|"polygons"|"cleanup"|"complete";work:number;done:boolean};
const valid=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
const invalid=(message:string):never=>{throw new RangeError(message);};
const map=(p:Point,m:readonly number[]):Point=>[m[0]!*p[0]+m[2]!*p[1]+m[4]!,m[1]!*p[0]+m[3]!*p[1]+m[5]!];
type OwnedContour={points:Point[];closed:boolean};
function retireContours(contours:(OwnedContour|Point[])[]):boolean{const contour=contours.at(-1);if(!contour)return true;const points=Array.isArray(contour)?contour:contour.points;if(points.length)points.pop();else contours.pop();return false;}
function distance(p:Point,a:Point,b:Point):number{const dx=b[0]-a[0],dy=b[1]-a[1],length=Math.hypot(dx,dy),t=length?Math.max(0,Math.min(length,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length)):0;return Math.hypot(p[0]-a[0]-(length?dx*t/length:0),p[1]-a[1]-(length?dy*t/length:0));}
function own(segment:PathSegment):PathGeometrySegment{switch(segment.kind){case "close":return{kind:"close"};case "move":case "line":return{kind:segment.kind,to:[...segment.to]};case "quad":return{...segment,ctrl:[...segment.ctrl],to:[...segment.to]};case "cubic":return{...segment,ctrl1:[...segment.ctrl1],ctrl2:[...segment.ctrl2],to:[...segment.to]};case "arc":return{...segment,to:[...segment.to]};}}
/** 🧱️ A completed scalar result carries no borrowed path or paint buffers. */
export class PaintedPathHitJob{
 /** 🎬️ Copies bounded query attributes from the actual resolved leaf while borrowing its source separately. */
 static fromPrepared(node:ScenePlanNode,point:Point,tolerance:number,flatness:number):PaintedPathHitJob{const c=node.content;if(c.kind!=="path")throw new RangeError("Painted query requires a resolved path leaf");const s=c.stroke;return new PaintedPathHitJob({point,transform:node.transform,tolerance,flatness,fill:c.fill!==null,fillRule:c.fillRule,stroke:s?{width:s.width,cap:s.cap,join:s.join,dash:s.dash?Array.from({length:s.dash.length},(_,at)=>s.dash!.at(at)!):[]}:null});}
 private query:PaintedPathQuery;private style:StrokeGeometryStyle|null;private segments:PathGeometrySegment[]=[];private flat:FlatContours|null=null;private flatCursor:RetainedWorkRetirementCursor|null=null;private contours:OwnedContour[]=[];private polygons:StrokeContours|null=null;private polygonCursor:RetainedWorkRetirementCursor|null=null;
 private flatten:PathFlattenJob|null=null;private flattenRetirement:RetainedWorkRetirementCursor|null=null;private outline:StrokeOutlineJob|null=null;private outlineRetirement:RetainedWorkRetirementCursor|null=null;
 private phase:PaintedPathProgress["phase"]="admitting";private next=0;private at=0;private edge=0;private work=0;private winding=0;private boundary=false;private strokeHit=false;private bounds:[number,number,number,number]=[Infinity,Infinity,-Infinity,-Infinity];private output:PaintedPathHit|null=null;private cancelled=false;private failure:unknown=null;private cleanup=0;private transferred=false;#runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
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
  private retireStep(work:RetainedWorkLedger,turn:RetainedWorkTurn):0|1|2{
   if(this.cleanup===8)return 2;
   if(this.cleanup===2&&this.flat!==null){this.flatCursor=this.flat.intoRetirement();this.flat=null;}
   if(this.cleanup===4&&this.polygons!==null){this.polygonCursor=this.polygons.intoRetirement();this.polygons=null;}
   if(advanceCursor(this.cleanup===0?this.flattenRetirement:this.cleanup===1?this.outlineRetirement:this.cleanup===2?this.flatCursor:this.cleanup===4?this.polygonCursor:null,work,turn))return 0;
   if(!work.canStep())return 0;
   turn.commitActualWork();let complete=true;
   switch(this.cleanup){
    case 0:this.flattenRetirement=null;break;
    case 1:this.outlineRetirement=null;break;
    case 2:this.flatCursor=null;break;
    case 3:complete=retireContours(this.contours);break;
    case 4:this.polygonCursor=null;break;
    case 5:if(this.segments.length){this.segments.pop();complete=false;}else this.segments=[];break;
    case 6:this.style=null;this.query={...this.query,stroke:null};break;
    default:this.failure=null;
   }
   if(complete)this.cleanup++;return this.cleanup===8?2:1;
  }
  private unit(source:(index:number)=>PathSegment|undefined):void{
   switch(this.phase){
    case "admitting":{const segment=source(this.next);if(segment){if(this.next>=65536)invalid("Painted query exceeds segment limit");this.segments.push(own(segment));this.next++;}else{this.flatten=new PathFlattenJob({segments:this.segments,transform:this.query.transform,tolerance:this.query.flatness});this.phase="flattening";}break;}
    case "fill":{const c=this.flat!.at(this.at);if(c){if(!this.query.fill||c.points.length<2){this.at++;this.edge=0;}else if(this.edge<c.points.length){const a=map(c.points.at(this.edge)!,this.query.transform),b=map(c.points.at((this.edge+1)%c.points.length)!,this.query.transform);this.edge++;this.line(a,b);}else{this.at++;this.edge=0;}}else{this.at=0;this.edge=0;this.phase="contours";}break;}
    case "contours":{if(!this.style){this.publish();this.phase="cleanup";break;}const c=this.flat!.at(this.at);if(!c){this.outline=new StrokeOutlineJob({contours:this.contours,transform:this.query.transform,tolerance:this.query.flatness,style:this.style});this.phase="stroke";}else if(this.edge===0){this.contours.push({points:[],closed:c.closed});this.edge++;}else if(this.edge<=c.points.length){this.contours.at(-1)!.points.push([...c.points.at(this.edge-1)!]as[number,number]);this.edge++;}else{this.at++;this.edge=0;}break;}
    case "polygons":{const polygon=this.polygons!.at(this.at);if(polygon){if(this.edge<polygon.length){const a=map(polygon.at(this.edge)!,this.query.transform),b=map(polygon.at((this.edge+1)%polygon.length)!,this.query.transform);this.edge++;this.line(a,b);}else{this.strokeHit ||=this.boundary||this.winding!==0;this.winding=0;this.boundary=false;this.at++;this.edge=0;}}else{this.output={contains:this.output!.contains||this.strokeHit,bounds:Number.isFinite(this.bounds[0])?[...this.bounds]:null};this.phase="cleanup";}break;}
    default:throw Error("Unknown painted query phase");
   }
  }
  private step(c:RetainedWorkProducerStep,source:(index:number)=>PathSegment|undefined):void{
   switch(this.phase){
    case "flattening":{if(c.advanceChild(this.flatten!).done){const moved=this.flatten!.intoRetirement();this.flattenRetirement=moved.job;this.flat=moved.output!;this.flatten=null;this.phase="flattenCleanup";}return;}
    case "flattenCleanup":{if(advanceCursor(this.flattenRetirement,c.work,c.retainedTurn))return;this.flattenRetirement=null;this.phase="fill";c.commitControlWork();return;}
    case "stroke":{if(c.advanceChild(this.outline!).done){const moved=this.outline!.intoRetirement();this.outlineRetirement=moved.job;this.polygons=moved.output!;this.outline=null;this.phase="strokeCleanup";}return;}
    case "strokeCleanup":{if(advanceCursor(this.outlineRetirement,c.work,c.retainedTurn))return;this.outlineRetirement=null;this.publish();this.winding=0;this.boundary=false;this.at=0;this.edge=0;this.phase="polygons";c.commitControlWork();return;}
    case "cleanup":{const r=this.retireStep(c.work,c.retainedTurn);if(r===0)return;c.postSemanticWork();if(r===2)this.phase="complete";return;}
   }
   this.unit(source);c.commitSemanticWork();
  }
  lastTurnWork():number{return this.#runtime.lastTurnWork();}
  lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
  nextWorkDemand():RetainedWorkDemand{return this.#demand;}
  failurePending():boolean{return this.failure!==null||this.flatten?.failurePending()===true||this.outline?.failurePending()===true;}
  settlementPending():boolean{return this.phase==="complete"||(this.phase==="flattening"&&this.flatten?.settlementPending()===true)||(this.phase==="stroke"&&this.outline?.settlementPending()===true);}
  advance(items:number,work:RetainedWorkLedger,source:(index:number)=>PathSegment|undefined):PaintedPathProgress{try{return this.#runtime.advance(items,work,{maximumWork:Number.MAX_SAFE_INTEGER,workLimitMessage:"Painted query work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Painted query cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c,source);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,work:this.work,done:this.phase==="complete"})});}catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.cancelled)this.failure=error;throw error;}}
  result():PaintedPathHit{if(this.cancelled)throw new DOMException("Painted query cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Painted query incomplete");return this.output!;}
  cancel():void{this.cancelled=true;}
  /** 🧹️ Adopts active children and releases its actual private owners under caller grants. */
  intoRetirement():{job:RetainedWorkRetirementCursor;output:PaintedPathHit|null}{
   if(this.transferred)invalid("Painted query ownership already transferred");this.transferred=true;const output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.output:null;this.cancelled=true;
   if(this.flatten){this.flatten.cancel();this.flattenRetirement=this.flatten.intoRetirement().job;this.flatten=null;}if(this.outline){this.outline.cancel();this.outlineRetirement=this.outline.intoRetirement().job;this.outline=null;}
   return{job:new RetainedWorkRetirement((work,turn)=>{const r=this.retireStep(work,turn);return r===2;}),output};
  }
}
