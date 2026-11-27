import{createHash}from"node:crypto";
import{readFileSync}from"node:fs";
import{mkdir,readFile,readdir,rename,rm,stat,utimes,writeFile}from"node:fs/promises";
import{join}from"node:path";
import{brotliCompressSync,brotliDecompressSync,constants}from"node:zlib";
import{refuseBuildV1}from"../../../../📖️texts/🟦️.ts";
import{graphStringInternerV1,internJsonStringsV1}from"../../../../🧠️memory/🟨️.mjs";
import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import table from"./🔣️.json"with{type:"json"};

if(validateJsonSchemaSubset(schema,table).length)throw refuseBuildV1("invalid","closure.graph-invalid",{part:"policy"});

/** 🌳️ The declared policy of the module closure graph.
 * @see ./🧬️schema/🔣️.json */
export const CLOSURE_GRAPH_POLICY_V1=Object.freeze(structuredClone(table));

/** 🏷️ The name of the `global` row a command walk is recorded as (`[name, workspaceRoot, question]`, the closure digest of the question as its fact).
 * @see ./🧬️schema/🔣️.json */
export const CLOSURE_GRAPH_FACT_V1=CLOSURE_GRAPH_POLICY_V1.fact;

const sha=text=>createHash("sha256").update(text).digest("hex");
const canonical=value=>Array.isArray(value)?value.map(canonical):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).filter(key=>value[key]!==undefined).sort().map(key=>[key,canonical(value[key])])):value;
const HEAD_SCHEMA_V1={...schema.definitions.Head},CHUNK_SCHEMA_V1={...schema.definitions.Chunk},EXTENSION=".json.br",MAXIMUM_CHUNK_BYTES=1073741824;

/** 🧾️ The digest of what a walk reads of one module: SHA-256 of the canonical JSON of `content` (keys sorted, `undefined` dropped), so any change of kind, name, resolved edges, ports or generated imports moves it and nothing else does. */
export function moduleNodeDigestV1(content){return sha(JSON.stringify(canonical(content)));}

/** 🏷️ The entry digest of one reached node in a closure digest. */
export const closureEntryDigestV1=(name,digest)=>sha(name+"\0"+digest);

/** 🛑️ The entry digest of one declared tool a closure is cut at. */
export const closureToolDigestV1=name=>sha("\0tool\0"+name);

/** 🌳️ Creates one module closure graph: `update(records, removed)` puts nodes `{key, name, kind, digest, edges, extra}` (edges are target keys) and drops keys; a node whose digest and edges did not change is left alone, any other change forgets the component digests of the node and of every node that reaches it (reverse edges), nothing else. The digest of a strongly connected component is the SHA-256 of its sorted member entry digests (`sha256(name NUL digest)`), an empty line, and the sorted distinct digests of the components and cut tools (`sha256(NUL tool NUL name)`) it imports; it is memoised per component and per family of stops and redirects, a component that reaches no redirected key sharing the digest of the family without redirects, so the whole graph is one linear pass (iterative Tarjan) and a question after it is a lookup. `closureDigest(roots, {stops, redirect})` is the SHA-256 of the sorted distinct component digests of `roots`, every edge first sent through `redirect` (key to key: a router read with selected command words is another node than the router read whole) and the `stops` (key to tool name; a root or a redirect target is never a stop) cut to tool entries; undefined while a reached key is not in the graph. A component digest is a function of the reached nodes and their edges alone, so an edit moves exactly the closures that reach it. */
export function createClosureGraphV1(){
 const records=new Map(),reverse=new Map(),families=new Map();
 let edgeCount=0;
 const byKey=([left],[right])=>left<right?-1:left>right?1:0;
 const familyOf=(stops,redirect)=>{
  const pairs=[...stops].sort(byKey),moved=[...redirect].sort(byKey),id=JSON.stringify([pairs,moved]);
  let family=families.get(id);if(family===undefined){const table=new Map(moved);families.set(id,family={stops:new Map(pairs),table,to:key=>table.get(key)??key,digests:new Map(),base:moved.length?familyOf(stops,new Map()):undefined,affected:undefined});}
  return family;
 };
 const affectedOf=family=>{
  if(family.affected!==undefined)return family.affected;
  const seen=new Set(family.table.keys()),queue=[...seen];
  while(queue.length)for(const above of reverse.get(queue.pop())??[])if(!seen.has(above)){seen.add(above);queue.push(above);}
  return family.affected=seen;
 };
 const settledOf=(family,key)=>family.digests.get(key)??(family.base!==undefined&&!affectedOf(family).has(key)?componentOf(family.base,key):undefined);
 const missing=key=>Object.assign(refuseBuildV1("missing","closure.graph-incomplete",{key:String(key).slice(0,160)}),{closureGraphIncomplete:true,key});
 const recordOf=key=>{const record=records.get(key);if(record===undefined)throw missing(key);return record;};
 function componentOf(family,start){
  const known=settledOf(family,start);if(known!==undefined)return known;
  recordOf(start);
  const index=new Map([[start,0]]),low=new Map([[start,0]]),stack=[start],on=new Set([start]),work=[[start,0]];let counter=1;
  while(work.length){
   const frame=work[work.length-1],key=frame[0],edges=records.get(key).edges;
   if(frame[1]<edges.length){
    const target=family.to(edges[frame[1]++]);
    if(family.stops.has(target)||settledOf(family,target)!==undefined)continue;
    if(!index.has(target)){recordOf(target);index.set(target,counter);low.set(target,counter);counter++;stack.push(target);on.add(target);work.push([target,0]);}
    else if(on.has(target)&&index.get(target)<low.get(key))low.set(key,index.get(target));
    continue;
   }
   work.pop();
   if(work.length){const parent=work[work.length-1][0];if(low.get(key)<low.get(parent))low.set(parent,low.get(key));}
   if(low.get(key)!==index.get(key))continue;
   const members=new Set();let top;
   do{top=stack.pop();on.delete(top);members.add(top);}while(top!==key);
   const own=[],below=new Set();
   for(const member of members){const record=records.get(member);own.push(record.entry);for(const edge of record.edges){const target=family.to(edge),tool=family.stops.get(target);if(tool!==undefined)below.add(closureToolDigestV1(tool));else if(!members.has(target))below.add(settledOf(family,target));}}
   const digest=sha(own.sort().join("\n")+"\n\n"+[...below].sort().join("\n"));
   for(const member of members)family.digests.set(member,digest);
  }
  return family.digests.get(start);
 }
 const forgetAbove=keys=>{
  const seen=new Set(keys),queue=[...keys];
  while(queue.length)for(const above of reverse.get(queue.pop())??[])if(!seen.has(above)){seen.add(above);queue.push(above);}
  for(const family of families.values()){family.affected=undefined;for(const key of seen)family.digests.delete(key);}
  return seen.size;
 };
 const link=(key,edges,sign)=>{for(const target of edges){let above=reverse.get(target);if(sign>0){if(above===undefined)reverse.set(target,above=new Set());above.add(key);}else{above?.delete(key);if(above?.size===0)reverse.delete(target);}}edgeCount+=sign*edges.length;};
 const sameEdges=(left,right)=>left.length===right.length&&left.every((edge,at)=>edge===right[at]);
 const familyFor=(roots,stops=new Map(),redirect=new Map())=>{const cut=new Map(stops);for(const root of roots)cut.delete(root);for(const target of redirect.values())cut.delete(target);return familyOf(cut,redirect);};
 return Object.freeze({
  update(put=[],removed=[]){
   const moved=[];
   for(const key of removed){const held=records.get(key);if(held===undefined)continue;records.delete(key);link(key,held.edges,-1);moved.push(key);}
   for(const raw of put){
    if(typeof raw?.key!=="string"||typeof raw.name!=="string"||typeof raw.kind!=="string"||!/^[0-9a-f]{64}$/u.test(raw.digest)||!Array.isArray(raw.edges)||!raw.edges.every(edge=>typeof edge==="string"))throw refuseBuildV1("invalid","closure.graph-invalid",{part:"node"});
    const record=Object.freeze({key:raw.key,name:raw.name,kind:raw.kind,digest:raw.digest,edges:Object.freeze([...new Set(raw.edges)].sort()),extra:raw.extra??null,entry:closureEntryDigestV1(raw.name,raw.digest)}),held=records.get(record.key);
    records.set(record.key,record);
    if(held!==undefined&&held.entry===record.entry&&sameEdges(held.edges,record.edges))continue;
    if(held!==undefined)link(record.key,held.edges,-1);
    link(record.key,record.edges,1);moved.push(record.key);
   }
   return{forgotten:moved.length?forgetAbove(moved):0};
  },
  get:key=>records.get(key),
  has:key=>records.has(key),
  *records(){yield*records.values();},
  closureDigest(roots,{stops,redirect}={}){
   const family=familyFor(roots,stops,redirect);
   try{return sha([...new Set(roots.map(root=>componentOf(family,root)))].sort().join("\n"));}
   catch(error){if(error?.closureGraphIncomplete)return undefined;throw error;}
  },
  reach(roots,{stops,redirect}={}){
   const family=familyFor(roots,stops,redirect),seen=new Set(),queue=[...roots],names=[],cut=new Set();
   while(queue.length){const key=queue.pop();if(seen.has(key))continue;seen.add(key);const record=recordOf(key);names.push(record.name);for(const edge of record.edges){const target=family.to(edge),tool=family.stops.get(target);if(tool!==undefined)cut.add(tool);else queue.push(target);}}
   return{names:names.sort(),tools:[...cut].sort(),keys:[...seen].sort()};
  },
  missing(roots){const seen=new Set(),queue=[...roots],absent=new Set();while(queue.length){const key=queue.pop();if(seen.has(key))continue;seen.add(key);const record=records.get(key);if(record===undefined){absent.add(key);continue;}for(const edge of record.edges)queue.push(edge);}return[...absent].sort();},
  stats:()=>{let digests=0;for(const family of families.values())digests+=family.digests.size;return{nodes:records.size,edges:edgeCount,families:families.size,digests};},
 });
}

const bucketOf=(key,buckets)=>parseInt(sha(key).slice(0,8),16)%buckets;
const placeAtomically=async(path,bytes)=>{const temporary=path+"."+process.pid+"."+Math.random().toString(36).slice(2)+".tmp";try{await writeFile(temporary,bytes);await rename(temporary,path);}catch(error){await rm(temporary,{force:true});throw error;}};

/** 💾️ Stores `graph` under `directory` digest addressed: one chunk per bucket of node keys (`c/<sha256 of the text>.json.br`), each written once (an unchanged chunk is reused by its digest), then the head (`head.json`) replaced atomically, then the chunks the new head does not name and that are older than `chunkGraceMilliseconds` removed (several processes of a machine save into one directory: a chunk a just replaced head still names stays readable for a reader that read that head); a save that stops (cancelled through `signal`, killed) leaves the previous head and its chunks whole. One chunk is in memory at a time; `progress({done,total})` is awaited after each. Answers `{chunks, written, bytes, nodes, edges, head}` (`head` the SHA-256 of the head text: the identity of the stored graph). */
export async function saveClosureGraphV1(graph,directory,{signal,progress,policy=CLOSURE_GRAPH_POLICY_V1}={}){
 const chunkDir=join(directory,"c"),groups=Array.from({length:policy.buckets},()=>[]);
 await mkdir(chunkDir,{recursive:true});
 let nodes=0,edges=0,written=0,bytes=0;const pool=createBasePoolV1();
 for(const record of graph.records()){groups[bucketOf(record.key,policy.buckets)].push(record);nodes++;edges+=record.edges.length;}
 const chunks=[];
 for(let at=0;at<groups.length;at++){
  signal?.throwIfAborted();
  const group=groups[at];
  if(!group.length){chunks.push(null);await progress?.({done:at+1,total:groups.length});continue;}
  group.sort((left,right)=>left.key<right.key?-1:left.key>right.key?1:0);
  const text=JSON.stringify({version:1,records:group.map(record=>[record.key,record.name,record.kind,record.digest,record.edges,pooledExtraV1(record.extra,pool)])}),digest=sha(text),path=join(chunkDir,digest+EXTENSION);
  groups[at]=undefined;
  if(!await stat(path).then(()=>true,()=>false)){const packed=brotliCompressSync(Buffer.from(text,"utf8"),{params:{[constants.BROTLI_PARAM_QUALITY]:policy.compressionQuality}});await placeAtomically(path,packed);written++;bytes+=packed.length;}
  else{const now=new Date();await utimes(path,now,now).catch(()=>{});}
  chunks.push(digest);
  await progress?.({done:at+1,total:groups.length});
 }
 signal?.throwIfAborted();
 let poolDigest=null;
 if(pool.bases.length){
  const poolText=JSON.stringify({version:1,bases:pool.bases});poolDigest=sha(poolText);const poolPath=join(chunkDir,poolDigest+EXTENSION);
  if(!await stat(poolPath).then(()=>true,()=>false)){const packed=brotliCompressSync(Buffer.from(poolText,"utf8"),{params:{[constants.BROTLI_PARAM_QUALITY]:policy.compressionQuality}});await placeAtomically(poolPath,packed);written++;bytes+=packed.length;}
  else{const now=new Date();await utimes(poolPath,now,now).catch(()=>{});}
 }
 const headText=JSON.stringify({version:1,chunks,pool:poolDigest,nodes,edges});
 await placeAtomically(join(directory,"head.json"),Buffer.from(headText));
 const named=new Set([...chunks,poolDigest].filter(Boolean).map(digest=>digest+EXTENSION)),oldest=Date.now()-policy.chunkGraceMilliseconds;
 for(const name of await readdir(chunkDir).catch(()=>[])){if(!name.endsWith(EXTENSION)||named.has(name))continue;const info=await stat(join(chunkDir,name)).catch(()=>undefined);if(info!==undefined&&info.mtimeMs<=oldest)await rm(join(chunkDir,name),{force:true});}
 return{chunks:chunks.filter(Boolean).length,pool:poolDigest,written,bytes,nodes,edges,head:sha(headText)};
}

/** 🧩️ The records of one stored chunk after its digest and schema are checked; a chunk that does not match refuses the graph. Every long string of the parsed records is replaced by the process-wide shared instance: the witnesses of ten thousand stored nodes name the same few tens of thousands of paths, and a loaded graph held each of them once per node (894 k path strings, 161 MB, behind `witness.typed` in the heap summary of a whole-closure read). */
function chunkRecordsV1(bytes,digest,bases=[]){
 let text;try{text=brotliDecompressSync(bytes,{maxOutputLength:MAXIMUM_CHUNK_BYTES}).toString("utf8");}catch{throw refuseBuildV1("invalid","closure.graph-invalid",{part:"chunk"});}
 if(sha(text)!==digest)throw refuseBuildV1("invalid","closure.graph-invalid",{part:"chunk"});
 const chunk=JSON.parse(text);
 if(validateJsonSchemaSubset(CHUNK_SCHEMA_V1,chunk).length)throw refuseBuildV1("invalid","closure.graph-invalid",{part:"chunk"});
  internJsonStringsV1(chunk.records,graphStringInternerV1);
 return chunk.records.map(([key,name,kind,digest,edges,extra])=>({key,name,kind,digest,edges,extra:unpooledExtraV1(extra,bases)}));
}
/** 🧺️ The shared layers of the witnesses of one stored graph: a witness names its `base` (what the nodes of one pass share, thousands of rows) by index and the graph stores each distinct base once, in the pool the head names, so a node costs what it read itself and a loaded graph holds one copy of each base.
 * @see ./🧬️schema/🔣️.json */
function createBasePoolV1(){const bases=[],byObject=new Map(),byText=new Map();return{bases,at(base){let found=byObject.get(base);if(found!==undefined)return found;const text=JSON.stringify(base);found=byText.get(text);if(found===undefined){found=bases.push(base)-1;byText.set(text,found);}byObject.set(base,found);return found;}};}
const pooledExtraV1=(extra,pool)=>{const witness=extra?.witness;return witness!==null&&typeof witness==="object"&&witness.base!==undefined&&witness.base!==null?{...extra,witness:{...witness,base:pool.at(witness.base)}}:extra;};
const unpooledExtraV1=(extra,bases)=>{const witness=extra?.witness;if(witness===null||typeof witness!=="object"||witness.base===undefined||witness.base===null)return extra;const base=bases[witness.base];if(base===undefined)throw refuseBuildV1("invalid","closure.graph-invalid",{part:"pool"});return{...extra,witness:{...witness,base}};};
/** 📥️ The bases of the pool a head names (none when it names none), checked against its digest. */
const poolBasesV1=(head,read)=>{if(head.pool===null)return[];let text;try{text=brotliDecompressSync(read(head.pool+EXTENSION),{maxOutputLength:MAXIMUM_CHUNK_BYTES}).toString("utf8");}catch{throw refuseBuildV1("invalid","closure.graph-invalid",{part:"pool"});}if(sha(text)!==head.pool)throw refuseBuildV1("invalid","closure.graph-invalid",{part:"pool"});const pool=JSON.parse(text);if(pool?.version!==1||!Array.isArray(pool.bases))throw refuseBuildV1("invalid","closure.graph-invalid",{part:"pool"});return internJsonStringsV1(pool.bases,graphStringInternerV1);};

/** 🧾️ The head a directory holds, checked against the head schema, with the SHA-256 of its text; undefined when there is none. */
function headOfV1(text){
 if(text===undefined)return undefined;
 let head;try{head=JSON.parse(text);}catch{return undefined;}
 if(validateJsonSchemaSubset(HEAD_SCHEMA_V1,head).length)throw refuseBuildV1("invalid","closure.graph-invalid",{part:"head"});
 return{head,digest:sha(text)};
}

/** 📂️ Loads the graph `directory` holds, chunk by chunk (each checked against its digest and the chunk schema), into a new graph whose `head` is the SHA-256 of the head it was loaded from; undefined when there is no head; a chunk that is missing or does not match its digest refuses the whole graph (`closure.graph-invalid`), so a caller builds it anew. `progress({done,total})` is awaited after each chunk; `signal` cancels between chunks. */
export async function loadClosureGraphV1(directory,{signal,progress}={}){
 const read=headOfV1(await readFile(join(directory,"head.json"),"utf8").catch(()=>undefined));if(read===undefined)return undefined;
 const graph=createClosureGraphV1(),total=read.head.chunks.length;
 let bases;try{bases=poolBasesV1(read.head,name=>readFileSync(join(directory,"c",name)));}catch(error){throw error?.code==="ENOENT"?refuseBuildV1("invalid","closure.graph-invalid",{part:"pool"}):error;}
 for(let at=0;at<total;at++){
  signal?.throwIfAborted();
  const digest=read.head.chunks[at];
  if(digest!==null){let bytes;try{bytes=await readFile(join(directory,"c",digest+EXTENSION));}catch{throw refuseBuildV1("invalid","closure.graph-invalid",{part:"chunk"});}graph.update(chunkRecordsV1(bytes,digest,bases));}
  await progress?.({done:at+1,total});
 }
 return Object.assign(Object.create(graph),{head:read.digest});
}

/** 📂️ The same load without turns, for a synchronous judge that must answer while it is asked (one read of the head and its chunks, a few megabytes for a whole tree). */
export function loadClosureGraphSyncV1(directory){
 let text;try{text=readFileSync(join(directory,"head.json"),"utf8");}catch{return undefined;}
 const read=headOfV1(text);if(read===undefined)return undefined;
 const graph=createClosureGraphV1();
 let bases;try{bases=poolBasesV1(read.head,name=>readFileSync(join(directory,"c",name)));}catch(error){throw error?.code==="ENOENT"?refuseBuildV1("invalid","closure.graph-invalid",{part:"pool"}):error;}
 for(const digest of read.head.chunks)if(digest!==null){let bytes;try{bytes=readFileSync(join(directory,"c",digest+EXTENSION));}catch{throw refuseBuildV1("invalid","closure.graph-invalid",{part:"chunk"});}graph.update(chunkRecordsV1(bytes,digest,bases));}
 return Object.assign(Object.create(graph),{head:read.digest});
}
