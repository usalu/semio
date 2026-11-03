import {base64StandardEncodeControlled} from "@semio-tech/framework/io/base64";
import {PngEncodeJob,type PngEncodedBytes} from "@semio-tech/framework/pixels/png/encode";
import {RetainedWorkRetirement,RetainedWorkLedger,type RetainedWorkDemand,type RetainedWorkRetirementCursor} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
/** 🖼️ Physical encoded image admission publishes ordered semantic RGBA sample tuples. */
import {ImageDecodeJob,type ImageDecodeInput,type ImageDecodeProgress} from "@semio-tech/framework/pixels/image/decode";
import type {DrawingImageAsset} from "../../🧬️schema/🟦️.ts";
export interface DrawingImageAdmissionProgress{decoding:ImageDecodeProgress|undefined;samples:number;totalSamples:number;work:number;done:boolean}
import type {RetainedCloneGrant,RetainedCloneProgress} from "@semio-tech/framework/value/retained-clone/contract";
export class DrawingImageAdmissionJob{
 private readonly original:string;private decoder:ImageDecodeJob;private decoding:ImageDecodeProgress|undefined;private image:ReturnType<ImageDecodeJob["result"]>|undefined;private asset:DrawingImageAsset|undefined;private samples=0;private work=0;private phase=0;private cancelled=false;private failure:unknown;
 constructor(input:ImageDecodeInput){this.original=input.data;this.decoder=new ImageDecodeJob(input);}
 private check():void{if(this.cancelled)throw new DOMException("Drawing image admission cancelled","AbortError");if(this.phase===4)throw Error("Drawing image admission incomplete");if(this.failure!==undefined)throw this.failure;}
 private grant(value:RetainedCloneGrant):void{for(const field of ["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"]as const)if(!Number.isSafeInteger(value[field])||value[field]<0)throw RangeError("Invalid drawing image admission funding");}
 progress():DrawingImageAdmissionProgress{return {decoding:this.decoding,samples:this.samples,totalSamples:this.asset?this.asset.width*this.asset.height:0,work:this.work,done:this.phase===3};}
 nextCopyByteDemand():number{return this.phase===0?this.decoder.nextCopyByteDemand():this.phase===1?64:this.phase===2?20:0;}
 nextCapacityByteDemand():number{return this.phase===0?this.decoder.nextCapacityByteDemand():this.phase===2&&this.samples<this.asset!.width*this.asset!.height?32:0;}
 private step(source:string,grant:RetainedCloneGrant):RetainedCloneProgress{
  if(this.phase===0){const child=this.decoder.advance(source,grant);this.decoding=child.progress;if(child.progress.done)this.phase=1;return child.receipt;}
  if(this.phase===1){const child=this.decoder.takeResult(grant);if(!child)return {copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};this.image=child.value;this.asset={width:this.image.width,height:this.image.height,samples:[]};this.phase=2;return {...child.receipt,copiedBytes:64};}
  const image=this.image!,at=this.samples*4;if(at===image.pixels.length){this.phase=3;return {copiedItems:1,copiedBytes:20,retainedCapacityBytes:0,releasedBytes:0};}
  this.asset!.samples.push([image.pixels[at]!,image.pixels[at+1]!,image.pixels[at+2]!,image.pixels[at+3]!]);this.samples++;return {copiedItems:1,copiedBytes:20,retainedCapacityBytes:32,releasedBytes:0};
 }
 advance(source:string,grant:RetainedCloneGrant):{progress:DrawingImageAdmissionProgress;receipt:RetainedCloneProgress}{this.grant(grant);this.check();if(source!==this.original)throw Error("Original drawing image source changed");const receipt:RetainedCloneProgress={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};try{for(let i=0;i<grant.maximumItems&&this.phase!==3;i++){const copy=this.nextCopyByteDemand(),capacity=this.nextCapacityByteDemand();if(!grant.maximumDepth||copy>grant.maximumCopyBytes-receipt.copiedBytes||capacity>grant.maximumCapacityBytes-receipt.retainedCapacityBytes)break;const step=this.step(source,{...grant,maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:grant.maximumCapacityBytes-receipt.retainedCapacityBytes,maximumReleaseBytes:grant.maximumReleaseBytes-receipt.releasedBytes});if(!step.copiedItems)break;this.work+=step.copiedItems;receipt.copiedItems+=step.copiedItems;receipt.copiedBytes+=step.copiedBytes;receipt.retainedCapacityBytes+=step.retainedCapacityBytes;receipt.releasedBytes+=step.releasedBytes;}}catch(error){this.failure=error;throw error;}return {progress:this.progress(),receipt};}
 takeResult(grant:RetainedCloneGrant):{value:DrawingImageAsset;receipt:RetainedCloneProgress}|undefined{this.grant(grant);const value=this.result();if(!grant.maximumItems||grant.maximumCopyBytes<32||!grant.maximumDepth)return;this.asset=undefined;this.phase=4;return {value,receipt:{copiedItems:1,copiedBytes:32,retainedCapacityBytes:0,releasedBytes:0}};}
 cancel():void{this.cancelled=true;}
 result():DrawingImageAsset{this.check();if(this.phase!==3||!this.asset)throw Error("Drawing image admission incomplete");return this.asset;}
}
/** 📤️ Borrowed semantic samples retain original authority throughout emission and refusal. */
export class DrawingImageEmissionJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private samples=0;private pixels:Uint8Array;private encoder:PngEncodeJob|undefined;private encoded:PngEncodedBytes|undefined;private closing:RetainedWorkRetirementCursor[]=[];private bytes:Uint8Array|undefined;private phase:"samples"|"encoding"|"closing"|"complete"="samples";private work=0;private cancelled=false;private transferred=false;private failure:unknown;
 constructor(private asset:DrawingImageAsset,maximumPixels:number,private maximumEncodedBytes:number){const count=asset.width*asset.height;if(!Number.isInteger(asset.width)||!Number.isInteger(asset.height)||asset.width<1||asset.height<1||!Number.isSafeInteger(count)||count<1||count>maximumPixels||maximumPixels>16777216||!Number.isSafeInteger(maximumPixels)||asset.samples.length!==count||!Number.isSafeInteger(maximumEncodedBytes)||maximumEncodedBytes<8||maximumEncodedBytes>67108864)throw RangeError("Invalid drawing image emission contract");this.pixels=new Uint8Array(count*4);}
 private step(c:RetainedWorkProducerStep):void{
  if(this.phase==="samples"){if(this.samples<this.asset.samples.length){const sample=this.asset.samples[this.samples]!;if(!Array.isArray(sample)||sample.length!==4||sample.some(component=>!Number.isInteger(component)||component<0||component>255))throw RangeError("Invalid intrinsic RGBA sample");this.pixels.set(sample,this.samples*4);this.samples++;c.commitSemanticWork();return;}this.encoder=new PngEncodeJob({width:this.asset.width,height:this.asset.height,pixels:this.pixels},this.maximumEncodedBytes);this.pixels=new Uint8Array(0);this.phase="encoding";c.commitSemanticWork();return;}
  if(this.phase==="encoding"){if(c.advanceChild(this.encoder!).done){const moved=this.encoder!.intoRetirement();this.encoder=undefined;this.encoded=moved.output!;this.bytes=Uint8Array.from(this.encoded.bytes);this.closing=[this.encoded,moved.job];this.phase="closing";}return;}
  if(this.phase==="closing"){const cursor=this.closing.at(-1);if(cursor===undefined){this.phase="complete";c.commitControlWork();return;}let done=false;try{done=cursor.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(cursor.lastTurnWork());}if(done&&!c.hasActualWork()){this.closing.pop();c.commitControlWork();}}
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==undefined||this.encoder?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="closing"||this.phase==="complete"||this.encoder?.settlementPending()===true;}
 advance(items:number,work:RetainedWorkLedger):{phase:string;samples:number;totalSamples:number;encodedBytes:number;work:number;done:boolean}{
  try{return this.#runtime.advance(items,work,{maximumWork:Number.MAX_SAFE_INTEGER,workLimitMessage:"Drawing image emission work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Drawing image emission cancelled","AbortError");if(this.failure!==undefined)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase==="closing"?"encoding":this.phase,samples:this.samples,totalSamples:this.asset.samples.length,encodedBytes:this.bytes?.length??0,work:this.work,done:this.phase==="complete"})});}
  catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===undefined&&!this.cancelled)this.failure=error;throw error;}
 }
 result():Uint8Array{if(this.cancelled||this.transferred)throw new DOMException("Drawing image emission cancelled","AbortError");if(this.failure!==undefined)throw this.failure;if(this.phase!=="complete"||!this.bytes)throw Error("Drawing image emission incomplete");return this.bytes;}
 cancel():void{this.cancelled=true;}
 /** 🧹️ Private encoder and byte owners retire under caller funding; a completed result is handed over. */
 intoRetirement():{job:RetainedWorkRetirementCursor;output:Uint8Array|null}{
  if(this.transferred)throw Error("Drawing image emission ownership already transferred");this.transferred=true;const output=this.phase==="complete"&&!this.cancelled&&this.failure===undefined?this.bytes??null:null;const cursors=[...this.closing];this.closing=[];
  if(this.encoder){const moved=this.encoder.intoRetirement();cursors.push(moved.job);if(moved.output)cursors.push(moved.output);this.encoder=undefined;}
  this.cancelled=true;this.pixels=new Uint8Array(0);this.bytes=undefined;
  return{output,job:new RetainedWorkRetirement((work,turn)=>{const cursor=cursors.at(-1);if(cursor){let done=false;const before=turn.actualWork();try{done=cursor.advance(1,work).done;}finally{turn.commitActualWork(cursor.lastTurnWork());}if(done&&turn.actualWork()===before){cursors.pop();turn.commitActualWork();}return false;}return true;})};
 }
}

/** 🌐️ Browser and SVG output remain controlled physical emissions from admitted samples. */
export function drawingImageDataUri(asset:DrawingImageAsset,progress:(state:{phase:string;samples:number;totalSamples:number;encodedBytes:number;work:number;done:boolean})=>boolean=()=>true):string{
 const job=new DrawingImageEmissionJob(asset,16777216,67108864),turn=()=>new RetainedWorkLedger({maximumItems:4096,maximumCopyBytes:2**30,maximumCapacityBytes:2**30,maximumReleaseBytes:2**30,maximumDepth:64});let work=0;
 for(;;){const state=job.advance(1,turn());work=state.work;if(!progress({...state,done:false})){job.cancel();throw new DOMException("Drawing image emission cancelled","AbortError");}if(state.done)break;}
 const bytes=job.result(),encoded=base64StandardEncodeControlled(bytes,{maximumOutputBytes:89478488,progress:state=>progress({phase:"base64",samples:asset.samples.length,totalSamples:asset.samples.length,encodedBytes:state.completed,work:++work,done:state.completed===state.total})});return "data:image/png;base64,"+encoded;
}
