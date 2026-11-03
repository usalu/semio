/** 📷️ Resumable authored path coverage and local paint into straight RGBA under caller funding. */
import {PathFlattenJob,type FlatContours} from "@semio-tech/framework/2d/path/flatten";
import {StrokeOutlineJob,type StrokeGeometryStyle,type StrokeContours} from "@semio-tech/framework/2d/path/stroke";
import {CoverageJob,type CoverageMask} from "@semio-tech/framework/pixels/coverage";
import {validateExtent,type PixelImage} from "@semio-tech/framework/pixels/editing";
import type {PathSegment,Vec2} from "@semio-tech/framework/2d";
import {PreparedFill} from "../../🎨️fill/🎨️sampling/🟦️.ts";
import type { Fill, Color } from "../../🎨️fill/🖌️paint-model/🟦️.ts";
import {parseStrokeCap,parseStrokeJoin,type StrokeCap,type StrokeJoin} from "../../🖊️stroke/🟦️.ts";
import {advanceCursor} from "../../🎬️scene/🎨️paint/🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
export type PathRasterStroke={readonly color:Readonly<Color>;readonly width:number;readonly cap:StrokeCap;readonly join:StrokeJoin;readonly dash?:readonly number[]|null};
export type PathRasterInput={width:number;height:number;origin:readonly [number,number];segments:readonly PathSegment[];transform:readonly number[];tolerance:number;fillRule:"nonzero"|"evenodd";fill:Fill|null;stroke:PathRasterStroke|null};
export type PathRasterPhase="preparing"|"flattening"|"flattenCleanup"|"contours"|"contoursCleanup"|"stroke"|"strokeCleanup"|"fillCoverage"|"fillCoverageCleanup"|"strokeCoverage"|"strokeCoverageCleanup"|"painting"|"complete";
export type PathRasterProgress={phase:PathRasterPhase;completed:number;total:number;work:number;done:boolean};
const invalid=(message:string):never=>{throw new RangeError(message);};
const coordinate=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
type OwnedContour={points:Vec2[];closed:boolean};
function retireContour(contours:(OwnedContour|Vec2[])[]):boolean {const contour=contours.at(-1);if(!contour)return true;const points=Array.isArray(contour)?contour:contour.points;if(points.length)points.pop();else contours.pop();return false;}

/** 🧱️ Private candidate and work-granted stages; source geometry stays immutable until completion. */
export class PathRasterJob {
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private readonly width:number;private readonly height:number;private readonly count:number;private readonly transform:number[];private readonly tolerance:number;private readonly rule:"nonzero"|"evenodd";private readonly inverse:number[]|null;
 private fill:PreparedFill|null;private fillReady=false;private strokeColor:Color|null;private style:StrokeGeometryStyle|null;
 private source:readonly PathSegment[];private flat:FlatContours|null=null;private flatCursor:RetainedWorkRetirementCursor|null=null;private fillContours:Vec2[][]=[];private strokeContours:OwnedContour[]=[];
 private flatten:PathFlattenJob|null=null;private outline:StrokeOutlineJob|null=null;private coverage:CoverageJob|null=null;private fillMask:CoverageMask|null=null;private strokeMask:CoverageMask|null=null;
 private retiredCoverages:RetainedWorkRetirementCursor[]=[];private outlineRetirement:RetainedWorkRetirementCursor|null=null;private strokePolygons:StrokeContours|null=null;private polygonCursor:RetainedWorkRetirementCursor|null=null;
 private coverageRetirement:RetainedWorkRetirementCursor|null=null;private flattenRetirement:RetainedWorkRetirementCursor|null=null;private fillRetirement:RetainedWorkRetirementCursor|null=null;private maskCursors:RetainedWorkRetirementCursor[]=[];
 private pixels:Uint8Array;private phase:PathRasterPhase="preparing";private at=0;private point=0;private work=0;private completed=0;private total:number;private cancelled=false;private transferred=false;private failed:unknown=null;
 constructor(input:PathRasterInput) {
  this.count=validateExtent(input.width,input.height);this.width=input.width;this.height=input.height;
  if(input.origin.length!==2||!input.origin.every(coordinate)||input.transform.length!==6||!input.transform.every(coordinate)||!["nonzero","evenodd"].includes(input.fillRule)||input.segments.length>65536||!Number.isFinite(input.tolerance)||input.tolerance<1e-6||input.tolerance>16)invalid("Invalid painted path contract");
  this.transform=[...input.transform];this.transform[4]!-=input.origin[0];this.transform[5]!-=input.origin[1];if(!this.transform.every(coordinate))invalid("Path raster origin exceeds coordinate budget");
  this.tolerance=input.tolerance;this.rule=input.fillRule;this.source=input.segments;this.total=input.segments.length;
  if(input.fill&&input.fill.kind!=="solid"&&!(input.fill.kind==="linearGradient"?[input.fill.x1,input.fill.y1,input.fill.x2,input.fill.y2]:[input.fill.cx,input.fill.cy,input.fill.r]).every(coordinate))invalid("Paint coordinates exceed raster budget");
  if(input.stroke?.dash&&input.stroke.dash.length>1024)invalid("Stroke dashes exceed raster budget");
  this.fill=input.fill?new PreparedFill(input.fill):null;this.fillReady=this.fill===null;this.strokeColor=input.stroke?[...input.stroke.color]:null;
  this.style=input.stroke?{width:input.stroke.width,cap:parseStrokeCap(input.stroke.cap),join:parseStrokeJoin(input.stroke.join),miterLimit:4,dash:[...(input.stroke.dash??[])],dashOffset:0}:null;
  if(this.style) {const check=new StrokeOutlineJob({contours:[],transform:this.transform,tolerance:this.tolerance,style:this.style});check.cancel();}
  const [a,b,c,d,e,f]=this.transform as [number,number,number,number,number,number],det=a*d-b*c;
  if(!Number.isFinite(det))invalid("Path raster transform exceeds numeric limits");
  this.inverse=det===0?null:[d/det,-b/det,-c/det,a/det,(c*f-d*e)/det,(b*e-a*f)/det];
  if(this.inverse&&!this.inverse.every(Number.isFinite))invalid("Path raster inverse exceeds numeric limits");
  this.pixels=new Uint8Array(this.count*4);
 }
 private enter(phase:PathRasterPhase,total=0):void {this.phase=phase;this.completed=0;this.total=total;}
 private startFill():void {
  if(this.fill&&this.inverse) {this.coverage=new CoverageJob({width:this.width,height:this.height,transform:this.transform,rule:this.rule,contours:this.fillContours});this.enter("fillCoverage");}
  else this.startStroke();
 }
 private startStroke():void {
  if(this.style&&this.inverse) {this.coverage=new CoverageJob({width:this.width,height:this.height,transform:this.transform,rule:"nonzero",contours:this.strokePolygons!});this.enter("strokeCoverage");}
  else {this.at=0;this.enter("painting",this.count);}
 }
 private step(c:RetainedWorkProducerStep):void {
  switch(this.phase){
   case"preparing":{
    if(!this.fillReady){if(c.advanceChild(this.fill!).done)this.fillReady=true;return;}
    this.flatten=new PathFlattenJob({segments:this.source,transform:this.transform,tolerance:this.tolerance});this.at=0;this.enter("flattening");c.commitSemanticWork();return;
   }
   case"flattening":{
    const p=c.advanceChild(this.flatten!);this.completed=p.completed;this.total=p.total;
    if(p.done) {const retired=this.flatten!.intoRetirement();this.flat=retired.output!;this.flattenRetirement=retired.job;this.flatten=null;this.enter("flattenCleanup",p.points+this.flat.length);}
    return;
   }
   case"flattenCleanup":{if(advanceCursor(this.flattenRetirement,c.work,c.retainedTurn))return;this.flattenRetirement=null;this.enter("contours",this.total);c.commitControlWork();return;}
   case"contours":{
    const flat=this.flat!.at(this.at);
    if(flat) {
     if(this.point===0) {if(this.fill)this.fillContours.push([]);if(this.style)this.strokeContours.push({points:[],closed:flat.closed});}
     if(this.point<flat.points.length) {const p=flat.points.at(this.point++)!;if(this.fill)this.fillContours[this.at]!.push([p[0],p[1]]);if(this.style)this.strokeContours[this.at]!.points.push([p[0],p[1]]);}
     else {this.at++;this.point=0;}
     this.completed++;c.commitSemanticWork();return;
    }
    this.enter("contoursCleanup");c.commitSemanticWork();return;
   }
   case"contoursCleanup":{
    if(this.flat!==null){this.flatCursor=this.flat.intoRetirement();this.flat=null;}
    if(advanceCursor(this.flatCursor,c.work,c.retainedTurn))return;this.flatCursor=null;
    if(this.style){this.outline=new StrokeOutlineJob({contours:this.strokeContours,transform:this.transform,tolerance:this.tolerance,style:this.style});this.enter("stroke");}else this.startFill();c.commitControlWork();return;
   }
   case"stroke":{
    const p=c.advanceChild(this.outline!);this.completed=p.completed;this.total=p.total;
    if(p.done){const retired=this.outline!.intoRetirement();this.strokePolygons=retired.output!;this.outlineRetirement=retired.job;this.outline=null;this.enter("strokeCleanup");}
    return;
   }
   case"strokeCleanup":{if(advanceCursor(this.outlineRetirement,c.work,c.retainedTurn))return;this.outlineRetirement=null;this.startFill();c.commitControlWork();return;}
   case"fillCoverage":case"strokeCoverage":{
    const p=c.advanceChild(this.coverage!);this.completed=p.completed;this.total=p.total;
    if(p.done) {
     const fill=this.phase==="fillCoverage",retired=this.coverage!.intoRetirement();if(this.coverageRetirement)this.retiredCoverages.push(this.coverageRetirement);this.coverageRetirement=retired.job;this.coverage=null;
     if(fill)this.fillMask=retired.output!;else this.strokeMask=retired.output!;this.enter(fill?"fillCoverageCleanup":"strokeCoverageCleanup");
    }
    return;
   }
   case"fillCoverageCleanup":case"strokeCoverageCleanup":{
    if(advanceCursor(this.coverageRetirement,c.work,c.retainedTurn))return;this.coverageRetirement=null;
    if(this.phase==="fillCoverageCleanup")this.startStroke();else{this.at=0;this.enter("painting",this.count);}c.commitControlWork();return;
   }
   case"painting":{
    if(this.at===this.count) {this.enter("complete",this.count);this.completed=this.count;c.commitSemanticWork();return;}
    const at=this.at++,fa=this.fillMask?.borrow()[at]??0,sa=this.strokeMask?.borrow()[at]??0;
    let color:Color=[0,0,0,0];
    if(fa&&this.fill&&this.inverse) {const m=this.inverse,x=at%this.width+.5,y=Math.floor(at/this.width)+.5;color=this.fill.sample([m[0]!*x+m[2]!*y+m[4]!,m[1]!*x+m[3]!*y+m[5]!]);color[3]*=fa/255;}
    const alpha=(this.strokeColor?.[3]??0)*sa/255,outputAlpha=alpha+color[3]*(1-alpha);
    if(outputAlpha>0) {for(let ch=0;ch<3;ch++)this.pixels[at*4+ch]=Math.round(255*(alpha*(this.strokeColor?.[ch]??0)+(1-alpha)*color[3]*color[ch]!)/outputAlpha);this.pixels[at*4+3]=Math.round(outputAlpha*255);}
    this.completed=this.at;c.commitSemanticWork();return;
   }
   default:throw Error("Unknown path raster phase");
  }
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failed!==null||this.fill?.failurePending()===true||this.flatten?.failurePending()===true||this.outline?.failurePending()===true||this.coverage?.failurePending()===true;}
 settlementPending():boolean{switch(this.phase){case"complete":return true;case"preparing":return !this.fillReady&&this.fill!.settlementPending();case"flattening":return this.flatten?.settlementPending()===true;case"stroke":return this.outline?.settlementPending()===true;case"fillCoverage":case"strokeCoverage":return this.coverage?.settlementPending()===true;default:return false;}}
 progress():PathRasterProgress {return {phase:this.phase,completed:this.completed,total:this.total,work:this.work,done:this.phase==="complete"};}
 advance(items:number,work:RetainedWorkLedger):PathRasterProgress {
  try{return this.#runtime.advance(items,work,{maximumWork:Number.MAX_SAFE_INTEGER,workLimitMessage:"Path raster work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Path raster cancelled","AbortError");if(this.failed!==null)throw this.failed;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failed=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>this.progress()});}
  catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failed===null&&!this.cancelled)this.failed=error;throw error;}
 }
 cancel():void {this.cancelled=true;}
 result():PixelImage {if(this.cancelled||this.transferred)throw new DOMException("Path raster cancelled","AbortError");if(this.failed)throw this.failed;if(this.phase!=="complete")throw Error("Path raster is incomplete");return {width:this.width,height:this.height,pixels:this.pixels};}
 /** 🧹️ Plain pixels carry no funded owner; the published image closes in one actual step. */
 retireOutput(_output:PixelImage):RetainedWorkRetirementCursor {let done=false;return new RetainedWorkRetirement((work,turn)=>{if(done)return true;if(!work.canStep())return false;done=true;turn.commitActualWork();return false;});}
 /** 🧹️ Consumes actual child owners and moves only completed valid pixels before granted private cleanup. */
 intoRetirement():{job:RetainedWorkRetirementCursor;output:PixelImage|null}{
  if(this.transferred)invalid("Path raster ownership already transferred");this.transferred=true;const output=this.phase==="complete"&&!this.cancelled&&!this.failed?{width:this.width,height:this.height,pixels:this.pixels}:null;if(output)this.pixels=new Uint8Array(0);this.cancelled=true;
  if(this.fill&&!this.fillReady)this.fill.cancel();
  if(this.flatten){this.flatten.cancel();this.flattenRetirement=this.flatten.intoRetirement().job;this.flatten=null;}
  if(this.outline){this.outline.cancel();this.outlineRetirement=this.outline.intoRetirement().job;this.outline=null;}
  if(this.coverage){this.coverage.cancel();if(this.coverageRetirement)this.retiredCoverages.push(this.coverageRetirement);this.coverageRetirement=this.coverage.intoRetirement().job;this.coverage=null;}
  if(this.fill){this.fillRetirement=this.fill.intoRetirement();this.fill=null;}
  if(this.fillMask){this.maskCursors.push(this.fillMask.intoRetirement());this.fillMask=null;}
  if(this.strokeMask){this.maskCursors.push(this.strokeMask.intoRetirement());this.strokeMask=null;}
  let slot=0;
  const job=new RetainedWorkRetirement((work,turn)=>{
   if(slot===5&&this.flat!==null){this.flatCursor=this.flat.intoRetirement();this.flat=null;}
   if(slot===9&&this.strokePolygons!==null){this.polygonCursor=this.strokePolygons.intoRetirement();this.strokePolygons=null;}
   const cursor=slot===0?this.flattenRetirement:slot===1?this.outlineRetirement:slot===2?(this.retiredCoverages.at(-1)??this.coverageRetirement):slot===3?this.fillRetirement:slot===4?(this.maskCursors.at(-1)??null):slot===5?this.flatCursor:slot===9?this.polygonCursor:null;
   if(advanceCursor(cursor,work,turn))return false;
   if(!work.canStep())return false;
   turn.commitActualWork();let complete=true;
   switch(slot){
    case 0:this.flattenRetirement=null;break;case 1:this.outlineRetirement=null;break;
    case 2:if(this.retiredCoverages.length){this.retiredCoverages.pop();complete=false;}else this.coverageRetirement=null;break;
    case 3:this.fillRetirement=null;break;
    case 4:if(this.maskCursors.length){this.maskCursors.pop();complete=false;}break;
    case 5:this.flatCursor=null;break;
    case 6:complete=retireContour(this.fillContours);break;case 7:complete=retireContour(this.strokeContours);break;
    case 8:this.source=[];break;
    case 9:this.polygonCursor=null;break;
    case 10:if(this.style)this.style={...this.style,dash:[]};break;case 11:this.style=null;this.strokeColor=null;break;case 12:this.pixels=new Uint8Array(0);break;case 13:this.transform.length=0;if(this.inverse)this.inverse.length=0;this.failed=null;break;
    default:return true;
   }
   if(complete)slot++;return false;
  });
  return{job,output};
 }
}
