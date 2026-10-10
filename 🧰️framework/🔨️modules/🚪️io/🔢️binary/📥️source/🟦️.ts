/** 🧾️ Cooperative MIME-qualified encoded bytes with private publication. */
import type {RetainedCloneGrant,RetainedCloneProgress} from "../../../🌱️value/🧬️retained-clone/🧬️contract/🟦️.ts";
import {decodeBase64Quad} from "../../🔤️base64/🟦️.ts";
export type BinarySourceInput={mime:string;data:string;minBytes:number;maxBytes:number;maxSourceBytes:number;maxWork:number};
export type BinarySourceProgress={phase:"header"|"validate"|"decode"|"complete"|"transferred";sourceCompleted:number;sourceTotal:number;bytes:number;totalBytes:number;work:number;done:boolean};
export type BinarySourceOptions={signal?:AbortSignal;workBudget?:number;workGrant:RetainedCloneGrant;onProgress?:(progress:BinarySourceProgress)=>void};
const fail=(message:string):never=>{throw new RangeError(message);};
const abort=():never=>{throw new DOMException("Binary source cancelled","AbortError");};
const integer=(v:number,min:number,max:number)=>Number.isSafeInteger(v)&&v>=min&&v<=max;
const hex=(v:number)=>v>=48&&v<=57?v-48:v>=65&&v<=70?v-55:v>=97&&v<=102?v-87:-1;
const alpha=(v:number)=>v>=65&&v<=90||v>=97&&v<=122||v>=48&&v<=57;
const safe=(v:number)=>alpha(v)||";/?:@&=+$,-_.!~*'()".includes(String.fromCharCode(v));
const key=(v:number)=>alpha(v)||v===96||"!$&'*+.^_|~-".includes(String.fromCharCode(v));
/** 🧩️ Complete validation and byte/work admission precede output allocation. */
export class BinarySourceJob{
 private source:string;private readonly sourceLength:number;private readonly mime:string;private readonly limits:Omit<BinarySourceInput,"data"|"mime">;
 private phase:BinarySourceProgress["phase"];private at:number;private bodyStart=0;private read:number;private sourceTotal:number;private base64=true;private url=false;
 private work=0;private bytes=0;private totalBytes=0;private quad=new Uint8Array(4);private quadAt=0;private scratch=new Uint8Array(3);
 private binary=new Uint8Array(0);private output:Uint8Array|undefined;private cancelled=false;private failed:unknown;
 constructor(input:BinarySourceInput){
  if(typeof input.data!=="string"||!integer(input.maxSourceBytes,1,268439552)||input.data.length>input.maxSourceBytes||typeof input.mime!=="string"||input.mime.length>128||!/^[A-Za-z0-9!#$&^_.+~-]+\/[A-Za-z0-9!#$&^_.+~-]+$/.test(input.mime)||!integer(input.maxBytes,1,67108864)||!integer(input.minBytes,0,input.maxBytes)||!integer(input.maxWork,1,1000000000))fail("Invalid binary source contract");
  this.source=input.data;this.sourceLength=input.data.length;this.mime=input.mime;this.limits={minBytes:input.minBytes,maxBytes:input.maxBytes,maxSourceBytes:input.maxSourceBytes,maxWork:input.maxWork};
  this.url=input.data.slice(0,5).toLowerCase()==="data:";this.phase=this.url?"header":"validate";this.at=this.url?5:0;this.read=this.at;this.sourceTotal=this.sourceLength*2;
 }
 private check():void{if(this.cancelled)abort();if(this.phase==="transferred")fail("Binary source is incomplete");if(this.failed!==undefined)throw this.failed;}
 private headerStep():void{
  if(this.at>=this.sourceLength||this.at>=4096)fail("Missing or excessive data URL header");
  const value=this.source.charCodeAt(this.at++);this.read++;
  if(value!==44){if(value<33||value>126||value===35)fail("Invalid data URL header");return;}
  const parts=this.source.slice(5,this.at-1).split(";");
  if(parts[0]!.toLowerCase()!==this.mime.toLowerCase())fail("Data URL media type does not match "+this.mime);
  this.base64=false;
  for(let i=1;i<parts.length;i++){
   const part=parts[i]!;
   if(part.toLowerCase()==="base64"){if(this.base64||i!==parts.length-1)fail("Invalid base64 flag placement");this.base64=true;continue;}
   const equals=part.indexOf("=");
   if(equals<1||equals===part.length-1)fail("Invalid data URL parameter");
   for(let j=0;j<equals;j++)if(!key(part.charCodeAt(j)))fail("Invalid data URL parameter");
   for(let j=equals+1;j<part.length;j++){const c=part.charCodeAt(j);if(c===37){if(j+2>=part.length||hex(part.charCodeAt(j+1))<0||hex(part.charCodeAt(j+2))<0)fail("Invalid parameter escape");j+=2;}else if(!safe(c))fail("Invalid parameter value");}
  }
  this.bodyStart=this.at;this.sourceTotal=this.sourceLength*2-this.bodyStart;this.phase="validate";
 }
 private token():number{
  let value=this.source.charCodeAt(this.at++);this.read++;
  if(this.url&&value===37){
   if(this.at+2>this.sourceLength)fail("Truncated source escape");
   const a=hex(this.source.charCodeAt(this.at)),b=hex(this.source.charCodeAt(this.at+1));if(a<0||b<0)fail("Invalid source escape");this.at+=2;this.read+=2;value=a*16+b;
  }else if(value>127||!this.base64&&!safe(value))fail("Invalid source byte");
  return value;
 }
 private sourceStep():void{
  const validating=this.phase==="validate";
  if(this.at===this.sourceLength){
   if(this.quadAt)fail("Invalid base64 length");
   if(validating){if(this.totalBytes<this.limits.minBytes)fail("Encoded source is too short");this.binary=new Uint8Array(this.totalBytes);this.at=this.bodyStart;this.phase="decode";}
   else{this.output=this.binary;this.binary=new Uint8Array(0);this.phase="complete";}
   return;
  }
  const value=this.token();
  if(this.base64){
   if(value>127)fail("Invalid base64 byte");
   this.quad[this.quadAt++]=value;
   if(this.quadAt===4){const count=decodeBase64Quad(this.quad,this.at-4,this.at===this.sourceLength,validating?this.scratch:this.binary,validating?0:this.bytes);this.quadAt=0;if(validating)this.totalBytes+=count;else this.bytes+=count;}
  }else if(validating)this.totalBytes++;else this.binary[this.bytes++]=value;
  if(this.totalBytes>this.limits.maxBytes)fail("Encoded source byte limit exceeded");
 }
 progress():BinarySourceProgress{return {phase:this.phase,sourceCompleted:this.read,sourceTotal:this.sourceTotal,bytes:this.bytes,totalBytes:this.totalBytes,work:this.work,done:this.phase==="complete"};}
 nextCopyByteDemand():number{return this.phase==="complete"||this.phase==="transferred"?0:32;}
 nextCapacityByteDemand():number{return this.phase==="validate"&&this.at===this.sourceLength&&!this.quadAt?this.totalBytes:0;}
 private grant(value:RetainedCloneGrant):void{for(const field of ["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"]as const)if(!integer(value[field],0,Number.MAX_SAFE_INTEGER))fail("Invalid binary source funding");}
 advance(source:string,grant:RetainedCloneGrant):{progress:BinarySourceProgress;receipt:RetainedCloneProgress}{
  this.grant(grant);this.check();if(source!==this.source)fail("Original binary source changed");const receipt:RetainedCloneProgress={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};
  try{for(let i=0;i<grant.maximumItems&&this.phase!=="complete";i++){const copy=this.nextCopyByteDemand(),capacity=this.nextCapacityByteDemand();if(grant.maximumDepth<1||copy>grant.maximumCopyBytes-receipt.copiedBytes||capacity>grant.maximumCapacityBytes-receipt.retainedCapacityBytes)break;if(this.work>=this.limits.maxWork)fail("Binary source work limit exceeded");if(this.phase==="header")this.headerStep();else this.sourceStep();this.work++;receipt.copiedItems++;receipt.copiedBytes+=copy;receipt.retainedCapacityBytes+=capacity;}}
  catch(error){this.failed=error;throw error;}
  return {progress:this.progress(),receipt};
 }
 cancel():void{this.cancelled=true;}
 takeResult(grant:RetainedCloneGrant):{value:Uint8Array;receipt:RetainedCloneProgress}|undefined{this.grant(grant);const value=this.result();if(grant.maximumItems<1||grant.maximumCopyBytes<24||grant.maximumDepth<1)return;this.output=undefined;this.phase="transferred";return {value,receipt:{copiedItems:1,copiedBytes:24,retainedCapacityBytes:0,releasedBytes:0}};}

 result():Uint8Array{this.check();if(this.phase!=="complete"||!this.output)fail("Binary source is incomplete");return this.output!;}
}
export async function decodeBinarySource(input:BinarySourceInput,options:BinarySourceOptions):Promise<Uint8Array>{
 if(options.signal?.aborted)abort();const job=new BinarySourceJob(input);
 try{for(;;){if(options.signal?.aborted)abort();const step=job.advance(input.data,{...options.workGrant,maximumItems:Math.min(options.workGrant.maximumItems,options.workBudget??4096)}),progress=step.progress;if(!progress.done&&!step.receipt.copiedItems)fail("Binary source funding does not admit the next transition");options.onProgress?.(progress);if(options.signal?.aborted)abort();if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}
 catch(error){job.cancel();throw error;}
}
