import{createHash}from"node:crypto";

/** 🔑️ The length of the JSON text of a fact above which the pool keys a row by the digest of the text and not by the text itself (a directory listing is kilobytes of names the pool would hold a second time as its key, as long as the row lives). */
const POOL_DIGEST_CHARACTERS=256;

/** 🔤️ The text identity of one fact inside an observation row: a primitive by type and value, any other value by its compact JSON text with the key order kept. */
export function factKeyV1(fact){
 if(fact===null||typeof fact!=="object")return typeof fact+String(fact);
 return JSON.stringify(fact);
}

/** 🧺️ Collects each exact consumed row once in first-occurrence order, retaining every different answer to the same source question. */
export function distinctObservationRowsV1(...captures){
 const byKind=new Map(),distinct=[];
 const same=(left,right)=>left===right||left.fact===right.fact||factKeyV1(left.fact)===factKeyV1(right.fact);
 for(const rows of captures)for(const row of rows){
  let byPath=byKind.get(row.kind);
  if(byPath===undefined){byPath=new Map();byKind.set(row.kind,byPath);}
  const previous=byPath.get(row.path);
  if(previous===undefined)byPath.set(row.path,row);
  else if(Array.isArray(previous)){
   if(previous.some(known=>same(known,row)))continue;
   previous.push(row);
  }else{
   if(same(previous,row))continue;
   byPath.set(row.path,[previous,row]);
  }
  distinct.push(row);
 }
 return distinct;
}

const SHAPE=Object.freeze({undefined:0,null:1,false:2,true:3,string:4,number:5,flat:6,analysis:7,json:8});
const FLAT_KEYS=Object.freeze(["size","mtimeMs","ctimeMs","ino","sha256"]);
const ANALYSIS_KEYS=Object.freeze(["provider","revision","source","resultSha256","nestedObservations"]);
const ROW_KEYS=Object.freeze(["kind","path","fact"]);
const FLAT_STAMPS=4,ANALYSIS_FIELDS=7,INITIAL_ROWS=1024;

const keysAre=(value,keys)=>{const own=Object.keys(value);if(own.length!==keys.length)return false;for(let at=0;at<keys.length;at++)if(own[at]!==keys[at])return false;return true;};
const isObject=value=>value!==null&&typeof value==="object"&&!Array.isArray(value);
const isFlat=fact=>isObject(fact)&&keysAre(fact,FLAT_KEYS)&&typeof fact.sha256==="string"&&Number.isFinite(fact.size)&&Number.isFinite(fact.mtimeMs)&&Number.isFinite(fact.ctimeMs)&&Number.isFinite(fact.ino);
const isRow=row=>isObject(row)&&keysAre(row,ROW_KEYS)&&typeof row.kind==="string"&&typeof row.path==="string";
const isAnalysis=fact=>isObject(fact)&&keysAre(fact,ANALYSIS_KEYS)&&typeof fact.provider==="string"&&typeof fact.revision==="string"&&typeof fact.resultSha256==="string"&&isFlat(fact.source)&&Array.isArray(fact.nestedObservations)&&fact.nestedObservations.every(isRow);
const mix=(hash,value)=>Math.imul((hash^value)>>>0,16777619)>>>0;

/**
 * 🧶️ Creates a table of texts: equal texts are one string and one number, so a path or a digest that thousands of rows name is stored once. Numbers are dense from zero until `clear()`.
 */
export function createStringTableV1(){
 let texts=[],ids=new Map(),characters=0;
 return Object.freeze({
  id(text){let id=ids.get(text);if(id===undefined){id=texts.length;texts.push(text);ids.set(text,id);characters+=text.length;}return id;},
  text:id=>texts[id],
  size:()=>texts.length,
  characters:()=>characters,
  clear(){texts=[];ids=new Map();characters=0;},
 });
}

const grown=(array,length)=>{const next=new array.constructor(length);next.set(array);return next;};

/**
 * 🔢️ Creates the compact store of canonical observation rows: one string table for every kind, path, digest and fact text, and one row as entries of typed-array columns (kind, path, fact shape, digest, stamps), found through chained hash buckets that are verified by comparing the columns. Equal (kind, path, fact) is one number; a row object is only a view that is made on demand and kept weakly, so a store holds the rows of thousands of settled modules in a few bytes each while no consumer holds their objects. Numbers are never reused: `clear()` (also done when a row or character budget is exceeded) starts empty storage but keeps counting, so a row object that outlives a clear is numbered again by its content instead of colliding.
 * @param {{maximumRows?:number;maximumCharacters?:number;hash?:(kind:number,path:number,shape:number,reference:number)=>number}} [options]
 */
export function createRowTableV1({maximumRows=Infinity,maximumCharacters=Infinity,hash=(kind,path,shape,reference)=>mix(mix(mix(mix(2166136261,kind),path),shape),reference)}={}){
 const strings=createStringTableV1();
 let slots=new WeakMap(),base=1,count=0,capacity=INITIAL_ROWS;
 let kinds,paths,shapes,references,auxiliaries,hashes,chain,heads,stamps,stampCount,records,recordCount,pool,poolCount,views;
 const reset=()=>{
  capacity=INITIAL_ROWS;count=0;stampCount=0;recordCount=0;poolCount=0;
  kinds=new Uint32Array(capacity);paths=new Uint32Array(capacity);shapes=new Uint8Array(capacity);references=new Uint32Array(capacity);auxiliaries=new Uint32Array(capacity);hashes=new Uint32Array(capacity);chain=new Int32Array(capacity);
  heads=new Int32Array(capacity).fill(-1);stamps=new Float64Array(FLAT_STAMPS*16);records=new Uint32Array(ANALYSIS_FIELDS*16);pool=new Uint32Array(64);views=[];
 };
 reset();
 const grow=()=>{
  capacity*=2;
  kinds=grown(kinds,capacity);paths=grown(paths,capacity);shapes=grown(shapes,capacity);references=grown(references,capacity);auxiliaries=grown(auxiliaries,capacity);hashes=grown(hashes,capacity);chain=grown(chain,capacity);
  heads=new Int32Array(capacity).fill(-1);
  for(let at=0;at<count;at++){const bucket=hashes[at]&(capacity-1);chain[at]=heads[bucket];heads[bucket]=at;}
 };
 const pushStamps=(a,b,c,d)=>{if(stampCount*FLAT_STAMPS>=stamps.length)stamps=grown(stamps,stamps.length*2);const at=stampCount++*FLAT_STAMPS;stamps[at]=a;stamps[at+1]=b;stamps[at+2]=c;stamps[at+3]=d;return at/FLAT_STAMPS;};
 const stampsEqual=(index,a,b,c,d)=>{const at=index*FLAT_STAMPS;return stamps[at]===a&&stamps[at+1]===b&&stamps[at+2]===c&&stamps[at+3]===d;};
 const pushRecord=fields=>{if(recordCount*ANALYSIS_FIELDS>=records.length)records=grown(records,records.length*2);const at=recordCount++*ANALYSIS_FIELDS;for(let field=0;field<ANALYSIS_FIELDS;field++)records[at+field]=fields[field];return recordCount-1;};
 const pushPool=ids=>{const start=poolCount;if(poolCount+ids.length>pool.length)pool=grown(pool,Math.max(pool.length*2,poolCount+ids.length));for(const id of ids)pool[poolCount++]=id;return start;};

 const describe=(fact,scratch)=>{
  if(fact===undefined)return SHAPE.undefined;
  if(fact===null)return SHAPE.null;
  if(fact===false)return SHAPE.false;
  if(fact===true)return SHAPE.true;
  if(typeof fact==="string"){scratch.reference=strings.id(fact);return SHAPE.string;}
  if(typeof fact==="number"){scratch.numbers=[fact,0,0,0];return SHAPE.number;}
  if(isFlat(fact)){scratch.reference=strings.id(fact.sha256);scratch.numbers=[fact.size,fact.mtimeMs,fact.ctimeMs,fact.ino];return SHAPE.flat;}
  if(isAnalysis(fact)){
   const source=fact.source;
   scratch.record=[strings.id(fact.provider),strings.id(fact.revision),strings.id(fact.resultSha256),strings.id(source.sha256),0,0,0];
   scratch.nested=fact.nestedObservations.map(row=>intern(row.kind,row.path,row.fact,false));
   scratch.numbers=[source.size,source.mtimeMs,source.ctimeMs,source.ino];
   scratch.reference=scratch.record[0]*31+scratch.record[2];
   return SHAPE.analysis;
  }
  scratch.reference=strings.id(JSON.stringify(fact));return SHAPE.json;
 };
 const same=(index,kind,path,shape,scratch)=>{
  if(kinds[index]!==kind||paths[index]!==path||shapes[index]!==shape)return false;
  switch(shape){
   case SHAPE.string:case SHAPE.json:return references[index]===scratch.reference;
   case SHAPE.number:return stamps[auxiliaries[index]*FLAT_STAMPS]===scratch.numbers[0];
   case SHAPE.flat:return references[index]===scratch.reference&&stampsEqual(auxiliaries[index],...scratch.numbers);
   case SHAPE.analysis:{
    const at=auxiliaries[index]*ANALYSIS_FIELDS,own=scratch.record;
    if(records[at]!==own[0]||records[at+1]!==own[1]||records[at+2]!==own[2]||records[at+3]!==own[3]||records[at+5]!==scratch.nested.length)return false;
    if(!stampsEqual(records[at+6],...scratch.numbers))return false;
    for(let item=0;item<scratch.nested.length;item++)if(pool[records[at+4]+item]!==scratch.nested[item])return false;
    return true;
   }
   default:return true;
  }
 };
 const intern=(kind,path,fact,top=true)=>{
  if(top&&(count>=maximumRows||strings.characters()>maximumCharacters))clear();
  const kindId=strings.id(kind),pathId=strings.id(path),scratch={reference:0,numbers:undefined,record:undefined,nested:undefined},shape=describe(fact,scratch);
  const place=hash(kindId,pathId,shape,scratch.reference)>>>0;let bucket=place&(capacity-1);
  for(let index=heads[bucket];index!==-1;index=chain[index])if(hashes[index]===place&&same(index,kindId,pathId,shape,scratch))return base+index;
  if(count===capacity){grow();bucket=place&(capacity-1);}
  const index=count++;
  kinds[index]=kindId;paths[index]=pathId;shapes[index]=shape;hashes[index]=place;references[index]=scratch.reference;auxiliaries[index]=0;
  if(shape===SHAPE.number)auxiliaries[index]=pushStamps(...scratch.numbers);
  else if(shape===SHAPE.flat)auxiliaries[index]=pushStamps(...scratch.numbers);
  else if(shape===SHAPE.analysis){
   const record=scratch.record;record[4]=pushPool(scratch.nested);record[5]=scratch.nested.length;record[6]=pushStamps(...scratch.numbers);
   auxiliaries[index]=pushRecord(record);
  }
  chain[index]=heads[bucket];heads[bucket]=index;
  return base+index;
 };

 const factAt=index=>{
  switch(shapes[index]){
   case SHAPE.undefined:return undefined;
   case SHAPE.null:return null;
   case SHAPE.false:return false;
   case SHAPE.true:return true;
   case SHAPE.string:return strings.text(references[index]);
   case SHAPE.number:return stamps[auxiliaries[index]*FLAT_STAMPS];
   case SHAPE.flat:{const at=auxiliaries[index]*FLAT_STAMPS;return{size:stamps[at],mtimeMs:stamps[at+1],ctimeMs:stamps[at+2],ino:stamps[at+3],sha256:strings.text(references[index])};}
   case SHAPE.analysis:{
    const at=auxiliaries[index]*ANALYSIS_FIELDS,stamp=records[at+6]*FLAT_STAMPS,nested=[];
    for(let item=0;item<records[at+5];item++)nested.push(rowAt(pool[records[at+4]+item]-base));
    return{provider:strings.text(records[at]),revision:strings.text(records[at+1]),source:{size:stamps[stamp],mtimeMs:stamps[stamp+1],ctimeMs:stamps[stamp+2],ino:stamps[stamp+3],sha256:strings.text(records[at+3])},resultSha256:strings.text(records[at+2]),nestedObservations:nested};
   }
   default:return JSON.parse(strings.text(references[index]));
  }
 };
 const build=index=>({kind:strings.text(kinds[index]),path:strings.text(paths[index]),fact:factAt(index)});
 const rowAt=index=>{const live=views[index]?.deref();if(live!==undefined)return live;const row=build(index);views[index]=new WeakRef(row);slots.set(row,base+index);return row;};
 const clear=()=>{base+=count;strings.clear();slots=new WeakMap();reset();};
 const live=id=>id>=base&&id<base+count;

 return Object.freeze({
  canonical(kind,path,fact,make){
   const id=intern(kind,path,fact),index=id-base,known=views[index]?.deref();
   if(known!==undefined)return known;
   const row=make(strings.text(paths[index]));views[index]=new WeakRef(row);slots.set(row,id);return row;
  },
  id(row){const known=slots.get(row);if(known!==undefined&&live(known))return known;const id=intern(row.kind,row.path,row.fact),index=id-base;if(views[index]?.deref()===undefined)views[index]=new WeakRef(row);slots.set(row,id);return id;},
  row:id=>live(id)?rowAt(id-base):undefined,
  materialize:id=>live(id)?build(id-base):undefined,
  size:()=>count,
  stats:()=>({rows:count,strings:strings.size(),characters:strings.characters(),bytes:kinds.byteLength+paths.byteLength+shapes.byteLength+references.byteLength+auxiliaries.byteLength+hashes.byteLength+chain.byteLength+heads.byteLength+stamps.byteLength+records.byteLength+pool.byteLength+strings.characters()*2+strings.size()*64+count*16}),
  clear,
 });
}


/**
 * 🪢️ Creates a weak pool that makes equal observation rows one live instance, however many stored records and detached computations hand them over: unlike the bounded {@link createRowTableV1} it never empties itself, because it holds nothing a live row does not hold, so a working set larger than any table bound still costs one object per distinct row. The rows an analysis row nests are shared first, so equal nested rows are equal by their instance and an analysis row is keyed by its fields and those instances, never by the text of the whole record. A row that is the pooled instance and a list that was handed over complete are remembered, so the many closure entries that carry the same shared layers cost their distinct rows once, not their rows once per entry.
 */
export function createRowPoolV1(){
 const pool=new Map(),numbers=new WeakMap(),canonical=new WeakSet(),settled=new WeakSet(),dead=new FinalizationRegistry(key=>{if(pool.get(key)?.deref()===undefined)pool.delete(key);});
 let next=0;
 const number=row=>{let known=numbers.get(row);if(known===undefined)numbers.set(row,known=++next);return known;};
 const stampOf=value=>value!==null&&typeof value==="object"&&Object.keys(value).length===5&&typeof value.sha256==="string"?value.size+":"+value.mtimeMs+":"+value.ctimeMs+":"+value.ino+":"+value.sha256:undefined;
 const identity=row=>{
  const fact=row.fact,head=row.kind+"\0"+row.path+"\0";
  if(fact===null||typeof fact!=="object")return head+typeof fact+String(fact);
  if(Array.isArray(fact.nestedObservations)&&Object.keys(fact).length===5&&typeof fact.provider==="string"&&typeof fact.revision==="string"&&typeof fact.resultSha256==="string"){const source=stampOf(fact.source);if(source!==undefined)return head+"a\0"+fact.provider+"\0"+fact.revision+"\0"+fact.resultSha256+"\0"+source+"\0"+fact.nestedObservations.map(number).join(",");}
  const flat=stampOf(fact);
  if(flat!==undefined)return head+"f\0"+flat;
  const text=JSON.stringify(fact);
  return head+(text.length>POOL_DIGEST_CHARACTERS?"h\0"+createHash("sha256").update(text).digest("base64"):"j\0"+text);
 };
 const share=row=>{
  if(canonical.has(row))return row;
  if(row.kind==="analysis"&&Array.isArray(row.fact?.nestedObservations)){const nested=row.fact.nestedObservations;for(let at=0;at<nested.length;at++)nested[at]=share(nested[at]);}
  const key=identity(row),known=pool.get(key)?.deref();
  if(known!==undefined)return known;
  pool.set(key,new WeakRef(row));dead.register(row,key);canonical.add(row);return row;
 };
 return Object.freeze({
  share,
  rows(rows){if(settled.has(rows))return rows;for(let at=0;at<rows.length;at++)rows[at]=share(rows[at]);settled.add(rows);return rows;},
  size:()=>pool.size,
 });
}

/** 🪢 Encodes dependency-ordered authority records and ordered root indexes. Each shared analysis dependency is represented by one backward reference. */
export function encodeAuthorityObservationsV1(observations,maximumBytes,check=()=>{}){
 const records=[],roots=[],objects=new WeakMap(),keys=new Map(),visiting=new WeakSet();
 let bytes=2,work=0;
 const charge=text=>{
  if(++work%256===0)check();
  bytes+=Buffer.byteLength(text)+(work>1?1:0);
  if(bytes>maximumBytes)throw Error("Build graph authority exceeds its finite observation limit");
  return text;
 };
 const index=row=>{
  const known=objects.get(row);
  if(known!==undefined)return known;
  if(visiting.has(row))throw Error("Cyclic graph authority observations");
  visiting.add(row);
  const fact=row.kind==="analysis"?{...row.fact,nestedObservations:row.fact.nestedObservations.map(index)}:row.fact;
  const text=JSON.stringify({kind:row.kind,path:row.path,fact});
  let id=keys.get(text);
  if(id===undefined){id=records.length;records.push(charge(text));keys.set(text,id);}
  objects.set(row,id);visiting.delete(row);
  return id;
 };
 check();
 for(const row of observations)roots.push(charge(String(index(row))));
 check();
 return "["+records.join(",")+(roots.length?",":"")+roots.join(",")+"]";
}

/** 🧮 Checks indexed authority observations in stream order without retaining row data. The function is also embedded in the cache maintenance child. */
export function createAuthorityObservationReaderV1(kinds){
 const refuse=()=>{throw Error("Closed indexed authority observations required");};
 let count=0,roots=false;
 const reference=id=>Number.isSafeInteger(id)&&id>=0&&id<count;
 return entry=>{
  if(typeof entry==="number"){
   if(!reference(entry))refuse();
   roots=true;
   return "root";
  }
  if(roots||entry===null||typeof entry!=="object"||Array.isArray(entry)||Object.keys(entry).sort().join()!=="fact,kind,path"||!kinds.includes(entry.kind)||typeof entry.path!=="string"||entry.path.length===0)refuse();
  if(entry.kind==="analysis"&&(!Array.isArray(entry.fact?.nestedObservations)||!entry.fact.nestedObservations.every(reference)))refuse();
  count++;
  return "record";
 };
}

/** 📖 Reconstructs the authority's ordered roots with the nested dependencies shared by identity. */
export function decodeAuthorityObservationsV1(observations,kinds,check=()=>{}){
 const read=createAuthorityObservationReaderV1(kinds),records=[],roots=[];
 if(!Array.isArray(observations))throw Error("Closed indexed authority observations required");
 for(let at=0;at<observations.length;at++){
  if(at%256===0)check();
  const entry=observations[at];
  if(read(entry)==="root")roots.push(records[entry]);
  else records.push(entry.kind==="analysis"?{...entry,fact:{...entry.fact,nestedObservations:entry.fact.nestedObservations.map(index=>records[index])}}:entry);
 }
 check();
 return roots;
}
