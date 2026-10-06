/** 🖼️ Affine sampling with premultiplied bilinear and per-cell area filtering. */
import {validateExtent,validateImage,type PixelImage} from "../../✍️editing/🟦️.ts";
import {CoverageJob,type CoverageMask} from "../../🖊️coverage/🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../../◻️2d/🧹️retire/🟦️.ts";
export type AffineSampling="nearest"|"bilinear"|"area"|"auto";
export type AffineImageInput={source:PixelImage;width:number;height:number;origin:readonly[number,number];transform:readonly number[];sampling:AffineSampling};
export type AffineImageProgress={phase:"coverage"|"coverageCleanup"|"sampling"|"complete";completed:number;total:number;sampled:number;sampleTotal:number;work:number;done:boolean};
export type AffineImageOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:AffineImageProgress)=>void};
const coordinate=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
const byte=(v:number)=>Math.round(Math.max(0,Math.min(255,v)));
const invalid=(message:string):never=>{throw new RangeError(message);};
/** ⏱️ Exact image boundary coverage with one source-cell visit per reduction work unit. */
export class AffineImageJob {
 private source:PixelImage|null;private output:PixelImage;private coverage:CoverageJob|null;private mask:CoverageMask|null=null;
 private coverageRetirement:WorkRetirement|null=null;private coverageContour:[number,number][]|null=null;private outputExposed=false;private transferred=false;
 private inverse:number[]|null=null;private determinant=0;private filter:AffineSampling;
 private phase:AffineImageProgress["phase"]="coverage";private at=0;private work=0;private cancelled=false;private failed:unknown=null;
 private sampled=0;private sampleTotal=0;private active=false;private x=0;private y=0;private left=0;private right=0;private bottom=0;
 private quad=new Float64Array(8);private scratchA=new Float64Array(24);private scratchB=new Float64Array(24);private sums=new Float64Array(4);
 constructor(input:AffineImageInput) {
  const count=validateExtent(input.width,input.height);validateImage(input.source);
  if(input.origin.length!==2||!input.origin.every(coordinate)||input.transform.length!==6||!input.transform.every(coordinate)||!["nearest","bilinear","area","auto"].includes(input.sampling))invalid("Invalid affine image contract");
  const m=[...input.transform];m[4]!-=input.origin[0];m[5]!-=input.origin[1];if(!m.every(coordinate))invalid("Affine viewport exceeds coordinate budget");
  this.source=input.source;this.output={width:input.width,height:input.height,pixels:new Uint8Array(count*4)};this.filter=input.sampling;
  const [a,b,c,d,e,f]=m as [number,number,number,number,number,number],det=a*d-b*c;this.determinant=Math.abs(det);
  if(det!==0) {
   const inverse=[d/det,-b/det,-c/det,a/det,(c*f-d*e)/det,(b*e-a*f)/det];
   if(!inverse.every(Number.isFinite))invalid("Affine inverse exceeds numeric limits");this.inverse=inverse;
   if(this.filter==="auto"){const u=inverse[0]!*inverse[0]!+inverse[1]!*inverse[1]!,v=inverse[2]!*inverse[2]!+inverse[3]!*inverse[3]!,cross=inverse[0]!*inverse[2]!+inverse[1]!*inverse[3]!;this.filter=(u+v+Math.hypot(u-v,2*cross))/2>1+1e-12?"area":"bilinear";}
   const w=input.source.width,h=input.source.height;this.coverageContour=[[0,0],[w,0],[w,h],[0,h]];this.coverage=new CoverageJob({width:input.width,height:input.height,transform:m,rule:"nonzero",contours:[this.coverageContour]});
  }else {this.coverage=null;this.phase="sampling";}
 }
 private point(x:number,y:number):[number,number] {const m=this.inverse!;const p:[number,number]=[m[0]!*x+m[2]!*y+m[4]!,m[1]!*x+m[3]!*y+m[5]!];if(!p.every(Number.isFinite))invalid("Sample footprint exceeds numeric limits");return p;}
 private write(alpha:number):void {
  const pixels=this.output.pixels,at=this.at*4,a=byte(alpha);
  if(a&&this.sums[3]!>0){for(let c=0;c<3;c++)pixels[at+c]=byte(this.sums[c]!/this.sums[3]!);pixels[at+3]=a;}
  this.at++;this.active=false;if(this.at===this.output.width*this.output.height)this.phase="complete";
 }
 private accumulate(x:number,y:number,weight:number):void {const source=this.source!,at=(y*source.width+x)*4,alpha=source.pixels[at+3]!*weight;this.sums[3]!+=alpha;for(let c=0;c<3;c++)this.sums[c]!+=source.pixels[at+c]!*alpha;}
 private area(x:number,y:number):number {
  let a=this.scratchA,b=this.scratchB,count=4;a.set(this.quad);
  for(let side=0;side<4&&count;side++) {
   const axis=side<2?0:1,bound=side===0?x:side===1?x+1:side===2?y:y+1,lower=side===0||side===2;let n=0;
   for(let i=0;i<count;i++) {
    const previous=(i+count-1)%count,px=a[previous*2]!,py=a[previous*2+1]!,qx=a[i*2]!,qy=a[i*2+1]!,p=axis===0?px:py,q=axis===0?qx:qy,insideP=lower?p>=bound:p<=bound,insideQ=lower?q>=bound:q<=bound;
    if(insideP!==insideQ){const t=(bound-p)/(q-p);b[n*2]=px+t*(qx-px);b[n*2+1]=py+t*(qy-py);n++;}
    if(insideQ){b[n*2]=qx;b[n*2+1]=qy;n++;}
   }
   count=n;const temporary=a;a=b;b=temporary;
  }
  let area=0;for(let i=1;i+1<count;i++)area+=(a[i*2]!-a[0]!)*(a[(i+1)*2+1]!-a[1]!)-(a[i*2+1]!-a[1]!)*(a[(i+1)*2]!-a[0]!);return Math.abs(area)/2;
 }
 private step():void {
  if(this.phase==="coverage"){if(this.coverage!.advance(1).done){const retired=this.coverage!.intoRetirement();this.mask=retired.output;this.coverageRetirement=retired.job;this.coverage=null;this.phase="coverageCleanup";}return;}
  if(this.phase==="coverageCleanup"){if(this.coverageRetirement){if(!this.coverageRetirement.terminalIsEmpty())this.coverageRetirement.advance(1);else this.coverageRetirement=null;}else if(this.coverageContour){if(this.coverageContour.length)this.coverageContour.pop();else this.coverageContour=null;}else this.phase="sampling";return;}
  if(this.active) {
   this.accumulate(this.x,this.y,this.area(this.x,this.y));this.sampled++;this.x++;
   if(this.x>=this.right){this.x=this.left;this.y++;}if(this.y>=this.bottom)this.write(this.sums[3]!*this.determinant);return;
  }
  const coverage=this.mask?.coverage[this.at]??0;this.sums.fill(0);
  if(!this.inverse||!coverage){this.write(0);return;}
  const source=this.source!,x=this.at%this.output.width,y=Math.floor(this.at/this.output.width);
  if(this.filter==="area") {
   let minX=Infinity,minY=Infinity,maxX=-Infinity,maxY=-Infinity;
   for(let i=0;i<4;i++){const p=this.point(x+(i===1||i===2?1:0),y+(i>=2?1:0));this.quad[i*2]=p[0];this.quad[i*2+1]=p[1];minX=Math.min(minX,p[0]);maxX=Math.max(maxX,p[0]);minY=Math.min(minY,p[1]);maxY=Math.max(maxY,p[1]);}
   this.left=Math.max(0,Math.min(source.width,Math.floor(minX)));this.right=Math.max(0,Math.min(source.width,Math.ceil(maxX)));this.y=Math.max(0,Math.min(source.height,Math.floor(minY)));this.bottom=Math.max(0,Math.min(source.height,Math.ceil(maxY)));this.x=this.left;
   this.sampled=0;this.sampleTotal=(this.right-this.left)*(this.bottom-this.y);this.active=this.sampleTotal>0;if(!this.active)this.write(0);return;
  }
  const p=this.point(x+.5,y+.5),clampX=(v:number)=>Math.max(0,Math.min(source.width-1,v)),clampY=(v:number)=>Math.max(0,Math.min(source.height-1,v));
  if(this.filter==="nearest")this.accumulate(clampX(Math.floor(p[0])),clampY(Math.floor(p[1])),1);
  else {const sx=clampX(p[0]-.5),sy=clampY(p[1]-.5),left=Math.floor(sx),top=Math.floor(sy),dx=sx-left,dy=sy-top;this.accumulate(left,top,(1-dx)*(1-dy));this.accumulate(clampX(left+1),top,dx*(1-dy));this.accumulate(left,clampY(top+1),(1-dx)*dy);this.accumulate(clampX(left+1),clampY(top+1),dx*dy);}
  this.write(this.sums[3]!*coverage/255);
 }
 advance(budget:number):AffineImageProgress {
  if(!Number.isSafeInteger(budget)||budget<=0)invalid("Affine sample work grant must be a positive integer");
  this.check();try{for(let i=0;i<budget&&this.phase!=="complete";i++){this.step();this.work++;}}catch(error){this.failed=error;throw error;}
  return {phase:this.phase,completed:this.at,total:this.output.width*this.output.height,sampled:this.sampled,sampleTotal:this.sampleTotal,work:this.work,done:this.phase==="complete"};
 }
 private check():void {if(this.cancelled)throw new DOMException("Affine sample job cancelled","AbortError");if(this.failed)throw this.failed;}
 cancel():void {this.cancelled=true;if(this.outputExposed)this.output={...this.output,pixels:new Uint8Array(0)};}
 result():PixelImage {this.check();if(this.phase!=="complete")throw Error("Affine sample job is incomplete");this.outputExposed=true;return this.output;}
 /** 🧹️ Moves complete pixels and grants the genuine coverage and sampling owners. */
 intoRetirement():{job:WorkRetirement;output:PixelImage|null}{
  if(this.transferred)throw Error("Affine sample ownership was already transferred");this.transferred=true;
  const output=!this.cancelled&&!this.failed&&this.phase==="complete"?this.output:null;if(output)this.output={...this.output,pixels:new Uint8Array(0)};
  this.cancelled=true;if(this.coverage){this.coverage.cancel();this.coverageRetirement=this.coverage.intoRetirement().job;this.coverage=null;}
  let slot=0;
  const job=new UnitRetirement(()=>{
   if(slot===0){if(this.coverageRetirement){if(!this.coverageRetirement.terminalIsEmpty()){this.coverageRetirement.advance(1);return false;}this.coverageRetirement=null;}}
   else if(slot===1){if(this.coverageContour?.length){this.coverageContour.pop();return false;}this.coverageContour=null;}
   else if(slot===2)this.mask=null;
   else if(slot===3)this.source=null;
   else if(slot===4)this.output={...this.output,pixels:new Uint8Array(0)};
   else if(slot===5)this.inverse=null;
   else if(slot===6)this.quad=new Float64Array(0);
   else if(slot===7)this.scratchA=new Float64Array(0);
   else if(slot===8)this.scratchB=new Float64Array(0);
   else if(slot===9)this.sums=new Float64Array(0);
   else{this.failed=null;this.active=false;}
   slot++;return slot===11;
  });return{job,output};
 }
}
/** 🕰️ Yields between grants and exposes only a completed image. */
export async function sampleAffineImage(input:AffineImageInput,options:AffineImageOptions={}):Promise<PixelImage> {
 const abort=()=>{if(options.signal?.aborted)throw new DOMException("Affine sample job cancelled","AbortError");};abort();const job=new AffineImageJob(input);
 try{for(;;){abort();const progress=job.advance(options.workBudget??4096);options.onProgress?.(progress);abort();if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();throw error;}finally{const retired=job.intoRetirement().job,grant=Number.isSafeInteger(options.workBudget)&&options.workBudget!>0?options.workBudget!:4096;while(!retired.terminalIsEmpty()){retired.advance(grant);if(!retired.terminalIsEmpty())await new Promise<void>(resolve=>setTimeout(resolve,0));}}
}
