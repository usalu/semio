/** 📷️ Bounded first-party PNG RGBA emission with exact preallocated output and private publication. */
import {validateImage,type PixelImage} from "../../✍️editing/🟦️.ts";
export interface PngEncodeProgress{completed:number;total:number;work:number;done:boolean}
const be=(out:Uint8Array,at:number,value:number)=>{out[at]=value>>>24;out[at+1]=value>>>16;out[at+2]=value>>>8;out[at+3]=value;};
const table=Uint32Array.from({length:256},(_,value)=>{let crc=value;for(let bit=0;bit<8;bit++)crc=(crc>>>1)^((crc&1)?0xedb88320:0);return crc>>>0;});
const crcByte=(crc:number,value:number)=>((crc>>>8)^table[(crc^value)&255]!)>>>0;
export class PngEncodeJob{
 private output:Uint8Array;private readonly total:number;private readonly idatEnd:number;private cursor=0;private at=43;private crc=0xffffffff;private adlerA=1;private adlerB=0;private work=0;private cancelled=false;private done=false;
 constructor(private image:PixelImage,maximumBytes=67108864){
  validateImage(image);this.total=image.pixels.length+image.height;const blocks=Math.ceil(this.total/4096),zlib=this.total+blocks*5+6,bytes=57+zlib;
  if(!Number.isSafeInteger(maximumBytes)||maximumBytes<8||maximumBytes>67108864||bytes>maximumBytes)throw RangeError("PNG encoded byte limit exceeded");this.output=new Uint8Array(bytes);this.output.set([137,80,78,71,13,10,26,10]);be(this.output,8,13);this.output.set([73,72,68,82],12);be(this.output,16,image.width);be(this.output,20,image.height);this.output.set([8,6,0,0,0],24);let header=0xffffffff;for(const byte of this.output.subarray(12,29))header=crcByte(header,byte);be(this.output,29,(header^0xffffffff)>>>0);be(this.output,33,zlib);this.output.set([73,68,65,84],37);for(const byte of [73,68,65,84])this.crc=crcByte(this.crc,byte);this.idatEnd=41+zlib;this.at=41;this.write(0x78);this.write(0x01);
 }
 private write(byte:number):void{this.output[this.at++]=byte;this.crc=crcByte(this.crc,byte);}
 advance(grant:number):PngEncodeProgress{
  if(!Number.isSafeInteger(grant)||grant<1)throw RangeError("Invalid PNG emission work grant");if(this.cancelled)throw new DOMException("PNG emission cancelled","AbortError");
  for(let step=0;step<grant&&!this.done;step++){
   const count=Math.min(4096,this.total-this.cursor),last=this.cursor+count===this.total;this.write(last?1:0);this.write(count&255);this.write(count>>>8);this.write((~count)&255);this.write((~count)>>>8&255);
   const stride=this.image.width*4+1;for(let end=this.cursor+count;this.cursor<end;this.cursor++){const column=this.cursor%stride,byte=column===0?0:this.image.pixels[Math.floor(this.cursor/stride)*(stride-1)+column-1]!;this.write(byte);this.adlerA=(this.adlerA+byte)%65521;this.adlerB=(this.adlerB+this.adlerA)%65521;}
   if(last){for(const shift of [24,16,8,0])this.write(((this.adlerB<<16)|this.adlerA)>>>shift&255);if(this.at!==this.idatEnd)throw Error("PNG encoded extent mismatch");be(this.output,this.at,(this.crc^0xffffffff)>>>0);this.at+=4;be(this.output,this.at,0);this.output.set([73,69,78,68],this.at+4);be(this.output,this.at+8,0xae426082);this.done=true;}this.work++;
  }
  return{completed:this.cursor,total:this.total,work:this.work,done:this.done};
 }
 result():Uint8Array{if(this.cancelled)throw new DOMException("PNG emission cancelled","AbortError");if(!this.done)throw Error("PNG emission incomplete");return this.output;}
 cancel():void{this.cancelled=true;this.output=new Uint8Array(0);}
}
