/** 🖼️ Cooperative encoded-image preparation with private pixel publication. */
import type {PixelImage} from "../../✍️editing/🟦️.ts";
import {PngDecodeJob} from "../../📷️png/📥️decode/🟦️.ts";
import {BinarySourceJob,type BinarySourceProgress} from "../../../🚪️io/🔢️binary/📥️source/🟦️.ts";
export type ImageDecodeInput={mime:string;data:string;maxSourceBytes:number;maxBytes:number;maxPixels:number;maxChunks:number};
export type ImageDecodeProgress={phase:"header"|"validate"|"decode"|"png"|"complete";sourceCompleted:number;sourceTotal:number;bytes:number;totalBytes:number;pixels:number;totalPixels:number;work:number;done:boolean};
export type ImageDecodeOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:ImageDecodeProgress)=>void};
const fail=(message:string):never=>{throw new RangeError(message);};
const abort=():never=>{throw new DOMException("Image preparation cancelled","AbortError");};
const integer=(v:number,min:number,max:number)=>Number.isSafeInteger(v)&&v>=min&&v<=max;
/** 🧩️ Real byte-source and PNG child jobs share private publication and caller grants. */
export class ImageDecodeJob{
 private readonly limits:Omit<ImageDecodeInput,"data"|"mime">;
 private phase:ImageDecodeProgress["phase"];private source:BinarySourceJob|undefined;private sourceProgress:BinarySourceProgress;
 private work=0;private pixels=0;private totalPixels=0;private png:PngDecodeJob|undefined;private output:PixelImage|undefined;private cancelled=false;private failed:unknown;
 constructor(input:ImageDecodeInput){
  if(typeof input.data!=="string"||!input.data.length||!integer(input.maxSourceBytes,1,268439552)||input.data.length>input.maxSourceBytes||typeof input.mime!=="string"||!input.mime.length||input.mime.length>128||!integer(input.maxBytes,8,67108864)||!integer(input.maxPixels,1,16777216)||!integer(input.maxChunks,1,65536))fail("Invalid image source contract");
  if(input.mime.toLowerCase()!=="image/png")fail("Unsupported image media type: "+input.mime);
  this.limits={maxSourceBytes:input.maxSourceBytes,maxBytes:input.maxBytes,maxPixels:input.maxPixels,maxChunks:input.maxChunks};
  this.source=new BinarySourceJob({mime:input.mime,data:input.data,minBytes:8,maxSourceBytes:input.maxSourceBytes,maxBytes:input.maxBytes,maxWork:1000000000});
  const at=input.data.slice(0,5).toLowerCase()==="data:"?5:0;this.phase=at?"header":"validate";
  this.sourceProgress={phase:this.phase,sourceCompleted:at,sourceTotal:input.data.length*2,bytes:0,totalBytes:0,work:0,done:false};
 }
 private check():void{if(this.cancelled)abort();if(this.failed!==undefined)throw this.failed;}
 private release():void{this.source?.cancel();this.source=undefined;this.png?.cancel();this.png=undefined;this.output=undefined;}
 private sourceStep():void{
  const p=this.source!.advance(1);this.sourceProgress=p;
  if(p.done){this.png=new PngDecodeJob({data:this.source!.result(),maxBytes:this.limits.maxBytes,maxPixels:this.limits.maxPixels,maxChunks:this.limits.maxChunks});this.source=undefined;this.phase="png";}
  else this.phase=p.phase;
 }
 private pngStep():void{
  const p=this.png!.advance(1);this.pixels=p.pixels;this.totalPixels=p.totalPixels;
  if(p.done){this.output=this.png!.result();this.png=undefined;this.phase="complete";}
 }
 advance(budget:number):ImageDecodeProgress{
  if(!integer(budget,1,Number.MAX_SAFE_INTEGER))fail("Image work grant must be a positive integer");this.check();
  try{for(let i=0;i<budget&&this.phase!=="complete";i++){if(this.phase==="png")this.pngStep();else this.sourceStep();this.work++;}}
  catch(error){this.failed=error;this.release();throw error;}
  const p=this.sourceProgress;
  return {phase:this.phase,sourceCompleted:p.sourceCompleted,sourceTotal:p.sourceTotal,bytes:p.bytes,totalBytes:p.totalBytes,pixels:this.pixels,totalPixels:this.totalPixels,work:this.work,done:this.phase==="complete"};
 }
 cancel():void{this.cancelled=true;this.release();}
 result():PixelImage{this.check();if(this.phase!=="complete"||!this.output)fail("Image preparation is incomplete");return this.output!;}
}
export async function decodeImageSource(input:ImageDecodeInput,options:ImageDecodeOptions={}):Promise<PixelImage>{
 if(options.signal?.aborted)abort();const job=new ImageDecodeJob(input);
 try{for(;;){if(options.signal?.aborted)abort();const progress=job.advance(options.workBudget??4096);options.onProgress?.(progress);if(options.signal?.aborted)abort();if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}
 catch(error){job.cancel();throw error;}
}
