import { validateSceneSourceAddress } from "../🗺️plan-model/🟦️.ts";
/** 🔍️ Actual encoded image records become private trace paths under caller grants. */
import {ScenePlanOwner,ScenePlanContentReplaceJob,type ScenePlanRead,type ScenePlanNode,type ScenePlanContent} from "../🗺️plan-model/🎟️owned/🟦️.ts";
import {BitmapTraceJob,type BitmapTraceProgress} from "@semio-tech/framework/2d/trace";
import type {PixelImage} from "@semio-tech/framework/pixels/editing";
import type {PathSegment} from "@semio-tech/framework/2d";
import type {RasterSceneAsset} from "../📷️raster/🟦️.ts";
import type {WorkRetirement} from "@semio-tech/framework/2d/retire";
import {ScenePlanCloseJob} from "../🧹️retire/🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkRetirementProgress,type RetainedWorkTurn} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {DrawingSceneWorkController,driveDrawingSceneWork} from "../🎟️work/🟦️.ts";
export type DocumentTraceRetirement=RetainedWorkRetirementCursor;
export type DocumentTraceRetirementProgress=RetainedWorkRetirementProgress;
export type DocumentTraceLimits={maxPixels:number;maxAdmittedPixels:number;maxSourceBytes:number;maxEdges:number;maxSegments:number;maxRetainedSegments:number;maxWork:number};
export type DocumentTraceInput={plan:ScenePlanOwner;limits:DocumentTraceLimits};
export type DocumentTraceProgress={phase:"assets"|"indexing"|"nodes"|"luma"|"tracing"|"publishing"|"complete";assets:number;nodes:number;resolved:number;admittedImages:number;pixels:number;sourceBytes:number;segments:number;work:number;trace:BitmapTraceProgress|null;done:boolean};
type Sequence<T>={readonly length:number;at(index:number):T|undefined};
function invalid(message:string):never{throw new RangeError(message);}
function row<T>(source:Sequence<T>,at:number):T{const value=source.at(at);if(value===undefined)invalid("Missing document trace row");return value;}
function identity(v:string):void{if(typeof v!=="string"||!v.length||v.length>4096||new TextEncoder().encode(v).length>4096)invalid("Invalid document trace identity");}
function drain(cursor:RetainedWorkRetirementCursor,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{const before=turn.actualWork();let done=false;try{done=cursor.advance(1,work).done;}finally{turn.commitActualWork(cursor.lastTurnWork());}return done&&turn.actualWork()===before;}
/** ⏱️ Decode and convert each referenced source once; trace and publish native pixel coordinates privately. */
export class DocumentTraceJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private phase:DocumentTraceProgress["phase"]|"tracerClose"|"childClose"|"replaceBirth"|"replace"|"replaceTake"|"publishNext"="assets";private assets=0;private nodes=0;private resolved=0;private admittedImages=0;private pixels=0;private sourceBytes=0;private segments=0;private work=0;private at=0;private pixel=0;private publishAt=0;
 private catalog=new Map<string,RasterSceneAsset>();private ids=new Set<string>();private masks=new Map<string,Uint8Array>();private dimensions=new Map<string,[number,number]>();private source="";private current:ScenePlanNode|null=null;
 private image:PixelImage|null=null;private mask:Uint8Array|null=null;private tracer:BitmapTraceJob|null=null;private trace:BitmapTraceProgress|null=null;private raw:PathSegment[]=[];private candidate:PathSegment[]=[];
 private failure:unknown=null;private cancelled=false;private transferred=false;private closingTracer:WorkRetirement|null=null;
 private owner:ScenePlanOwner|null;private view:ScenePlanRead|null;private limits:DocumentTraceLimits;private closing:RetainedWorkRetirementCursor|null=null;private afterClose="";private replacement:ScenePlanContentReplaceJob|null=null;
 /** 🔒️ Limits validate without a plan so owners of other jobs can refuse early. */
 static checkLimits(l:DocumentTraceLimits):void{for(const [value,min,max]of [[l.maxPixels,1,16777216],[l.maxAdmittedPixels,1,67108864],[l.maxSourceBytes,1,268439552],[l.maxEdges,1,65536],[l.maxSegments,1,65536],[l.maxRetainedSegments,1,65536],[l.maxWork,1,1e9]])if(!Number.isSafeInteger(value)||value!<min!||value!>max!)invalid("Invalid document trace limits");}
 constructor(input:DocumentTraceInput){
  DocumentTraceJob.checkLimits(input.limits);const view=ScenePlanOwner.prototype.borrow.call(input.plan);
  if(view.assets.length>1024||view.nodes.length>1024)invalid("Invalid document trace plan");
  this.limits={...input.limits};this.owner=ScenePlanOwner.transfer(input.plan);this.view=view;
 }
 private active():ScenePlanRead{if(this.view===null)invalid("Document trace source transferred");return this.view;}
 private startTrace(mask:Uint8Array):void{
  const c=this.current!.content;if(c.kind!=="trace")invalid("Expected document trace record");const asset=this.masks.get(c.source);if(asset!==mask)invalid("Missing document trace mask");
  const width=this.dimensions.get(c.source)![0],height=this.dimensions.get(c.source)![1],l=this.limits;this.tracer=new BitmapTraceJob({width,height,mask,threshold:c.threshold,simplifyEpsilon:c.simplifyEpsilon,maxPixels:l.maxPixels,maxEdges:l.maxEdges,maxSegments:Math.max(1,Math.min(l.maxSegments,l.maxRetainedSegments-this.segments)),maxWork:l.maxWork});this.phase="tracing";
 }
 private unit():void{
  const view=this.active();
  if(this.phase==="assets"){const a=view.assets.at(this.assets);if(!a){this.phase="indexing";return;}identity(a.id);if(!a.image||!Number.isSafeInteger(a.image.width)||!Number.isSafeInteger(a.image.height)||a.image.width<1||a.image.height<1||a.image.pixels.length!==a.image.width*a.image.height*4)invalid("Invalid intrinsic document trace image");if(this.catalog.has(a.id))invalid("Invalid or duplicate document trace asset");this.catalog.set(a.id,a);this.assets++;return;}
  if(this.phase==="indexing"){const n=view.nodes.at(this.nodes);if(!n){this.phase="nodes";return;}validateSceneSourceAddress({sourcePath:Array.from({length:n.sourcePath.length},(_,at)=>row(n.sourcePath,at)),lockedAncestors:n.lockedAncestors});identity(n.id);if(this.ids.has(n.id)||!n.content||!["path","image","group","text","boolean","trace"].includes(n.content.kind)||!Array.isArray(n.transform)||n.transform.length!==6||!n.transform.every(v=>Number.isFinite(v)&&Math.abs(v)<=1e9))invalid("Invalid or duplicate document trace node");this.ids.add(n.id);const c=n.content;
   if(c.kind==="path"){if(typeof c.segments?.length!=="number")invalid("Invalid document trace path");this.segments+=c.segments.length;if(this.segments>this.limits.maxRetainedSegments)invalid("Document trace retained segment limit exceeded");}
   if(c.kind==="trace"){identity(c.source);if(!this.catalog.has(c.source))invalid("Missing document trace source: "+n.id+" -> "+c.source);if(!Number.isFinite(c.threshold)||c.threshold<0||c.threshold>1||!Number.isFinite(c.simplifyEpsilon)||c.simplifyEpsilon<0||c.simplifyEpsilon>8192)invalid("Invalid document trace parameters: "+n.id);}this.nodes++;return;
  }
  if(this.phase==="nodes"){const n=view.nodes.at(this.at);if(!n){this.phase="complete";return;}if(n.content.kind!=="trace"){this.at++;return;}this.current=n;const c=n.content,mask=this.masks.get(c.source);if(mask){this.startTrace(mask);return;}
   const a=this.catalog.get(c.source)!,l=this.limits,count=a.image.width*a.image.height;if(this.sourceBytes+a.image.pixels.length>l.maxSourceBytes||count>Math.min(l.maxPixels,l.maxAdmittedPixels-this.pixels))invalid("Document trace intrinsic image budget exceeded");this.sourceBytes+=a.image.pixels.length;this.source=c.source;this.image=a.image;this.pixels+=count;this.admittedImages++;this.dimensions.set(this.source,[this.image.width,this.image.height]);this.mask=new Uint8Array(count);this.pixel=0;this.phase="luma";return;
  }
  if(this.phase==="luma"){if(this.pixel===this.mask!.length){const mask=this.mask!;this.masks.set(this.source,mask);this.image=null;this.mask=null;this.startTrace(mask);return;}const p=this.image!.pixels,i=this.pixel*4;this.mask![this.pixel++]=Math.round((299*p[i]!+587*p[i+1]!+114*p[i+2]!)*p[i+3]!/255000);return;}
  if(this.phase==="publishing"){
   if(this.publishAt<this.raw.length){if(this.segments>=this.limits.maxRetainedSegments)invalid("Document trace retained segment limit exceeded");const s=this.raw[this.publishAt++]!;if(s.kind==="move"||s.kind==="line")this.candidate.push({kind:s.kind,to:[s.to[0],s.to[1]]});else if(s.kind==="close")this.candidate.push({kind:"close"});else invalid("Unresolved document trace curve");this.segments++;return;}
   this.closingTracer=this.tracer!.intoRetirement().job;this.tracer=null;this.raw=[];this.publishAt=0;this.phase="tracerClose";return;
  }
  throw Error("Unknown document trace phase");
 }
 private stage(cursor:RetainedWorkRetirementCursor,after:string):void{if(this.closing!==null)throw Error("Document trace resident child close already occupied");this.closing=cursor;this.afterClose=after;this.phase="childClose";}
 private step(c:RetainedWorkProducerStep):void{
  switch(this.phase){
   case"childClose":{let done=false;try{done=this.closing!.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.closing!.lastTurnWork());}if(!done||c.hasActualWork())return;this.closing=null;this.phase=this.afterClose as typeof this.phase;this.afterClose="";c.commitControlWork();return;}
   case"tracing":{this.trace=c.advanceChild(this.tracer!);if(this.trace.done){this.raw=this.tracer!.result();this.candidate=[];this.publishAt=0;this.phase="publishing";}return;}
   case"tracerClose":{if(!c.work.canStep())return;if(this.closingTracer!.advance(1).done){this.closingTracer=null;this.phase="replaceBirth";}c.commitControlWork();return;}
   case"replaceBirth":{c.requireSemanticWork();const old=this.current!.content;if(old.kind!=="trace")invalid("Expected document trace record");const content:ScenePlanContent={kind:"path",segments:this.candidate,fillRule:old.fillRule,fill:old.fill,stroke:old.stroke};this.replacement=new ScenePlanContentReplaceJob(this.owner!,this.at,content,{maxWork:this.limits.maxWork});this.owner=null;this.phase="replace";c.commitSemanticWork();return;}
   case"replace":if(c.advanceChild(this.replacement!).done)this.phase="replaceTake";return;
   case"replaceTake":{const moved=this.replacement!.intoRetirement();if(moved.output===null)throw Error("Completed trace Scene owner absent");this.owner=moved.output;this.replacement=null;this.stage(moved.job,"publishNext");c.commitControlWork();return;}
   case"publishNext":this.current=null;this.candidate=[];this.at++;this.resolved++;this.phase="nodes";c.commitControlWork();return;
   case"complete":return;
  }
  this.unit();c.commitSemanticWork();
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.replacement?.failurePending()===true;}
 settlementPending():boolean{
  if(this.transferred)return false;
  switch(this.phase){
   case"complete":case"childClose":case"tracerClose":case"replaceTake":case"publishNext":return true;
   case"replace":return this.replacement?.settlementPending()===true;
   default:return false;
  }
 }
 private publicPhase():DocumentTraceProgress["phase"]{switch(this.phase){case"tracerClose":case"childClose":case"replaceBirth":case"replace":case"replaceTake":case"publishNext":return"publishing";default:return this.phase;}}
 private progress():DocumentTraceProgress{return{phase:this.publicPhase(),assets:this.assets,nodes:this.nodes,resolved:this.resolved,admittedImages:this.admittedImages,pixels:this.pixels,sourceBytes:this.sourceBytes,segments:this.segments,work:this.work,trace:this.trace,done:this.phase==="complete"};}
 advance(items:number,work:RetainedWorkLedger):DocumentTraceProgress{
  try{return this.#runtime.advance(items,work,{maximumWork:this.limits.maxWork,workLimitMessage:"Document trace work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Document trace cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>this.progress()});}
  catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.cancelled)this.failure=error;throw error;}
 }
 result():ScenePlanRead{if(this.cancelled||this.transferred)throw new DOMException("Document trace cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")invalid("Document trace incomplete");return this.owner!.borrow();}
 retireOutput(output:ScenePlanOwner):RetainedWorkRetirementCursor{return new ScenePlanCloseJob(output);}
 /** 🧹️ Return the valid output owner while retiring all private scene and trace owners. */
 intoRetirement():{job:DocumentTraceRetirement;output:ScenePlanOwner|null}{
  if(this.transferred)throw Error("Document trace ownership already transferred");this.transferred=true;
  const output=this.phase==="complete"&&!this.failure&&!this.cancelled?this.owner:null;this.cancelled=true;
  if(this.tracer){this.closingTracer=this.tracer.intoRetirement().job;this.tracer=null;}
  let prior=this.closing,replacementClose:RetainedWorkRetirementCursor|null=null;this.closing=null;
  if(this.replacement){const moved=this.replacement.intoRetirement();replacementClose=moved.job;if(moved.output!==null)this.owner=moved.output;this.replacement=null;}
  let scene=this.owner!==null&&this.owner!==output?new ScenePlanCloseJob(this.owner):null;this.owner=null;let cleared=false;
  const job=new RetainedWorkRetirement((work,turn)=>{
   if(prior){if(!drain(prior,work,turn))return false;prior=null;turn.commitActualWork();return false;}
   if(replacementClose){if(!drain(replacementClose,work,turn))return false;replacementClose=null;turn.commitActualWork();return false;}
   if(this.closingTracer){if(!work.canStep())return false;if(this.closingTracer.advance(1).done)this.closingTracer=null;turn.commitActualWork();return false;}
   if(scene){if(!drain(scene,work,turn))return false;scene=null;turn.commitActualWork();return false;}
   if(!cleared){if(!work.canStep())return false;this.clear();cleared=true;turn.commitActualWork();return false;}
   return true;
  });
  return{job,output};
 }
 private clear():void{this.view=null;this.catalog.clear();this.ids.clear();this.masks.clear();this.dimensions.clear();this.image=null;this.mask=null;this.raw=[];this.candidate=[];this.current=null;this.source="";this.afterClose="";}
 cancel():void{this.cancelled=true;}
}
/** ⏳️ Caller-funded turns publish the traced plan only after its owned close. */
export async function resolveDocumentTraces(input:DocumentTraceInput,control:{work:DrawingSceneWorkController;operation:object;source:object;onProgress?:(p:DocumentTraceProgress)=>void}):Promise<ScenePlanOwner>{
 return driveDrawingSceneWork(control.work,control.operation,control.source,new DocumentTraceJob(input),control.onProgress);
}
