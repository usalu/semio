import{FACT_CODEC_POLICY_V1,collectFactStringsV1,compareCodePointsV1,createFactSinkV1,openFactIndexV1,encodeFactIndexV1,writeFactListV1,writeFactValueV1}from"../🧮️fact-codec/🟨️.mjs";
import{classesOfChunksV1,streamUpsertClassesV1}from"../🧱️chunk-classes/🟨️.mjs";
import{INDEX_CHUNK_POLICY_V1}from"../🧱️chunk-classes/🟨️.mjs";
import{rowKeyV1}from"../🧮️fact-rows/🟨️.mjs";
import{P,createYielderV1}from"../🧱️guards/🟨️.mjs";
import{createHash}from"node:crypto";
import{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

const T=FACT_CODEC_POLICY_V1.tags,MAGIC=Buffer.from(FACT_CODEC_POLICY_V1.magic,"latin1"),LONE_SURROGATE=/[\ud800-\udfff]/u,fail=message=>Error(message);

/** 🍱️ The row batches of one upsert source: a source that knows its batches (`batches()`, an async iterable of arrays, so its rows can be loaded where they are consumed) or any array or iterable of rows as one batch. */
const batchesOf=source=>source!==null&&typeof source==="object"&&typeof source.batches==="function"?source.batches():(async function*(){yield Array.isArray(source)?source:[...source];})();

/** 🔗️ The row numbers of the index a row log list holds, in batches of `Uint32Array`: a log row is parsed and keyed once, the first time any list names it (`memo` maps log numbers to index row numbers for one patch), so the thousands of derived facts that share one closure cost an array lookup per row, never a parse and a key; a source without numbers goes through `batchesOf` and its rows through `idOfRow`. The rows the list carries beyond the log (its tails) are batches of rows. */
async function*numberedBatchesV1(source,memos,idOfRow,pause){
 const log=source.log,rows=log.size();let memo=memos.get(log);
 if(memo===undefined||memo.length<rows){const grown=new Int32Array(rows).fill(-1);if(memo!==undefined)grown.set(memo);memos.set(log,memo=grown);}
 const ids=source.ids,step=ROW_BATCH;
 for(let at=0;at<ids.length;at+=step){
  const slice=ids.subarray(at,Math.min(ids.length,at+step)),unknown=[];
  for(const id of slice)if(memo[id]<0)unknown.push(id);
  if(unknown.length){const found=log.read(Uint32Array.from(unknown));for(let item=0;item<unknown.length;item++)memo[unknown[item]]=idOfRow(found[item]);}
  const mapped=new Uint32Array(slice.length);for(let item=0;item<slice.length;item++)mapped[item]=memo[slice[item]];
  yield mapped;await pause?.();
 }
 for(const tail of source.tails)yield tail;
}
const ROW_BATCH=2048;

/** 🫙️ The empty fact index as an opened view: the base of a cold build, so a cold build and a patch are one code path. */
export function emptyFactIndexViewV1(){return openFactIndexV1(encodeFactIndexV1({version:1,owners:[],rows:[],classes:[]},{maximumBytes:64}));}

/** 🧶️ Merges the classes of an index into the coarsest exact partition over typed lists: rows that sit in the same classes share one group, groups with equal consumers merge (`classesOfChunksV1` over the classes as runs), and an owner without any row keeps one empty class. */
function refineClassesV1(classes,ownerCount,rowCount){
 const merged=classesOfChunksV1(rowCount,{chunks:classes.map(cls=>cls.r),holders:classes.map(cls=>cls.o)},{typed:true}),covered=new Uint8Array(ownerCount);
 for(const cls of merged)for(const owner of cls.o)covered[owner]=1;
 for(const cls of classes)for(const owner of cls.o)if(!covered[owner]){covered[owner]=1;merged.push({r:[],o:[owner]});}
 return merged;
}

/** 🪡️ Reads the facts of a packed index in place: `scan` visits the strings of one fact, `copy` writes one fact to a sink with its strings renumbered. */
function factReaderV1(buffer){
 let at=0;
 const varint=()=>{let value=0,scale=1;for(;;){const byte=buffer[at++];value+=(byte&127)*scale;if(byte<128)return value;scale*=128;}};
 const scan=(visit,depth)=>{
  switch(buffer[at++]){
   case T.null:case T.false:case T.true:return;
   case T.uint:case T.negint:varint();return;
   case T.float:at+=8;return;
   case T.string:visit(varint());return;
   case T.hex:at+=32;return;
   case T.array:{const items=varint();for(let item=0;item<items;item++)scan(visit,depth+1);return;}
   default:{const items=varint();for(let item=0;item<items;item++){visit(varint());scan(visit,depth+1);}}
  }
 };
 const copy=(sink,remap,depth)=>{
  const tag=buffer[at++];sink.byte(tag);
  switch(tag){
   case T.null:case T.false:case T.true:return;
   case T.uint:case T.negint:sink.varint(varint());return;
   case T.float:sink.bytes(buffer.subarray(at,at+8));at+=8;return;
   case T.string:sink.varint(remap[varint()]);return;
   case T.hex:sink.bytes(buffer.subarray(at,at+32));at+=32;return;
   case T.array:{const items=varint();sink.varint(items);for(let item=0;item<items;item++)copy(sink,remap,depth+1);return;}
   default:{const items=varint();sink.varint(items);for(let item=0;item<items;item++){sink.varint(remap[varint()]);copy(sink,remap,depth+1);}}
  }
 };
 return{scan:(start,visit)=>{at=start;scan(visit,1);},copy:(start,sink,remap)=>{at=start;copy(sink,remap,1);}};
}

/**
 * 🌊️ Patches a packed fact index without ever decoding it: `view` is an opened index (`openFactIndexV1`), the answer the packed bytes of the patched index. The classes of the owners that stay are read one by one from the view and kept as typed lists; the upserted owners arrive as row batches (an array, or an object with `batches()` that loads its rows where they are consumed) and become runs of rows shared by owners; the strings, facts and rows that stay are copied from the old bytes with renumbered strings and only the rows, facts and strings that are new are encoded. The result is canonical: the same logical index gives the same bytes as `encodeFactIndexV1`, so the peak is the old bytes, the new bytes and four bytes per class entry whatever the number of owners and rows.
 * [layout](../🧮️fact-codec/🧬️schema/🔣️.json)
 */
export async function patchFactIndexBytesV1(view,{drop=[],upserts=new Map(),append=new Map()},{maximumBytes=P.maximumIndexBytes,signal}={}){
 const D=view.document,reader=factReaderV1(D.buffer),pause=createYielderV1(),ownerIds=new Map(view.owners.map((name,id)=>[name,id])),owners=view.owners.slice(),oldRows=D.rowCount;
 const idOf=name=>{let id=ownerIds.get(name);if(id===undefined){id=owners.length;owners.push(name);ownerIds.set(name,id);}return id;};
 const touched=new Set(),gone=new Set();
 for(const name of drop)if(ownerIds.has(name)){touched.add(ownerIds.get(name));if(!upserts.has(name))gone.add(ownerIds.get(name));}
 for(const name of upserts.keys())touched.add(idOf(name));
 const classes=[];
 for(let at=0;at<D.classCount;at++){
  signal?.throwIfAborted();await pause();
  const cls=D.classTyped(at);let kept=0;for(const id of cls.o)if(!touched.has(id))kept++;
  if(!kept)continue;
  if(kept===cls.o.length){classes.push(cls);continue;}
  const o=new Uint32Array(kept);let fill=0;for(const id of cls.o)if(!touched.has(id))o[fill++]=id;
  classes.push({r:cls.r,o});
 }
 runtimeCollectGarbageV1(true);
 let paths;
 const pathsOf=()=>{
  if(paths!==undefined)return paths;
  const first=new Uint32Array(D.stringCount+1);for(let id=0;id<oldRows;id++)first[D.rowPath[id]+1]++;
  for(let rank=0;rank<D.stringCount;rank++)first[rank+1]+=first[rank];
  const list=new Uint32Array(oldRows),fill=first.slice(0,D.stringCount);for(let id=0;id<oldRows;id++)list[fill[D.rowPath[id]]++]=id;
  return paths={first,list};
 };
 const rankOf=text=>{let low=0,high=D.stringCount-1;while(low<=high){const mid=(low+high)>>>1,order=compareCodePointsV1(D.textAt(mid,false),text);if(order===0)return mid;if(order<0)low=mid+1;else high=mid-1;}return-1;};
 const fresh=[],added=new Map(),byObject=new Map();
 const idOfRow=row=>{
  let id=byObject.get(row);if(id!==undefined)return id;
  const key=rowKeyV1(row);id=added.get(key);
  if(id===undefined){
   const rank=oldRows?rankOf(row.path):-1;
   if(rank>=0){const{first,list}=pathsOf();for(let slot=first[rank];slot<first[rank+1];slot++){const candidate=list[slot];if(D.kinds[D.rowKind[candidate]]===row.kind&&rowKeyV1(view.rows[candidate])===key){id=candidate;break;}}}
   if(id===undefined){id=oldRows+fresh.length;fresh.push(row);added.set(key,id);}
  }
  byObject.set(row,id);return id;
 };
 const lengthOf=source=>Array.isArray(source)?source.length:typeof source?.length==="number"?source.length:INDEX_CHUNK_POLICY_V1.bulkUpsertRows;
 const memos=new Map();
 let pairs=0;for(const source of upserts.values())pairs+=lengthOf(source);
 await streamUpsertClassesV1({names:[...upserts.keys()].sort(compareCodePointsV1),rowBatches:name=>{const source=upserts.get(name);return source!==null&&typeof source==="object"&&source.ids instanceof Uint32Array&&source.log!==undefined?numberedBatchesV1(source,memos,idOfRow,pause):batchesOf(source);},ownerIds,idOfRow,rowCount:oldRows,classes,bulk:pairs>=INDEX_CHUNK_POLICY_V1.bulkUpsertRows,pause});
 for(const name of[...append.keys()].sort(compareCodePointsV1)){
  const owner=idOf(name),ids=new Set();
  for await(const batch of batchesOf(append.get(name)))for(const row of batch)ids.add(idOfRow(row));
  classes.push({r:Uint32Array.from(ids).sort(),o:Uint32Array.of(owner)});
 }
 runtimeCollectGarbageV1(true);
 const rowTotal=oldRows+fresh.length,live=new Uint8Array(owners.length),used=new Uint8Array(rowTotal);
 for(const cls of classes)for(const id of cls.o)if(!gone.has(id))live[id]=1;
 for(const cls of classes)if(cls.o.some(id=>live[id]))for(const id of cls.r)used[id]=1;
 const rowMap=new Int32Array(rowTotal).fill(-1),ownerMap=new Int32Array(owners.length).fill(-1),ownersOut=[];let rowsOut=0;
 for(let id=0;id<rowTotal;id++)if(used[id])rowMap[id]=rowsOut++;
 for(let id=0;id<owners.length;id++)if(live[id]){ownerMap[id]=ownersOut.length;ownersOut.push(id);}
 let compact=[],ids=0;
 for(const cls of classes){
  const o=cls.o;let kept=0;for(let at=0;at<o.length;at++){const mapped=ownerMap[o[at]];if(mapped>=0)o[kept++]=mapped;}
  if(!kept)continue;
  const r=cls.r;for(let at=0;at<r.length;at++)r[at]=rowMap[r[at]];
  ids+=r.length;compact.push({r,o:o.subarray(0,kept)});
 }
 classes.length=0;
 if(compact.length>P.compactionFactor*ownersOut.length+P.compactionFloor&&compact.length>P.compactionGrowth*D.classCount||ids>P.compactionFloor&&ids>P.compactionDuplication*rowsOut)compact=refineClassesV1(compact,ownersOut.length,rowsOut);
 runtimeCollectGarbageV1(true);

 const marked=new Uint8Array(D.stringCount),seenFacts=new Uint8Array(D.factCount),freshText=new Set(),freshFacts=new Set(),mark=rank=>{marked[rank]=1;};
 for(const id of ownersOut)if(id<D.ownerString.length)marked[D.ownerString[id]]=1;else freshText.add(owners[id]);
 for(let id=0;id<rowTotal;id++){
  if(!used[id])continue;
  if(id<oldRows){
   marked[D.kindString[D.rowKind[id]]]=1;marked[D.rowPath[id]]=1;
   const fact=D.rowFact[id];if(!seenFacts[fact]){seenFacts[fact]=1;reader.scan(D.factAt[fact],mark);}
  }else{
   const row=fresh[id-oldRows];freshText.add(row.kind);freshText.add(row.path);
   if(typeof row.fact!=="object"||row.fact===null||!freshFacts.has(row.fact)){if(typeof row.fact==="object"&&row.fact!==null)freshFacts.add(row.fact);collectFactStringsV1(row.fact,freshText);}
  }
 }
 const sortedFresh=[...freshText].map(text=>{if(LONE_SURROGATE.test(text))throw fail("Well formed text required in a fact index: "+JSON.stringify(text.slice(0,64)));return{text,bytes:Buffer.from(text,"utf8")};}).sort((a,b)=>Buffer.compare(a.bytes,b.bytes));
 const remap=new Uint32Array(D.stringCount),freshId=new Map(),stringSink=createFactSinkV1(maximumBytes);
 let strings=0,previous=Buffer.allocUnsafe(256),previousLength=0,current=Buffer.allocUnsafe(256),next=0;
 const emit=bytes=>{
  const limit=Math.min(previousLength,bytes.length);let lcp=0;while(lcp<limit&&previous[lcp]===bytes[lcp])lcp++;
  stringSink.varint(lcp);stringSink.varint(bytes.length-lcp);stringSink.bytes(bytes,lcp);
  if(bytes.length>previous.length)previous=Buffer.allocUnsafe(Math.max(bytes.length,previous.length*2));
  bytes.copy(previous,0,0,bytes.length);previousLength=bytes.length;strings++;
 };
 for(let rank=0;rank<D.stringCount;rank++){
  signal?.throwIfAborted();await pause();
  const lcp=D.lcps[rank],suffix=D.suffixLength[rank],length=lcp+suffix;
  if(length>current.length){const grown=Buffer.allocUnsafe(Math.max(length,current.length*2));current.copy(grown,0,0,lcp);current=grown;}
  D.buffer.copy(current,lcp,D.suffixAt[rank],D.suffixAt[rank]+suffix);
  const bytes=current.subarray(0,length);
  while(next<sortedFresh.length&&Buffer.compare(sortedFresh[next].bytes,bytes)<0){freshId.set(sortedFresh[next].text,strings);emit(sortedFresh[next].bytes);next++;}
  let keep=marked[rank]===1;
  if(next<sortedFresh.length&&Buffer.compare(sortedFresh[next].bytes,bytes)===0){keep=true;freshId.set(sortedFresh[next].text,strings);next++;}
  if(keep){remap[rank]=strings;emit(bytes);}
 }
 while(next<sortedFresh.length){freshId.set(sortedFresh[next].text,strings);emit(sortedFresh[next].bytes);next++;}

 const factSink=createFactSinkV1(maximumBytes),rowSink=createFactSinkV1(maximumBytes),kindIds=new Map(),kindStrings=[],spans=[],byBytes=new Map(),oldFact=new Int32Array(D.factCount).fill(-1),newFact=new Map(),textId=text=>freshId.get(text);
 const commit=start=>{
  const span=factSink.result().subarray(start,factSink.size()),key=createHash("sha256").update(span).digest("latin1"),known=byBytes.get(key);
  if(known!==undefined)for(const id of known){const[from,to]=spans[id];if(Buffer.compare(factSink.result().subarray(from,to),span)===0){factSink.truncate(start);return id;}}
  const id=spans.length;spans.push([start,factSink.size()]);if(known===undefined)byBytes.set(key,[id]);else known.push(id);return id;
 };
 for(let id=0;id<rowTotal;id++){
  if(!used[id])continue;
  signal?.throwIfAborted();await pause();
  let kind,kindString,path,fact;
  if(id<oldRows){
   const fromOld=D.rowFact[id];kind=D.kinds[D.rowKind[id]];kindString=remap[D.kindString[D.rowKind[id]]];path=remap[D.rowPath[id]];
   fact=oldFact[fromOld];if(fact<0){const start=factSink.size();reader.copy(D.factAt[fromOld],factSink,remap);fact=oldFact[fromOld]=commit(start);}
  }else{
   const row=fresh[id-oldRows];kind=row.kind;kindString=textId(kind);path=textId(row.path);
   fact=typeof row.fact==="object"&&row.fact!==null?newFact.get(row.fact):undefined;
   if(fact===undefined){const start=factSink.size();writeFactValueV1(factSink,row.fact,textId,1);fact=commit(start);if(typeof row.fact==="object"&&row.fact!==null)newFact.set(row.fact,fact);}
  }
  let at=kindIds.get(kind);if(at===undefined){at=kindStrings.length;kindIds.set(kind,at);kindStrings.push(kindString);}
  rowSink.varint(at);rowSink.varint(path);rowSink.varint(fact);
 }
 const out=createFactSinkV1(maximumBytes);
 out.bytes(MAGIC);
 for(const count of[strings,kindStrings.length,ownersOut.length,spans.length,rowsOut,compact.length])out.varint(count);
 out.bytes(stringSink.result());
 for(const string of kindStrings)out.varint(string);
 for(const id of ownersOut)out.varint(id<D.ownerString.length?remap[D.ownerString[id]]:textId(owners[id]));
 out.bytes(factSink.result());out.bytes(rowSink.result());
 for(const cls of compact){writeFactListV1(out,cls.r);writeFactListV1(out,cls.o);}
 return out.result();
}
