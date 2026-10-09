/** 📷️ Resumable authored path coverage and local paint into straight RGBA. */
import {PathFlattenJob,type FlatContour} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🟦️.ts";
import {StrokeOutlineJob,type StrokeGeometryStyle} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧹️retire/🟦️.ts";
import {CoverageJob,type CoverageMask} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🟦️.ts";
import {validateExtent,type PixelImage} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/✍️editing/🟦️.ts";
import type {PathSegment,Vec2} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🟦️.ts";
import {PreparedFill} from "../../🎨️fill/🎨️sampling/🟦️.ts";
import type {Fill,Color} from "../../🎨️fill/🟦️.ts";
import {parseStrokeCap,parseStrokeJoin,type StrokeCap,type StrokeJoin} from "../../🖊️stroke/🟦️.ts";
export type PathRasterStroke={readonly color:Readonly<Color>;readonly width:number;readonly cap:StrokeCap;readonly join:StrokeJoin;readonly dash?:readonly number[]|null};
export type PathRasterInput={width:number;height:number;origin:readonly [number,number];segments:readonly PathSegment[];transform:readonly number[];tolerance:number;fillRule:"nonzero"|"evenodd";fill:Fill|null;stroke:PathRasterStroke|null};
export type PathRasterPhase="preparing"|"flattening"|"flattenCleanup"|"contours"|"contoursCleanup"|"stroke"|"strokeCleanup"|"fillCoverage"|"fillCoverageCleanup"|"strokeCoverage"|"strokeCoverageCleanup"|"painting"|"complete";
export type PathRasterProgress={phase:PathRasterPhase;completed:number;total:number;work:number;done:boolean};
export type PathRasterOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:PathRasterProgress)=>void};
const invalid=(message:string):never=>{throw new RangeError(message);};
const coordinate=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
const abort=():never=>{throw new DOMException("Path raster cancelled","AbortError");};
function retireContour(contours:(FlatContour|Vec2[])[]):boolean {const contour=contours.at(-1);if(!contour)return true;const points=Array.isArray(contour)?contour:contour.points;if(points.length)points.pop();else contours.pop();return false;}

/** 🧱️ Private candidate and work-granted stages; source geometry stays immutable until completion. */
export class PathRasterJob {
 private readonly width:number;private readonly height:number;private readonly count:number;private readonly transform:number[];private readonly tolerance:number;private readonly rule:"nonzero"|"evenodd";private readonly inverse:number[]|null;
 private fill:PreparedFill|null;private strokeColor:Color|null;private style:StrokeGeometryStyle|null;
 private source:readonly PathSegment[];private segments:PathSegment[]=[];private flat:FlatContour[]=[];private fillContours:Vec2[][]=[];private strokeContours:FlatContour[]=[];
 private flatten:PathFlattenJob|null=null;private outline:StrokeOutlineJob|null=null;private coverage:CoverageJob|null=null;private fillMask:CoverageMask|null=null;private strokeMask:CoverageMask|null=null;
 private retiredCoverages:WorkRetirement[]=[];private outlineRetirement:WorkRetirement|null=null;private strokePolygons:Vec2[][]=[];
 private coverageRetirement:WorkRetirement|null=null;
 private flattenRetirement:WorkRetirement|null=null;
 private fillRetirement:WorkRetirement|null=null;
 private pixels:Uint8Array;private phase:PathRasterPhase="preparing";private at=0;private point=0;private work=0;private completed=0;private total:number;private cancelled=false;private transferred=false;private outputExposed=false;private failed:unknown=null;
 constructor(input:PathRasterInput) {
  this.count=validateExtent(input.width,input.height);this.width=input.width;this.height=input.height;
  if(input.origin.length!==2||!input.origin.every(coordinate)||input.transform.length!==6||!input.transform.every(coordinate)||!["nonzero","evenodd"].includes(input.fillRule)||input.segments.length>65536||!Number.isFinite(input.tolerance)||input.tolerance<1e-6||input.tolerance>16)invalid("Invalid painted path contract");
  this.transform=[...input.transform];this.transform[4]!-=input.origin[0];this.transform[5]!-=input.origin[1];if(!this.transform.every(coordinate))invalid("Path raster origin exceeds coordinate budget");
  this.tolerance=input.tolerance;this.rule=input.fillRule;this.source=input.segments;this.total=input.segments.length;
  if(input.fill&&input.fill.kind!=="solid"&&!(input.fill.kind==="linearGradient"?[input.fill.x1,input.fill.y1,input.fill.x2,input.fill.y2]:[input.fill.cx,input.fill.cy,input.fill.r]).every(coordinate))invalid("Paint coordinates exceed raster budget");
  if(input.stroke?.dash&&input.stroke.dash.length>1024)invalid("Stroke dashes exceed raster budget");
  this.fill=input.fill?new PreparedFill(input.fill):null;this.strokeColor=input.stroke?[...input.stroke.color]:null;
  this.style=input.stroke?{width:input.stroke.width,cap:parseStrokeCap(input.stroke.cap),join:parseStrokeJoin(input.stroke.join),miterLimit:4,dash:[...(input.stroke.dash??[])],dashOffset:0}:null;
  if(this.style) {new PreparedFill({kind:"solid",color:this.strokeColor!});new StrokeOutlineJob({contours:[],transform:this.transform,tolerance:this.tolerance,style:this.style});}
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
  if(this.style&&this.inverse) {this.coverage=new CoverageJob({width:this.width,height:this.height,transform:this.transform,rule:"nonzero",contours:this.strokePolygons});this.enter("strokeCoverage");}
  else {this.at=0;this.enter("painting",this.count);}
 }
 private step():void {
  if(this.phase==="preparing") {
   if(this.at<this.source.length) {this.segments.push(this.source[this.at++]!);this.completed=this.at;}
   else {this.flatten=new PathFlattenJob({segments:this.segments,transform:this.transform,tolerance:this.tolerance});this.at=0;this.enter("flattening");}
  } else if(this.phase==="flattening") {
   const p=this.flatten!.advance(1);this.completed=p.completed;this.total=p.total;
   if(p.done) {const retired=this.flatten!.intoRetirement();this.flat=retired.output!;this.flattenRetirement=retired.job;this.flatten=null;this.enter("flattenCleanup",p.points+this.flat.length);}
  } else if(this.phase==="flattenCleanup") {this.enter("contours",this.total);
  } else if(this.phase==="contours") {
   const c=this.flat[this.at];
   if(c) {
    if(this.point===0) {if(this.fill)this.fillContours.push([]);if(this.style)this.strokeContours.push({points:[],closed:c.closed});}
    if(this.point<c.points.length) {const p=c.points[this.point++]!;if(this.fill)this.fillContours[this.at]!.push([...p]);if(this.style)this.strokeContours[this.at]!.points.push([...p]);}
    else {this.at++;this.point=0;}
    this.completed++;
   } else this.enter("contoursCleanup");
  } else if(this.phase==="contoursCleanup") {
   if(this.style){this.outline=new StrokeOutlineJob({contours:this.strokeContours,transform:this.transform,tolerance:this.tolerance,style:this.style});this.enter("stroke");}else this.startFill();
  } else if(this.phase==="stroke") {
   const p=this.outline!.advance(1);this.completed=p.completed;this.total=p.total;
   if(p.done){const retired=this.outline!.intoRetirement();this.strokePolygons=retired.output!;this.outlineRetirement=retired.job;this.outline=null;this.enter("strokeCleanup");}
  } else if(this.phase==="strokeCleanup") {this.startFill();
  } else if(this.phase==="fillCoverage"||this.phase==="strokeCoverage") {
   const p=this.coverage!.advance(1);this.completed=p.completed;this.total=p.total;
   if(p.done) {
    const fill=this.phase==="fillCoverage",retired=this.coverage!.intoRetirement();if(this.coverageRetirement)this.retiredCoverages.push(this.coverageRetirement);this.coverageRetirement=retired.job;this.coverage=null;
    if(fill)this.fillMask=retired.output!;else this.strokeMask=retired.output!;this.enter(fill?"fillCoverageCleanup":"strokeCoverageCleanup");
   }
  } else if(this.phase==="fillCoverageCleanup"||this.phase==="strokeCoverageCleanup") {
   if(this.phase==="fillCoverageCleanup")this.startStroke();else{this.at=0;this.enter("painting",this.count);}
  } else if(this.phase==="painting") {
   if(this.at===this.count) {this.enter("complete",this.count);this.completed=this.count;return;}
   const at=this.at++,fa=this.fillMask?.coverage[at]??0,sa=this.strokeMask?.coverage[at]??0;
   let color:Color=[0,0,0,0];
   if(fa&&this.fill&&this.inverse) {const m=this.inverse,x=at%this.width+.5,y=Math.floor(at/this.width)+.5;color=this.fill.sample([m[0]!*x+m[2]!*y+m[4]!,m[1]!*x+m[3]!*y+m[5]!]);color[3]*=fa/255;}
   const alpha=(this.strokeColor?.[3]??0)*sa/255,outputAlpha=alpha+color[3]*(1-alpha);
   if(outputAlpha>0) {for(let c=0;c<3;c++)this.pixels[at*4+c]=Math.round(255*(alpha*(this.strokeColor?.[c]??0)+(1-alpha)*color[3]*color[c]!)/outputAlpha);this.pixels[at*4+3]=Math.round(outputAlpha*255);}
   this.completed=this.at;
  }
 }
 advance(budget:number):PathRasterProgress {
  if(!Number.isSafeInteger(budget)||budget<=0)invalid("Path raster work grant must be a positive integer");
  if(this.cancelled)abort();if(this.failed)throw this.failed;
  try {for(let at=0;at<budget&&this.phase!=="complete";at++) {this.step();this.work++;}}catch(error){this.failed=error;throw error;}
  return {phase:this.phase,completed:this.completed,total:this.total,work:this.work,done:this.phase==="complete"};
 }
 cancel():void {this.cancelled=true;if(!this.transferred&&this.outputExposed)this.pixels=new Uint8Array(0);}
 result():PixelImage {if(this.cancelled)abort();if(this.failed)throw this.failed;if(this.phase!=="complete")throw Error("Path raster is incomplete");this.outputExposed=true;return {width:this.width,height:this.height,pixels:this.pixels};}
 /** 🧹️ Consumes actual child owners and moves only completed valid pixels before granted private cleanup. */
 intoRetirement():{job:WorkRetirement;output:PixelImage|null}{
  if(this.transferred)invalid("Path raster ownership already transferred");this.transferred=true;const output=this.phase==="complete"&&!this.cancelled&&!this.failed?{width:this.width,height:this.height,pixels:this.pixels}:null;if(output)this.pixels=new Uint8Array(0);this.cancelled=true;
  if(this.flatten){this.flatten.cancel();this.flattenRetirement=this.flatten.intoRetirement().job;this.flatten=null;}
  if(this.outline){this.outline.cancel();this.outlineRetirement=this.outline.intoRetirement().job;this.outline=null;}
  if(this.coverage){this.coverage.cancel();if(this.coverageRetirement)this.retiredCoverages.push(this.coverageRetirement);this.coverageRetirement=this.coverage.intoRetirement().job;this.coverage=null;}
  if(this.fill){this.fillRetirement=this.fill.intoRetirement();this.fill=null;}let slot=0;
  const child=(key:"flattenRetirement"|"outlineRetirement"|"coverageRetirement"|"fillRetirement"):boolean=>{const owner=this[key];if(owner&&!owner.terminalIsEmpty()){owner.advance(1);return false;}this[key]=null;return true;};
  const step=():boolean=>{let complete=true;switch(slot){
   case 0:complete=child("flattenRetirement");break;case 1:complete=child("outlineRetirement");break;case 2:if(this.retiredCoverages.length){const owner=this.retiredCoverages.at(-1)!;if(owner.terminalIsEmpty())this.retiredCoverages.pop();else owner.advance(1);complete=false;}else complete=child("coverageRetirement");break;case 3:complete=child("fillRetirement");break;
   case 4:this.source=[];break;case 5:this.segments=[];break;
   case 6:complete=retireContour(this.flat);if(complete)this.flat=[];break;case 7:complete=retireContour(this.fillContours);if(complete)this.fillContours=[];break;case 8:complete=retireContour(this.strokeContours);if(complete)this.strokeContours=[];break;case 9:complete=retireContour(this.strokePolygons);if(complete)this.strokePolygons=[];break;
   case 10:this.fillMask=null;break;case 11:this.strokeMask=null;break;case 12:if(this.style)this.style={...this.style,dash:[]};break;case 13:this.style=null;this.strokeColor=null;break;case 14:this.pixels=new Uint8Array(0);break;case 15:this.transform.length=0;if(this.inverse)this.inverse.length=0;this.failed=null;break;
  }if(complete)slot++;return slot===16;};return{job:new UnitRetirement(step),output};
 }
}
/** ⏳️ Paints a private candidate, yields between grants and publishes only after completion. */
export async function rasterizePath(input:PathRasterInput,options:PathRasterOptions={}):Promise<PixelImage> {
 const check=()=>{if(options.signal?.aborted)abort();};check();const job=new PathRasterJob(input);
 try {for(;;) {check();const p=job.advance(options.workBudget??4096);options.onProgress?.(p);check();if(p.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}
 catch(error){job.cancel();throw error;}finally{const retired=job.intoRetirement(),grant=Number.isSafeInteger(options.workBudget)&&options.workBudget!>0?options.workBudget!:4096;while(!retired.job.advance(grant).done)await new Promise<void>(resolve=>setTimeout(resolve,0));}
}
