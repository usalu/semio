import {DocumentSceneJob,type DrawingSceneSource,type DocumentSceneLimits,type DocumentSceneProgress} from "./🎟️owned/🟦️.ts";
import {DrawingAssetsOwner} from '../../🟦️.ts';
import { parseDrawingFontFamily } from "../../📝️text/🔤️family/🟦️.ts";
/** 📋️ Complete authored documents become private typed preparation plans under work grants. */
import {binary64Value} from "@semio-tech/framework/value/ieee754";
import type {DrawingArtifact,DrawingLayerNode,DrawingPathSegment} from "../../🟦️.ts";
import {DRAWING_BLEND_MODES,drawingDrawingArtifactGuardRefusal} from "../../🟦️.ts";
import {drawingTransformToMatrix} from "../../🧮️geometry/↗️affine/🟦️.ts";
import type { RasterSceneAsset, RasterSceneGroup, RasterSceneNode, RasterSceneInput } from "../📷️raster/🟦️.ts";
import {RasterSceneJob,type RasterSceneProgress} from "../📷️raster/🟦️.ts";
import type {PixelImage} from "@semio-tech/framework/pixels/editing";
import type { Fill, Color } from "../../🎨️fill/🖌️paint-model/🟦️.ts";
import type {PathSegment} from "@semio-tech/framework/2d";
import {DocumentBooleanJob,type DocumentBooleanLimits,type DocumentBooleanProgress} from "../🔀️booleans/🟦️.ts";
import {DocumentTraceJob,type DocumentTraceLimits,type DocumentTraceProgress} from "../🔍️trace/🟦️.ts";
import {ScenePlanCloseJob} from "../🧹️retire/🟦️.ts";
import {DocumentTextJob} from "../🔤️text/🟦️.ts";
import {loadBuiltinUnicodeSource} from "@semio-tech/framework/2d/text/font/shape/unicode";
import {loadBuiltinFontSources,FONT_CATALOG_DEFAULT_FAMILY,type FontByteSource,type FontByteProvider} from "@semio-tech/framework/2d/text/font/catalog";
import type {WorkRetirementProgress} from "@semio-tech/framework/2d/retire";
import {RetainedWorkRetirement,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkDemand,type RetainedWorkTurn} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {ScenePlanOwner,type ScenePlanRead,type ScenePlanNode,type ScenePlanContent} from "../🗺️plan-model/🎟️owned/🟦️.ts";
import {DrawingSceneWorkController,driveDrawingSceneWork,type DrawingSceneWorkProducer} from "../🎟️work/🟦️.ts";
import type { Paint, DocumentSceneContent, DocumentSceneNode, Matrix } from "../🗺️plan-model/🟦️.ts";
/** 👁️ Actual roots and assets supply scene work without a fabricated complete document. */
export type {DrawingSceneSource} from "./🎟️owned/🟦️.ts";
export type {DocumentSceneRetirement} from "./🎟️owned/🟦️.ts";
export type DocumentVectorRetirement=RetainedWorkRetirementCursor;
export type {DocumentSceneRetirementProgress} from "./🎟️owned/🟦️.ts";
export type DocumentVectorRetirementProgress=WorkRetirementProgress;
export type DocumentSceneViewport=Omit<RasterSceneInput,"assets"|"nodes">;
export type DocumentAlgorithmLimits={booleans:DocumentBooleanLimits;trace:DocumentTraceLimits;maxWork:number};
export type DocumentRasterProgress={phase:"preparing"|"tracing"|"text"|"algorithms"|"resolving"|"raster"|"complete";preparation:DocumentSceneProgress;tracing:DocumentTraceProgress|null;resolution:DocumentBooleanProgress|null;raster:RasterSceneProgress|null;nodes:number;work:number;done:boolean};
export type DocumentWorkControl<P>={work:DrawingSceneWorkController;operation:object;signal?:AbortSignal;quantum?:number;fontProvider?:FontByteProvider;onProgress?:(progress:P)=>void};
export type DocumentRasterOptions=DocumentWorkControl<DocumentRasterProgress>;
export type DocumentVectorProgress={phase:"preparing"|"tracing"|"text"|"algorithms"|"complete";preparation:DocumentSceneProgress;tracing:DocumentTraceProgress|null;resolution:DocumentBooleanProgress|null;work:number;done:boolean};
export type DocumentVectorOptions=DocumentWorkControl<DocumentVectorProgress>;
const initialVectorProgress=():DocumentVectorProgress=>({phase:"preparing",preparation:{phase:"assets",layers:0,assets:0,segments:0,references:0,sourceBytes:0,work:0,done:false},tracing:null,resolution:null,work:0,done:false});
function drain(cursor:RetainedWorkRetirementCursor,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{const before=turn.actualWork();let done=false;try{done=cursor.advance(1,work).done;}finally{turn.commitActualWork(cursor.lastTurnWork());}return done&&turn.actualWork()===before;}
type VectorNext="tracing"|"text"|"algorithms"|"complete";
/** 🎬️ Resolves authored algorithms into a complete vector plan while preserving semantic text. */
export class DocumentVectorJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private preparation:DocumentSceneJob|null;private traces:DocumentTraceJob|null=null;private texts:DocumentTextJob|null=null;private algorithms:DocumentBooleanJob|null=null;private progress:DocumentVectorProgress=initialVectorProgress();
 private output:ScenePlanOwner|null=null;private handoff:ScenePlanOwner|null=null;private next:VectorNext="tracing";private closing:RetainedWorkRetirementCursor|null=null;private failure:unknown=null;private aborted=false;private transferred=false;private readonly limits:DocumentAlgorithmLimits;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){if(!Number.isSafeInteger(algorithms.maxWork)||algorithms.maxWork<1||algorithms.maxWork>1e9)fail("Invalid document vector work limit");this.limits={booleans:{...algorithms.booleans},trace:{...algorithms.trace},maxWork:algorithms.maxWork};DocumentBooleanJob.checkLimits(this.limits.booleans);DocumentTraceJob.checkLimits(this.limits.trace);this.preparation=new DocumentSceneJob(document,limits,algorithms.maxWork);}
 private close(job:RetainedWorkRetirementCursor,owner:ScenePlanOwner,next:VectorNext):void{this.closing=job;this.handoff=owner;this.next=next;}
 private step(c:RetainedWorkProducerStep):void{
  if(this.closing!==null){
   let done=false;try{done=this.closing.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.closing.lastTurnWork());}
   if(!done||c.hasActualWork())return;
   this.closing=null;const owner=this.handoff!;this.handoff=null;
   if(this.next==="tracing")this.traces=new DocumentTraceJob({plan:owner,limits:this.limits.trace});else if(this.next==="text")this.texts=new DocumentTextJob(owner);else if(this.next==="algorithms")this.algorithms=new DocumentBooleanJob({plan:owner,limits:this.limits.booleans});else{this.output=owner;this.progress.done=true;}
   this.progress.phase=this.next;c.commitControlWork();return;
  }
  if(this.progress.phase==="preparing"){this.progress.preparation=c.advanceChild(this.preparation!);if(this.progress.preparation.done){const moved=this.preparation!.intoRetirement();this.preparation=null;this.close(moved.job,moved.output!,"tracing");}return;}
  if(this.progress.phase==="tracing"){this.progress.tracing=c.advanceChild(this.traces!);if(this.progress.tracing.done){const moved=this.traces!.intoRetirement();this.traces=null;this.close(moved.job,moved.output!,"text");}return;}
  if(this.progress.phase==="text"){if(c.advanceChild(this.texts!).done){const moved=this.texts!.intoRetirement();this.texts=null;this.close(moved.job,moved.output!,"algorithms");}return;}
  this.progress.resolution=c.advanceChild(this.algorithms!);if(this.progress.resolution.done){const moved=this.algorithms!.intoRetirement();this.algorithms=null;this.close(moved.job,moved.output!,"complete");}
 }
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 failurePending():boolean{return this.failure!==null||this.preparation?.failurePending()===true||this.traces?.failurePending()===true||this.texts?.failurePending()===true||this.algorithms?.failurePending()===true;}
 settlementPending():boolean{return this.closing!==null||this.progress.done||this.preparation?.settlementPending()===true||this.traces?.settlementPending()===true||this.texts?.settlementPending()===true||this.algorithms?.settlementPending()===true;}
 advance(items:number,work:RetainedWorkLedger):DocumentVectorProgress{
  try{return this.#runtime.advance(items,work,{maximumWork:this.limits.maxWork,workLimitMessage:"Document vector work limit exceeded",validate:()=>{if(this.aborted||this.transferred)cancel();if(this.failure!==null)throw this.failure;},stopped:()=>this.progress.done||this.needsFontSources(),settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.progress.work,recordSemanticWork:n=>{this.progress.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({...this.progress})});}
  catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.aborted)this.failure=error;throw error;}
 }
 needsFontSources():boolean{return this.texts?.needsFontSources()??false;}
 fontSourceFamily():string{return this.texts!.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.texts)throw Error("Text source phase inactive");this.texts.admitFontSources(sources,unicodeBytes);}
 result():ScenePlanRead{if(this.aborted||this.transferred)cancel();if(this.failure)throw this.failure;if(!this.progress.done)fail("Document vector incomplete");return this.output!.borrow();}
 cancel():void{this.aborted=true;}
 retireOutput(output:ScenePlanOwner):RetainedWorkRetirementCursor{return new ScenePlanCloseJob(output);}
 /** 🧹️ Adopt every child and intermediate source plan before relinquishing vector ownership. */
 intoRetirement():{job:DocumentVectorRetirement;output:ScenePlanOwner|null}{
  if(this.transferred)throw Error("Vector ownership already transferred");this.transferred=true;
  const output=this.progress.done&&!this.aborted&&!this.failure?this.output:null;this.aborted=true;
  const cursors:RetainedWorkRetirementCursor[]=[];if(this.closing)cursors.push(this.closing);this.closing=null;
  const adopt=(moved:{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null})=>{cursors.push(moved.job);if(moved.output)cursors.push(new ScenePlanCloseJob(moved.output));};
  if(this.preparation){adopt(this.preparation.intoRetirement());this.preparation=null;}
  if(this.traces){adopt(this.traces.intoRetirement());this.traces=null;}
  if(this.texts){adopt(this.texts.intoRetirement());this.texts=null;}
  if(this.algorithms){adopt(this.algorithms.intoRetirement());this.algorithms=null;}
  if(this.handoff)cursors.push(new ScenePlanCloseJob(this.handoff));this.handoff=null;
  if(this.output&&this.output!==output)cursors.push(new ScenePlanCloseJob(this.output));this.output=null;
  const job=new RetainedWorkRetirement((work,turn)=>{const cursor=cursors.at(-1);if(cursor){if(!drain(cursor,work,turn))return false;cursors.pop();turn.commitActualWork();return false;}return true;});
  return{job,output};
 }
}
/** 👁️ Borrowed root and asset identities remain explicit for every producer turn. */
export class DocumentVectorPreparationJob{
 private job:DocumentVectorJob;private layers:DrawingSceneSource["layers"];private assets:DrawingSceneSource["assets"];
 constructor(source:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){this.layers=source.layers;this.assets=source.assets;this.job=new DocumentVectorJob(source,limits,algorithms);}
 advance(source:DrawingSceneSource,grant:number,work:RetainedWorkLedger):DocumentVectorProgress{if(source.layers!==this.layers||source.assets!==this.assets)fail("Captured scene source ownership changed");return this.job.advance(grant,work);}
 nextWorkDemand():RetainedWorkDemand{return this.job.nextWorkDemand();}
 lastTurnWork():number{return this.job.lastTurnWork();}
 lastTurnSemanticWork():number{return this.job.lastTurnSemanticWork();}
 failurePending():boolean{return this.job.failurePending();}
 settlementPending():boolean{return this.job.settlementPending();}
 needsFontSources():boolean{return this.job.needsFontSources();}
 fontSourceFamily():string{return this.job.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{this.job.admitFontSources(sources,unicodeBytes);}
 cancel():void{this.job.cancel();}
 result():ScenePlanRead{return this.job.result();}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null}{const moved=this.job.intoRetirement();this.layers=[];this.assets=DrawingAssetsOwner.fromEntriesCold([]);return moved;}
}

async function completeSceneProducer<T,P extends {done:boolean}>(job:DrawingSceneWorkProducer<P,T>&{needsFontSources?:()=>boolean;fontSourceFamily?:()=>string;admitFontSources?:(sources:FontByteSource[],unicodeBytes:Uint8Array)=>void},document:DrawingSceneSource,options:DocumentWorkControl<P>):Promise<T>{
 options.work.checkBinding(options.operation,document,"advance");if(options.quantum!==undefined&&(!Number.isSafeInteger(options.quantum)||options.quantum<1))throw RangeError("Invalid Drawing semantic quantum");let unicodeBytes:Uint8Array|null=null;
 return driveDrawingSceneWork(options.work,options.operation,document,{advance:(items,work)=>job.advance(Math.min(items,options.quantum??items),work),nextWorkDemand:()=>job.nextWorkDemand(),lastTurnWork:()=>job.lastTurnWork(),cancel:()=>job.cancel(),intoRetirement:()=>job.intoRetirement(),retireOutput:output=>job.retireOutput(output),beforeTurn:async()=>{if(options.signal?.aborted)cancel();if(job.needsFontSources?.()){unicodeBytes??=await loadBuiltinUnicodeSource(options.fontProvider,options.signal);options.work.checkBinding(options.operation,document,"advance");const sources=await loadBuiltinFontSources(job.fontSourceFamily!(),options.fontProvider,{signal:options.signal});options.work.checkBinding(options.operation,document,"advance");job.admitFontSources!(sources,unicodeBytes);}}},options.onProgress);
}
/** ⏳️ Publishes complete vector geometry after yielded work and observer cancellation checks. */
export async function prepareDocumentVector(document:DrawingArtifact,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits,options:DocumentVectorOptions):Promise<ScenePlanOwner>{
 options.work.checkBinding(options.operation,document,"advance");if(options.signal?.aborted)cancel();const job=new DocumentVectorJob(document,limits,algorithms);return completeSceneProducer(job,document,options);
}
export const list=<T,U>(source:{readonly length:number;at(index:number):T|undefined},map:(value:T)=>U):U[]=>Array.from({length:source.length},(_,at)=>{const value=source.at(at);if(value===undefined)fail("Missing scene row");return map(value);});
type PlanPaint=Extract<ScenePlanContent,{kind:"path"}>;
export const plainFill=(fill:PlanPaint["fill"]):Fill|null=>fill===null?null:fill.kind==="solid"?{kind:"solid",color:[...fill.color]as Color}:{...fill,stops:list(fill.stops as Parameters<typeof list>[0],(stop:any)=>({offset:stop.offset,color:[...stop.color]as Color}))}as unknown as Fill;
export const plainStroke=(stroke:PlanPaint["stroke"]):Extract<RasterSceneNode["content"],{kind:"path"}>["stroke"]=>stroke===null?null:{...stroke,color:[...stroke.color]as Color,dash:stroke.dash?list(stroke.dash,value=>value):stroke.dash}as never;
function rasterNode(node:ScenePlanNode):RasterSceneNode{const content=node.content;if(content.kind!=="path"&&content.kind!=="glyphs"&&content.kind!=="image")fail("Unresolved "+content.kind+" layer: "+node.id);return{id:node.id,groups:list(node.groups,group=>group),transform:[...node.transform]as RasterSceneNode["transform"],opacity:node.opacity,blendMode:node.blendMode,visible:node.visible,content:content.kind==="image"?{kind:"image",asset:content.asset,width:content.width,height:content.height}:{kind:"path",segments:list(content.segments,segment=>segment),fillRule:content.fillRule,fill:plainFill(content.fill),stroke:plainStroke(content.stroke)}};}
/** 📷️ Uses the same complete vector producer before per-record raster admission. */
export class DocumentRasterJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private vector:DocumentVectorJob|null;private prepared:DocumentVectorProgress=initialVectorProgress();
 private plan:ScenePlanOwner|null=null;private view:ScenePlanRead|null=null;private assets:RasterSceneAsset[]=[];private nodes:RasterSceneNode[]=[];private at=0;private assetAt=0;private work=0;private phase:DocumentRasterProgress["phase"]="preparing";
 private raster:RasterSceneJob|null=null;private rendered:RasterSceneProgress|null=null;private output:PixelImage|null=null;private failure:unknown=null;private aborted=false;private readonly viewport:DocumentSceneViewport;private readonly maxWork:number;private closing:RetainedWorkRetirementCursor|null=null;private afterClose:"resolving"|"complete"="resolving";private transferred=false;private handoff:ScenePlanOwner|null=null;private rasterClose:RetainedWorkRetirementCursor|null=null;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolutionLimits:DocumentAlgorithmLimits){this.viewport={...viewport,origin:[...viewport.origin]};const check=new RasterSceneJob({...this.viewport,assets:[],nodes:[]});check.cancel();this.vector=new DocumentVectorJob(document,limits,resolutionLimits);this.maxWork=resolutionLimits.maxWork;}
 private pixelScale:readonly[number,number]=[1,1];
 /** 🔍️ Maps the immutable artboard world extent to requested output pixel dimensions. */
 setPixelScale(x:number,y:number):void{if(this.work!==0||![x,y].every(n=>Number.isFinite(n)&&n>0))fail("Invalid document pixel scale");this.pixelScale=[x,y];}
 needsFontSources():boolean{return this.vector?.needsFontSources()??false;}
 fontSourceFamily():string{return this.vector!.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.vector)throw Error("Text source phase inactive");this.vector.admitFontSources(sources,unicodeBytes);}
 private step(c:RetainedWorkProducerStep):void{
  if(this.closing!==null){let done=false;try{done=this.closing.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.closing.lastTurnWork());}if(!done||c.hasActualWork())return;this.closing=null;if(this.afterClose==="resolving"){this.plan=this.handoff;this.handoff=null;this.view=this.plan!.borrow();this.phase="resolving";}else this.phase="complete";c.commitControlWork();return;}
  if(this.vector){this.prepared=c.advanceChild(this.vector);if(this.prepared.done){const moved=this.vector.intoRetirement();this.vector=null;this.handoff=moved.output!;this.closing=moved.job;this.afterClose="resolving";}else this.phase=this.prepared.phase;return;}
  if(this.phase==="resolving"){const view=this.view!;if(this.assetAt<view.assets.length){const asset=view.assets.at(this.assetAt++)!;this.assets.push(asset);c.commitSemanticWork();return;}const n=view.nodes.at(this.at++);if(!n){this.raster=new RasterSceneJob({...this.viewport,assets:this.assets,nodes:this.nodes});this.phase="raster";c.commitSemanticWork();return;}if(n.content.kind!=="group"){const node=rasterNode(n),m=node.transform,[x,y]=this.pixelScale;this.nodes.push({...node,transform:[m[0]*x,m[1]*y,m[2]*x,m[3]*y,m[4]*x,m[5]*y]});}c.commitSemanticWork();return;}
  if(this.phase==="raster"&&this.raster){this.rendered=c.advanceChild(this.raster);if(this.rendered.done){const moved=this.raster!.intoRetirement();this.output=moved.output;this.raster=null;this.rasterClose=moved.job;}return;}
  if(this.phase==="raster"){let done=false;try{done=this.rasterClose!.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.rasterClose!.lastTurnWork());}if(!done||c.hasActualWork())return;this.rasterClose=null;this.view=null;this.closing=new ScenePlanCloseJob(this.plan!);this.plan=null;this.afterClose="complete";c.commitControlWork();return;}
 }
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 failurePending():boolean{return this.failure!==null||this.vector?.failurePending()===true||this.raster?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="complete"||this.closing!==null||this.rasterClose!==null||this.vector?.settlementPending()===true||this.raster?.settlementPending()===true;}
 advance(items:number,work:RetainedWorkLedger):DocumentRasterProgress{
  try{return this.#runtime.advance(items,work,{maximumWork:this.maxWork,workLimitMessage:"Document raster work limit exceeded",validate:()=>{if(this.aborted||this.transferred)cancel();if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete"||this.needsFontSources(),settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,preparation:this.prepared.preparation,tracing:this.prepared.tracing,resolution:this.prepared.resolution,raster:this.rendered,nodes:Math.min(this.at,this.prepared.preparation.layers),work:this.work,done:this.phase==="complete"})});}
  catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.aborted)this.failure=error;throw error;}
 }
 /** 🧹️ Composes the actual preparation, raster, remaining plan and unpublished pixel owners. */
 intoRetirement():{job:RetainedWorkRetirementCursor;output:PixelImage|null}{
  if(this.transferred)throw Error("Document raster ownership already transferred");this.transferred=true;const cursors:RetainedWorkRetirementCursor[]=[];const images:PixelImage[]=[];
  if(this.closing)cursors.push(this.closing);this.closing=null;if(this.rasterClose)cursors.push(this.rasterClose);this.rasterClose=null;
  if(this.vector){const moved=this.vector.intoRetirement();cursors.push(moved.job);if(moved.output)cursors.push(new ScenePlanCloseJob(moved.output));this.vector=null;}
  if(this.raster){const moved=this.raster.intoRetirement();cursors.push(moved.job);if(moved.output)images.push(moved.output);this.raster=null;}
  if(this.handoff)cursors.push(new ScenePlanCloseJob(this.handoff));this.handoff=null;if(this.plan)cursors.push(new ScenePlanCloseJob(this.plan));this.plan=null;this.view=null;
  let assets=this.assets,nodes=this.nodes;this.assets=[];this.nodes=[];
  const output=this.phase==="complete"&&!this.aborted&&!this.failure?this.output:null;if(this.output&&!output)images.push(this.output);this.output=null;this.aborted=true;let slot=0;
  return{output,job:new RetainedWorkRetirement((work,turn)=>{
   if(slot===0){const cursor=cursors.at(-1);if(cursor){if(!drain(cursor,work,turn))return false;cursors.pop();turn.commitActualWork();return false;}}
   if(!work.canStep())return false;turn.commitActualWork();
   switch(slot){case 0:break;case 1:if(assets.length){assets.pop();return false;}assets=[];break;case 2:if(nodes.pop())return false;nodes=[];break;case 3:if(images.pop())return false;break;default:return true;}
   slot++;return false;
  })};
 }
 cancel():void{this.aborted=true;}
 retireOutput(_output:PixelImage):RetainedWorkRetirementCursor{let done=false;return new RetainedWorkRetirement((work,turn)=>{if(done)return true;if(!work.canStep())return false;done=true;turn.commitActualWork();return false;});}
 result():PixelImage{if(this.aborted||this.transferred)cancel();if(this.failure)throw this.failure;if(this.phase!=="complete")fail("Document raster incomplete");return this.output!;}
}
/** ⏳️ Publishes document pixels only after responsive preparation, raster completion and owned close. */
export async function rasterizeDocument(document:DrawingArtifact,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolutionLimits:DocumentAlgorithmLimits,options:DocumentRasterOptions):Promise<PixelImage>{
 options.work.checkBinding(options.operation,document,"advance");if(options.signal?.aborted)cancel();return completeSceneProducer(new DocumentRasterJob(document,limits,viewport,resolutionLimits),document,options);
}
export {DocumentSceneJob,prepareDocumentScene} from "./🎟️owned/🟦️.ts";
export type {DocumentSceneLimits,DocumentSceneProgress,DocumentSceneOptions} from "./🎟️owned/🟦️.ts";
/** 🚪️ Hands prepared paths/images to raster; unresolved work is an explicit refusal. */
export function resolvedSceneInput(plan:ScenePlanRead,input:Omit<RasterSceneInput,"assets"|"nodes">):RasterSceneInput{
 const nodes:RasterSceneNode[]=[];for(let at=0;at<plan.nodes.length;at++){const n=plan.nodes.at(at)!;if(n.content.kind==="group")continue;nodes.push(rasterNode(n));}return{...input,assets:list(plan.assets,asset=>asset),nodes};
}
export {sceneSelectionRelation,type SceneSelectionRelation} from "./🧭️selection/🟦️.ts";
function fail(message:string):never{throw new drawingDrawingArtifactGuardRefusal("$scene",message);}
const cancel=():never=>{throw new DOMException("Document scene preparation cancelled","AbortError");};
