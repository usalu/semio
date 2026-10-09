/** 📋️ Complete authored documents become private typed preparation plans under work grants. */
import {binary64Value} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type {DrawingArtifact,DrawingLayerNode,DrawingPathSegment} from "../../🟦️.ts";
import {DRAWING_BLEND_MODES,drawingDrawingArtifactGuardRefusal} from "../../🟦️.ts";
import {drawingTransformToMatrix} from "../../🧮️geometry/↗️affine/🟦️.ts";
import type {RasterSceneAsset,RasterSceneGroup,RasterSceneNode,RasterSceneInput,RasterSceneContent} from "../📷️raster/🟦️.ts";
import {RasterSceneJob,type RasterSceneProgress} from "../📷️raster/🟦️.ts";
import type {PixelImage} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/✍️editing/🟦️.ts";
import type {PathRasterStroke} from "../../🧮️geometry/📷️raster/🟦️.ts";
import type {Fill,Color} from "../../🎨️fill/🟦️.ts";
import type {PathSegment} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🟦️.ts";
import {DocumentBooleanJob,type DocumentBooleanLimits,type DocumentBooleanProgress} from "../🔀️booleans/🟦️.ts";
import {DocumentTraceJob,type DocumentTraceLimits,type DocumentTraceProgress} from "../🔍️trace/🟦️.ts";
import {ScenePlanCloseJob} from "../🧹️retire/🟦️.ts";
import {UnitRetirement,type WorkRetirement,type WorkRetirementProgress} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧹️retire/🟦️.ts";
/** 👁️ Actual roots and assets supply scene work without a fabricated complete document. */
export type DrawingSceneSource=Pick<DrawingArtifact,"layers"|"assets">&{rootVisible?:(index:number)=>boolean};
export type DocumentSceneRetirement=WorkRetirement;
export type DocumentVectorRetirement=WorkRetirement;
export type DocumentSceneRetirementProgress=WorkRetirementProgress;
export type DocumentVectorRetirementProgress=WorkRetirementProgress;
type Paint={fillRule:"nonzero"|"evenodd";fill:Fill|null;stroke:PathRasterStroke|null};
export type DocumentSceneViewport=Omit<RasterSceneInput,"assets"|"nodes">;
export type DocumentAlgorithmLimits={booleans:DocumentBooleanLimits;trace:DocumentTraceLimits;maxWork:number};
export type DocumentRasterProgress={phase:"preparing"|"tracing"|"algorithms"|"resolving"|"raster"|"complete";preparation:DocumentSceneProgress;tracing:DocumentTraceProgress|null;resolution:DocumentBooleanProgress|null;raster:RasterSceneProgress|null;nodes:number;work:number;done:boolean};
export type DocumentRasterOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:DocumentRasterProgress)=>void};
export type DocumentVectorProgress={phase:"preparing"|"tracing"|"algorithms"|"complete";preparation:DocumentSceneProgress;tracing:DocumentTraceProgress|null;resolution:DocumentBooleanProgress|null;work:number;done:boolean};
export type DocumentVectorOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:DocumentVectorProgress)=>void};
const initialVectorProgress=():DocumentVectorProgress=>({phase:"preparing",preparation:{phase:"assets",layers:0,assets:0,segments:0,references:0,sourceBytes:0,work:0,done:false},tracing:null,resolution:null,work:0,done:false});
/** 🎬️ Resolves authored algorithms into a complete vector plan while preserving semantic text. */
export class DocumentVectorJob{
 private preparation:DocumentSceneJob|null;private traces:DocumentTraceJob|null=null;private algorithms:DocumentBooleanJob|null=null;private progress:DocumentVectorProgress=initialVectorProgress();
 private retiredChildren:WorkRetirement[]=[];private retiredPlans:ScenePlanCloseJob[]=[];private output:DocumentScenePlan|null=null;private failure:unknown=null;private aborted=false;private transferred=false;private readonly limits:DocumentAlgorithmLimits;private closing:WorkRetirement|null=null;private discard:ScenePlanCloseJob|null=null;private handoff:DocumentScenePlan|null=null;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){if(!Number.isSafeInteger(algorithms.maxWork)||algorithms.maxWork<1||algorithms.maxWork>1e9)fail("Invalid document vector work limit");this.limits={booleans:{...algorithms.booleans},trace:{...algorithms.trace},maxWork:algorithms.maxWork};const boolean=new DocumentBooleanJob({plan:{assets:[],nodes:[]},limits:this.limits.booleans});boolean.cancel();const trace=new DocumentTraceJob({plan:{assets:[],nodes:[]},limits:this.limits.trace});trace.cancel();this.preparation=new DocumentSceneJob(document,limits);}
 private step():void{
  if(this.closing){this.retiredChildren.push(this.closing);this.closing=null;return;}
  if(this.discard){this.retiredPlans.push(this.discard);this.discard=null;return;}
  if(this.handoff){const plan=this.handoff;this.handoff=null;if(this.progress.phase==="preparing"){this.traces=new DocumentTraceJob({plan,limits:this.limits.trace});this.progress.phase="tracing";}else if(this.progress.phase==="tracing"){this.algorithms=new DocumentBooleanJob({plan,limits:this.limits.booleans});this.progress.phase="algorithms";}else{this.output=plan;this.progress.phase="complete";this.progress.done=true;}return;}
  if(this.progress.phase==="preparing"){this.progress.preparation=this.preparation!.advance(1);if(this.progress.preparation.done){const moved=this.preparation!.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.preparation=null;}return;}
  if(this.progress.phase==="tracing"){this.progress.tracing=this.traces!.advance(1);if(this.progress.tracing.done){const moved=this.traces!.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.discard=new ScenePlanCloseJob(moved.input);this.traces=null;}return;}
  this.progress.resolution=this.algorithms!.advance(1);if(this.progress.resolution.done){const moved=this.algorithms!.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.discard=new ScenePlanCloseJob(moved.input);this.algorithms=null;}
 }
 advance(budget:number):DocumentVectorProgress{if(!Number.isSafeInteger(budget)||budget<1)fail("Invalid document vector work grant");if(this.aborted)cancel();if(this.failure)throw this.failure;try{for(let at=0;at<budget&&!this.progress.done;at++){if(this.progress.work>=this.limits.maxWork)fail("Document vector work limit exceeded");this.step();this.progress.work++;}}catch(error){this.failure=error;throw error;}return{...this.progress};}
 /** 🧹️ Adopt every child and intermediate source plan before relinquishing vector ownership. */
 intoRetirement():{job:DocumentVectorRetirement;output:DocumentScenePlan|null}{
  if(this.transferred)throw Error("Vector ownership already transferred");this.transferred=true;let child=this.closing;this.closing=null;const plans=this.retiredPlans;this.retiredPlans=[];const children=this.retiredChildren;this.retiredChildren=[];if(this.discard){plans.push(this.discard);this.discard=null;}const adopt=(plan:DocumentScenePlan|null)=>{if(plan)plans.push(new ScenePlanCloseJob(plan));};
  if(this.preparation){const moved=this.preparation.intoRetirement();child=moved.job;adopt(moved.output);this.preparation=null;}
  if(this.traces){const moved=this.traces.intoRetirement();child=moved.job;adopt(moved.input);adopt(moved.output);this.traces=null;}
  if(this.algorithms){const moved=this.algorithms.intoRetirement();child=moved.job;adopt(moved.input);adopt(moved.output);this.algorithms=null;}
  adopt(this.handoff);this.handoff=null;const output=this.progress.done&&!this.aborted&&!this.failure?this.output:null;if(!output)adopt(this.output);this.output=null;this.aborted=true;
  if(child)children.push(child);child=null;let slot=0;const job=new UnitRetirement(()=>{switch(slot){case 0:{const retained=children.at(-1);if(retained){if(retained.advance(1).done)children.pop();return false;}break;}case 1:{const plan=plans.at(-1);if(plan){if(plan.advance(1).done)plans.pop();return false;}break;}case 2:break;}return ++slot===3;});return{job,output};
 }
 cancel():void{this.aborted=true;}
 result():DocumentScenePlan{if(this.aborted)cancel();if(this.failure)throw this.failure;if(!this.progress.done)fail("Document vector incomplete");return this.output!;}
}
/** 👁️ Borrowed root and asset identities remain explicit for every producer turn. */
export class DocumentVectorPreparationJob{
 private job:DocumentVectorJob;private layers:DrawingSceneSource["layers"];private assets:DrawingSceneSource["assets"];
 constructor(source:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){this.layers=source.layers;this.assets=source.assets;this.job=new DocumentVectorJob(source,limits,algorithms);}
 advance(source:DrawingSceneSource,grant:number):DocumentVectorProgress{if(source.layers!==this.layers||source.assets!==this.assets)fail("Captured scene source ownership changed");return this.job.advance(grant);}
 cancel():void{this.job.cancel();}
 result():DocumentScenePlan{return this.job.result();}
 intoRetirement():{job:WorkRetirement;output:DocumentScenePlan|null}{const moved=this.job.intoRetirement();this.layers=[];this.assets={};return moved;}
}/** 🧺️ Keeps completed or refused producer ownership until explicit yielded close finishes. */
async function completeSceneProducer<T,P extends {done:boolean}>(job:{advance:(grant:number)=>P;cancel:()=>void;intoRetirement:()=>{job:WorkRetirement;output:T|null}},options:{signal?:AbortSignal;workBudget?:number;onProgress?:(progress:P)=>void},check:()=>void):Promise<T>{
 let close:WorkRetirement|null=null,transferred=false;
 try{for(;;){check();const progress=job.advance(options.workBudget??4096);options.onProgress?.(progress);check();if(progress.done){const moved=job.intoRetirement();transferred=true;close=moved.job;while(!close.advance(4096).done)await new Promise<void>(resolve=>setTimeout(resolve,0));close=null;check();return moved.output!;}await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();if(!transferred)close=job.intoRetirement().job;while(close&&!close.advance(4096).done)await new Promise<void>(resolve=>setTimeout(resolve,0));throw error;}
}
/** ⏳️ Publishes complete vector geometry after yielded work and observer cancellation checks. */
export async function prepareDocumentVector(document:DrawingArtifact,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits,options:DocumentVectorOptions={}):Promise<DocumentScenePlan>{
 const check=()=>{if(options.signal?.aborted)cancel();};check();const job=new DocumentVectorJob(document,limits,algorithms);return completeSceneProducer(job,options,check);
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
 private step():void{
  if(this.closing){this.retiredChildren.push(this.closing);this.closing=null;return;}
  if(this.handoff){this.source=this.handoff.nodes;this.assets=this.handoff.assets;this.handoff=null;return;}
  if(this.vector){this.prepared=this.vector.advance(1);if(this.prepared.done){const moved=this.vector.intoRetirement();this.closing=moved.job;this.handoff=moved.output!;this.vector=null;this.phase="resolving";}else this.phase=this.prepared.phase;return;}
  if(this.phase==="resolving"){const n=this.source[this.at++];if(!n){this.raster=new RasterSceneJob({...this.viewport,assets:this.assets,nodes:this.nodes});this.phase="raster";return;}if(n.content.kind==="group")return;const node=rasterNode(n),m=node.transform,[x,y]=this.pixelScale;this.nodes.push({...node,transform:[m[0]*x,m[1]*y,m[2]*x,m[3]*y,m[4]*x,m[5]*y]});return;}
  if(this.phase==="raster"){this.rendered=this.raster!.advance(1);if(this.rendered.done){const moved=this.raster!.intoRetirement();this.output=moved.output;this.closing=moved.job;this.raster=null;this.phase="complete";}}
 }
 advance(budget:number):DocumentRasterProgress{if(!Number.isSafeInteger(budget)||budget<1)fail("Invalid document raster work grant");if(this.aborted)cancel();if(this.failure)throw this.failure;try{for(let at=0;at<budget&&this.phase!=="complete";at++){if(this.work>=this.maxWork)fail("Document raster work limit exceeded");this.step();this.work++;}}catch(error){this.failure=error;throw error;}return{phase:this.phase,preparation:this.prepared.preparation,tracing:this.prepared.tracing,resolution:this.prepared.resolution,raster:this.rendered,nodes:Math.min(this.at,this.prepared.preparation.layers),work:this.work,done:this.phase==="complete"};}
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
 const check=()=>{if(options.signal?.aborted)cancel();};check();const job=new DocumentRasterJob(document,limits,viewport,resolutionLimits);return completeSceneProducer(job,options,check);
}
export type DocumentSceneContent=({kind:"path";segments:PathSegment[]}&Paint)|Extract<RasterSceneContent,{kind:"image"}>|{kind:"group";children:string[];isolation:boolean}|({kind:"text";content:string;x:number;y:number;size:number}&Paint)|({kind:"boolean";operation:"union"|"difference"|"intersection"|"xor";children:string[];referenceTransform:Matrix}&Paint)|({kind:"trace";source:string;threshold:number;simplifyEpsilon:number}&Paint);
export type DocumentSceneNode=Omit<RasterSceneNode,"content"|"groups">&{sourcePath:number[];lockedAncestors:number;groups:RasterSceneGroup[];content:DocumentSceneContent};
export type DocumentScenePlan={assets:RasterSceneAsset[];nodes:DocumentSceneNode[]};
/** 🧭️ Validate at most 32 authored indices and the exact unsigned lock-mask width. */
export function validateSceneSourceAddress(node:{readonly sourcePath:readonly number[];readonly lockedAncestors:number}):void{
 const path=node.sourcePath,locks=node.lockedAncestors;if(!Array.isArray(path)||path.length<1||path.length>32||!path.every(index=>Number.isSafeInteger(index)&&index>=0&&index<=65535)||!Number.isSafeInteger(locks)||locks<0||locks>=2**path.length)throw Error("Invalid scene source ancestry");
}
export type DocumentSceneLimits={maxNodes:number;maxDepth:number;maxSegments:number;maxReferences:number;maxSourceBytes:number};
export type DocumentSceneProgress={phase:"assets"|"layers"|"paint"|"geometry"|"references"|"validation"|"cycles"|"complete";layers:number;assets:number;segments:number;references:number;sourceBytes:number;work:number;done:boolean};
export type DocumentSceneOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:DocumentSceneProgress)=>void};
type Matrix=[number,number,number,number,number,number];
type Frame={path:number[];lockedAncestors:number;layers:readonly DrawingLayerNode[];at:number;matrix:Matrix;visible:boolean;groups:RasterSceneGroup[]};
const identity:Matrix=[1,0,0,1,0,0];
function fail(message:string):never{throw new drawingDrawingArtifactGuardRefusal("$scene",message);}
const cancel=():never=>{throw new DOMException("Document scene preparation cancelled","AbortError");};
const numeric=(v:Parameters<typeof binary64Value>[0]):number=>{const n=binary64Value(v);if(!Number.isFinite(n)||Math.abs(n)>1e9)fail("Invalid authored scene coordinate");return n;};
const positive=(n:number)=>{if(n<=0)fail("Expected positive authored scene dimension");return n;};
const unit=(n:number)=>{if(n<0||n>1)fail("Expected unit paint or opacity");return n;};
const id=(v:string)=>{if(typeof v!=="string"||!v.length||v.length>4096||new TextEncoder().encode(v).length>4096)fail("Invalid scene identity");return v;};
const blend=(v:string)=>{if(!(DRAWING_BLEND_MODES as readonly string[]).includes(v))fail("Invalid authored scene blend");return v as RasterSceneNode["blendMode"];};
const color=(v:readonly Parameters<typeof binary64Value>[0][]):Color=>{if(v.length!==4)fail("Invalid authored scene color");return v.map(n=>unit(numeric(n))) as Color;};
const multiply=(a:Matrix,b:Matrix):Matrix=>{const m:Matrix=[a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5]];if(!m.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))fail("World scene matrix exceeds coordinate limit");return m.map(n=>n===0?0:n) as Matrix;};
const point=(v:readonly Parameters<typeof binary64Value>[0][]):[number,number]=>{if(v.length!==2)fail("Invalid authored scene point");return[numeric(v[0]!),numeric(v[1]!)];};
function segment(s:DrawingPathSegment):PathSegment{switch(s.kind){case"move":case"line":return{kind:s.kind,to:point(s.to)};case"quad":return{kind:s.kind,ctrl:point(s.ctrl),to:point(s.to)};case"cubic":return{kind:s.kind,ctrl1:point(s.ctrl1),ctrl2:point(s.ctrl2),to:point(s.to)};case"arc":return{kind:s.kind,rx:numeric(s.rx),ry:numeric(s.ry),rotation:numeric(s.rotation),largeArc:s.largeArc,sweep:s.sweep,to:point(s.to)};case"close":return{kind:s.kind};default:return fail("Unknown authored scene segment");}}
function shapeSegment(layer:Extract<DrawingLayerNode,{kind:"shape"}>,at:number):PathSegment|null{
 if(layer.shapeKind==="polygon"){const points=layer.polygon?.points;if(!points)fail("Missing polygon geometry");return at<points.length?{kind:at?"line":"move",to:point(points[at]!)}:points.length&&at===points.length?{kind:"close"}:null;}
 if(layer.shapeKind==="rect"){const r=layer.rect;if(!r)fail("Missing rectangle geometry");const x=numeric(r.x),y=numeric(r.y),w=numeric(r.width),h=numeric(r.height);return ([{kind:"move",to:[x,y]},{kind:"line",to:[x+w,y]},{kind:"line",to:[x+w,y+h]},{kind:"line",to:[x,y+h]},{kind:"close"}] as PathSegment[])[at]??null;}
 if(layer.shapeKind==="line"){const l=layer.line;if(!l)fail("Missing line geometry");return ([{kind:"move",to:[numeric(l.x1),numeric(l.y1)]},{kind:"line",to:[numeric(l.x2),numeric(l.y2)]}] as PathSegment[])[at]??null;}
 if(layer.shapeKind==="ellipse"||layer.shapeKind==="circle"){const e=layer.shapeKind==="ellipse"?layer.ellipse:layer.circle;if(!e)fail("Missing ellipse geometry");const cx=numeric(e.cx),cy=numeric(e.cy),rx=numeric("rx" in e?e.rx:e.r),ry=numeric("ry" in e?e.ry:e.r);if(at===5)return{kind:"close"};const to=([[cx,cy-ry],[cx+rx,cy],[cx,cy+ry],[cx-rx,cy],[cx,cy-ry]] as [number,number][])[at];return to?at===0?{kind:"move",to}:{kind:"arc",rx:Math.abs(rx),ry:Math.abs(ry),rotation:0,largeArc:false,sweep:(rx>=0)===(ry>=0),to}:null;}
 return fail("Unknown primitive shape");
}
/** 🧱️ Traverses the entire document without recursive flattening or duplicated image sources. */
export class DocumentSceneJob {
 private rootVisible:((index:number)=>boolean)|undefined;private source:DrawingArtifact["assets"];private assetKeys:string[];private assetAt=0;private assetChar=0;private assetCurrent:RasterSceneAsset|null=null;private sourceBytes=0;private frames:Frame[];private plan:DocumentScenePlan={assets:[],nodes:[]};private ids=new Map<string,number>();private assetIds=new Set<string>();
 private current:DrawingLayerNode|null=null;private node:DocumentSceneNode|null=null;private frame:Frame|null=null;private at=0;private strokeAt=0;private textAt=0;private textChars=0;private phase:DocumentSceneProgress["phase"]="assets";private work=0;private segments=0;private references=0;private layers=0;private assets=0;private failure:unknown=null;private aborted=false;
 private validateAt=0;private refAt=0;private cycleAt=0;private graph:{index:number;at:number}[]=[];private visited=new Set<number>();private visiting=new Set<number>();private readonly limits:DocumentSceneLimits;private transferred=false;private cleanupSlot=0;private cleanupEntries:Iterator<unknown>|null=null;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits){
  for(const [key,cap] of [["maxNodes",1024],["maxDepth",32],["maxSegments",65536],["maxReferences",32768],["maxSourceBytes",268439552]] as const)if(!Number.isSafeInteger(limits[key])||limits[key]<1||limits[key]>cap)fail("Invalid scene preparation limits");
  if(!Array.isArray(document.layers)||document.layers.length>limits.maxNodes||!document.assets||typeof document.assets!=="object"||Array.isArray(document.assets))fail("Invalid drawing document scene input");
  this.limits={...limits};this.rootVisible=document.rootVisible;this.source=document.assets;this.assetKeys=Object.keys(this.source).sort();if(this.assetKeys.length>1024)fail("Scene asset catalog exceeds limit");
  this.frames=[{path:[],lockedAncestors:0,layers:document.layers,at:0,matrix:identity,visible:true,groups:[]}];
 }
 private start(layer:DrawingLayerNode,frame:Frame):void{
  if(this.layers>=this.limits.maxNodes||this.ids.has(id(layer.id)))fail("Duplicate scene identity or layer limit");if(typeof layer.visible!=="boolean")fail("Invalid scene visibility");
  this.ids.set(layer.id,this.plan.nodes.length);this.layers++;const t=layer.transform,local=drawingTransformToMatrix({x:numeric(t.x),y:numeric(t.y),scaleX:numeric(t.scaleX),scaleY:numeric(t.scaleY),shear:numeric(t.shear),rotation:numeric(t.rotation)}),matrix=multiply(frame.matrix,local),opacity=unit(numeric(layer.opacity)),mode=blend(layer.blendMode);
  let content:DocumentSceneContent;const empty:Paint={fillRule:layer.attributes.fillRule,fill:null,stroke:null};if(!["nonzero","evenodd"].includes(empty.fillRule))fail("Invalid scene fill rule");
  switch(layer.kind){
   case"path":case"shape":content={kind:"path",segments:[],...empty};break;
   case"image":content={kind:"image",asset:id(layer.imageKey),width:positive(numeric(layer.width)),height:positive(numeric(layer.height))};break;
   case"group":if(typeof layer.isolation!=="boolean"||!Array.isArray(layer.children)||layer.children.length>1024)fail("Invalid scene group");content={kind:"group",children:[],isolation:layer.isolation};break;
   case"text":if(typeof layer.content!=="string"||layer.content.length>131072)fail("Scene text exceeds limit");content={kind:"text",content:layer.content,x:numeric(layer.x),y:numeric(layer.y),size:positive(numeric(layer.size)),...empty};break;
   case"boolean":if(!["union","difference","intersection","xor"].includes(layer.operation)||!Array.isArray(layer.children)||layer.children.length>1024)fail("Invalid boolean scene work");content={kind:"boolean",operation:layer.operation as "union",children:[],referenceTransform:[...frame.matrix],...empty};break;
   case"trace":content={kind:"trace",source:id(layer.sourceKey),threshold:unit(numeric(layer.params.threshold)),simplifyEpsilon:numeric(layer.params.simplifyEpsilon),...empty};if(content.simplifyEpsilon<0)fail("Invalid trace epsilon");break;
   default:return fail("Unknown document layer kind");
  }
  if(frame.path.length>=this.limits.maxDepth||typeof layer.locked!=="boolean")fail("Invalid scene source ancestry");
  this.current=layer;this.frame=frame;this.node={sourcePath:[...frame.path,frame.at-1],lockedAncestors:frame.lockedAncestors+(layer.locked?2**frame.path.length:0),id:layer.id,groups:frame.groups.slice(),transform:matrix,opacity,blendMode:mode,visible:frame.visible&&layer.visible&&(frame.path.length!==0||(this.rootVisible?.(frame.at-1)??true)),content};this.at=0;this.strokeAt=0;this.textAt=0;this.textChars=0;this.phase="paint";
 }
 private paint():void{
  const attrs=this.current!.attributes,c=this.node!.content;
  if("fill" in c){
   const f=attrs.fill;
   if(this.at===0){if(f){if(f.kind==="solid")c.fill={kind:f.kind,color:color(f.color)};else if(f.kind==="linearGradient")c.fill={kind:f.kind,x1:numeric(f.x1),y1:numeric(f.y1),x2:numeric(f.x2),y2:numeric(f.y2),stops:[]};else if(f.kind==="radialGradient")c.fill={kind:f.kind,cx:numeric(f.cx),cy:numeric(f.cy),r:positive(numeric(f.r)),stops:[]};else fail("Unknown scene paint");if("stops" in f&&(!f.stops.length||f.stops.length>64))fail("Invalid scene gradient stops");}this.at++;return;}
   if(f&&f.kind!=="solid"&&this.at<=f.stops.length){const stop=f.stops[this.at++-1]!;(c.fill as Extract<Fill,{stops:unknown}>).stops.push({offset:unit(numeric(stop.offset)),color:color(stop.color)});return;}
   const s=attrs.stroke;
   if(this.strokeAt===0){if(s){if(!["butt","round","square"].includes(s.cap)||!["miter","round","bevel"].includes(s.join))fail("Invalid scene stroke");const width=numeric(s.width);if(width<0||s.dash&&s.dash.length>1024)fail("Invalid scene stroke");c.stroke={color:color(s.color),width,cap:s.cap,join:s.join,...(s.dash?{dash:[]}:{} )};}this.strokeAt++;return;}
   if(s?.dash&&this.strokeAt<=s.dash.length){const value=numeric(s.dash[this.strokeAt++-1]!);if(value<0)fail("Invalid scene dash");(c.stroke!.dash as number[]).push(value);return;}
  }
  this.at=0;this.phase=this.current!.kind==="path"||this.current!.kind==="shape"?"geometry":this.current!.kind==="group"||this.current!.kind==="boolean"?"references":"geometry";
 }
 private finish():void{
  const n=this.node!,layer=this.current!;if(layer.kind==="group"&&this.frames.length>=this.limits.maxDepth&&layer.children.length)fail("Scene depth limit exceeded");this.plan.nodes.push(n);
  if(layer.kind==="group"){const scopes=n.groups.slice();if(layer.isolation||n.opacity!==1||n.blendMode!=="normal")scopes.push({id:n.id,opacity:n.opacity,blendMode:n.blendMode});this.frames.push({path:n.sourcePath.slice(),lockedAncestors:n.lockedAncestors,layers:layer.children,at:0,matrix:[...n.transform],visible:n.visible,groups:scopes});}
  this.current=null;this.node=null;this.frame=null;this.phase="layers";
 }
 private step():void{
  if(this.phase==="assets"){
   if(this.assetCurrent){const asset=this.assetCurrent,source=this.source[asset.id]!;if(this.assetChar===source.samples.length){this.plan.assets.push(asset);this.assetIds.add(asset.id);this.assets++;this.assetCurrent=null;return;}const sample=source.samples[this.assetChar++]!;if(!Array.isArray(sample)||sample.length!==4||sample.some(component=>!Number.isInteger(component)||component<0||component>255))fail("Invalid intrinsic RGBA sample");asset.image.pixels.set(sample,(this.assetChar-1)*4);this.sourceBytes+=4;return;}
   const key=this.assetKeys[this.assetAt++];if(key===undefined){this.source={};this.assetKeys=[];this.phase="layers";return;}const asset=this.source[key]!;id(key);const count=asset.width*asset.height;if(!Number.isSafeInteger(count)||count<1||count>16777216||!Array.isArray(asset.samples)||asset.samples.length!==count||count*4>this.limits.maxSourceBytes-this.sourceBytes)fail("Invalid intrinsic image extent or sample byte limit");this.assetCurrent={id:key,image:{width:asset.width,height:asset.height,pixels:new Uint8Array(count*4)}};this.assetChar=0;return;
  }
  if(this.phase==="layers"){const f=this.frames.at(-1);if(!f){this.phase="validation";return;}if(f.at===f.layers.length){if(!f.groups.pop())this.frames.pop();return;}this.start(f.layers[f.at++]!,f);return;}
  if(this.phase==="paint"){this.paint();return;}
  if(this.phase==="geometry"){
   const layer=this.current!,c=this.node!.content;if(c.kind==="path"){const next=layer.kind==="path"?layer.segments[this.at]?segment(layer.segments[this.at]!):null:shapeSegment(layer as Extract<DrawingLayerNode,{kind:"shape"}>,this.at);if(next){const values=next.kind==="close"?[]:next.kind==="quad"?[...next.to,...next.ctrl]:next.kind==="cubic"?[...next.to,...next.ctrl1,...next.ctrl2]:next.kind==="arc"?[...next.to,next.rx,next.ry,next.rotation]:next.to;if(!values.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))fail("Derived primitive coordinates exceed scene limit");if(++this.segments>this.limits.maxSegments||c.segments.length>=65536)fail("Scene segment limit exceeded");c.segments.push(next);this.at++;return;}}
   if(layer.kind==="text"&&this.textAt<layer.content.length){const code=layer.content.codePointAt(this.textAt)!;this.textAt+=code>65535?2:1;if(++this.textChars>65536)fail("Scene text exceeds limit");return;}this.finish();return;
  }
  if(this.phase==="references"){const layer=this.current!;if(layer.kind!=="group"&&layer.kind!=="boolean")fail("Expected reference-bearing layer");const child=layer.children[this.at++];if(child!==undefined){if(++this.references>this.limits.maxReferences)fail("Scene reference limit exceeded");const key=id(typeof child==="string"?child:child.id);(this.node!.content as {children:string[]}).children.push(key);return;}this.finish();return;}
  if(this.phase==="validation"){const n=this.plan.nodes[this.validateAt];if(!n){this.phase="cycles";return;}const c=n.content;if(c.kind==="image"&&!this.assetIds.has(c.asset)||c.kind==="trace"&&!this.assetIds.has(c.source))fail("Missing scene source asset");if("children" in c&&this.refAt<c.children.length){if(!this.ids.has(c.children[this.refAt++]!))fail("Missing scene operand");return;}this.validateAt++;this.refAt=0;return;}
  if(this.phase==="cycles"){if(!this.graph.length){if(this.cycleAt===this.plan.nodes.length){if(this.cleanup())this.phase="complete";return;}const index=this.cycleAt++;if(this.visited.has(index))return;this.visiting.add(index);this.graph.push({index,at:0});return;}const top=this.graph.at(-1)!,c=this.plan.nodes[top.index]!.content;if("children" in c&&top.at<c.children.length){const index=this.ids.get(c.children[top.at++]!)!;if(this.visiting.has(index))fail("Cyclic scene operands");if(!this.visited.has(index)){this.visiting.add(index);this.graph.push({index,at:0});}return;}this.graph.pop();this.visiting.delete(top.index);this.visited.add(top.index);}
 }
 private removeEntry(collection:Map<unknown,unknown>|Set<unknown>):boolean{this.cleanupEntries??=collection.keys();const entry=this.cleanupEntries.next();if(entry.done){this.cleanupEntries=null;return false;}collection.delete(entry.value);return true;}
 private cleanup():boolean{const collection=this.cleanupSlot===0?this.ids:this.cleanupSlot===1?this.assetIds:this.cleanupSlot===2?this.visited:this.cleanupSlot===3?this.visiting:null;if(collection){if(this.removeEntry(collection))return false;collection.clear();}else if(this.cleanupSlot===4)this.graph=[];return ++this.cleanupSlot===7;}
 advance(budget:number):DocumentSceneProgress{
  if(!Number.isSafeInteger(budget)||budget<1)fail("Invalid scene preparation work grant");if(this.aborted)cancel();if(this.failure)throw this.failure;
  try{for(let at=0;at<budget&&this.phase!=="complete";at++){this.step();this.work++;}}catch(error){this.failure=error;throw error;}
  return{phase:this.phase,layers:this.layers,assets:this.assets,segments:this.segments,references:this.references,sourceBytes:this.sourceBytes,work:this.work,done:this.phase==="complete"};
 }
 /** 🧹️ Preserve valid output and drain private nodes, frames and indexes without touching the source. */
 intoRetirement():{job:DocumentSceneRetirement;output:DocumentScenePlan|null}{
  if(this.transferred)throw Error("Preparation ownership already transferred");this.transferred=true;const output=this.phase==="complete"&&!this.aborted&&!this.failure?this.plan:null;let draft:ScenePlanCloseJob|null=output?null:new ScenePlanCloseJob(this.plan);this.plan={assets:[],nodes:[]};let node:ScenePlanCloseJob|null=this.node?new ScenePlanCloseJob({assets:[],nodes:[this.node]}):null;this.node=null;this.frame=null;this.current=null;this.cleanupEntries=null;this.aborted=true;
  let slot=0;const job=new UnitRetirement(()=>{switch(slot){
   case 0:if(draft){if(!draft.advance(1).done)return false;draft=null;}break;
   case 1:if(node){if(!node.advance(1).done)return false;node=null;}break;
   case 2:{const frame=this.frames.at(-1);if(frame){if(!frame.groups.pop())this.frames.pop();return false;}this.frames=[];break;}
   case 3:if(this.removeEntry(this.ids))return false;this.ids.clear();break;case 4:if(this.removeEntry(this.assetIds))return false;this.assetIds.clear();break;
   case 5:if(this.removeEntry(this.visited))return false;this.visited.clear();break;case 6:if(this.removeEntry(this.visiting))return false;this.visiting.clear();break;
   case 7:this.graph=[];break;case 8:this.current=null;break;case 9:this.assetKeys=[];this.assetCurrent=null;break;case 10:this.source={};break;case 11:this.plan={assets:[],nodes:[]};break;
  }return ++slot===12;});return{job,output};
 }
 cancel():void{this.aborted=true;}
 result():DocumentScenePlan{if(this.aborted)cancel();if(this.failure)throw this.failure;if(this.phase!=="complete")fail("Document scene preparation incomplete");return this.plan;}
}
/** ⏳️ Yields preparation between grants and checks observers before publication. */
export async function prepareDocumentScene(document:DrawingArtifact,limits:DocumentSceneLimits,options:DocumentSceneOptions={}):Promise<DocumentScenePlan>{
 const check=()=>{if(options.signal?.aborted)cancel();};check();const job=new DocumentSceneJob(document,limits);return completeSceneProducer(job,options,check);
}
function rasterNode(node:DocumentSceneNode):RasterSceneNode{const content=node.content;if(content.kind!=="path"&&content.kind!=="image")fail("Unresolved "+content.kind+" layer: "+node.id);return{id:node.id,groups:node.groups,transform:node.transform,opacity:node.opacity,blendMode:node.blendMode,visible:node.visible,content};}
/** 🚪️ Hands prepared paths/images to raster; unresolved work is an explicit refusal. */
export function resolvedSceneInput(plan:DocumentScenePlan,input:Omit<RasterSceneInput,"assets"|"nodes">):RasterSceneInput{
 const nodes:RasterSceneNode[]=[];for(const n of plan.nodes){if(n.content.kind==="group")continue;nodes.push(rasterNode(n));}return{...input,assets:plan.assets,nodes};
}
export {sceneSelectionRelation,type SceneSelectionRelation} from "./🧭️selection/🟦️.ts";
