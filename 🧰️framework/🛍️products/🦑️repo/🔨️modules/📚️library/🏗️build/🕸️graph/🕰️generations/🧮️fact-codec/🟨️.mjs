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
 const walk=item=>{if(item===null||typeof item!=="object")return item;if(Array.isArray(item))return item.map(walk);const out={};for(const key of Object.keys(item).sort(compareCodePointsV1))Object.defineProperty(out,key,{value:walk(item[key]),enumerable:true,writable:true,configurable:true});return out;};
 return JSON.stringify(walk(value));
}

/** ✍️ A growable byte sink that refuses to grow beyond the finite document limit. */
function createSink(maximumBytes){
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
  result(){return buffer.subarray(0,length);},
 };
}

/** 🧱️ Encodes the logical fact index (`{version,owners,rows,classes}`) into the binary document the schema file describes; the output is canonical: the same logical index yields the same bytes. */
export function encodeFactIndexV1(index,{maximumBytes}){
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1)throw fail("Finite generation document byte limit required");
 if(index?.version!==1||!Array.isArray(index.owners)||!Array.isArray(index.rows)||!Array.isArray(index.classes))throw fail("Closed fact index required");
 const strings=new Set(),kindIds=new Map(),kinds=[],factByObject=new Map(),factByText=new Map(),facts=[],rowKind=new Uint32Array(index.rows.length),rowFact=new Uint32Array(index.rows.length);
 const collect=value=>{
  if(typeof value==="string"){if(!HEX.test(value))strings.add(value);}
  else if(Array.isArray(value)){for(const item of value)collect(item);}
  else if(value!==null&&typeof value==="object"){for(const key of Object.keys(value))if(value[key]!==undefined&&typeof value[key]!=="function"&&typeof value[key]!=="symbol"){strings.add(key);collect(value[key]);}}
 };
 for(const owner of index.owners){if(typeof owner!=="string")throw fail("Owner names must be text");strings.add(owner);}
 for(let at=0;at<index.rows.length;at++){
  const row=index.rows[at];
  if(typeof row?.kind!=="string"||!row.kind||typeof row.path!=="string"||!("fact" in row))throw fail("Closed fact row required");
  let kind=kindIds.get(row.kind);if(kind===undefined){kind=kinds.length;kindIds.set(row.kind,kind);kinds.push(row.kind);strings.add(row.kind);}
  rowKind[at]=kind;strings.add(row.path);
  let id=factByObject.get(row.fact);
  if(id===undefined||typeof row.fact!=="object"||row.fact===null){
   const text=canonicalJsonV1(row.fact)??"null";id=factByText.get(text);
   if(id===undefined){id=facts.length;factByText.set(text,id);facts.push(row.fact);collect(row.fact);}
   if(typeof row.fact==="object"&&row.fact!==null)factByObject.set(row.fact,id);
  }
  rowFact[at]=id;
 }
 const table=[...strings].map(text=>{if(LONE_SURROGATE.test(text))throw fail("Well formed text required in a fact index: "+JSON.stringify(text.slice(0,64)));return{text,bytes:Buffer.from(text,"utf8")};}).sort((a,b)=>Buffer.compare(a.bytes,b.bytes)),ids=new Map(table.map((entry,rank)=>[entry.text,rank]));
 const sink=createSink(maximumBytes),writeId=value=>{if(value>MAX_ID)throw fail("Bounded fact index ids required");sink.varint(value);};
 sink.bytes(MAGIC);
 for(const count of[table.length,kinds.length,index.owners.length,facts.length,index.rows.length,index.classes.length])sink.varint(count);
 let previous=Buffer.alloc(0);
 for(const{bytes}of table){
  const limit=Math.min(previous.length,bytes.length);let lcp=0;while(lcp<limit&&previous[lcp]===bytes[lcp])lcp++;
  sink.varint(lcp);sink.varint(bytes.length-lcp);sink.bytes(bytes,lcp);previous=bytes;
 }
 for(const kind of kinds)sink.varint(ids.get(kind));
 for(const owner of index.owners)sink.varint(ids.get(owner));
 const writeValue=(value,depth)=>{
   if(value===null||value===undefined||typeof value==="function"||typeof value==="symbol"){sink.byte(T.null);return;}
  if(typeof value==="boolean"){sink.byte(value?T.true:T.false);return;}
  if(typeof value==="number"){
   if(!Number.isFinite(value)){sink.byte(T.null);return;}
   if(Number.isSafeInteger(value)){if(value>=0||Object.is(value,-0)){sink.byte(T.uint);sink.varint(Math.abs(value));}else{sink.byte(T.negint);sink.varint(-value);}return;}
   const raw=Buffer.allocUnsafe(8);raw.writeDoubleLE(value);sink.byte(T.float);sink.bytes(raw);return;
  }
  if(typeof value==="string"){if(HEX.test(value)){sink.byte(T.hex);sink.bytes(Buffer.from(value,"hex"));}else{sink.byte(T.string);writeId(ids.get(value));}return;}
  if(typeof value==="object"&&depth>P.maximumDepth)throw fail("Bounded fact depth required");
  if(Array.isArray(value)){sink.byte(T.array);sink.varint(value.length);for(const item of value)writeValue(item,depth+1);return;}
  if(typeof value==="bigint")throw fail("JSON data required in a fact index");
  const keys=Object.keys(value).filter(key=>value[key]!==undefined&&typeof value[key]!=="function"&&typeof value[key]!=="symbol").sort((a,b)=>ids.get(a)-ids.get(b));
  sink.byte(T.object);sink.varint(keys.length);for(const key of keys){writeId(ids.get(key));writeValue(value[key],depth+1);}
 };
 for(const fact of facts)writeValue(fact,1);
 for(let at=0;at<index.rows.length;at++){sink.varint(rowKind[at]);writeId(ids.get(index.rows[at].path));writeId(rowFact[at]);}
 const writeList=list=>{
  sink.varint(list.length);let previousId=0;
  for(const id of list){if(!Number.isSafeInteger(id)||id<0||id>MAX_ID)throw fail("Bounded fact index ids required");const delta=id-previousId;sink.varint(delta>=0?delta*2:-delta*2-1);previousId=id;}
 };
 for(const cls of index.classes){writeList(cls.r);writeList(cls.o);}
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
