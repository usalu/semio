/** 📥️ Budgeted PNG reconstruction with private RGBA publication. */
import {validateExtent,type PixelImage} from "../../✍️editing/🟦️.ts";
import {InflateCursor} from "../../../🗜️deflate/📥️decode/🟦️.ts";
export type PngDecodeInput={data:Uint8Array;maxPixels:number;maxBytes:number;maxChunks:number};
export type PngDecodeProgress={phase:"chunks"|"inflating"|"pixels"|"complete";bytes:number;totalBytes:number;pixels:number;totalPixels:number;work:number;done:boolean};
export type PngDecodeOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:PngDecodeProgress)=>void};
type Header={width:number;height:number;depth:number;color:number;interlace:number;channels:number};
type Chunk={kind:string;start:number;length:number;at:number;end:number;crc:number};
const signature=[137,80,78,71,13,10,26,10];
const passes=[[0,0,8,8],[4,0,8,8],[0,4,4,8],[2,0,4,4],[0,2,2,4],[1,0,2,2],[0,1,1,2]] as const;
const fail=(message:string):never=>{throw new RangeError(message);};
const abort=():never=>{throw new DOMException("PNG decode cancelled","AbortError");};
const be=(data:Uint8Array,at:number)=>data[at]!*16777216+data[at+1]!*65536+data[at+2]!*256+data[at+3]!;
const validInt=(n:number,min:number,max:number)=>Number.isSafeInteger(n)&&n>=min&&n<=max;
const crcTable=Uint32Array.from({length:256},(_,value)=>{let c=value;for(let i=0;i<8;i++)c=(c>>>1)^((c&1)?0xedb88320:0);return c>>>0;});
const crcByte=(crc:number,value:number)=>((crc>>>8)^crcTable[(crc^value)&255]!)>>>0;
const paeth=(a:number,b:number,c:number)=>{const p=a+b-c,pa=Math.abs(p-a),pb=Math.abs(p-b),pc=Math.abs(p-c);return pa<=pb&&pa<=pc?a:pb<=pc?b:c;};
/** 🧩️ Source bytes stay immutable until this job completes or is cancelled. */
export class PngDecodeJob{
 private data:Uint8Array;private readonly totalBytes:number;private readonly limits:Omit<PngDecodeInput,"data">;
 private phase:PngDecodeProgress["phase"]="chunks";private work=0;private bytes=8;private pixels=0;private totalPixels=0;
 private at=8;private chunks=0;private chunk:Chunk|undefined;private header:Header|undefined;
 private palette=new Uint8Array(0);private alpha=new Uint8Array(0);private transparent:number[]|undefined;private hadPalette=false;private hadTransparency=false;
 private hadIdat=false;private closedIdat=false;private ranges:[number,number][]=[];private zLength=0;private range=0;private rangeAt=0;private zAt=0;private zHeader:number[]=[];private tail:number[]=[];
 private inflater:InflateCursor|undefined;private pending:number|undefined;private candidate:PixelImage|undefined;
 private pass=-1;private passWidth=0;private passHeight=0;private sx=0;private sy=0;private dx=1;private dy=1;private row=0;private column=0;private rowAt=-1;private rowBytes=0;private bpp=0;private filter=0;
 private current=new Uint8Array(0);private previous=new Uint8Array(0);private rawBytes=0;private expectedBytes=0;private adlerA=1;private adlerB=0;private streamDone=false;
 private cancelled=false;private failed:unknown;private samples=new Uint32Array(4);
 constructor(input:PngDecodeInput){
  if(!(input.data instanceof Uint8Array)||!validInt(input.maxPixels,1,16777216)||!validInt(input.maxBytes,8,67108864)||!validInt(input.maxChunks,1,65536)||input.data.length<8||input.data.length>input.maxBytes||signature.some((v,i)=>input.data[i]!==v))fail("Invalid PNG input contract");
  this.data=input.data;this.totalBytes=input.data.length;this.limits={maxPixels:input.maxPixels,maxBytes:input.maxBytes,maxChunks:input.maxChunks};
 }
 private check():void{if(this.cancelled)abort();if(this.failed!==undefined)throw this.failed;}
 private release():void{this.data=new Uint8Array(0);this.ranges=[];this.current=new Uint8Array(0);this.previous=new Uint8Array(0);this.inflater=undefined;this.candidate=undefined;this.pending=undefined;}
 private parseChunk(c:Chunk):void{
  const h=this.header,data=this.data.subarray(c.start,c.end);
  if(!h&&c.kind!=="IHDR")fail("IHDR must be first");
  if(c.kind!=="IDAT"&&this.hadIdat)this.closedIdat=true;
  switch(c.kind){
   case "IHDR":{
    if(h||this.chunks!==1||data.length!==13)fail("Invalid or duplicate IHDR");
    const width=be(data,0),height=be(data,4),depth=data[8]!,color=data[9]!,interlace=data[12]!;
    if(data[10]!==0||data[11]!==0||interlace>1||!([0,2,3,4,6].includes(color))||!(color===0?[1,2,4,8,16]:color===3?[1,2,4]:[]).includes(depth)&&depth!==8&&depth!==16||color===3&&depth===16)fail("Invalid PNG header");
    this.totalPixels=validateExtent(width,height);if(this.totalPixels>this.limits.maxPixels)fail("PNG pixel limit exceeded");
    this.header={width,height,depth,color,interlace,channels:color===2?3:color===4?2:color===6?4:1};break;
   }
   case "PLTE":
    if(this.hadPalette||this.hadTransparency||this.hadIdat||h!.color===0||h!.color===4||!data.length||data.length>768||data.length%3||h!.color===3&&data.length/3>2**h!.depth)fail("Invalid PNG palette");
    this.palette=data.slice();this.hadPalette=true;break;
   case "tRNS":{
    if(this.hadTransparency||this.hadIdat)fail("Invalid transparency placement");this.hadTransparency=true;const color=h!.color;
    if(color===3){if(!this.hadPalette||data.length===0||data.length>this.palette.length/3)fail("Invalid palette transparency");this.alpha=data.slice();}
    else if(color===0||color===2){if(data.length!==(color===0?2:6))fail("Invalid sample transparency");this.transparent=[];for(let i=0;i<data.length;i+=2){const v=data[i]!*256+data[i+1]!;if(v>2**h!.depth-1)fail("Transparency exceeds sample depth");this.transparent.push(v);}}
    else fail("Transparency is forbidden for alpha images");break;
   }
   case "IDAT":
    if(this.closedIdat||h!.color===3&&!this.hadPalette)fail("Invalid IDAT placement");
    this.hadIdat=true;this.ranges.push([c.start,c.end]);this.zLength+=data.length;break;
   case "IEND":
    if(data.length||!this.hadIdat||this.at!==this.totalBytes||this.zLength<6)fail("Invalid PNG end");
    this.phase="inflating";this.rangeAt=this.ranges[0]![0];
    this.candidate={width:h!.width,height:h!.height,pixels:new Uint8Array(this.totalPixels*4)};
    this.bpp=Math.max(1,Math.ceil(h!.channels*h!.depth/8));const stride=Math.ceil(h!.width*h!.channels*h!.depth/8);
    this.current=new Uint8Array(stride);this.previous=new Uint8Array(stride);
    for(let p=0;p<(h!.interlace?7:1);p++){const spec=h!.interlace?passes[p]!:([0,0,1,1]as const),w=Math.max(0,Math.ceil((h!.width-spec[0])/spec[2])),height=Math.max(0,Math.ceil((h!.height-spec[1])/spec[3]));if(w&&height)this.expectedBytes+=(Math.ceil(w*h!.channels*h!.depth/8)+1)*height;}
    this.nextPass();break;
   default:if((c.kind.charCodeAt(0)&32)===0)fail("Unknown critical PNG chunk");
  }
 }
 private chunkStep():void{
  if(!this.chunk){
   if(this.at+12>this.totalBytes||++this.chunks>this.limits.maxChunks)fail("Truncated PNG or chunk limit exceeded");
   const length=be(this.data,this.at),end=this.at+8+length;if(length>2147483647||end+4>this.totalBytes)fail("Invalid chunk length");
   const type=this.data.subarray(this.at+4,this.at+8);if(type.some(v=>v<65||v>90&&v<97||v>122))fail("Invalid chunk type");
   this.chunk={kind:String.fromCharCode(...type),start:this.at+8,length,at:this.at+4,end,crc:0xffffffff};this.bytes=this.at+4;return;
  }
  const c=this.chunk;
  if(c.at<c.end){c.crc=crcByte(c.crc,this.data[c.at++]!);this.bytes=c.at;return;}
  if(((c.crc^0xffffffff)>>>0)!==be(this.data,c.end))fail("PNG CRC mismatch");
  this.at=c.end+4;this.bytes=this.at;this.parseChunk(c);this.chunk=undefined;
 }
 private nextPass():void{
  const h=this.header!;this.pass++;while(this.pass<(h.interlace?7:1)){
   const p=h.interlace?passes[this.pass]!:([0,0,1,1]as const);[this.sx,this.sy,this.dx,this.dy]=p;
   this.passWidth=Math.max(0,Math.ceil((h.width-this.sx)/this.dx));this.passHeight=Math.max(0,Math.ceil((h.height-this.sy)/this.dy));
   if(this.passWidth&&this.passHeight){this.row=0;this.rowAt=-1;this.column=0;this.rowBytes=Math.ceil(this.passWidth*h.channels*h.depth/8);return;}this.pass++;
  }
  this.passWidth=0;
 }
 private zByte():number|undefined{
  const r=this.ranges[this.range];if(!r)return undefined;
  if(this.rangeAt===r[1]){this.range++;this.rangeAt=this.ranges[this.range]?.[0]??0;return undefined;}
  this.zAt++;return this.data[this.rangeAt++]!;
 }
 private raw(value:number):void{
  if(++this.rawBytes>this.expectedBytes||!this.passWidth)fail("Excess PNG scanline data");
  this.adlerA=(this.adlerA+value)%65521;this.adlerB=(this.adlerB+this.adlerA)%65521;
  if(this.rowAt<0){if(value>4)fail("Invalid PNG filter");this.filter=value;this.rowAt=0;return;}
  const i=this.rowAt,left=i>=this.bpp?this.current[i-this.bpp]!:0,up=this.row?this.previous[i]!:0,corner=this.row&&i>=this.bpp?this.previous[i-this.bpp]!:0;
  const predictor=this.filter===0?0:this.filter===1?left:this.filter===2?up:this.filter===3?Math.floor((left+up)/2):paeth(left,up,corner);
  this.current[i]=(value+predictor)&255;if(++this.rowAt===this.rowBytes){this.phase="pixels";this.column=0;}
 }
 private pixelStep():void{
  const h=this.header!,samples=this.samples;
  for(let c=0;c<h.channels;c++){const sample=this.column*h.channels+c,bit=sample*h.depth,at=bit>>>3;samples[c]=h.depth===16?this.current[at]!*256+this.current[at+1]!:h.depth===8?this.current[at]!:(this.current[at]!>>>(8-h.depth-(bit&7)))&((1<<h.depth)-1);}
  let r=0,g=0,b=0,a=255;
  if(h.color===3){const i=samples[0]!;if(i>=this.palette.length/3)fail("Palette index outside entries");r=this.palette[i*3]!;g=this.palette[i*3+1]!;b=this.palette[i*3+2]!;a=this.alpha[i]??255;}
  else if(h.color===0||h.color===4){r=g=b=this.scale(samples[0]!);a=h.color===4?this.scale(samples[1]!):this.transparent?.[0]===samples[0]?0:255;}
  else{r=this.scale(samples[0]!);g=this.scale(samples[1]!);b=this.scale(samples[2]!);a=h.color===6?this.scale(samples[3]!):this.transparent!==undefined&&this.transparent[0]===samples[0]&&this.transparent[1]===samples[1]&&this.transparent[2]===samples[2]?0:255;}
  const at=((this.sy+this.row*this.dy)*h.width+this.sx+this.column*this.dx)*4,p=this.candidate!.pixels;p[at]=r;p[at+1]=g;p[at+2]=b;p[at+3]=a;this.pixels++;
  if(++this.column===this.passWidth){const swap=this.previous;this.previous=this.current;this.current=swap;this.rowAt=-1;if(++this.row===this.passHeight)this.nextPass();this.phase="inflating";}
 }
 private scale(v:number):number{const depth=this.header!.depth;return depth===16?v>>>8:depth===8?v:Math.round(v*255/((1<<depth)-1));}
 private inflateStep():void{
  if(this.zHeader.length<2){const b=this.zByte();if(b!==undefined)this.zHeader.push(b);if(this.zHeader.length===2){const [cmf,flg]=this.zHeader as [number,number];if((cmf&15)!==8||(cmf>>>4)>7||((cmf<<8)|flg)%31||(flg&32))fail("Invalid PNG zlib header");this.inflater=new InflateCursor(2**((cmf>>>4)+8));}return;}
  if(this.streamDone){
   if(this.tail.length<4){const b=this.zByte();if(b!==undefined)this.tail.push(b);return;}
   if(this.zAt!==this.zLength||be(Uint8Array.from(this.tail),0)!==((this.adlerB*65536+this.adlerA)>>>0)||this.rawBytes!==this.expectedBytes||this.passWidth||this.pixels!==this.totalPixels)fail("PNG stream size or Adler mismatch");
   this.phase="complete";this.data=new Uint8Array(0);this.ranges=[];this.current=new Uint8Array(0);this.previous=new Uint8Array(0);this.inflater=undefined;return;
  }
  if(this.pending===undefined&&this.zAt<this.zLength-4){const b=this.zByte();if(b===undefined)return;this.pending=b;}
  const result=this.inflater!.advance(this.pending,this.zAt===this.zLength-4);if(result.consumed)this.pending=undefined;
  if(result.value!==undefined)this.raw(result.value);
  if(result.done){if(this.pending!==undefined||this.zAt!==this.zLength-4||this.inflater!.unusedWholeBytes)fail("Trailing DEFLATE data");this.streamDone=true;}
 }
 advance(budget:number):PngDecodeProgress{
  if(!validInt(budget,1,Number.MAX_SAFE_INTEGER))fail("PNG work grant must be a positive integer");this.check();
  try{for(let i=0;i<budget&&this.phase!=="complete";i++){if(this.phase==="chunks")this.chunkStep();else if(this.phase==="pixels")this.pixelStep();else this.inflateStep();this.work++;}}
  catch(error){this.failed=error;this.release();throw error;}
  return {phase:this.phase,bytes:this.bytes,totalBytes:this.totalBytes,pixels:this.pixels,totalPixels:this.totalPixels,work:this.work,done:this.phase==="complete"};
 }
 cancel():void{this.cancelled=true;this.release();}
 result():PixelImage{this.check();if(this.phase!=="complete"||!this.candidate)fail("PNG decode is incomplete");return this.candidate!;}
}
export async function decodePngImage(input:PngDecodeInput,options:PngDecodeOptions={}):Promise<PixelImage>{
 if(options.signal?.aborted)abort();const job=new PngDecodeJob(input);
 try{for(;;){if(options.signal?.aborted)abort();const progress=job.advance(options.workBudget??4096);options.onProgress?.(progress);if(options.signal?.aborted)abort();if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}
 catch(error){job.cancel();throw error;}
}
