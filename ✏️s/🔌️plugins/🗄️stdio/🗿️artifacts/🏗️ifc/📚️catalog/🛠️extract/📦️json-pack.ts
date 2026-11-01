import {deflateRawSync,inflateRawSync} from "node:zlib";
/** 🧾️ A JSON value without floating point numbers: the closed subset every IFC neutral resource uses. */
export type PackJson=null|boolean|number|string|readonly PackJson[]|{readonly[key:string]:PackJson};
const MAGIC=[0x49,0x46,0x43,0x4a,1];
const NULL=0,FALSE=1,TRUE=2,UNSIGNED=3,NEGATIVE=4,STRING=5,ARRAY=6,OBJECT=7;
export const MAXIMUM_PACK_DEPTH=512;
class Writer{readonly bytes:number[]=[];varint(value:number):void{if(!Number.isSafeInteger(value)||value<0)throw Error("ifc json pack: unsigned integer expected");for(;value>=128;value=Math.floor(value/128))this.bytes.push(value%128|128);this.bytes.push(value);}byte(value:number):void{this.bytes.push(value);}}
class Reader{at=0;constructor(readonly bytes:Uint8Array){}byte():number{if(this.at>=this.bytes.length)throw Error("ifc json pack: truncated");return this.bytes[this.at++]!;}varint():number{let value=0,scale=1;for(;;){const b=this.byte();value+=(b&127)*scale;if(b<128)return value;scale*=128;if(scale>2**49)throw Error("ifc json pack: oversized integer");}}take(length:number):Uint8Array{if(this.at+length>this.bytes.length)throw Error("ifc json pack: truncated");const slice=this.bytes.subarray(this.at,this.at+length);this.at+=length;return slice;}}
/** 🥡️ Encode a JSON value into the language-neutral IFCJ container: interned strings in first-use order, LEB128 integers, object members in source order. */
export function encodeJsonPayload(root:PackJson):Uint8Array{
 const strings=new Map<string,number>(),body=new Writer();
 const str=(value:string):number=>{let id=strings.get(value);if(id===undefined){id=strings.size;strings.set(value,id);}return id;};
 const walk=(value:PackJson,depth:number):void=>{
  if(depth>MAXIMUM_PACK_DEPTH)throw Error("ifc json pack: depth");
  if(value===null){body.byte(NULL);return;}
  if(typeof value==="boolean"){body.byte(value?TRUE:FALSE);return;}
  if(typeof value==="number"){if(!Number.isSafeInteger(value))throw Error("ifc json pack: integer expected");if(value>=0){body.byte(UNSIGNED);body.varint(value);}else{body.byte(NEGATIVE);body.varint(-value-1);}return;}
  if(typeof value==="string"){body.byte(STRING);body.varint(str(value));return;}
  if(Array.isArray(value)){body.byte(ARRAY);body.varint(value.length);for(const item of value)walk(item,depth+1);return;}
  const entries=Object.entries(value as{readonly[key:string]:PackJson});body.byte(OBJECT);body.varint(entries.length);
  for(const[key,item]of entries){body.varint(str(key));walk(item,depth+1);}
 };
 walk(root,0);
 const head=new Writer(),encoder=new TextEncoder();MAGIC.forEach(b=>head.byte(b));head.varint(strings.size);
 for(const value of strings.keys()){const utf8=encoder.encode(value);head.varint(utf8.length);utf8.forEach(b=>head.byte(b));}
 return Uint8Array.from([...head.bytes,...body.bytes]);
}
/** 🗜️ Compress the IFCJ payload as one raw DEFLATE stream (RFC 1951) at maximum effort; every committed resource is exactly this output. */
export const encodeJsonPack=(root:PackJson):Uint8Array=>deflateRawSync(encodeJsonPayload(root),{level:9});
/** 🥠️ Independent TypeScript reader of the IFCJ payload, the oracle for the Rust reader. */
export function decodeJsonPayload(payload:Uint8Array):PackJson{
 const input=new Reader(payload),decoder=new TextDecoder("utf-8",{fatal:true});
 for(const b of MAGIC)if(input.byte()!==b)throw Error("ifc json pack: bad magic");
 const strings=Array.from({length:input.varint()},()=>decoder.decode(input.take(input.varint()))),str=(id:number):string=>strings[id]??(()=>{throw Error("ifc json pack: string index");})();
 const read=(depth:number):PackJson=>{
  if(depth>MAXIMUM_PACK_DEPTH)throw Error("ifc json pack: depth");
  switch(input.byte()){
   case NULL:return null;
   case FALSE:return false;
   case TRUE:return true;
   case UNSIGNED:return input.varint();
   case NEGATIVE:return -input.varint()-1;
   case STRING:return str(input.varint());
   case ARRAY:return Array.from({length:input.varint()},()=>read(depth+1));
   case OBJECT:{const result:Record<string,PackJson>={};for(let n=input.varint();n--;){const key=str(input.varint());result[key]=read(depth+1);}return result;}
   default:throw Error("ifc json pack: tag");
  }
 };
 const root=read(0);if(input.at!==payload.length)throw Error("ifc json pack: trailing bytes");return root;
}
/** 📤️ Inflate and decode a committed IFCJ resource. */
export const decodeJsonPack=(pack:Uint8Array):PackJson=>decodeJsonPayload(inflateRawSync(pack));
