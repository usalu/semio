/** 📤️ Exports the authoritative immutable document scene through yielded raster and PNG owners. */
import type {DrawingArtifact} from "../../../🧬️schema/🟦️.ts";
import {DocumentRasterJob,type DocumentSceneLimits,type DocumentAlgorithmLimits,type DocumentRasterProgress} from "../../../🧬️schema/🎬️scene/📋️prepare/🟦️.ts";
import {PngEncodeJob,PngEncodedBytes,type PngEncodeProgress} from "@semio-tech/framework/pixels/png/encode";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkTurn} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {DrawingSceneWorkController,driveDrawingSceneWork,resumeDrawingSceneWorkClose} from "../../../🧬️schema/🎬️scene/🎟️work/🟦️.ts";
import {binary64Value} from "@semio-tech/framework/value/ieee754";
import type {PixelImage} from "@semio-tech/framework/pixels/editing";
import {loadBuiltinUnicodeSource} from "@semio-tech/framework/2d/text/font/shape/unicode";
import {loadBuiltinFontSources,FONT_CATALOG_DEFAULT_FAMILY,type FontByteSource,type FontByteProvider} from "@semio-tech/framework/2d/text/font/catalog";
export type DrawingPngOptions={width?:number|null;height?:number|null;transparent?:boolean|null};
export type DrawingPngLimits={maxPixels:number;maxSourceBytes:number;maxWork:number;maxEncodedBytes:number};
export const DRAWING_PNG_LIMITS:DrawingPngLimits={maxPixels:67108864,maxSourceBytes:268439552,maxWork:1e9,maxEncodedBytes:67108864};
export type DrawingPngProgress={phase:"raster"|"background"|"encode"|"closing"|"complete";raster:DocumentRasterProgress|null;encoding:PngEncodeProgress|null;work:number;done:boolean};
const abort=():never=>{throw new DOMException("Drawing PNG export cancelled","AbortError");};
/** 🧾️ Every dimension, pixel and output limit is checked before a private candidate is admitted. */
export function drawingPngExtent(document:DrawingArtifact,options:DrawingPngOptions={},limits:DrawingPngLimits=DRAWING_PNG_LIMITS):{width:number;height:number;scale:[number,number]}{
 const worldWidth=document.artboard?binary64Value(document.artboard.width):1024,worldHeight=document.artboard?binary64Value(document.artboard.height):1024;
 if(![worldWidth,worldHeight].every(n=>Number.isFinite(n)&&n>0&&n<=1e9))throw RangeError("Invalid PNG artboard extent");
 const width=options.width??Math.ceil(worldWidth),height=options.height??Math.ceil(worldHeight);
 if(![width,height].every(n=>Number.isSafeInteger(n)&&n>0&&n<=16384)||width*height>limits.maxPixels||![limits.maxPixels,limits.maxSourceBytes,limits.maxWork,limits.maxEncodedBytes].every(Number.isSafeInteger)||limits.maxPixels<1||limits.maxPixels>67108864||limits.maxSourceBytes<1||limits.maxSourceBytes>268439552||limits.maxWork<1||limits.maxWork>1e9||limits.maxEncodedBytes<8||limits.maxEncodedBytes>67108864||options.transparent!=null&&typeof options.transparent!=="boolean")throw RangeError("Invalid PNG export contract");
 return{width,height,scale:[width/worldWidth,height/worldHeight]};
}
function drain(cursor:RetainedWorkRetirementCursor,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{const before=turn.actualWork();let done=false;try{done=cursor.advance(1,work).done;}finally{turn.commitActualWork(cursor.lastTurnWork());}return done&&turn.actualWork()===before;}
/** 🖼️ Retains the true scene producer, background pixels and encoder until complete publication or close. */
export class DrawingPngExportJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private raster:DocumentRasterJob|null;private encoder:PngEncodeJob|null=null;private image:PixelImage|null=null;private output:PngEncodedBytes|null=null;private closing:RetainedWorkRetirementCursor|null=null;private afterClose:DrawingPngProgress["phase"]="background";private phase:DrawingPngProgress["phase"]="raster";private rasterProgress:DocumentRasterProgress|null=null;private encodingProgress:PngEncodeProgress|null=null;private pixel=0;private work=0;private failure:unknown=null;private aborted=false;private transferred=false;
 constructor(document:DrawingArtifact,scene:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits,private options:DrawingPngOptions={},private limits:DrawingPngLimits=DRAWING_PNG_LIMITS){const extent=drawingPngExtent(document,options,limits);this.options={...options};this.limits={...limits};this.raster=new DocumentRasterJob(document,{...scene,maxSourceBytes:Math.min(scene.maxSourceBytes,limits.maxSourceBytes)},{width:extent.width,height:extent.height,origin:[0,0],tolerance:.01,maxPixels:limits.maxPixels,maxSourceBytes:limits.maxSourceBytes},{...algorithms,maxWork:Math.min(algorithms.maxWork,limits.maxWork)});this.raster.setPixelScale(...extent.scale);}
 private step(c:RetainedWorkProducerStep):void{
  if(this.closing!==null){let done=false;try{done=this.closing.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.closing.lastTurnWork());}if(!done||c.hasActualWork())return;this.closing=null;this.phase=this.afterClose;c.commitControlWork();return;}
  if(this.phase==="raster"){this.rasterProgress=c.advanceChild(this.raster!);if(this.rasterProgress.done){const moved=this.raster!.intoRetirement();this.closing=moved.job;this.afterClose="background";this.image=moved.output!;this.raster=null;}return;}
  if(this.phase==="background"){if(this.options.transparent===false&&this.pixel<this.image!.pixels.length){const p=this.image!.pixels,at=this.pixel,alpha=p[at+3]!;for(let ch=0;ch<3;ch++)p[at+ch]=Math.round(p[at+ch]!*alpha/255+255-alpha);p[at+3]=255;this.pixel+=4;c.commitSemanticWork();return;}this.encoder=new PngEncodeJob(this.image!,this.limits.maxEncodedBytes);this.image=null;this.phase="encode";c.commitSemanticWork();return;}
  if(this.phase==="encode"){this.encodingProgress=c.advanceChild(this.encoder!);if(this.encodingProgress.done){const moved=this.encoder!.intoRetirement();this.output=moved.output!;this.closing=moved.job;this.afterClose="complete";this.encoder=null;this.phase="closing";}return;}
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.raster?.failurePending()===true||this.encoder?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="complete"||this.closing!==null||this.raster?.settlementPending()===true||this.encoder?.settlementPending()===true;}
 advance(items:number,work:RetainedWorkLedger):DrawingPngProgress{
  try{return this.#runtime.advance(items,work,{maximumWork:this.limits.maxWork,workLimitMessage:"PNG export work limit exceeded",validate:()=>{if(this.aborted||this.transferred)abort();if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete"||this.needsFontSources(),settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,raster:this.rasterProgress,encoding:this.encodingProgress,work:this.work,done:this.phase==="complete"})});}
  catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.aborted)this.failure=error;throw error;}
 }
 result():PngEncodedBytes{if(this.aborted||this.transferred)abort();if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("PNG export incomplete");return this.output!;}
 needsFontSources():boolean{return this.raster?.needsFontSources()??false;}
 fontSourceFamily():string{return this.raster!.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.raster)throw Error("Text source phase inactive");this.raster.admitFontSources(sources,unicodeBytes);}
 cancel():void{this.aborted=true;}
 retireOutput(output:PngEncodedBytes):RetainedWorkRetirementCursor{return output;}
 /** 🧹️ Publication and cancellation transfer the genuine children and unpublished output to retained close. */
 intoRetirement():{job:RetainedWorkRetirementCursor;output:PngEncodedBytes|null}{
  if(this.transferred)throw Error("PNG export ownership already transferred");this.transferred=true;const cursors:RetainedWorkRetirementCursor[]=[];
  if(this.closing)cursors.push(this.closing);this.closing=null;
  if(this.raster){const moved=this.raster.intoRetirement();cursors.push(moved.job);this.raster=null;}
  if(this.encoder){const moved=this.encoder.intoRetirement();cursors.push(moved.job);if(moved.output)cursors.push(moved.output);this.encoder=null;}
  const output=this.phase==="complete"&&!this.failure&&!this.aborted?this.output:null;if(!output&&this.output)cursors.push(this.output);this.output=null;this.image=null;this.aborted=true;
  return{output,job:new RetainedWorkRetirement((work,turn)=>{const cursor=cursors.at(-1);if(cursor){if(!drain(cursor,work,turn))return false;cursors.pop();turn.commitActualWork();return false;}return true;})};
 }
}
/** ⏳️ Observers may abort at every grant; the byte owner is copied out and then closed under the same caller funding. */
export async function exportDrawingPng(document:DrawingArtifact,scene:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits,options:DrawingPngOptions,control:{work:DrawingSceneWorkController;operation:object;signal?:AbortSignal;fontProvider?:FontByteProvider;onProgress?:(progress:DrawingPngProgress)=>void;limits?:DrawingPngLimits}):Promise<Uint8Array>{
 const check=()=>{if(control.signal?.aborted)abort();};check();control.work.checkBinding(control.operation,document,"advance");const job=new DrawingPngExportJob(document,scene,algorithms,options,control.limits);let unicodeBytes:Uint8Array|null=null;
 const owned=await driveDrawingSceneWork(control.work,control.operation,document,{advance:(items,work)=>job.advance(items,work),nextWorkDemand:()=>job.nextWorkDemand(),lastTurnWork:()=>job.lastTurnWork(),cancel:()=>job.cancel(),intoRetirement:()=>job.intoRetirement(),retireOutput:output=>job.retireOutput(output),beforeTurn:async()=>{check();if(job.needsFontSources()){unicodeBytes??=await loadBuiltinUnicodeSource(control.fontProvider,control.signal);check();job.admitFontSources(await loadBuiltinFontSources(job.fontSourceFamily(),control.fontProvider,{signal:control.signal}),unicodeBytes);check();}}},control.onProgress);
 const bytes=Uint8Array.from(owned.bytes);await resumeDrawingSceneWorkClose(control.work,control.operation,document,owned);check();return bytes;
}
