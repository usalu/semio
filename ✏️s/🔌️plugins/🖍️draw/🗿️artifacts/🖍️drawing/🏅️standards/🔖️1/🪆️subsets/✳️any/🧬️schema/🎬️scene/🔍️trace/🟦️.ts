import {validateSceneSourceAddress} from "../📋️prepare/🟦️.ts";
/** 🔍️ Actual encoded image records become private trace paths under grants. */
import type {DocumentScenePlan,DocumentSceneNode} from "../📋️prepare/🟦️.ts";
import {BitmapTraceJob,type BitmapTraceProgress} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🔍️trace/🟦️.ts";
import type {PixelImage} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/✍️editing/🟦️.ts";
import type {PathSegment} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🟦️.ts";
import type {RasterSceneAsset} from "../📷️raster/🟦️.ts";
import {UnitRetirement,type WorkRetirement,type WorkRetirementProgress} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧹️retire/🟦️.ts";
import {ScenePlanCloseJob} from "../🧹️retire/🟦️.ts";
import {SceneNodeCopyJob} from "../📋️prepare/📋️copy/🟦️.ts";
export type DocumentTraceRetirement=WorkRetirement;
export type DocumentTraceRetirementProgress=WorkRetirementProgress;
export type DocumentTraceLimits={maxPixels:number;maxAdmittedPixels:number;maxSourceBytes:number;maxEdges:number;maxSegments:number;maxRetainedSegments:number;maxWork:number};
export type DocumentTraceInput={plan:DocumentScenePlan;limits:DocumentTraceLimits};
export type DocumentTraceProgress={phase:"assets"|"indexing"|"nodes"|"luma"|"tracing"|"publishing"|"complete";assets:number;nodes:number;resolved:number;admittedImages:number;pixels:number;sourceBytes:number;segments:number;work:number;trace:BitmapTraceProgress|null;done:boolean};
export type DocumentTraceOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(p:DocumentTraceProgress)=>void};
function invalid(message:string):never{throw new RangeError(message);}
function identity(v:string):void{if(typeof v!=="string"||!v.length||v.length>4096||new TextEncoder().encode(v).length>4096)invalid("Invalid document trace identity");}
/** ⏱️ Decode and convert each referenced source once; trace and publish native pixel coordinates privately. */
export class DocumentTraceJob{
 private phase:DocumentTraceProgress["phase"]="assets";private assets=0;private nodes=0;private resolved=0;private admittedImages=0;private pixels=0;private sourceBytes=0;private segments=0;private work=0;private at=0;private pixel=0;private publishAt=0;
 private catalog=new Map<string,RasterSceneAsset>();private ids=new Set<string>();private masks=new Map<string,Uint8Array>();private source="";private current:DocumentSceneNode|null=null;
 private image:PixelImage|null=null;private mask:Uint8Array|null=null;private tracer:BitmapTraceJob|null=null;private trace:BitmapTraceProgress|null=null;private raw:PathSegment[]=[];private candidate:PathSegment[]=[];
 private output:DocumentScenePlan={assets:[],nodes:[]};private failure:unknown=null;private cancelled=false;private transferred=false;private closingTracer:WorkRetirement|null=null;private copyNode:SceneNodeCopyJob|null=null;private cleanupSlot=0;private cleanupEntries:Iterator<string>|null=null;private publishAssetAt=0;
 constructor(private input:DocumentTraceInput){
  const l=input.limits;for(const [value,min,max]of [[l.maxPixels,1,16777216],[l.maxAdmittedPixels,1,67108864],[l.maxSourceBytes,1,268439552],[l.maxEdges,1,65536],[l.maxSegments,1,65536],[l.maxRetainedSegments,1,65536],[l.maxWork,1,1e9]])if(!Number.isSafeInteger(value)||value!<min!||value!>max!)invalid("Invalid document trace limits");
  if(!input.plan||!Array.isArray(input.plan.assets)||input.plan.assets.length>1024||!Array.isArray(input.plan.nodes)||input.plan.nodes.length>1024)invalid("Invalid document trace plan");this.input={plan:input.plan,limits:{...l}};
 }
 private startTrace(mask:Uint8Array):void{
  const c=this.current!.content;if(c.kind!=="trace")invalid("Expected document trace record");const asset=this.masks.get(c.source);if(asset!==mask)invalid("Missing document trace mask");
  const width=this.dimensions.get(c.source)![0],height=this.dimensions.get(c.source)![1],l=this.input.limits;this.tracer=new BitmapTraceJob({width,height,mask,threshold:c.threshold,simplifyEpsilon:c.simplifyEpsilon,maxPixels:l.maxPixels,maxEdges:l.maxEdges,maxSegments:Math.max(1,Math.min(l.maxSegments,l.maxRetainedSegments-this.segments)),maxWork:l.maxWork});this.phase="tracing";
 }
 private dimensions=new Map<string,[number,number]>();
 private step():void{
  if(this.phase==="assets"){const a=this.input.plan.assets[this.assets];if(!a){this.phase="indexing";return;}identity(a.id);if(!a.image||!Number.isSafeInteger(a.image.width)||!Number.isSafeInteger(a.image.height)||a.image.width<1||a.image.height<1||a.image.pixels.length!==a.image.width*a.image.height*4)invalid("Invalid intrinsic document trace image");if(this.catalog.has(a.id))invalid("Invalid or duplicate document trace asset");this.catalog.set(a.id,a);this.assets++;return;}
  if(this.phase==="indexing"){const n=this.input.plan.nodes[this.nodes];if(!n){this.phase="nodes";return;}validateSceneSourceAddress(n);identity(n.id);if(this.ids.has(n.id)||!n.content||!["path","image","group","text","boolean","trace"].includes(n.content.kind)||!Array.isArray(n.transform)||n.transform.length!==6||!n.transform.every(v=>Number.isFinite(v)&&Math.abs(v)<=1e9))invalid("Invalid or duplicate document trace node");this.ids.add(n.id);const c=n.content;
   if(c.kind==="path"){if(!Array.isArray(c.segments))invalid("Invalid document trace path");this.segments+=c.segments.length;if(this.segments>this.input.limits.maxRetainedSegments)invalid("Document trace retained segment limit exceeded");}
   if(c.kind==="trace"){identity(c.source);if(!this.catalog.has(c.source))invalid("Missing document trace source: "+n.id+" -> "+c.source);if(!Number.isFinite(c.threshold)||c.threshold<0||c.threshold>1||!Number.isFinite(c.simplifyEpsilon)||c.simplifyEpsilon<0||c.simplifyEpsilon>8192)invalid("Invalid document trace parameters: "+n.id);}this.nodes++;return;
  }
  if(this.phase==="nodes"){const n=this.input.plan.nodes[this.at];if(!n){const asset=this.input.plan.assets[this.publishAssetAt];if(asset){this.output.assets.push({id:asset.id,image:asset.image});this.publishAssetAt++;return;}if(this.cleanup())this.phase="complete";return;}if(n.content.kind!=="trace"){if(!this.copyNode){this.copyNode=new SceneNodeCopyJob(n);return;}if(this.copyNode.advanceOne()){this.output.nodes.push(this.copyNode.take());this.copyNode=null;this.at++;}return;}this.current=n;const c=n.content,mask=this.masks.get(c.source);if(mask){this.startTrace(mask);return;}
   const a=this.catalog.get(c.source)!,l=this.input.limits,count=a.image.width*a.image.height;if(this.sourceBytes+a.image.pixels.length>l.maxSourceBytes||count>Math.min(l.maxPixels,l.maxAdmittedPixels-this.pixels))invalid("Document trace intrinsic image budget exceeded");this.sourceBytes+=a.image.pixels.length;this.source=c.source;this.image=a.image;this.pixels+=count;this.admittedImages++;this.dimensions.set(this.source,[this.image.width,this.image.height]);this.mask=new Uint8Array(count);this.pixel=0;this.phase="luma";return;
  }
  if(this.phase==="luma"){if(this.pixel===this.mask!.length){const mask=this.mask!;this.masks.set(this.source,mask);this.image=null;this.mask=null;this.startTrace(mask);return;}const p=this.image!.pixels,i=this.pixel*4;this.mask![this.pixel++]=Math.round((299*p[i]!+587*p[i+1]!+114*p[i+2]!)*p[i+3]!/255000);return;}
  if(this.phase==="tracing"){this.trace=this.tracer!.advance(1);if(this.trace.done){this.raw=this.tracer!.result();this.candidate=[];this.publishAt=0;this.phase="publishing";}return;}
  if(this.phase==="publishing"){if(this.closingTracer){if(this.closingTracer.advance(1).done)this.closingTracer=null;return;}if(this.publishAt===this.raw.length){if(this.tracer){this.closingTracer=this.tracer.intoRetirement().job;this.tracer=null;this.raw=[];this.publishAt=0;return;}if(!this.copyNode){this.copyNode=new SceneNodeCopyJob(this.current!,this.candidate);this.candidate=[];return;}if(this.copyNode.advanceOne()){this.output.nodes.push(this.copyNode.take());this.copyNode=null;this.current=null;this.at++;this.resolved++;this.phase="nodes";}return;}if(this.segments>=this.input.limits.maxRetainedSegments)invalid("Document trace retained segment limit exceeded");const s=this.raw[this.publishAt++]!;if(s.kind==="move"||s.kind==="line")this.candidate.push({kind:s.kind,to:[s.to[0],s.to[1]]});else if(s.kind==="close")this.candidate.push({kind:"close"});else invalid("Unresolved document trace curve");this.segments++;}
 }
 private removeEntry(collection:Map<string,unknown>|Set<string>):boolean{this.cleanupEntries??=collection.keys();const entry=this.cleanupEntries.next();if(entry.done){this.cleanupEntries=null;return false;}collection.delete(entry.value);return true;}
 private cleanup():boolean{const collection=this.cleanupSlot===0?this.catalog:this.cleanupSlot===1?this.ids:this.cleanupSlot===2?this.masks:this.cleanupSlot===3?this.dimensions:null;if(collection){if(this.removeEntry(collection))return false;collection.clear();}else this.source="";return ++this.cleanupSlot===5;}
 advance(budget:number):DocumentTraceProgress{
  if(!Number.isSafeInteger(budget)||budget<1)invalid("Invalid document trace work grant");if(this.cancelled)throw new DOMException("Document trace cancelled","AbortError");if(this.failure)throw this.failure;
  try{for(let at=0;at<budget&&this.phase!=="complete";at++){if(this.work>=this.input.limits.maxWork)invalid("Document trace work limit exceeded");this.step();this.work++;}}catch(error){this.failure=error;throw error;}
  return{phase:this.phase,assets:this.assets,nodes:this.nodes,resolved:this.resolved,admittedImages:this.admittedImages,pixels:this.pixels,sourceBytes:this.sourceBytes,segments:this.segments,work:this.work,trace:this.trace,done:this.phase==="complete"};
 }
 result():DocumentScenePlan{if(this.cancelled)throw new DOMException("Document trace cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")invalid("Document trace incomplete");return this.output;}
 /** 🧹️ Return the genuine source and valid output while retiring all private scene and trace owners. */
 intoRetirement():{job:DocumentTraceRetirement;input:DocumentScenePlan;output:DocumentScenePlan|null}{
  if(this.transferred)throw Error("Document trace ownership already transferred");this.transferred=true;if(this.tracer){this.closingTracer=this.tracer.intoRetirement().job;this.tracer=null;}
  const input=this.input.plan,output=this.phase==="complete"&&!this.failure&&!this.cancelled?this.output:null;this.input={plan:{assets:[],nodes:[]},limits:this.input.limits};let draft:ScenePlanCloseJob|null=output?null:new ScenePlanCloseJob(this.output);this.output={assets:[],nodes:[]};const partial=this.copyNode?.takePartial();let node:ScenePlanCloseJob|null=partial?new ScenePlanCloseJob({assets:[],nodes:[partial]}):null;this.copyNode=null;this.cleanupEntries=null;this.cancelled=true;
  let slot=0;const job=new UnitRetirement(()=>{switch(slot){
   case 0:if(this.closingTracer){if(!this.closingTracer.advance(1).done)return false;this.closingTracer=null;}break;
   case 1:break;
   case 2:if(draft){if(!draft.advance(1).done)return false;draft=null;}break;
   case 3:if(node){if(!node.advance(1).done)return false;node=null;}break;
   case 4:if(this.removeEntry(this.catalog))return false;this.catalog.clear();break;
   case 5:if(this.removeEntry(this.ids))return false;this.ids.clear();break;
   case 6:if(this.removeEntry(this.masks))return false;this.masks.clear();break;
   case 7:if(this.removeEntry(this.dimensions))return false;this.dimensions.clear();break;
   case 8:this.image=null;break;case 9:this.mask=null;break;case 10:this.raw=[];break;case 11:this.candidate=[];break;
   case 12:this.source="";this.current=null;break;
  }return ++slot===13;});return{job,input,output};
 }
 private clear():void{this.tracer?.cancel();this.tracer=null;this.closingTracer=null;this.copyNode=null;this.cleanupEntries=null;this.catalog.clear();this.ids.clear();this.masks.clear();this.dimensions.clear();this.image=null;this.mask=null;this.raw=[];this.candidate=[];this.current=null;this.source="";this.output={assets:[],nodes:[]};this.input={plan:{assets:[],nodes:[]},limits:this.input.limits};}
 cancel():void{this.cancelled=true;if(!this.transferred)this.clear();}
}
/** ⏳️ Yield decode/luma/contour work and recheck observers before returning complete plans. */
export async function resolveDocumentTraces(input:DocumentTraceInput,options:DocumentTraceOptions={}):Promise<DocumentScenePlan>{
 const check=()=>{if(options.signal?.aborted)throw new DOMException("Document trace cancelled","AbortError");};check();const job=new DocumentTraceJob(input);
 try{for(;;){check();const p=job.advance(options.workBudget??4096);options.onProgress?.(p);check();if(p.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();throw error;}
}
