/** 🖼️ Original encoded sources and private image children share independently funded work. */
import type {PixelImage} from "../../✍️editing/🟦️.ts";
import {PngDecodeJob} from "../../📷️png/📥️decode/🟦️.ts";
import {JpegDecodeJob} from "../../📸️jpeg/📥️decode/🟦️.ts";
import {BinarySourceJob,type BinarySourceProgress} from "../../../🚪️io/🔢️binary/📥️source/🟦️.ts";
import type {RetainedCloneGrant,RetainedCloneProgress} from "../../../🌱️value/🧬️retained-clone/🧬️contract/🟦️.ts";
export type ImageDecodeInput={mime:string;data:string;maxSourceBytes:number;maxBytes:number;maxPixels:number;maxChunks:number};
export type ImageDecodeProgress={phase:"header"|"validate"|"decode"|"source-handoff"|"decoder"|"png"|"jpeg"|"image-handoff"|"complete"|"transferred";sourceCompleted:number;sourceTotal:number;bytes:number;totalBytes:number;pixels:number;totalPixels:number;work:number;done:boolean};
export type ImageDecodeOptions={workGrant:RetainedCloneGrant;signal?:AbortSignal;workBudget?:number;onProgress?:(progress:ImageDecodeProgress)=>void};
const fail=(message:string):never=>{throw new RangeError(message);};
const abort=():never=>{throw new DOMException("Image preparation cancelled","AbortError");};
const integer=(v:number,min:number,max:number)=>Number.isSafeInteger(v)&&v>=min&&v<=max;
const empty=():RetainedCloneProgress=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
/** 🧩️ Child source, PNG and JPEG owners remain retained until explicit publication or cancellation closure. */
export class ImageDecodeJob{
 private readonly limits:Omit<ImageDecodeInput,"data"|"mime">;private readonly original:string;private readonly isJpeg:boolean;
 private phase:ImageDecodeProgress["phase"];private source:BinarySourceJob;private sourceProgress:BinarySourceProgress;
 private work=0;private pixels=0;private totalPixels=0;private bytes:Uint8Array|undefined;private png:PngDecodeJob|undefined;private jpeg:JpegDecodeJob|undefined;private output:PixelImage|undefined;private cancelled=false;private failed:unknown;
 constructor(input:ImageDecodeInput){
  if(typeof input.data!=="string"||!input.data.length||!integer(input.maxSourceBytes,1,268439552)||input.data.length>input.maxSourceBytes||typeof input.mime!=="string"||!input.mime.length||input.mime.length>128||!integer(input.maxBytes,8,67108864)||!integer(input.maxPixels,1,16777216)||!integer(input.maxChunks,1,65536))fail("Invalid image source contract");
  this.isJpeg=input.mime.toLowerCase()==="image/jpeg";if(!this.isJpeg&&input.mime.toLowerCase()!=="image/png")fail("Unsupported image media type");
  this.original=input.data;this.limits={maxSourceBytes:input.maxSourceBytes,maxBytes:input.maxBytes,maxPixels:input.maxPixels,maxChunks:input.maxChunks};
  this.source=new BinarySourceJob({mime:input.mime,data:input.data,minBytes:8,maxSourceBytes:input.maxSourceBytes,maxBytes:input.maxBytes,maxWork:1000000000});this.sourceProgress=this.source.progress();this.phase=this.sourceProgress.phase;
 }
 private check():void{if(this.cancelled)abort();if(this.phase==="transferred")fail("Image preparation is incomplete");if(this.failed!==undefined)throw this.failed;}
 private grant(value:RetainedCloneGrant):void{for(const field of ["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"]as const)if(!integer(value[field],0,Number.MAX_SAFE_INTEGER))fail("Invalid image funding");}
 progress():ImageDecodeProgress{const p=this.sourceProgress;return {phase:this.phase,sourceCompleted:p.sourceCompleted,sourceTotal:p.sourceTotal,bytes:p.bytes,totalBytes:p.totalBytes,pixels:this.pixels,totalPixels:this.totalPixels,work:this.work,done:this.phase==="complete"};}
 nextCopyByteDemand():number{switch(this.phase){case "complete":case "transferred":return 0;case "source-handoff":return 24;case "decoder":return 32;case "image-handoff":return this.isJpeg?24:32;case "png":return this.png!.nextCopyByteDemand();case "jpeg":return this.jpeg!.nextCopyBytes();default:return this.source.nextCopyByteDemand();}}
 nextCapacityByteDemand():number{switch(this.phase){case "decoder":return this.isJpeg?5072:0;case "png":return this.png!.nextCapacityByteDemand();case "jpeg":return this.jpeg!.nextCapacityBytes();case "source-handoff":case "image-handoff":case "complete":case "transferred":return 0;default:return this.source.nextCapacityByteDemand();}}
 private step(source:string,grant:RetainedCloneGrant):RetainedCloneProgress{
  if(this.phase==="source-handoff"){const handoff=this.source.takeResult(grant);if(!handoff)return empty();this.bytes=handoff.value;this.phase="decoder";return handoff.receipt;}
  if(this.phase==="decoder"){const data=this.bytes!;if(this.isJpeg)this.jpeg=new JpegDecodeJob({data,maxBytes:this.limits.maxBytes,maxPixels:this.limits.maxPixels,maxSegments:this.limits.maxChunks,maxWorkingBytes:536870912});else this.png=new PngDecodeJob({data,maxBytes:this.limits.maxBytes,maxPixels:this.limits.maxPixels,maxChunks:this.limits.maxChunks});this.bytes=undefined;this.phase=this.isJpeg?"jpeg":"png";return {copiedItems:1,copiedBytes:32,retainedCapacityBytes:this.isJpeg?5072:0,releasedBytes:0};}
  if(this.phase==="png"||this.phase==="jpeg"){const child=this.phase==="png"?this.png!.advance(grant):this.jpeg!.advance(grant);this.pixels=child.progress.pixels;this.totalPixels=child.progress.totalPixels;if(child.progress.done)this.phase="image-handoff";return child.receipt;}
  if(this.phase==="image-handoff"){if(this.isJpeg){const handoff=this.jpeg!.takeResult(grant);if(!handoff)return empty();this.output=handoff.image;this.phase="complete";return handoff.receipt;}const handoff=this.png!.takeResult(grant);if(!handoff)return empty();this.output=handoff.value;this.phase="complete";return handoff.receipt;}
  const child=this.source.advance(source,grant);this.sourceProgress=child.progress;this.phase=child.progress.done?"source-handoff":child.progress.phase;return child.receipt;
 }
 advance(source:string,grant:RetainedCloneGrant):{progress:ImageDecodeProgress;receipt:RetainedCloneProgress}{
  this.grant(grant);this.check();if(source!==this.original)fail("Original encoded image source changed");const receipt=empty();
  try{for(let i=0;i<grant.maximumItems&&this.phase!=="complete";i++){const copy=this.nextCopyByteDemand(),capacity=this.nextCapacityByteDemand();if(!grant.maximumDepth||copy>grant.maximumCopyBytes-receipt.copiedBytes||capacity>grant.maximumCapacityBytes-receipt.retainedCapacityBytes)break;const child=this.step(source,{...grant,maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:grant.maximumCapacityBytes-receipt.retainedCapacityBytes,maximumReleaseBytes:grant.maximumReleaseBytes-receipt.releasedBytes});if(!child.copiedItems)break;this.work+=child.copiedItems;receipt.copiedItems+=child.copiedItems;receipt.copiedBytes+=child.copiedBytes;receipt.retainedCapacityBytes+=child.retainedCapacityBytes;receipt.releasedBytes+=child.releasedBytes;}}
  catch(error){this.failed=error;throw error;}
  return {progress:this.progress(),receipt};
 }
 takeResult(grant:RetainedCloneGrant):{value:PixelImage;receipt:RetainedCloneProgress}|undefined{this.grant(grant);const value=this.result();if(!grant.maximumItems||grant.maximumCopyBytes<32||!grant.maximumDepth)return;this.output=undefined;this.phase="transferred";return {value,receipt:{copiedItems:1,copiedBytes:32,retainedCapacityBytes:0,releasedBytes:0}};}
 cancel():void{this.cancelled=true;}
 result():PixelImage{this.check();if(this.phase!=="complete"||!this.output)fail("Image preparation is incomplete");return this.output!;}
}
export async function decodeImageSource(input:ImageDecodeInput,options:ImageDecodeOptions):Promise<PixelImage>{
 if(options.signal?.aborted)abort();const job=new ImageDecodeJob(input);
 try{for(;;){if(options.signal?.aborted)abort();const step=job.advance(input.data,{...options.workGrant,maximumItems:Math.min(options.workGrant.maximumItems,options.workBudget??4096)}),progress=step.progress;if(!progress.done&&!step.receipt.copiedItems)fail("Image funding does not admit the next transition");options.onProgress?.(progress);if(options.signal?.aborted)abort();if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}
 catch(error){job.cancel();throw error;}
}
