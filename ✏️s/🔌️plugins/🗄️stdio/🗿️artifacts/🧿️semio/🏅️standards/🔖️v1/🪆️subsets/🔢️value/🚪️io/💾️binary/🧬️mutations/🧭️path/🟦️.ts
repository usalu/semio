/** 🧭️ Encodes the schema-defined Semio value path frame using framework UTF-8 primitives. */
import type {SemioValuePath} from "../../../../🧬️schema/🧬️mutations/🟦️.ts";

const encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true});

function integer(value:number):bigint{
 if(!Number.isSafeInteger(value)||value<0)throw Error("Semio path requires an addressable nonnegative integer");
 return BigInt(value);
}

function write(value:bigint,out:number[]):void{
 while(value>=128n){out.push(Number(value&127n)|128);value>>=7n;}
 out.push(Number(value));
}

export function encodeSemioValuePath(path:SemioValuePath):Uint8Array{
 const out:number[]=[];write(integer(path.length),out);
 for(const part of path){
  if(part.kind==="key"){
   const bytes=encoder.encode(part.key);if(decoder.decode(bytes)!==part.key)throw Error("Semio path key is not valid Unicode");
   out.push(0);write(BigInt(bytes.length),out);for(const byte of bytes)out.push(byte);
  }else if(part.kind==="index"){out.push(1);write(integer(part.index),out);}
  else throw Error("Unknown Semio path segment");
 }
 return Uint8Array.from(out);
}

/** 📖️ Decodes exact UTF-8 keys while refusing truncated, overflowing, and unaddressable children. */
export function decodeSemioValuePath(bytes:Uint8Array):SemioValuePath{
 let offset=0;
 const read=():bigint=>{
  let value=0n;
  for(let index=0;index<10;index++){
   const byte=bytes[offset++];if(byte===undefined)throw Error("Truncated Semio path integer");
   if(index===9&&byte>1)throw Error("Semio path integer overflows u64");
   value|=BigInt(byte&127)<<BigInt(index*7);if(byte<128)return value;
  }
  throw Error("Semio path integer overflows u64");
 };
 const count=read();if(count>BigInt(Math.floor((bytes.length-offset)/2)))throw Error("Semio path count exceeds its original frame");
 const path:SemioValuePath=[];
 for(let index=0;index<Number(count);index++){
  const tag=bytes[offset++];
  if(tag===0){
   const length=read();if(length>BigInt(bytes.length-offset))throw Error("Truncated Semio path key");
   const end=offset+Number(length);path.push({kind:"key",key:decoder.decode(bytes.subarray(offset,end))});offset=end;
  }else if(tag===1){
   const value=read();if(value>BigInt(Number.MAX_SAFE_INTEGER))throw Error("Semio path index exceeds address space");path.push({kind:"index",index:Number(value)});
  }else throw Error("Unknown Semio path segment");
 }
 if(offset!==bytes.length)throw Error("Semio path frame has trailing bytes");
 return path;
}
