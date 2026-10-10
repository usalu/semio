import type {PixelImage} from "../../✍️editing/🟦️.ts";
export interface JpegDecodeInput {data:Uint8Array;maxPixels:number;maxBytes:number;maxSegments:number;maxWorkingBytes:number}
import type {RetainedCloneGrant,RetainedCloneProgress} from "../../../🌱️value/🧬️retained-clone/🧬️contract/🟦️.ts";
export type JpegGrant=RetainedCloneGrant;
export type JpegReceipt=RetainedCloneProgress;
export interface JpegProgress {phase:string;bytes:number;totalBytes:number;blocks:number;pixels:number;totalPixels:number;work:number;done:boolean}
type Component={id:number;h:number;v:number;quant:number;cols:number;rows:number;actualCols:number;actualRows:number;coefficients:Int32Array;samples:Uint8Array;predictor:number;bands:Int8Array};
type Huffman={counts:Uint8Array;symbols:Uint8Array;minimum:Int32Array;maximum:Int32Array;offset:Uint16Array;valid:boolean};
const zigzag=[0,1,8,16,9,2,3,10,17,24,32,25,18,11,4,5,12,19,26,33,40,48,41,34,27,20,13,6,7,14,21,28,35,42,49,56,57,50,43,36,29,22,15,23,30,37,44,51,58,59,52,45,38,31,39,46,53,60,61,54,47,55,62,63]as const;
const emptyReceipt=():JpegReceipt=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
/** 📸️ JPEG coefficient reconstruction retains its original source and private candidates through cancellation. */
export class JpegDecodeJob {
 private source:Uint8Array;private phase="markers";private at=2;private consumed=2;private work=0;private blocks=0;private pixels=0;private segments=0;private width=0;private height=0;private progressive=false;private framed=false;private scanned=false;private ended=false;private cancelled=false;private failure:Error|undefined;
 private marker=0;private end=0;private bodyAt=0;private table=-1;private tableAt=0;private tableCount=0;private tableSymbols=0;private tableCode=0;private tableOffset=0;private precision=0;
 private quant=Array.from({length:4},()=>new Uint16Array(64));private quantValid=[false,false,false,false];private huffman:Huffman[]=Array.from({length:8},()=>({counts:new Uint8Array(16),symbols:new Uint8Array(256),minimum:new Int32Array(17),maximum:new Int32Array(17).fill(-1),offset:new Uint16Array(17),valid:false}));
 private components:Component[]=[];private hmax=1;private vmax=1;private mcusX=0;private mcusY=0;private allocateAt=0;private initializeAt=0;private workingBytes=0;private image:PixelImage={width:0,height:0,pixels:new Uint8Array(0)};
 private scan:number[]=[];private dc=[0,0,0,0];private ac=[0,0,0,0];private spectralStart=0;private spectralEnd=63;private successiveHigh=0;private successiveLow=0;private unit=0;private scanComponent=0;private subBlock=0;private units=0;private bits=0;private bitCount=0;private eobRun=0;private restartInterval=0;private restartIndex=0;private restartUnits=0;private transformComponent=0;private transformBlock=0;private adobe=-1;private jfif=false;private natural=new Float64Array(64);private temporary=new Float64Array(64);
 constructor(private readonly input:JpegDecodeInput){this.source=input.data;if(!Number.isSafeInteger(input.maxPixels)||input.maxPixels<1||input.maxPixels>16777216||!Number.isSafeInteger(input.maxBytes)||input.maxBytes<4||input.maxBytes>67108864||!Number.isSafeInteger(input.maxSegments)||input.maxSegments<1||input.maxSegments>65536||!Number.isSafeInteger(input.maxWorkingBytes)||input.maxWorkingBytes<256||input.maxWorkingBytes>536870912||input.data.length<4||input.data.length>input.maxBytes||input.data[0]!==255||input.data[1]!==216)throw Error("Invalid JPEG input contract");}
 sourceIdentity():Uint8Array{return this.source;}
 progress():JpegProgress{return {phase:this.phase,bytes:this.consumed,totalBytes:this.source.length,blocks:this.blocks,pixels:this.pixels,totalPixels:this.width*this.height,work:this.work,done:this.phase==="complete"};}
 nextCopyBytes():number {if(this.phase==="complete"||this.phase==="transferred")return 0;if(this.phase==="allocate")return 0;if(this.phase==="initialize")return 256;if(this.phase==="entropy")return 256;if(this.phase==="transform")return 64;if(this.phase==="pixels")return 4;return 32;}
 nextCapacityBytes():number {if(this.phase!=="allocate")return 0;const c=this.components[Math.floor(this.allocateAt/2)];return c?c.cols*c.rows*64*(this.allocateAt%2===0?4:1):this.width*this.height*4;}
 private check():void{if(this.cancelled)throw new DOMException("JPEG decode cancelled","AbortError");if(this.failure)throw this.failure;}
 advance(grant:JpegGrant):{progress:JpegProgress;receipt:JpegReceipt}{this.check();for(const value of Object.values(grant))if(!Number.isSafeInteger(value)||value<0)throw Error("Invalid JPEG grant");const receipt=emptyReceipt();try{for(let unit=0;unit<grant.maximumItems;unit++){if(this.phase==="complete"||this.phase==="transferred")break;const copy=this.nextCopyBytes(),capacity=this.nextCapacityBytes();if(grant.maximumDepth<1||copy>grant.maximumCopyBytes-receipt.copiedBytes||capacity>grant.maximumCapacityBytes-receipt.retainedCapacityBytes)break;this.step();this.work++;receipt.copiedItems++;receipt.copiedBytes+=copy;receipt.retainedCapacityBytes+=capacity;}}catch(error){this.failure=error instanceof Error?error:Error(String(error));throw this.failure;}return {progress:this.progress(),receipt};}
 cancel():void{this.cancelled=true;}
 result():PixelImage{this.check();if(this.phase!=="complete")throw Error("JPEG decode incomplete");return this.image;}
 takeResult(grant:JpegGrant):{image:PixelImage;receipt:JpegReceipt}|undefined{this.result();if(grant.maximumItems<1||grant.maximumCopyBytes<24||grant.maximumDepth<1)return undefined;const image=this.image;this.image={width:0,height:0,pixels:new Uint8Array(0)};this.phase="transferred";return {image,receipt:{...emptyReceipt(),copiedItems:1,copiedBytes:24}};}
 private byte():number{const value=this.source[this.at++];if(value===undefined)throw Error("Truncated JPEG source");this.consumed=Math.max(this.consumed,this.at);return value;}
 private u16():number{return this.byte()*256+this.byte();}
 private step():void{switch(this.phase){case "markers":this.markerStep();break;case "tables":this.tableStep();break;case "allocate":this.allocateStep();break;case "initialize":this.initializeStep();break;case "entropy":this.entropyStep();break;case "transform":this.transformStep();break;case "pixels":this.pixelStep();break;default:throw Error("Invalid JPEG phase");}}
 private markerStep():void{
  if(this.ended){this.phase="transform";return;}
  if(this.byte()!==255)throw Error("JPEG marker prefix missing");const marker=this.byte();if(marker===255){this.at--;return;}
  if(marker===217){if(!this.framed||!this.scanned||this.at!==this.source.length)throw Error("Invalid JPEG end marker");this.ended=true;this.phase="transform";return;}
  if(marker===216||marker===0||marker>=208&&marker<=215)throw Error("Unexpected JPEG marker");
  if(++this.segments>this.input.maxSegments)throw Error("JPEG segment limit exceeded");const length=this.u16();if(length<2||this.at+length-2>this.source.length)throw Error("Invalid JPEG segment extent");this.marker=marker;this.bodyAt=this.at;this.end=this.at+length-2;
  if(marker===219||marker===196){this.table=-1;this.phase="tables";return;}
  if(marker===192||marker===194){this.frame(marker);this.at=this.end;return;}
  if(marker===218){this.startScan();return;}
  if(marker===221){if(length!==4)throw Error("Invalid JPEG restart interval");this.restartInterval=this.u16();return;}
  if(marker===224&&length>=16&&this.source[this.at]===74&&this.source[this.at+1]===70&&this.source[this.at+2]===73&&this.source[this.at+3]===70&&this.source[this.at+4]===0)this.jfif=true;
  if(marker===238&&length>=14&&String.fromCharCode(...this.source.subarray(this.at,this.at+5))==="Adobe")this.adobe=this.source[this.at+11]!;
  if(!(marker>=224&&marker<=239||marker===254||marker===1))throw Error("Unsupported JPEG coding process");this.at=this.end;this.consumed=Math.max(this.consumed,this.at);
 }
 private frame(marker:number):void{
  if(this.framed||this.end-this.at<6)throw Error("Duplicate or truncated JPEG frame");const precision=this.byte();this.height=this.u16();this.width=this.u16();const count=this.byte();if(precision!==8||this.width===0||this.height===0||![1,3].includes(count)||this.end-this.at!==count*3)throw Error("Unsupported JPEG frame");if(this.width*this.height>this.input.maxPixels)throw Error("JPEG pixel limit exceeded");this.progressive=marker===194;
  for(let i=0;i<count;i++){const id=this.byte(),sampling=this.byte(),quant=this.byte(),h=sampling>>>4,v=sampling&15;if(h<1||h>4||v<1||v>4||quant>3||this.components.some(c=>c.id===id))throw Error("Invalid JPEG frame component");this.hmax=Math.max(this.hmax,h);this.vmax=Math.max(this.vmax,v);this.components.push({id,h,v,quant,cols:0,rows:0,actualCols:0,actualRows:0,coefficients:new Int32Array(0),samples:new Uint8Array(0),predictor:0,bands:new Int8Array(64).fill(-1)});}
  this.mcusX=Math.ceil(this.width/(this.hmax*8));this.mcusY=Math.ceil(this.height/(this.vmax*8));for(const c of this.components){if(this.hmax%c.h!==0||this.vmax%c.v!==0)throw Error("Unsupported JPEG sampling ratio");c.cols=this.mcusX*c.h;c.rows=this.mcusY*c.v;c.actualCols=Math.ceil(this.width*c.h/this.hmax/8);c.actualRows=Math.ceil(this.height*c.v/this.vmax/8);this.workingBytes+=c.cols*c.rows*64*5;}this.workingBytes+=this.width*this.height*4;if(this.workingBytes>this.input.maxWorkingBytes)throw Error("JPEG working storage limit exceeded");this.framed=true;
 }
 private tableStep():void{
  if(this.at===this.end&&this.table===-1){this.phase="markers";return;}if(this.at>=this.end)throw Error("Truncated JPEG table");
  if(this.table===-1){const info=this.byte();this.table=info&15;this.precision=info>>>4;this.tableAt=0;this.tableCount=0;this.tableSymbols=0;this.tableCode=0;this.tableOffset=0;if(this.table>3||this.precision>(this.marker===219?1:1))throw Error("Invalid JPEG table selector");if(this.marker===196){this.table+=this.precision*4;const h=this.huffman[this.table]!;h.valid=false;h.maximum.fill(-1);}return;}
  if(this.marker===219){const value=this.precision===0?this.byte():this.u16();if(value===0||this.at>this.end)throw Error("Invalid JPEG quantization value");this.quant[this.table]![this.tableAt++]=value;if(this.tableAt===64){this.quantValid[this.table]=true;this.table=-1;}return;}
  const h=this.huffman[this.table]!;
  if(this.tableAt<16){const count=this.byte(),len=this.tableAt+1;h.counts[this.tableAt++]=count;h.minimum[len]=this.tableCode;h.maximum[len]=count===0?-1:this.tableCode+count-1;h.offset[len]=this.tableSymbols;this.tableSymbols+=count;if(this.tableSymbols>256||this.tableCode+count>2**len)throw Error("Oversubscribed JPEG Huffman table");this.tableCode=(this.tableCode+count)*2;return;}
  if(this.tableCount<this.tableSymbols){h.symbols[this.tableCount++]=this.byte();if(this.tableCount===this.tableSymbols){h.valid=true;this.table=-1;}return;}throw Error("Empty JPEG Huffman table");
 }
 private startScan():void{
  if(!this.framed||this.end-this.at<4)throw Error("JPEG scan before frame");const count=this.byte();if(count<1||count>this.components.length||this.end-this.at!==count*2+3)throw Error("Invalid JPEG scan components");this.scan=[];for(let i=0;i<count;i++){const id=this.byte(),tables=this.byte(),index=this.components.findIndex(c=>c.id===id);if(index<0||this.scan.includes(index)||tables>>>4>3||(tables&15)>3)throw Error("Invalid JPEG scan selector");this.scan.push(index);this.dc[index]=tables>>>4;this.ac[index]=tables&15;}
  this.spectralStart=this.byte();this.spectralEnd=this.byte();const approximation=this.byte();this.successiveHigh=approximation>>>4;this.successiveLow=approximation&15;
  if(this.progressive){if(this.spectralStart>this.spectralEnd||this.spectralEnd>63||this.spectralStart===0&&this.spectralEnd!==0||this.spectralStart!==0&&count!==1||this.successiveLow>13||this.successiveHigh>13||this.successiveHigh!==0&&this.successiveHigh!==this.successiveLow+1)throw Error("Invalid progressive JPEG scan");}else if(this.spectralStart!==0||this.spectralEnd!==63||approximation!==0)throw Error("Invalid sequential JPEG scan");
  for(const index of this.scan){const c=this.components[index]!;if(!this.quantValid[c.quant])throw Error("Missing JPEG quantization table");for(let k=this.spectralStart;k<=this.spectralEnd;k++){if(c.bands[k]!== (this.successiveHigh===0?-1:this.successiveHigh))throw Error("Invalid JPEG coefficient scan order");c.bands[k]=this.successiveLow;}c.predictor=0;}
  this.unit=0;this.scanComponent=0;this.subBlock=0;this.eobRun=0;this.bits=0;this.bitCount=0;this.restartUnits=0;this.restartIndex=0;const first=this.components[this.scan[0]!]!;this.units=count===1?first.actualCols*first.actualRows:this.mcusX*this.mcusY;this.scanned=true;this.phase=this.image.pixels.length===0?"allocate":"entropy";
 }
 private allocateStep():void{const c=this.components[Math.floor(this.allocateAt/2)];if(c){if(this.allocateAt%2===0)c.coefficients=new Int32Array(c.cols*c.rows*64);else c.samples=new Uint8Array(c.cols*c.rows*64);this.allocateAt++;return;}this.image={width:this.width,height:this.height,pixels:new Uint8Array(this.width*this.height*4)};this.phase="entropy";}
 private initializeStep():void{this.initializeAt++;this.phase="entropy";}
 private readBit():number{if(this.bitCount===0){this.bits=this.byte();if(this.bits===255&&this.byte()!==0)throw Error("Unexpected JPEG entropy marker");this.bitCount=8;}this.bitCount--;return this.bits>>>this.bitCount&1;}
 private readBits(count:number):number{let value=0;for(let i=0;i<count;i++)value=value*2+this.readBit();return value;}
 private receive(count:number):number{if(count===0)return 0;if(count>16)throw Error("Invalid JPEG coefficient magnitude");const value=this.readBits(count);return value<2**(count-1)?value-(2**count-1):value;}
 private symbol(table:number):number{const h=this.huffman[table];if(!h?.valid)throw Error("Missing JPEG Huffman table");let code=0;for(let len=1;len<=16;len++){code=code*2+this.readBit();if(h.maximum[len]!>=0&&code>=h.minimum[len]!&&code<=h.maximum[len]!)return h.symbols[h.offset[len]!+code-h.minimum[len]!]!;}throw Error("Invalid JPEG Huffman symbol");}
 private refine(c:Component,offset:number,k:number):void{const at=offset+zigzag[k]!,value=c.coefficients[at]!;if(this.readBit()!==0&&(Math.abs(value)&1<<this.successiveLow)===0)c.coefficients[at]=value+(value<0?-1:1)*(1<<this.successiveLow);}
 private decodeBlock(c:Component,index:number,block:number):void{
  const offset=block*64,low=this.successiveLow;
  if(!this.progressive){const size=this.symbol(this.dc[index]!);if(size>11)throw Error("Invalid JPEG DC category");c.predictor+=this.receive(size);c.coefficients[offset]=c.predictor;let k=1;while(k<=63){const symbol=this.symbol(4+this.ac[index]!),run=symbol>>>4,size=symbol&15;if(size===0){if(run===15){k+=16;if(k>64)throw Error("JPEG zero run exceeds block");continue;}if(run!==0)throw Error("Invalid sequential JPEG EOB");break;}if(size>10)throw Error("Invalid JPEG AC category");k+=run;if(k>63)throw Error("JPEG AC run exceeds block");c.coefficients[offset+zigzag[k]!] =this.receive(size);k++;}return;}
  if(this.spectralStart===0){if(this.successiveHigh===0){const size=this.symbol(this.dc[index]!);if(size>11)throw Error("Invalid JPEG DC category");c.predictor+=this.receive(size);c.coefficients[offset]=c.predictor*(1<<low);}else c.coefficients[offset]=c.coefficients[offset]!|this.readBit()<<low;return;}
  let k=this.spectralStart;
  if(this.successiveHigh===0){if(this.eobRun>0){this.eobRun--;return;}while(k<=this.spectralEnd){const symbol=this.symbol(4+this.ac[index]!),run=symbol>>>4,size=symbol&15;if(size===0){if(run===15){k+=16;if(k>this.spectralEnd+1)throw Error("JPEG zero run exceeds spectral band");continue;}this.eobRun=(1<<run)+this.readBits(run)-1;break;}if(size>10)throw Error("Invalid JPEG AC category");k+=run;if(k>this.spectralEnd)throw Error("JPEG AC run exceeds spectral band");c.coefficients[offset+zigzag[k]!]=this.receive(size)*(1<<low);k++;}return;}
  if(this.eobRun===0){while(k<=this.spectralEnd){const symbol=this.symbol(4+this.ac[index]!),size=symbol&15;let run=symbol>>>4,newValue=0;if(size===0){if(run<15){this.eobRun=(1<<run)+this.readBits(run);break;}run=16;}else{if(size!==1)throw Error("Invalid JPEG AC refinement size");newValue=(this.readBit()?1:-1)*(1<<low);}for(;k<=this.spectralEnd;k++){if(c.coefficients[offset+zigzag[k]!]!==0)this.refine(c,offset,k);else if(run>0)run--;else break;}if(newValue!==0){if(k>this.spectralEnd)throw Error("JPEG refined coefficient exceeds band");c.coefficients[offset+zigzag[k]!]=newValue;k++;}else if(run>0)throw Error("JPEG refinement zero run exceeds band");}}
  if(this.eobRun>0){for(;k<=this.spectralEnd;k++)if(c.coefficients[offset+zigzag[k]!]!==0)this.refine(c,offset,k);this.eobRun--;}
 }
 private entropyStep():void{
  if(this.unit===this.units){if(this.eobRun!==0)throw Error("JPEG EOB run exceeds scan");this.bits=0;this.bitCount=0;this.phase="markers";return;}
  if(this.scanComponent===0&&this.subBlock===0&&this.restartInterval>0&&this.restartUnits===this.restartInterval){this.bits=0;this.bitCount=0;if(this.byte()!==255||this.byte()!==208+this.restartIndex)throw Error("Invalid JPEG restart marker sequence");this.restartIndex=(this.restartIndex+1)%8;this.restartUnits=0;this.eobRun=0;for(const c of this.components)c.predictor=0;return;}
  const index=this.scan[this.scanComponent]!,c=this.components[index]!;let block:number;
  if(this.scan.length===1)block=Math.floor(this.unit/c.actualCols)*c.cols+this.unit%c.actualCols;else block=(Math.floor(this.unit/this.mcusX)*c.v+Math.floor(this.subBlock/c.h))*c.cols+(this.unit%this.mcusX)*c.h+this.subBlock%c.h;
  this.decodeBlock(c,index,block);this.blocks++;this.subBlock++;if(this.subBlock===(this.scan.length===1?1:c.h*c.v)){this.subBlock=0;this.scanComponent++;if(this.scanComponent===this.scan.length){this.scanComponent=0;this.unit++;this.restartUnits++;}}
 }
 private transformStep():void{
  const c=this.components[this.transformComponent];if(!c){this.phase="pixels";return;}if(c.bands[0]===-1)throw Error("JPEG component missing DC scan");const offset=this.transformBlock*64,q=this.quant[c.quant]!,natural=this.natural,temporary=this.temporary;
  for(let z=0;z<64;z++)natural[zigzag[z]!]=c.coefficients[offset+zigzag[z]!]! *q[z]!;
  for(let y=0;y<8;y++)for(let x=0;x<8;x++){let value=0;for(let u=0;u<8;u++)value+=(u===0?Math.SQRT1_2:1)*natural[y*8+u]!*Math.cos((2*x+1)*u*Math.PI/16);temporary[y*8+x]=value/2;}
  const bx=this.transformBlock%c.cols*8,by=Math.floor(this.transformBlock/c.cols)*8,stride=c.cols*8;
  for(let y=0;y<8;y++)for(let x=0;x<8;x++){let value=0;for(let u=0;u<8;u++)value+=(u===0?Math.SQRT1_2:1)*temporary[u*8+x]!*Math.cos((2*y+1)*u*Math.PI/16);c.samples[(by+y)*stride+bx+x]=Math.max(0,Math.min(255,Math.round(value/2+128)));}
  if(++this.transformBlock===c.cols*c.rows){this.transformBlock=0;this.transformComponent++;}
 }
 private pixelStep():void{
  if(this.pixels===this.width*this.height){this.phase="complete";return;}const x=this.pixels%this.width,y=Math.floor(this.pixels/this.width),values=[0,0,0];for(let i=0;i<this.components.length;i++){const c=this.components[i]!;values[i]=c.samples[Math.floor(y*c.v/this.vmax)*c.cols*8+Math.floor(x*c.h/this.hmax)]!;}
  let [r,g,b]=values as [number,number,number];if(this.components.length===1){g=r;b=r;}else if(this.adobe===1||this.adobe!==0&&(this.jfif||!(this.components[0]!.id===82&&this.components[1]!.id===71&&this.components[2]!.id===66))){const cb=g-128,cr=b-128;g=r-0.344136*cb-0.714136*cr;b=r+1.772*cb;r+=1.402*cr;}
  const at=this.pixels*4;this.image.pixels[at]=Math.max(0,Math.min(255,Math.round(r)));this.image.pixels[at+1]=Math.max(0,Math.min(255,Math.round(g)));this.image.pixels[at+2]=Math.max(0,Math.min(255,Math.round(b)));this.image.pixels[at+3]=255;this.pixels++;
 }
}
