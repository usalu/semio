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
import {UnitRetirement,type WorkRetirement,type WorkRetirementProgress} from "@semio-tech/framework/2d/retire";
import {RetainedWorkBuffer,RetainedWorkLedger,RetainedWorkRetirement,retireRetainedWorkBuffer,retireRetainedWorkChildren,retireRetainedWorkArchive,type RetainedWorkRetirementCursor,type RetainedWorkDemand} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {DrawingSceneWorkController,driveDrawingSceneWork,type DrawingSceneWorkProducer} from "../🎟️work/🟦️.ts";
import type { Paint, DocumentSceneContent, DocumentSceneNode, DocumentScenePlan, Matrix } from "../🗺️plan-model/🟦️.ts";
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
/** 🎬️ Resolves authored algorithms into a complete vector plan while preserving semantic text. */
export class DocumentVectorJob{
 private preparation:DocumentSceneJob|null;private traces:DocumentTraceJob|null=null;private texts:DocumentTextJob|null=null;private algorithms:DocumentBooleanJob|null=null;private progress:DocumentVectorProgress=initialVectorProgress();
 private retiredChildren=new RetainedWorkBuffer<RetainedWorkRetirementCursor>(16);private retiredPlans=new RetainedWorkBuffer<ScenePlanCloseJob>(16);private pending:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};private last=0;private output:DocumentScenePlan|null=null;private failure:unknown=null;private aborted=false;private transferred=false;private readonly limits:DocumentAlgorithmLimits;private closing:RetainedWorkRetirementCursor|null=null;private discard:ScenePlanCloseJob|null=null;private handoff:DocumentScenePlan|null=null;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){if(!Number.isSafeInteger(algorithms.maxWork)||algorithms.maxWork<1||algorithms.maxWork>1e9)fail("Invalid document vector work limit");this.limits={booleans:{...algorithms.booleans},trace:{...algorithms.trace},maxWork:algorithms.maxWork};const boolean=new DocumentBooleanJob({plan:{assets:[],nodes:[]},limits:this.limits.booleans});boolean.cancel();const trace=new DocumentTraceJob({plan:{assets:[],nodes:[]},limits:this.limits.trace});trace.cancel();this.preparation=new DocumentSceneJob(document,limits,algorithms.maxWork);}
 private step(work:RetainedWorkLedger):void{
  if(this.closing){if(!work.slot(this.retiredChildren))return;work.push(this.retiredChildren,this.closing);this.closing=null;return;}
  if(this.discard){if(!work.slot(this.retiredPlans))return;work.push(this.retiredPlans,this.discard);this.discard=null;return;}
  if(this.handoff){const plan=this.handoff;this.handoff=null;if(this.progress.phase==="preparing"){this.traces=new DocumentTraceJob({plan,limits:this.limits.trace});this.progress.phase="tracing";}else if(this.progress.phase==="tracing"){this.texts=new DocumentTextJob(plan);this.progress.phase="text";}else if(this.progress.phase==="text"){this.algorithms=new DocumentBooleanJob({plan,limits:this.limits.booleans});this.progress.phase="algorithms";}else{this.output=plan;this.progress.phase="complete";this.progress.done=true;}return;}
  if(this.progress.phase==="preparing"){this.progress.preparation=this.preparation!.advance(1,work);if(this.progress.preparation.done){const moved=this.preparation!.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.preparation=null;}return;}
  if(this.progress.phase==="tracing"){this.progress.tracing=this.traces!.advance(1,work);if(this.progress.tracing.done){const moved=this.traces!.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.discard=new ScenePlanCloseJob(moved.input);this.traces=null;}return;}
  if(this.progress.phase==="text"){if(this.texts!.advance(1,work).done){const moved=this.texts!.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.texts=null;}return;}
  this.progress.resolution=this.algorithms!.advance(1,work);if(this.progress.resolution.done){const moved=this.algorithms!.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.discard=new ScenePlanCloseJob(moved.input);this.algorithms=null;}
 }
 nextWorkDemand():RetainedWorkDemand{return{...this.pending};}
 lastTurnWork():number{return this.last;}
 advance(budget:number,work:RetainedWorkLedger):DocumentVectorProgress{this.last=0;if(!Number.isSafeInteger(budget)||budget<1)fail("Invalid document vector work grant");if(this.aborted)cancel();if(this.failure)throw this.failure;const entered=work.enter();try{for(let at=0;entered!==null&&at<budget&&!this.progress.done&&work.remaining.maximumItems>0;at++){if(this.progress.work>=this.limits.maxWork)fail("Document vector work limit exceeded");const before={...work.receipt};try{this.step(work);}catch(error){this.last++;this.progress.work++;throw error;}if(work.blocked){if(Object.keys(before).some(key=>before[key as keyof typeof before]!==work.receipt[key as keyof typeof before])){this.last++;this.progress.work++;}break;}this.progress.work++;this.last++;work.noteSource(entered);if(this.needsFontSources())break;}}catch(error){this.failure=error;throw error;}finally{this.pending=work.nextDemand;if(entered!==null)work.exit();}return{...this.progress};}
 needsFontSources():boolean{return this.texts?.needsFontSources()??false;}
 fontSourceFamily():string{return this.texts!.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.texts)throw Error("Text source phase inactive");this.texts.admitFontSources(sources,unicodeBytes);}
 /** 🧹️ Adopt every child and intermediate source plan before relinquishing vector ownership. */
 intoRetirement():{job:DocumentVectorRetirement;output:DocumentScenePlan|null}{
  if(this.transferred)throw Error("Vector ownership already transferred");this.transferred=true;let child=this.closing;this.closing=null;const plans=this.retiredPlans,children=this.retiredChildren;this.retiredPlans=new RetainedWorkBuffer<ScenePlanCloseJob>(16);this.retiredChildren=new RetainedWorkBuffer<RetainedWorkRetirementCursor>(16);let discard=this.discard;this.discard=null;let firstPlan:ScenePlanCloseJob|null=null,secondPlan:ScenePlanCloseJob|null=null;
  const adopt=(plan:DocumentScenePlan|null)=>{if(plan){if(firstPlan===null)firstPlan=new ScenePlanCloseJob(plan);else if(secondPlan===null)secondPlan=new ScenePlanCloseJob(plan);else throw Error("Vector active plan custody exceeded");}};
  if(this.preparation){const moved=this.preparation.intoRetirement();child=moved.job;adopt(moved.output);this.preparation=null;}
  if(this.traces){const moved=this.traces.intoRetirement();child=moved.job;adopt(moved.input);adopt(moved.output);this.traces=null;}
  if(this.texts){const moved=this.texts.intoRetirement();child=moved.job;adopt(moved.output);this.texts=null;}
  if(this.algorithms){const moved=this.algorithms.intoRetirement();child=moved.job;adopt(moved.input);adopt(moved.output);this.algorithms=null;}
  const handoff=this.handoff?new ScenePlanCloseJob(this.handoff):null;this.handoff=null;const output=this.progress.done&&!this.aborted&&!this.failure?this.output:null;const refusedOutput=!output&&this.output?new ScenePlanCloseJob(this.output):null;this.output=null;this.aborted=true;
  const job=new RetainedWorkRetirement(work=>{if(child){if(!child.advance(1,work).done)return false;child=null;}if(!retireRetainedWorkChildren(children,work))return false;if(!retireRetainedWorkArchive(plans,work,(plan,work)=>plan.advance(1,work).done))return false;for(const plan of [discard,firstPlan,secondPlan,handoff,refusedOutput])if(plan&&!plan.terminalIsEmpty()&&!plan.advance(1,work).done)return false;discard=null;firstPlan=null;secondPlan=null;return true;});return{job,output};
 }
 cancel():void{this.aborted=true;}
 result():DocumentScenePlan{if(this.aborted)cancel();if(this.failure)throw this.failure;if(!this.progress.done)fail("Document vector incomplete");return this.output!;}
}
/** 👁️ Borrowed root and asset identities remain explicit for every producer turn. */
export class DocumentVectorPreparationJob{
 private job:DocumentVectorJob;private layers:DrawingSceneSource["layers"];private assets:DrawingSceneSource["assets"];
 constructor(source:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){this.layers=source.layers;this.assets=source.assets;this.job=new DocumentVectorJob(source,limits,algorithms);}
 advance(source:DrawingSceneSource,grant:number,work:RetainedWorkLedger):DocumentVectorProgress{if(source.layers!==this.layers||source.assets!==this.assets)fail("Captured scene source ownership changed");return this.job.advance(grant,work);}
 nextWorkDemand():RetainedWorkDemand{return this.job.nextWorkDemand();}
 lastTurnWork():number{return this.job.lastTurnWork();}
 needsFontSources():boolean{return this.job.needsFontSources();}
 fontSourceFamily():string{return this.job.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{this.job.admitFontSources(sources,unicodeBytes);}
 cancel():void{this.job.cancel();}
 result():DocumentScenePlan{return this.job.result();}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:DocumentScenePlan|null}{const moved=this.job.intoRetirement();this.layers=[];this.assets=DrawingAssetsOwner.fromEntriesCold([]);return moved;}
}/** 🧺️ Keeps completed or refused producer ownership until explicit yielded close finishes. */
async function completeUnfundedSceneProducer<T,P extends {done:boolean}>(job:{advance:(grant:number)=>P;cancel:()=>void;needsFontSources?:()=>boolean;fontSourceFamily?:()=>string;admitFontSources?:(sources:FontByteSource[],unicodeBytes:Uint8Array)=>void;intoRetirement:()=>{job:WorkRetirement;output:T|null}},options:{signal?:AbortSignal;workBudget?:number;fontProvider?:FontByteProvider;onProgress?:(progress:P)=>void},check:()=>void):Promise<T>{
 let close:WorkRetirement|null=null,transferred=false;
 let unicodeBytes:Uint8Array|null=null;try{for(;;){check();if(job.needsFontSources?.()){unicodeBytes??=await loadBuiltinUnicodeSource(options.fontProvider,options.signal);check();job.admitFontSources!(await loadBuiltinFontSources(job.fontSourceFamily!(),options.fontProvider,{signal:options.signal}),unicodeBytes);check();}const progress=job.advance(options.workBudget??4096);options.onProgress?.(progress);check();if(progress.done){const moved=job.intoRetirement();transferred=true;close=moved.job;while(!close.advance(4096).done)await new Promise<void>(resolve=>setTimeout(resolve,0));close=null;check();return moved.output!;}await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();if(!transferred)close=job.intoRetirement().job;while(close&&!close.advance(4096).done)await new Promise<void>(resolve=>setTimeout(resolve,0));throw error;}
}
async function completeSceneProducer<T,P extends {done:boolean}>(job:DrawingSceneWorkProducer<P,T>&{needsFontSources?:()=>boolean;fontSourceFamily?:()=>string;admitFontSources?:(sources:FontByteSource[],unicodeBytes:Uint8Array)=>void},document:DrawingSceneSource,options:DocumentWorkControl<P>):Promise<T>{
 options.work.checkBinding(options.operation,document,"advance");if(options.quantum!==undefined&&(!Number.isSafeInteger(options.quantum)||options.quantum<1))throw RangeError("Invalid Drawing semantic quantum");let unicodeBytes:Uint8Array|null=null;
 return driveDrawingSceneWork(options.work,options.operation,document,{advance:(items,work)=>job.advance(Math.min(items,options.quantum??items),work),nextWorkDemand:()=>job.nextWorkDemand(),lastTurnWork:()=>job.lastTurnWork(),cancel:()=>job.cancel(),intoRetirement:()=>job.intoRetirement(),beforeTurn:async()=>{if(options.signal?.aborted)cancel();if(job.needsFontSources?.()){unicodeBytes??=await loadBuiltinUnicodeSource(options.fontProvider,options.signal);options.work.checkBinding(options.operation,document,"advance");const sources=await loadBuiltinFontSources(job.fontSourceFamily!(),options.fontProvider,{signal:options.signal});options.work.checkBinding(options.operation,document,"advance");job.admitFontSources!(sources,unicodeBytes);}}},options.onProgress);
}
/** ⏳️ Publishes complete vector geometry after yielded work and observer cancellation checks. */
export async function prepareDocumentVector(document:DrawingArtifact,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits,options:DocumentVectorOptions):Promise<DocumentScenePlan>{
 options.work.checkBinding(options.operation,document,"advance");if(options.signal?.aborted)cancel();const job=new DocumentVectorJob(document,limits,algorithms);return completeSceneProducer(job,document,options);
}
/** 📷️ Uses the same complete vector producer before per-record raster admission. */
export class DocumentRasterJob{
 private vector:DocumentVectorJob|null;private prepared:DocumentVectorProgress=initialVectorProgress();
 private source:readonly DocumentSceneNode[]=[];private assets:readonly RasterSceneAsset[]=[];private nodes:RasterSceneNode[]=[];private at=0;private work=0;private phase:DocumentRasterProgress["phase"]="preparing";
 private retiredChildren:WorkRetirement[]=[];private raster:RasterSceneJob|null=null;private rendered:RasterSceneProgress|null=null;private output:PixelImage|null=null;private failure:unknown=null;private aborted=false;private readonly viewport:DocumentSceneViewport;private readonly maxWork:number;private closing:WorkRetirement|null=null;private handoff:DocumentScenePlan|null=null;private transferred=false;
 constructor(document:DrawingArtifact,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolutionLimits:DocumentAlgorithmLimits){this.viewport={...viewport,origin:[...viewport.origin]};const check=new RasterSceneJob({...this.viewport,assets:[],nodes:[]});check.cancel();this.vector=new DocumentVectorJob(document,limits,resolutionLimits);this.maxWork=resolutionLimits.maxWork;}
 private pixelScale:readonly[number,number]=[1,1];
 /** 🔍️ Maps the immutable artboard world extent to requested output pixel dimensions. */
 setPixelScale(x:number,y:number):void{if(this.work!==0||![x,y].every(n=>Number.isFinite(n)&&n>0))fail("Invalid document pixel scale");this.pixelScale=[x,y];}
 needsFontSources():boolean{return this.vector?.needsFontSources()??false;}
 fontSourceFamily():string{return this.vector!.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.vector)throw Error("Text source phase inactive");this.vector.admitFontSources(sources,unicodeBytes);}
 private step():void{
  if(this.closing){this.retiredChildren.push(this.closing);this.closing=null;return;}
  if(this.handoff){this.source=this.handoff.nodes;this.assets=this.handoff.assets;this.handoff=null;return;}
  if(this.vector){this.prepared=this.vector.advance(1);if(this.prepared.done){const moved=this.vector.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.vector=null;this.phase="resolving";}else this.phase=this.prepared.phase;return;}
  if(this.phase==="resolving"){const n=this.source[this.at++];if(!n){this.raster=new RasterSceneJob({...this.viewport,assets:this.assets,nodes:this.nodes});this.phase="raster";return;}if(n.content.kind==="group")return;const node=rasterNode(n),m=node.transform,[x,y]=this.pixelScale;this.nodes.push({...node,transform:[m[0]*x,m[1]*y,m[2]*x,m[3]*y,m[4]*x,m[5]*y]});return;}
  if(this.phase==="raster"){this.rendered=this.raster!.advance(1);if(this.rendered.done){const moved=this.raster!.intoRetirement();this.output=moved.output;this.closing=moved.job;this.raster=null;this.phase="complete";}}
 }
 advance(budget:number):DocumentRasterProgress{if(!Number.isSafeInteger(budget)||budget<1)fail("Invalid document raster work grant");if(this.aborted)cancel();if(this.failure)throw this.failure;try{for(let at=0;at<budget&&this.phase!=="complete";at++){if(this.work>=this.maxWork)fail("Document raster work limit exceeded");this.step();this.work++;if(this.needsFontSources())break;}}catch(error){this.failure=error;throw error;}return{phase:this.phase,preparation:this.prepared.preparation,tracing:this.prepared.tracing,resolution:this.prepared.resolution,raster:this.rendered,nodes:Math.min(this.at,this.prepared.preparation.layers),work:this.work,done:this.phase==="complete"};}
 /** 🧹️ Composes the actual preparation, raster, remaining plan and unpublished pixel owners. */
 intoRetirement():{job:WorkRetirement;output:PixelImage|null}{
  if(this.transferred)throw Error("Document raster ownership already transferred");this.transferred=true;const children=this.retiredChildren;this.retiredChildren=[];const plans:ScenePlanCloseJob[]=[];const images:PixelImage[]=[];
  if(this.closing)children.push(this.closing);this.closing=null;if(this.vector){const moved=this.vector.intoRetirement();children.push(moved.job);if(moved.output)plans.push(new ScenePlanCloseJob(moved.output));this.vector=null;}if(this.raster){const moved=this.raster.intoRetirement();children.push(moved.job);if(moved.output)images.push(moved.output);this.raster=null;}
  if(this.handoff)plans.push(new ScenePlanCloseJob(this.handoff));this.handoff=null;let source=this.source,assets=this.assets,nodes=this.nodes;this.source=[];this.assets=[];this.nodes=[];let at=0,assetAt=0;
  const output=this.phase==="complete"&&!this.aborted&&!this.failure?this.output:null;if(this.output&&!output)images.push(this.output);this.output=null;this.aborted=true;let slot=0;
  return{output,job:new UnitRetirement(()=>{switch(slot){case 0:{const child=children.at(-1);if(child){if(child.advance(1).done)children.pop();return false;}break;}case 1:{const plan=plans.at(-1);if(plan){if(plan.advance(1).done)plans.pop();return false;}break;}case 2:if(at++<source.length)return false;source=[];break;case 3:if(assetAt++<assets.length)return false;assets=[];break;case 4:if(nodes.pop())return false;nodes=[];break;case 5:if(images.pop())return false;break;}return ++slot===6;})};
 }
 cancel():void{this.aborted=true;}
 result():PixelImage{if(this.aborted)cancel();if(this.failure)throw this.failure;if(this.phase!=="complete")fail("Document raster incomplete");return this.output!;}
}
/** ⏳️ Publishes document pixels only after responsive preparation and raster completion. */
export async function rasterizeDocument(document:DrawingArtifact,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolutionLimits:DocumentAlgorithmLimits,options:DocumentRasterOptions={}):Promise<PixelImage>{
 const check=()=>{if(options.signal?.aborted)cancel();};check();const job=new DocumentRasterJob(document,limits,viewport,resolutionLimits);return completeUnfundedSceneProducer(job,options,check);
}
export {DocumentSceneJob,prepareDocumentScene} from "./🎟️owned/🟦️.ts";
export type {DocumentSceneLimits,DocumentSceneProgress,DocumentSceneOptions} from "./🎟️owned/🟦️.ts";
function rasterNode(node:DocumentSceneNode):RasterSceneNode{const content=node.content;if(content.kind!=="path"&&content.kind!=="glyphs"&&content.kind!=="image")fail("Unresolved "+content.kind+" layer: "+node.id);return{id:node.id,groups:node.groups,transform:node.transform,opacity:node.opacity,blendMode:node.blendMode,visible:node.visible,content:content.kind==="glyphs"?{kind:"path",segments:content.segments,fillRule:content.fillRule,fill:content.fill,stroke:content.stroke}:content};}
/** 🚪️ Hands prepared paths/images to raster; unresolved work is an explicit refusal. */
export function resolvedSceneInput(plan:DocumentScenePlan,input:Omit<RasterSceneInput,"assets"|"nodes">):RasterSceneInput{
 const nodes:RasterSceneNode[]=[];for(const n of plan.nodes){if(n.content.kind==="group")continue;nodes.push(rasterNode(n));}return{...input,assets:plan.assets,nodes};
}
export {sceneSelectionRelation,type SceneSelectionRelation} from "./🧭️selection/🟦️.ts";
function fail(message:string):never{throw new drawingDrawingArtifactGuardRefusal("$scene",message);}
const cancel=():never=>{throw new DOMException("Document scene preparation cancelled","AbortError");};
