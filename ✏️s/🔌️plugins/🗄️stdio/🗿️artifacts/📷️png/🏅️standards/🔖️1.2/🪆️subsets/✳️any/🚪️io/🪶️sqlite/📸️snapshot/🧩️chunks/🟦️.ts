/** 🧩️ Native chunk fields and exact compression recipes preserve every byte occurrence. */
import {CompressionSyntaxError,parseCompression,encodeCompression,type DeflateStream, type CompressionCheckpoint} from "../🗜️compression/🟦️.ts";
import {PngSyntaxError,validateHeader,parseScanlines,encodeScanlines,type PngHeader,type PngScanlines} from "../🌈️scanline/🟦️.ts";
export interface PngText { readonly keyword:string; readonly value:string; readonly language:string; readonly translated:string; readonly compressed:number; readonly stream:DeflateStream|null }
export interface PngChunk { readonly kind:string; readonly fields:readonly number[]; readonly octets:readonly number[]; readonly text:PngText|null }
export interface PngNative { readonly header:PngHeader; readonly chunks:readonly PngChunk[]; readonly image:DeflateStream; readonly scanlines:PngScanlines }
function invalid(reason:string):never{throw new PngSyntaxError("PNG "+reason)}
export function crc(bytes:readonly number[]):number{let value=0xffffffff;for(const byte of bytes){value^=byte;for(let at=0;at<8;at++)value=(value>>>1)^((value&1)?0xedb88320:0)}return(~value)>>>0}
function u32(bytes:readonly number[],at:number):number{return(bytes[at]!*16777216+bytes[at+1]!*65536+bytes[at+2]!*256+bytes[at+3]!)>>>0}
function u16(bytes:readonly number[],at:number):number{return bytes[at]!*256+bytes[at+1]!}
function packed(value:number,bits:number):number[]{if(!Number.isInteger(value)||value<0||value>=2**bits)invalid("field integer range");const result:number[]=[];for(let at=bits-8;at>=0;at-=8)result.push((value/2**at)&255);return result}
function latin1(bytes:readonly number[]):string{let value="";for(const byte of bytes)value+=String.fromCharCode(byte);return value}
function latin1Bytes(value:string):number[]{const bytes:number[]=[];for(let at=0;at<value.length;at++){const byte=value.charCodeAt(at);if(byte>255)invalid("Latin-1 scalar");bytes.push(byte)}return bytes}
function utf8(bytes:readonly number[]):string{try{return new TextDecoder("utf-8",{fatal:true}).decode(Uint8Array.from(bytes))}catch{return invalid("UTF-8 text")}}
function terminated(bytes:readonly number[],start:number):[number[],number]{const end=bytes.indexOf(0,start);if(end<0)invalid("missing text separator");return[bytes.slice(start,end),end+1]}
const singleton=["IHDR","PLTE","tRNS","gAMA","cHRM","sRGB","pHYs","tIME","bKGD","IEND"];
export function validateChunks(header:PngHeader,chunks:readonly PngChunk[]):void{
 validateHeader(header);if(chunks[0]?.kind!=="IHDR"||chunks.at(-1)?.kind!=="IEND")invalid("first or last chunk");for(const kind of singleton)if(chunks.filter(chunk=>chunk.kind===kind).length>1)invalid("singleton chunk");for(const chunk of chunks)if(!/^[A-Za-z]{2}[A-Z][A-Za-z]$/.test(chunk.kind))invalid("chunk type bits");
 const first=chunks.findIndex(chunk=>chunk.kind==="IDAT"),last=chunks.findLastIndex(chunk=>chunk.kind==="IDAT"),palette=chunks.findIndex(chunk=>chunk.kind==="PLTE");if(first<0||chunks.slice(first,last+1).some(chunk=>chunk.kind!=="IDAT"))invalid("IDAT adjacency");const entries=palette<0?0:chunks[palette]!.fields.length/3;
 if(palette>=0&&(palette>first||[0,4].includes(header.colorType)||entries<1||entries>256||!Number.isInteger(entries)||header.colorType===3&&entries>2**header.bitDepth)||palette<0&&header.colorType===3)invalid("palette profile");
 const maximum=2**header.bitDepth-1;for(let at=0;at<chunks.length;at++){const chunk=chunks[at]!,fields=chunk.fields;
  if(["gAMA","cHRM","sRGB"].includes(chunk.kind)&&(at>first||palette>=0&&at>palette))invalid("color metadata order");
  switch(chunk.kind){case "IHDR":case "IDAT":break;case "IEND":if(fields.length||chunk.octets.length)invalid("IEND content");break;
   case "PLTE":for(const value of fields)if(value<0||value>255||!Number.isInteger(value))invalid("palette value");break;
   case "gAMA":if(fields.length!==1||fields[0]===0)invalid("gamma field");break;
   case "cHRM":if(fields.length!==8)invalid("chromaticity fields");break;
   case "sRGB":if(fields.length!==1||fields[0]!>3)invalid("sRGB intent");break;
   case "pHYs":if(at>first||fields.length!==3||fields[2]!>1)invalid("physical density");break;
   case "tIME":if(fields.length!==6||fields.slice(1).some(value=>value>255))invalid("time shape");break;
   case "tRNS":if(at>first||palette>=0&&at<palette||header.colorType===0&&(fields.length!==1||fields[0]!>maximum)||header.colorType===2&&(fields.length!==3||fields.some(value=>value>maximum))||header.colorType===3&&(fields.length<1||fields.length>entries||fields.some(value=>value>255))||[4,6].includes(header.colorType))invalid("transparency profile");break;
   case "bKGD":if(at>first||palette>=0&&at<palette||[0,4].includes(header.colorType)&&(fields.length!==1||fields[0]!>maximum)||[2,6].includes(header.colorType)&&(fields.length!==3||fields.some(value=>value>maximum))||header.colorType===3&&(fields.length!==1||fields[0]!>=entries))invalid("background profile");break;
   case "tEXt":case "zTXt":case "iTXt":if(chunk.text===null)invalid("text owner");if(chunk.kind==="tEXt"&&chunk.text.compressed!==0||chunk.kind==="zTXt"&&chunk.text.compressed!==1||chunk.kind!=="iTXt"&&(chunk.text.language!==""||chunk.text.translated!==""))invalid("text role fields");break;
   default:if(chunk.kind[0]===chunk.kind[0]!.toUpperCase())invalid("unsupported critical chunk");
  }
 }
}
export async function parsePngNative(bytes:readonly number[],checkpoint:CompressionCheckpoint):Promise<PngNative|null>{
 try{
  const signature=[137,80,78,71,13,10,26,10];if(!signature.every((byte,at)=>bytes[at]===byte))return null;const sources:{kind:string;data:number[]}[]=[],chunks:PngChunk[]=[],idat:number[]=[];let at=8,work=0;
  while(at<bytes.length){if(at+12>bytes.length)invalid("truncated chunk");const length=u32(bytes,at);if(length>bytes.length-at-12)invalid("chunk extent");const kind=latin1(bytes.slice(at+4,at+8)),data=bytes.slice(at+8,at+8+length);if(crc(bytes.slice(at+4,at+8+length))!==u32(bytes,at+8+length))invalid("chunk CRC");sources.push({kind,data});at+=length+12;if(++work%256===0)await checkpoint(work)}
  const first=sources[0];if(first?.kind!=="IHDR"||first.data.length!==13)invalid("IHDR shape");const h=first.data,header={width:u32(h,0),height:u32(h,4),bitDepth:h[8]!,colorType:h[9]!,compression:h[10]!,filter:h[11]!,interlace:h[12]!};validateHeader(header);
  for(const{kind,data}of sources){let fields:number[]=[],octets:number[]=[],text:PngText|null=null;
   switch(kind){
    case "IHDR":fields=[header.width,header.height,header.bitDepth,header.colorType,header.compression,header.filter,header.interlace];break;
    case "IEND":if(data.length!==0)invalid("IEND length");break;
    case "IDAT":fields=[data.length];for(const byte of data){idat.push(byte);if(++work%256===0)await checkpoint(work)}break;
    case "PLTE":if(data.length%3)invalid("palette length");fields=data;break;
    case "tRNS":case "bKGD":if(header.colorType===3)fields=data;else{if(data.length%2)invalid("sample metadata length");for(let offset=0;offset<data.length;offset+=2)fields.push(u16(data,offset))}break;
    case "gAMA":if(data.length!==4)invalid("gamma length");fields=[u32(data,0)];break;
    case "cHRM":if(data.length!==32)invalid("chromaticity length");for(let offset=0;offset<32;offset+=4)fields.push(u32(data,offset));break;
    case "sRGB":if(data.length!==1)invalid("sRGB length");fields=data;break;
    case "pHYs":if(data.length!==9)invalid("physical density length");fields=[u32(data,0),u32(data,4),data[8]!];break;
    case "tIME":if(data.length!==7)invalid("time length");fields=[u16(data,0),...data.slice(2)];break;
    case "tEXt":case "zTXt":case "iTXt":{
     const[keywordBytes,start]=terminated(data,0),keyword=latin1(keywordBytes);let cursor=start,language="",translated="",compressed=0,payload:number[]=[];
     if(kind==="zTXt"){if(data[cursor++]!==0)invalid("text compression method");compressed=1;payload=data.slice(cursor)}
     else if(kind==="iTXt"){compressed=data[cursor++]!;if(![0,1].includes(compressed)||data[cursor++]!==0)invalid("international compression flags");const[languageBytes,next]=terminated(data,cursor);if(languageBytes.some(byte=>byte>127))invalid("language ASCII");language=latin1(languageBytes);const[translatedBytes,end]=terminated(data,next);translated=utf8(translatedBytes);payload=data.slice(end)}else payload=data.slice(cursor);
     const decoded=compressed?await parseCompression(payload,checkpoint):null,raw=decoded?.raw??payload,value=kind==="iTXt"?utf8(raw):latin1(raw);text={keyword,value,language,translated,compressed,stream:decoded?.stream??null};break;
    }
    default:octets=data;
   }
   chunks.push({kind,fields,octets,text});await checkpoint(++work);
  }
  validateChunks(header,chunks);const image=await parseCompression(idat,checkpoint),scanlines=await parseScanlines(image.raw,header,checkpoint);return{header,chunks,image:image.stream,scanlines};
 }catch(error){if(error instanceof PngSyntaxError||error instanceof CompressionSyntaxError)return null;throw error}
}
export async function encodePngNative(model:PngNative,checkpoint:CompressionCheckpoint):Promise<number[]>{
 validateChunks(model.header,model.chunks);const image=await encodeCompression(model.image,checkpoint),raw=await encodeScanlines(model.scanlines,checkpoint);if(raw.length!==image.raw.length||raw.some((byte,at)=>byte!==image.raw[at]))invalid("scanline and compression relationship");
 const bytes=[137,80,78,71,13,10,26,10];let imageAt=0,work=0;
 for(const chunk of model.chunks){const f=chunk.fields;let data:number[]=[];
  switch(chunk.kind){
   case "IHDR":data=[...packed(model.header.width,32),...packed(model.header.height,32),model.header.bitDepth,model.header.colorType,model.header.compression,model.header.filter,model.header.interlace];break;
   case "IEND":break;
   case "IDAT":{const length=f[0]!;if(!Number.isSafeInteger(length)||length<0||imageAt+length>image.bytes.length)invalid("IDAT partition");data=image.bytes.slice(imageAt,imageAt+length);imageAt+=length;break}
   case "PLTE":case "sRGB":data=[...f];break;
   case "tRNS":case "bKGD":data=model.header.colorType===3?[...f]:f.flatMap(value=>packed(value,16));break;
   case "gAMA":case "cHRM":data=f.flatMap(value=>packed(value,32));break;
   case "pHYs":data=[...packed(f[0]!,32),...packed(f[1]!,32),f[2]!];break;
   case "tIME":data=[...packed(f[0]!,16),...f.slice(1)];break;
   case "tEXt":case "zTXt":case "iTXt":{const text=chunk.text!;if(text.keyword.includes("\0")||text.language.includes("\0")||text.translated.includes("\0"))invalid("text separators");const body=chunk.kind==="iTXt"?[...new TextEncoder().encode(text.value)]:latin1Bytes(text.value);let payload=body;if(text.compressed){if(text.stream===null)invalid("missing text compression");const encoded=await encodeCompression(text.stream,checkpoint);if(encoded.raw.length!==body.length||encoded.raw.some((byte,at)=>byte!==body[at]))invalid("text and compression relationship");payload=encoded.bytes}else if(text.stream!==null)invalid("unexpected text compression");
    data=[...latin1Bytes(text.keyword),0];if(chunk.kind==="zTXt")data.push(0);if(chunk.kind==="iTXt"){if(text.language.split("").some(value=>value.charCodeAt(0)>127))invalid("language ASCII");data.push(text.compressed,0,...latin1Bytes(text.language),0,...new TextEncoder().encode(text.translated),0)}for(const byte of payload)data.push(byte);break;
   }
   default:data=[...chunk.octets];
  }
  const typed=latin1Bytes(chunk.kind);bytes.push(...packed(data.length,32),...typed);for(const byte of data){if(!Number.isInteger(byte)||byte<0||byte>255)invalid("native octet range");bytes.push(byte);if(++work%256===0)await checkpoint(work)}bytes.push(...packed(crc([...typed,...data]),32));await checkpoint(++work);
 }
 if(imageAt!==image.bytes.length)invalid("unclaimed image compression");return bytes;
}
