import{isUtf8}from"node:buffer";
import{createHash}from"node:crypto";
import{createBoundedCacheV1}from"../../../🧊️bounded/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🧮️ The declared constants of the binary fact index; the schema file is the single source and spells the byte layout. */
export const FACT_CODEC_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
const P=FACT_CODEC_POLICY_V1,T=P.tags,MAGIC=Buffer.from(P.magic,"latin1"),HEX=/^[a-f0-9]{64}$/u,LONE_SURROGATE=/[\ud800-\udfff]/u,MAX_ID=4294967295,fail=message=>Error(message);

/** 🔤️ Orders strings by code point so every language reproduces the same canonical bytes. */
export function compareCodePointsV1(a,b){
 const length=Math.min(a.length,b.length);
 for(let index=0;index<length;){const x=a.codePointAt(index),y=b.codePointAt(index);if(x!==y)return x<y?-1:1;index+=x>0xffff?2:1;}
 return a.length===b.length?0:a.length<b.length?-1:1;
}

/** 🧾️ Serializes JSON with recursively code point sorted keys and no whitespace: the byte identity of every stored document and the identity of a fact. */
export function canonicalJsonV1(value){
 const ordered=object=>{let previous;for(const key of Object.keys(object)){if(previous!==undefined&&compareCodePointsV1(previous,key)>0)return false;previous=key;}return true;};
 const sorted=object=>{const out={};for(const key of Object.keys(object).sort(compareCodePointsV1))Object.defineProperty(out,key,{value:object[key],enumerable:true,writable:true,configurable:true});return out;};
 return JSON.stringify(value,(_,item)=>item!==null&&typeof item==="object"&&!Array.isArray(item)&&!ordered(item)?sorted(item):item);
}

/** 🪪️ The identity of a value: its canonical text when that is at most `limit` characters, else `#` and the SHA-256 of the canonical text, which is hashed in bounded pieces while it is written so that a fact of megabytes never exists as one string (the same answer as hashing `canonicalJsonV1`). */
export function canonicalKeyV1(value,limit){
 let pending="",length=0,hash;
 const emit=text=>{
  length+=text.length;
  if(hash===undefined&&length>limit){hash=createHash("sha256");}
  pending+=text;
  if(hash!==undefined&&pending.length>=65536){hash.update(pending,"utf8");pending="";}
 };
 const write=item=>{
  if(item!==null&&typeof item==="object"&&typeof item.toJSON==="function")item=item.toJSON();
  if(item===null||typeof item!=="object"){emit(JSON.stringify(item)??"null");return;}
  if(Array.isArray(item)){emit("[");for(let at=0;at<item.length;at++){if(at)emit(",");write(item[at]);}emit("]");return;}
  const keys=Object.keys(item).filter(key=>{const member=item[key];return member!==undefined&&typeof member!=="function"&&typeof member!=="symbol";}).sort(compareCodePointsV1);
  emit("{");keys.forEach((key,at)=>{if(at)emit(",");emit(JSON.stringify(key)+":");write(item[key]);});emit("}");
 };
 write(value);
 if(hash===undefined)return pending;
 hash.update(pending,"utf8");return"#"+hash.digest("hex");
}

/** ✍️ A growable byte sink that refuses to grow beyond the finite document limit; `size()` is the bytes written so far and `truncate(size)` takes the bytes back to an earlier size. */
export function createFactSinkV1(maximumBytes){
 let buffer=Buffer.allocUnsafe(1<<20),length=0;
 const room=count=>{
  const needed=length+count;if(needed>maximumBytes)throw fail("Stored document exceeds its finite limit ("+needed+" bytes expanded, limit "+maximumBytes+")");
  if(needed>buffer.length){const grown=Buffer.allocUnsafe(Math.min(Math.max(buffer.length*2,needed),maximumBytes));buffer.copy(grown,0,0,length);buffer=grown;}
 };
 return{
  byte(value){room(1);buffer[length++]=value;},
  varint(value){
   if(!Number.isSafeInteger(value)||value<0)throw fail("Unsigned integer required in a fact index");
   room(9);
   while(value>=128){buffer[length++]=value%128|128;value=Math.floor(value/128);}
   buffer[length++]=value;
  },
  bytes(source,from=0){const count=source.length-from;room(count);source.copy(buffer,length,from);length+=count;},
  size:()=>length,
  truncate(to){length=to;},
  result(){return buffer.subarray(0,length);},
 };
}

/** 🧵️ Adds to `strings` every text a fact spends in the table of strings: object keys and string values except the digests (`HEX` text is stored as 32 raw bytes). */
export function collectFactStringsV1(value,strings){
 if(typeof value==="string"){if(!HEX.test(value))strings.add(value);}
 else if(Array.isArray(value)){for(const item of value)collectFactStringsV1(item,strings);}
 else if(value!==null&&typeof value==="object"){for(const key of Object.keys(value))if(value[key]!==undefined&&typeof value[key]!=="function"&&typeof value[key]!=="symbol"){strings.add(key);collectFactStringsV1(value[key],strings);}}
}

/** 🖋️ Writes one fact as tagged values into `sink`; `textId` names the rank of a text in the table of strings, and object keys are written in the order of their ranks (the order of the code points). */
export function writeFactValueV1(sink,value,textId,depth){
 const writeId=id=>{if(!(id<=MAX_ID))throw fail("Bounded fact index ids required");sink.varint(id);};
 if(value===null||value===undefined||typeof value==="function"||typeof value==="symbol"){sink.byte(T.null);return;}
 if(typeof value==="boolean"){sink.byte(value?T.true:T.false);return;}
 if(typeof value==="number"){
  if(!Number.isFinite(value)){sink.byte(T.null);return;}
  if(Number.isSafeInteger(value)){if(value>=0||Object.is(value,-0)){sink.byte(T.uint);sink.varint(Math.abs(value));}else{sink.byte(T.negint);sink.varint(-value);}return;}
  const raw=Buffer.allocUnsafe(8);raw.writeDoubleLE(value);sink.byte(T.float);sink.bytes(raw);return;
 }
 if(typeof value==="string"){if(HEX.test(value)){sink.byte(T.hex);sink.bytes(Buffer.from(value,"hex"));}else{sink.byte(T.string);writeId(textId(value));}return;}
 if(typeof value==="object"&&depth>P.maximumDepth)throw fail("Bounded fact depth required");
 if(Array.isArray(value)){sink.byte(T.array);sink.varint(value.length);for(const item of value)writeFactValueV1(sink,item,textId,depth+1);return;}
 if(typeof value==="bigint")throw fail("JSON data required in a fact index");
 const keys=Object.keys(value).filter(key=>value[key]!==undefined&&typeof value[key]!=="function"&&typeof value[key]!=="symbol").sort((a,b)=>textId(a)-textId(b));
 sink.byte(T.object);sink.varint(keys.length);for(const key of keys){writeId(textId(key));writeFactValueV1(sink,value[key],textId,depth+1);}
}

/** 🖊️ Writes one list of row or owner numbers: its length, then the zigzag deltas of its numbers. */
export function writeFactListV1(sink,list){
 sink.varint(list.length);let previousId=0;
 for(const id of list){if(!Number.isSafeInteger(id)||id<0||id>MAX_ID)throw fail("Bounded fact index ids required");const delta=id-previousId;sink.varint(delta>=0?delta*2:-delta*2-1);previousId=id;}
}

/** 🧱️ Encodes the logical fact index (`{version,owners,rows,classes}`) into the binary document the schema file describes; the output is canonical: the same logical index yields the same bytes. */
export function encodeFactIndexV1(index,{maximumBytes}){
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1)throw fail("Finite generation document byte limit required");
 if(index?.version!==1||!Array.isArray(index.owners)||!Array.isArray(index.rows)||!Array.isArray(index.classes))throw fail("Closed fact index required");
 const strings=new Set(),kindIds=new Map(),kinds=[],factByObject=new Map(),factByText=new Map(),facts=[],rowKind=new Uint32Array(index.rows.length),rowFact=new Uint32Array(index.rows.length);
 for(const owner of index.owners){if(typeof owner!=="string")throw fail("Owner names must be text");strings.add(owner);}
 for(let at=0;at<index.rows.length;at++){
  const row=index.rows[at];
  if(typeof row?.kind!=="string"||!row.kind||typeof row.path!=="string"||!("fact" in row))throw fail("Closed fact row required");
  let kind=kindIds.get(row.kind);if(kind===undefined){kind=kinds.length;kindIds.set(row.kind,kind);kinds.push(row.kind);strings.add(row.kind);}
  rowKind[at]=kind;strings.add(row.path);
  let id=factByObject.get(row.fact);
  if(id===undefined||typeof row.fact!=="object"||row.fact===null){
   const text=canonicalJsonV1(row.fact)??"null";id=factByText.get(text);
   if(id===undefined){id=facts.length;factByText.set(text,id);facts.push(row.fact);collectFactStringsV1(row.fact,strings);}
   if(typeof row.fact==="object"&&row.fact!==null)factByObject.set(row.fact,id);
  }
  rowFact[at]=id;
 }
 const table=[...strings].map(text=>{if(LONE_SURROGATE.test(text))throw fail("Well formed text required in a fact index: "+JSON.stringify(text.slice(0,64)));return{text,bytes:Buffer.from(text,"utf8")};}).sort((a,b)=>Buffer.compare(a.bytes,b.bytes)),ids=new Map(table.map((entry,rank)=>[entry.text,rank]));
 const sink=createFactSinkV1(maximumBytes),writeId=value=>{if(value>MAX_ID)throw fail("Bounded fact index ids required");sink.varint(value);};
 sink.bytes(MAGIC);
 for(const count of[table.length,kinds.length,index.owners.length,facts.length,index.rows.length,index.classes.length])sink.varint(count);
 let previous=Buffer.alloc(0);
 for(const{bytes}of table){
  const limit=Math.min(previous.length,bytes.length);let lcp=0;while(lcp<limit&&previous[lcp]===bytes[lcp])lcp++;
  sink.varint(lcp);sink.varint(bytes.length-lcp);sink.bytes(bytes,lcp);previous=bytes;
 }
 for(const kind of kinds)sink.varint(ids.get(kind));
 for(const owner of index.owners)sink.varint(ids.get(owner));
 const textId=text=>ids.get(text);
 for(const fact of facts)writeFactValueV1(sink,fact,textId,1);
 for(let at=0;at<index.rows.length;at++){sink.varint(rowKind[at]);writeId(ids.get(index.rows[at].path));writeId(rowFact[at]);}
 for(const cls of index.classes){writeFactListV1(sink,cls.r);writeFactListV1(sink,cls.o);}
 return sink.result();
}

/**
 * 🔬️ Reads a binary fact index from `bytes`: every count is bounded by the bytes that remain, every id by its table, strings are strictly ascending well formed UTF-8, nesting is bounded and no byte may trail. `share` maps decoded strings to shared instances. With `build:false` nothing but the table strings is materialized (`strings` lists them): the validating scan the storage closure reader runs.
 */
export function decodeFactIndexV1(bytes,{maximumBytes=bytes.length,share=text=>text,build=true}={}){
 if(!(bytes instanceof Uint8Array)||bytes.length>maximumBytes)throw fail("Bounded fact index bytes required");
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.length),decoder=new TextDecoder("utf-8",{fatal:true});let at=0;
 const refuse=()=>{throw fail("Closed binary fact index required");};
 const varint=()=>{
  let value=0,scale=1;
  for(let count=0;count<P.maximumVarintBytes;count++){
   if(at>=buffer.length)refuse();
   const byte=buffer[at++];value+=(byte&127)*scale;
   if(byte<128){if(!Number.isSafeInteger(value))refuse();return value;}
   scale*=128;
  }
  return refuse();
 };
 const count=()=>{const value=varint();if(value>buffer.length-at)refuse();return value;};
 if(buffer.length<MAGIC.length||Buffer.compare(buffer.subarray(0,MAGIC.length),MAGIC)!==0)refuse();
 at=MAGIC.length;
 const stringCount=count(),kindCount=count(),ownerCount=count(),factCount=count(),rowCount=count(),classCount=count(),table=new Array(stringCount);
 let current=Buffer.allocUnsafe(256),currentLength=0;
 for(let rank=0;rank<stringCount;rank++){
  const lcp=varint(),suffix=varint();
  if(lcp>currentLength||suffix>buffer.length-at)refuse();
  if(rank===0?lcp!==0:lcp===currentLength?suffix===0:suffix===0||buffer[at]<=current[lcp])refuse();
  const length=lcp+suffix;
  if(length>current.length){const grown=Buffer.allocUnsafe(Math.max(length,current.length*2));current.copy(grown,0,0,currentLength);current=grown;}
  buffer.copy(current,lcp,at,at+suffix);at+=suffix;currentLength=length;
  let text;try{text=decoder.decode(current.subarray(0,length));}catch{refuse();}
  table[rank]=share(text);
 }
 const stringAt=()=>{const id=varint();if(id>=stringCount)refuse();return table[id];};
 if(!build){
  const skipValue=depth=>{
   if(at>=buffer.length)refuse();
   const tag=buffer[at++];
   if((tag===T.array||tag===T.object)&&depth>P.maximumDepth)refuse();
   switch(tag){
    case T.null:case T.false:case T.true:return;
    case T.uint:varint();return;
    case T.negint:if(varint()===0)refuse();return;
    case T.float:if(buffer.length-at<8||!Number.isFinite(buffer.readDoubleLE(at)))refuse();at+=8;return;
    case T.string:if(varint()>=stringCount)refuse();return;
    case T.hex:if(buffer.length-at<32)refuse();at+=32;return;
    case T.array:{const items=count();for(let item=0;item<items;item++)skipValue(depth+1);return;}
    case T.object:{const items=count();let previous=-1;for(let item=0;item<items;item++){const key=varint();if(key>=stringCount||key<=previous)refuse();previous=key;skipValue(depth+1);}return;}
    default:refuse();
   }
  };
  for(let kind=0;kind<kindCount;kind++)stringAt();
  for(let owner=0;owner<ownerCount;owner++)stringAt();
  for(let fact=0;fact<factCount;fact++)skipValue(1);
  for(let row=0;row<rowCount;row++){if(varint()>=kindCount)refuse();stringAt();if(varint()>=factCount)refuse();}
  for(let cls=0;cls<classCount;cls++)for(const limit of[rowCount,ownerCount]){const items=count();let previous=0;for(let item=0;item<items;item++){const zigzag=varint(),delta=zigzag%2?-(zigzag+1)/2:zigzag/2;previous+=delta;if(previous<0||previous>=limit)refuse();}}
  if(at!==buffer.length)refuse();
  return{strings:table};
 }
 const kinds=new Array(kindCount);for(let kind=0;kind<kindCount;kind++)kinds[kind]=stringAt();
 const owners=new Array(ownerCount);for(let owner=0;owner<ownerCount;owner++)owners[owner]=stringAt();
 const readValue=depth=>{
  if(at>=buffer.length)refuse();
  const tag=buffer[at++];
  if((tag===T.array||tag===T.object)&&depth>P.maximumDepth)refuse();
  switch(tag){
   case T.null:return null;
   case T.false:return false;
   case T.true:return true;
   case T.uint:return varint();
   case T.negint:{const magnitude=varint();if(magnitude===0)refuse();return-magnitude;}
   case T.float:{if(buffer.length-at<8)refuse();const value=buffer.readDoubleLE(at);at+=8;if(!Number.isFinite(value))refuse();return value;}
   case T.string:return stringAt();
   case T.hex:{if(buffer.length-at<32)refuse();const text=buffer.toString("hex",at,at+32);at+=32;return share(text);}
   case T.array:{const items=count(),list=new Array(items);for(let item=0;item<items;item++)list[item]=readValue(depth+1);return list;}
   case T.object:{const items=count(),out={};let previous=-1;for(let item=0;item<items;item++){const key=varint();if(key>=stringCount||key<=previous)refuse();previous=key;const name=table[key],value=readValue(depth+1);if(name==="__proto__")Object.defineProperty(out,name,{value,enumerable:true,writable:true,configurable:true});else out[name]=value;}return out;}
   default:return refuse();
  }
 };
 const facts=new Array(factCount);for(let fact=0;fact<factCount;fact++)facts[fact]=readValue(1);
 const rows=new Array(rowCount);
 for(let row=0;row<rowCount;row++){
  const kind=varint();if(kind>=kindCount)refuse();
  const path=stringAt(),fact=varint();if(fact>=factCount)refuse();
  rows[row]={fact:facts[fact],kind:kinds[kind],path};
 }
 const classes=new Array(classCount);
 const readList=limit=>{
  const items=count(),list=new Array(items);let previous=0;
  for(let item=0;item<items;item++){const zigzag=varint();previous+=zigzag%2?-(zigzag+1)/2:zigzag/2;if(previous<0||previous>=limit)refuse();list[item]=previous;}
  return list;
 };
 for(let cls=0;cls<classCount;cls++){const r=readList(rowCount),o=readList(ownerCount);classes[cls]={r,o};}
 if(at!==buffer.length)refuse();
 return{version:1,owners,rows,classes};
}

/**
 * 🪶️ Opens a binary fact index as a bounded view: the document stays one packed buffer, strings, facts, rows and class lists are decoded when they are read and live in bounded least recently used caches (`🧊️bounded`, sizes in the schema policy `view`), so the resident size is the document plus small offset tables whatever is read. Every bound the full decoder checks is checked here, including the well formed UTF-8 of every string. `owners` is a plain array; `rows` and `classes` are read only array views (`length`, index, iteration and the array methods work, nothing is stored when they are walked); `rowKindOf(id)` answers a kind without reading the row; `document` names the offset tables of the packed bytes for a streaming reader of the same layout (`../🌊️stream-patch`); `stats()` lists the caches.
 * [bounded caches](../../../🧊️bounded/🟨️.mjs)
 */
export function openFactIndexV1(bytes,{maximumBytes=bytes.length,share=text=>text,name}={}){
 if(!(bytes instanceof Uint8Array)||bytes.length>maximumBytes)throw fail("Bounded fact index bytes required");
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.length),decoder=new TextDecoder("utf-8",{fatal:true}),V=P.view;let at=0;
 const refuse=()=>{throw fail("Closed binary fact index required");};
 const varint=()=>{
  let value=0,scale=1;
  for(let count=0;count<P.maximumVarintBytes;count++){
   if(at>=buffer.length)refuse();
   const byte=buffer[at++];value+=(byte&127)*scale;
   if(byte<128){if(!Number.isSafeInteger(value))refuse();return value;}
   scale*=128;
  }
  return refuse();
 };
 const count=()=>{const value=varint();if(value>buffer.length-at)refuse();return value;};
 if(buffer.length<MAGIC.length||Buffer.compare(buffer.subarray(0,MAGIC.length),MAGIC)!==0)refuse();
 at=MAGIC.length;
 const stringCount=count(),kindCount=count(),ownerCount=count(),factCount=count(),rowCount=count(),classCount=count();
 const lcps=new Uint32Array(stringCount),suffixAt=new Uint32Array(stringCount),suffixLength=new Uint32Array(stringCount),shorter=new Int32Array(stringCount),open=[];let current=Buffer.allocUnsafe(256),currentLength=0;
 for(let rank=0;rank<stringCount;rank++){
  const lcp=varint(),suffix=varint();
  if(lcp>currentLength||suffix>buffer.length-at)refuse();
  if(rank===0?lcp!==0:lcp===currentLength?suffix===0:suffix===0||buffer[at]<=current[lcp])refuse();
  const length=lcp+suffix;
  if(length>current.length){const grown=Buffer.allocUnsafe(Math.max(length,current.length*2));current.copy(grown,0,0,currentLength);current=grown;}
  buffer.copy(current,lcp,at,at+suffix);if(!isUtf8(current.subarray(0,length)))refuse();lcps[rank]=lcp;while(open.length&&lcps[open[open.length-1]]>=lcp)open.pop();shorter[rank]=open.length?open[open.length-1]:-1;open.push(rank);suffixAt[rank]=at;suffixLength[rank]=suffix;at+=suffix;currentLength=length;
 }
 const prefix=name??"fact-index",texts=createBoundedCacheV1({name:prefix+":texts",maxEntries:V.textEntries,maxBytes:V.textBytes,sizeOf:text=>text.length*2+40}),
  factCache=createBoundedCacheV1({name:prefix+":facts",maxEntries:V.factEntries,maxBytes:V.factBytes,sizeOf:entry=>entry.size}),
  rowCache=createBoundedCacheV1({name:prefix+":rows",maxEntries:V.rowEntries,maxBytes:V.rowBytes,sizeOf:row=>row.path.length*2+96}),
  classCache=createBoundedCacheV1({name:prefix+":classes",maxEntries:V.classEntries,maxBytes:V.classBytes,sizeOf:cls=>(cls.r.length+cls.o.length)*8+96});
 const textAt=(id,remember=true)=>{
  let text=remember?texts.get(id):undefined;
  if(text===undefined){
   const parts=[buffer.subarray(suffixAt[id],suffixAt[id]+suffixLength[id])];
   for(let need=lcps[id],back=shorter[id];need>0;back=shorter[back]){const lcp=lcps[back];parts.push(buffer.subarray(suffixAt[back],suffixAt[back]+need-lcp));need=lcp;}
   try{text=decoder.decode(parts.length===1?parts[0]:Buffer.concat(parts.reverse()));}catch{refuse();}
   if(remember){text=share(text);texts.set(id,text);}
  }
  return text;
 };
 const stringId=()=>{const id=varint();if(id>=stringCount)refuse();return id;};
 const kinds=new Array(kindCount),kindString=new Uint32Array(kindCount);for(let kind=0;kind<kindCount;kind++){kindString[kind]=stringId();kinds[kind]=textAt(kindString[kind],false);}
 const owners=new Array(ownerCount),ownerString=new Uint32Array(ownerCount);for(let owner=0;owner<ownerCount;owner++){ownerString[owner]=stringId();owners[owner]=textAt(ownerString[owner],false);}
 const skipValue=depth=>{
  if(at>=buffer.length)refuse();
  const tag=buffer[at++];
  if((tag===T.array||tag===T.object)&&depth>P.maximumDepth)refuse();
  switch(tag){
   case T.null:case T.false:case T.true:return;
   case T.uint:varint();return;
   case T.negint:if(varint()===0)refuse();return;
   case T.float:if(buffer.length-at<8||!Number.isFinite(buffer.readDoubleLE(at)))refuse();at+=8;return;
   case T.string:if(varint()>=stringCount)refuse();return;
   case T.hex:if(buffer.length-at<32)refuse();at+=32;return;
   case T.array:{const items=count();for(let item=0;item<items;item++)skipValue(depth+1);return;}
   case T.object:{const items=count();let previous=-1;for(let item=0;item<items;item++){const key=varint();if(key>=stringCount||key<=previous)refuse();previous=key;skipValue(depth+1);}return;}
   default:refuse();
  }
 };
 const factAt=new Uint32Array(factCount);
 for(let fact=0;fact<factCount;fact++){factAt[fact]=at;skipValue(1);}
 const factsEnd=at;
 const rowKind=new Uint16Array(rowCount),rowPath=new Uint32Array(rowCount),rowFact=new Uint32Array(rowCount);
 for(let row=0;row<rowCount;row++){const kind=varint();if(kind>=kindCount)refuse();rowKind[row]=kind;rowPath[row]=stringId();const fact=varint();if(fact>=factCount)refuse();rowFact[row]=fact;}
 const listAt=new Uint32Array(classCount*2);
 const skipList=limit=>{const items=count();let previous=0;for(let item=0;item<items;item++){const zigzag=varint();previous+=zigzag%2?-(zigzag+1)/2:zigzag/2;if(previous<0||previous>=limit)refuse();}};
 for(let cls=0;cls<classCount;cls++){listAt[cls*2]=at;skipList(rowCount);listAt[cls*2+1]=at;skipList(ownerCount);}
 if(at!==buffer.length)refuse();
 const readList=start=>{
  let position=start,value=0,scale=1;
  const next=()=>{value=0;scale=1;for(;;){const byte=buffer[position++];value+=(byte&127)*scale;if(byte<128)return value;scale*=128;}};
  const items=next(),list=new Array(items);let previous=0;
  for(let item=0;item<items;item++){const zigzag=next();previous+=zigzag%2?-(zigzag+1)/2:zigzag/2;list[item]=previous;}
  return list;
 };
 const readTyped=start=>{
  let position=start,value=0,scale=1;
  const next=()=>{value=0;scale=1;for(;;){const byte=buffer[position++];value+=(byte&127)*scale;if(byte<128)return value;scale*=128;}};
  const items=next(),list=new Uint32Array(items);let previous=0;
  for(let item=0;item<items;item++){const zigzag=next();previous+=zigzag%2?-(zigzag+1)/2:zigzag/2;list[item]=previous;}
  return list;
 };
 const readValue=depth=>{
  const tag=buffer[at++];
  switch(tag){
   case T.null:return null;
   case T.false:return false;
   case T.true:return true;
   case T.uint:return varint();
   case T.negint:return-varint();
   case T.float:{const value=buffer.readDoubleLE(at);at+=8;return value;}
   case T.string:return textAt(varint());
   case T.hex:{const text=buffer.toString("hex",at,at+32);at+=32;return share(text);}
   case T.array:{const items=varint(),list=new Array(items);for(let item=0;item<items;item++)list[item]=readValue(depth+1);return list;}
   default:{const items=varint(),out={};for(let item=0;item<items;item++){const key=textAt(varint()),value=readValue(depth+1);if(key==="__proto__")Object.defineProperty(out,key,{value,enumerable:true,writable:true,configurable:true});else out[key]=value;}return out;}
  }
 };
 const seen=new Uint8Array(rowCount);let read=0;
 const factOf=id=>{
  let entry=factCache.get(id);
  if(entry===undefined){const resume=at,start=factAt[id];at=start;try{entry={fact:readValue(1),size:0};entry.size=(at-start)*V.factBytesPerStored+64;}finally{at=resume;}factCache.set(id,entry);}
  return entry.fact;
 };
 const rowAt=id=>{
  let row=rowCache.get(id);
  if(row===undefined){row={fact:factOf(rowFact[id]),kind:kinds[rowKind[id]],path:textAt(rowPath[id],false)};rowCache.set(id,row);}
  if(!seen[id]){seen[id]=1;read++;}
  return row;
 };
 const classAt=id=>{
  let cls=classCache.get(id);
  if(cls===undefined){cls={r:readList(listAt[id*2]),o:readList(listAt[id*2+1])};classCache.set(id,cls);}
  return cls;
 };
 const view=(length,item)=>new Proxy(new Array(length),{
  get(target,key,receiver){
   if(typeof key==="string"){const id=Number(key);if(Number.isInteger(id)&&id>=0&&id<length&&String(id)===key)return item(id);}
   return Reflect.get(target,key,receiver);
  },
  has(target,key){if(typeof key==="string"){const id=Number(key);if(Number.isInteger(id)&&id>=0&&id<length&&String(id)===key)return true;}return Reflect.has(target,key);},
  ownKeys(target){const keys=[];for(let id=0;id<length;id++)keys.push(String(id));keys.push("length");return keys;},
  getOwnPropertyDescriptor(target,key){
   if(typeof key==="string"){const id=Number(key);if(Number.isInteger(id)&&id>=0&&id<length&&String(id)===key)return{value:item(id),enumerable:true,writable:false,configurable:true};}
   return Reflect.getOwnPropertyDescriptor(target,key);
  },
  set(){return false;},
  deleteProperty(){return false;},
  defineProperty(){return false;},
 });
 const caches=[texts,factCache,rowCache,classCache];
 const document=Object.freeze({classTyped:id=>({r:readTyped(listAt[id*2]),o:readTyped(listAt[id*2+1])}),buffer,stringCount,kinds,kindString,ownerString,factCount,factAt,factsEnd,rowCount,rowKind,rowPath,rowFact,classCount,suffixAt,suffixLength,lcps,textAt});
 return{version:1,owners,rows:view(rowCount,rowAt),classes:view(classCount,classAt),rowKindOf:id=>kinds[rowKind[id]],document,rowsRead:()=>read,stats:()=>({bytes:buffer.length,caches:caches.map(cache=>cache.stats())}),drop:()=>{for(const cache of caches)cache.clear();}};
}
