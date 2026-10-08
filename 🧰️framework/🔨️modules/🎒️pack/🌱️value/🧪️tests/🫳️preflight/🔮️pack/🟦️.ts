/** 🔮️ Independent framed Pack readback uses Node DEFLATE and literal closed Record tags. */
import {inflateRawSync} from "node:zlib";
class Reader{
 position=0;
 constructor(readonly bytes:Uint8Array){}
 byte(){if(this.position===this.bytes.length)throw Error("Truncated independent Pack");return this.bytes[this.position++]!;}
 span(length:number){if(!Number.isSafeInteger(length)||length<0||length>this.bytes.length-this.position)throw Error("Invalid independent span");const output=this.bytes.subarray(this.position,this.position+length);this.position+=length;return output;}
 unsigned(){let output=0n;for(let index=0;index<10;index++){const byte=this.byte();if(index===9&&byte>1)throw Error("Independent unsigned word overflow");output|=BigInt(byte&127)<<BigInt(index*7);if(byte<128)return output;}throw Error("Invalid independent varint");}
 varint(){const value=Number(this.unsigned());if(!Number.isSafeInteger(value))throw Error("Independent count exceeds safe integer");return value;}
 text(length:number){return new TextDecoder("utf-8",{fatal:true}).decode(this.span(length));}
}
function crc32c(bytes:Uint8Array){let crc=0xffffffff;for(const byte of bytes){crc^=byte;for(let bit=0;bit<8;bit++)crc=(crc>>>1)^(crc&1?0x82f63b78:0);}return (~crc)>>>0;}
export type ClosedValue=string|boolean|number|null|ClosedValue[]|{[id:string]:ClosedValue};
function record(reader:Reader,symbols:string[]):{[id:number]:ClosedValue}{const result:{[id:number]:ClosedValue}={};const count=reader.varint();for(let index=0;index<count;index++){const id=reader.varint();if(id in result)throw Error("Repeated independent field");result[id]=value(reader,symbols);}return result;}
function value(reader:Reader,symbols:string[]):ClosedValue{
 const symbol=()=>{const index=reader.varint();if(index>=symbols.length)throw Error("Unknown independent symbol");return symbols[index]!;};
 const tag=reader.byte();switch(tag){
  case 0:case 18:return null;case 1:return false;case 2:return true;
  case 3:{const word=reader.unsigned();return String((word>>1n)^-(word&1n));}
  case 4:return String(reader.unsigned());
  case 5:{const raw=reader.span(8);return {bits:new DataView(raw.buffer,raw.byteOffset,8).getBigUint64(0,true).toString(16).padStart(16,"0")};}
  case 6:return symbol();case 7:return reader.text(reader.varint());case 10:return reader.varint();
  case 11:case 12:{const count=reader.varint(),values:ClosedValue[]=[];for(let index=0;index<count;index++)values.push(value(reader,symbols));return values;}
  case 21:{const count=reader.varint(),values:ClosedValue[]=[];for(let index=0;index<count;index++){const raw=reader.span(8);values.push({bits:new DataView(raw.buffer,raw.byteOffset,8).getBigUint64(0,true).toString(16).padStart(16,"0")});}return values;}
  case 22:{const count=reader.varint(),values:ClosedValue[]=[];for(let index=0;index<count;index++){const word=reader.unsigned();values.push(String((word>>1n)^-(word&1n)));}return values;}
  case 13:return record(reader,symbols);
  case 16:{const count=reader.varint(),result:{[key:string]:ClosedValue}={};for(let index=0;index<count;index++){const key=value(reader,symbols);if(typeof key!=="string")throw Error("Independent map key is not text");Object.defineProperty(result,key,{value:value(reader,symbols),enumerable:true,configurable:true,writable:true});}return result;}
  case 17:return value(reader,symbols);
  case 15:{const count=reader.varint(),statements:ClosedValue[]=[];for(let index=0;index<count;index++)statements.push({keyword:symbol(),record:record(reader,symbols)});return {statements};}
  case 20:{
   const count=reader.varint(),width=reader.varint(),rows:Record<string,ClosedValue>[] = Array.from({length:count},()=>({}));let previous=-1;
   for(let column=0;column<width;column++){
    const id=reader.varint();if(id<=previous)throw Error("Unordered independent table column");previous=id;const sparse=reader.byte();if(sparse!==0&&sparse!==1)throw Error("Invalid independent table presence");const presence=sparse?reader.span(Math.ceil(count/8)):null,element=reader.byte(),bits=element===1?reader.span(Math.ceil(count/8)):null;
    for(let row=0;row<count;row++){const present=presence===null||Boolean(presence[row>>3]!&(1<<(row&7)));if(!present){rows[row]![id]=null;continue;}switch(element){
     case 0:rows[row]![id]=value(reader,symbols);break;
     case 1:rows[row]![id]=Boolean(bits![row>>3]!&(1<<(row&7)));break;
     case 2:{const word=reader.unsigned();rows[row]![id]=String((word>>1n)^-(word&1n));break;}
     case 3:rows[row]![id]=String(reader.unsigned());break;
     case 4:{const raw=reader.span(8);rows[row]![id]={bits:new DataView(raw.buffer,raw.byteOffset,8).getBigUint64(0,true).toString(16).padStart(16,"0")};break;}
     case 5:rows[row]![id]=symbol();break;
     case 6:rows[row]![id]=reader.varint();break;
     default:throw Error("Unauthored independent table element");
    }}
   }return {tableRows:rows};
  }
  default:throw Error("Unauthored independent closed tag "+tag);
 }
}

/** 🧺️ Reads explicit intrinsic Body framing without importing any Product or domain extension. */
export function readIntrinsicBody(bytes:Uint8Array):ClosedValue{
 const reader=new Reader(bytes),symbols:string[]=[];const count=reader.varint();for(let index=0;index<count;index++)symbols.push(reader.text(reader.varint()));if(reader.varint()!==1||reader.varint()!==1||reader.byte()!==17)throw Error("Independent intrinsic field identity");const result=value(reader,symbols);if(reader.position!==bytes.length)throw Error("Independent intrinsic Body tail");return result;
}
export function readClosedRecordPack(bytes:Uint8Array,token:string):{[id:number]:ClosedValue}{
 const outer=new Reader(bytes);if(Buffer.compare(Buffer.from(outer.span(8)),Buffer.from([137,83,69,77,13,10,26,10])))throw Error("Independent envelope magic");const lengthBytes=outer.span(4),length=new DataView(lengthBytes.buffer,lengthBytes.byteOffset,4).getUint32(0,true);if(outer.text(length)!==token+".pack v1")throw Error("Independent envelope identity");return readBareClosedRecordPack(outer.span(bytes.length-outer.position));
}
export function readBareClosedRecordPack(pack:Uint8Array):{[id:number]:ClosedValue}{
if(Buffer.compare(Buffer.from(pack.subarray(0,8)),Buffer.from([137,83,80,75,13,10,26,10])))throw Error("Independent Pack magic");const segments=new Reader(pack);segments.span(32);let symbols:string[]=[];const documents:Uint8Array[]=[];let ended=false;while(segments.position<pack.length){const start=segments.position,kind=segments.byte(),flags=segments.byte(),storedLength=segments.varint(),rawLength=flags&1?segments.varint():storedLength,stored=segments.span(storedLength),checksum=segments.span(4);if(new DataView(checksum.buffer,checksum.byteOffset,4).getUint32(0,true)!==crc32c(pack.subarray(start,segments.position-4)))throw Error("Independent segment checksum");if(flags!==0&&flags!==3)throw Error("Unknown independent compression");const raw=flags===3?new Uint8Array(inflateRawSync(stored)):stored;if(raw.length!==rawLength)throw Error("Independent decompression extent");if(kind===3){const reader=new Reader(raw),count=reader.varint();for(let index=0;index<count;index++)symbols.push(reader.text(reader.varint()));if(reader.position!==raw.length)throw Error("Independent symbols tail");}else if(kind===4){documents.push(raw);}else if(kind===0){ended=true;break;}}
 if(!ended||!documents.length)throw Error("Independent document framing incomplete");const reader=new Reader(Buffer.concat(documents.map(part=>Buffer.from(part)))),owner=record(reader,symbols);if(reader.position!==reader.bytes.length)throw Error("Independent Record tail");return owner;
}
