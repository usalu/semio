import{canonicalKeyV1,compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
import{INDEX_CHUNK_POLICY_V1,addOwnerRunsV1,bulkUpsertClassesV1,classesOfChunksV1,createChunkInternerV1}from"../🧱️chunk-classes/🟨️.mjs";
import{P,aborted,createYielderV1,fail,requireKeys}from"../🧱️guards/🟨️.mjs";

const rowKeys=new WeakMap();
/** 🪪️ The identity of one observation row: its canonical text, or `#` and the SHA-256 of that text when the text is longer than `ROW_KEY_TEXT_LIMIT_V1` (an analysis row carries kilobytes of nested facts, and a build keys every row of a graph: the key stays below 70 bytes however large the fact is); two rows have one key exactly when their canonical texts are equal, and a shared row object is keyed once. */
export function rowKeyV1(row){let key=rowKeys.get(row);if(key===undefined){key=canonicalKeyV1([row.kind,row.path,row.fact],ROW_KEY_TEXT_LIMIT_V1);rowKeys.set(row,key);}return key;}
export const ROW_KEY_TEXT_LIMIT_V1=P.rowKeyTextLimit;

const ownerIdMaps=new WeakMap();
const ownerIdMap=index=>{let map=ownerIdMaps.get(index);if(!map){map=new Map(index.owners.map((name,id)=>[name,id]));ownerIdMaps.set(index,map);}return map;};

/** 🗃️ Builds the inverted fact index: rows are interned once, and owners partition the pairs into classes of rows that share exactly the same consumers, so one stale row names its consumers without reading any fragment. */
export function createFactIndexV1(ownerRows){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],byKey=new Map(),byObject=new Map(),empty=[],interner=createChunkInternerV1();let seen=new Int32Array(1<<16),scratch=new Uint32Array(1<<12);
 owners.forEach((name,owner)=>{
  const list=ownerRows.get(name);if(scratch.length<list.length)scratch=new Uint32Array(Math.max(list.length,scratch.length*2));
  let count=0;
  for(const row of list){
   let id=byObject.get(row);
   if(id===undefined){const key=rowKeyV1(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}
   if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+1));grown.set(seen);seen=grown;}
   if(seen[id]!==owner+1){seen[id]=owner+1;scratch[count++]=id;}
  }
  if(!count){empty.push({r:[],o:[owner]});return;}
  addOwnerRunsV1(interner,scratch,count,owner);
 });
 return{version:1,owners,rows,classes:[...classesOfChunksV1(rows.length,interner),...empty]};
}

/** 🩹️ The in-memory model of the patch of a decoded fact index (a build patches the packed bytes instead, `../🌊️stream-patch`, and its tests compare the two). Replaces the consumers in `upserts` (their old classes disappear), adds rows to the consumers in `append`, drops `drop`, prunes unreferenced rows and rebuilds the partition once classes outgrow the owners or the class lists hold more than `compactionDuplication` times the rows (the duplication of an index patched generation after generation, which no comparison with the previous generation can see). */
export function patchFactIndexV1(index,{drop=[],upserts=new Map(),append=new Map()},{bulkRows=INDEX_CHUNK_POLICY_V1.bulkUpsertRows}={}){
 const ownerIds=new Map(ownerIdMap(index)),rows=index.rows.slice(),owners=index.owners.slice(),original=index.rows.length,byPath=pathIndexOf(index),added=new Map();
 const idOf=name=>{let id=ownerIds.get(name);if(id===undefined){id=owners.length;owners.push(name);ownerIds.set(name,id);}return id;};
 const touched=new Set(),gone=new Set();
 for(const name of drop)if(ownerIds.has(name)){touched.add(ownerIds.get(name));if(!upserts.has(name))gone.add(ownerIds.get(name));}
 for(const name of upserts.keys())touched.add(idOf(name));
 const classes=[];
 for(const cls of index.classes){const kept=cls.o.filter(id=>!touched.has(id));if(kept.length)classes.push({r:cls.r,o:kept});}
 const idOfRow=row=>{
  const key=rowKeyV1(row);let id=added.get(key);if(id!==undefined)return id;
  const candidates=byPath.get(row.path);
  if(candidates!==undefined)for(const candidate of typeof candidates==="number"?[candidates]:candidates)if(rows[candidate].kind===row.kind&&rowKeyV1(rows[candidate])===key)return candidate;
  id=rows.length;rows.push(row);added.set(key,id);return id;
 };
 const rowIdsFor=list=>{const out=new Set();for(const row of list)out.add(idOfRow(row));return[...out].sort((a,b)=>a-b);};
 let pairs=0;for(const list of upserts.values())pairs+=list.length;
 if(pairs>=bulkRows)bulkUpsertClassesV1({upserts,ownerIds,idOfRow,rowCount:rows.length,classes});
 else
 for(const name of[...upserts.keys()].sort(compareCodePointsV1))classes.push({r:rowIdsFor(upserts.get(name)),o:[ownerIds.get(name)]});
 for(const name of[...append.keys()].sort(compareCodePointsV1)){const extra=rowIdsFor(append.get(name));classes.push({r:extra,o:[idOf(name)]});}
 const used=new Uint8Array(rows.length),live=new Uint8Array(owners.length);
 for(const cls of classes){for(const id of cls.r)used[id]=1;for(const id of cls.o)if(!gone.has(id))live[id]=1;}
 const rowMap=new Int32Array(rows.length).fill(-1),ownerMap=new Int32Array(owners.length).fill(-1),rowsOut=[],ownersOut=[];
 for(let id=0;id<rows.length;id++)if(used[id]){rowMap[id]=rowsOut.length;rowsOut.push(rows[id]);}
 for(let id=0;id<owners.length;id++)if(live[id]){ownerMap[id]=ownersOut.length;ownersOut.push(owners[id]);}
 const compact=classes.filter(cls=>cls.o.some(id=>live[id])).map(cls=>({r:cls.r.map(id=>rowMap[id]),o:cls.o.filter(id=>live[id]).map(id=>ownerMap[id])}));
 const result={version:1,owners:ownersOut,rows:rowsOut,classes:compact};
 const ids=result.classes.reduce((sum,cls)=>sum+cls.r.length,0);
 return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor&&result.classes.length>P.compactionGrowth*index.classes.length||ids>P.compactionFloor&&ids>P.compactionDuplication*result.rows.length?refineFactIndexV1(result):result;
}

const pathIndexes=new WeakMap();
/** 🔎️ The rows of an index by path (a row id, or the ids of the rows that share the path), read once per index: finds the stored twin of a row from its path and kind, so a patch compares the facts of a handful of candidates instead of building the identity of every stored row. */
function pathIndexOf(index){
 let map=pathIndexes.get(index);if(map)return map;
 map=new Map();
 for(let id=0;id<index.rows.length;id++){const path=index.rows[id].path,held=map.get(path);if(held===undefined)map.set(path,id);else if(typeof held==="number")map.set(path,[held,id]);else held.push(id);}
 pathIndexes.set(index,map);return map;
}

/** ⚖️ Merges the classes of an index into the coarsest exact partition: rows with the same consumers share one class. */
export function refineFactIndexV1(index){
 const rowCount=index.rows.length,start=new Uint32Array(rowCount+1);
 for(const cls of index.classes)for(const id of cls.r)start[id+1]+=cls.o.length;
 for(let id=0;id<rowCount;id++)start[id+1]+=start[id];
 const flat=new Uint32Array(start[rowCount]),fill=start.slice(0,rowCount);
 for(const cls of index.classes)for(const id of cls.r)for(const owner of cls.o)flat[fill[id]++]=owner;
 const groups=new Map(),classes=[];
 for(let id=0;id<rowCount;id++){
  const list=flat.subarray(start[id],start[id+1]).sort();let unique=0;
  for(let at=0;at<list.length;at++)if(at===0||list[at]!==list[at-1])list[unique++]=list[at];
  let hash=unique|0;for(let at=0;at<unique;at++)hash=Math.imul(hash^list[at],0x01000193)>>>0;
  let bucket=groups.get(hash);if(bucket===undefined)groups.set(hash,bucket=[]);
  let group=bucket.find(candidate=>candidate.o.length===unique&&candidate.o.every((owner,at)=>owner===list[at]));
  if(!group){group={r:[],o:Array.from(list.subarray(0,unique))};bucket.push(group);classes.push(group);}
  group.r.push(id);
 }
 const covered=new Set(classes.flatMap(cls=>cls.o));
 for(const cls of index.classes)for(const owner of cls.o)if(!covered.has(owner)){covered.add(owner);classes.push({r:[],o:[owner]});}
 return{version:1,owners:index.owners,rows:index.rows,classes};
}

/** 📚️ The sorted consumers of every row, as `[rowKey, names]` pairs. */
export function factConsumersV1(index){
 const consumers=index.rows.map(()=>new Set());
 for(const cls of index.classes)for(const id of cls.r)for(const owner of cls.o)consumers[id].add(index.owners[owner]);
 return index.rows.map((row,id)=>[rowKeyV1(row),[...consumers[id]].sort(compareCodePointsV1)]);
}

/** 🕵️ Finds exactly the stale rows among `ids`: a holding chunk is cleared by one call, a failing one is bisected within a call budget, then checked row by row. */
export async function staleFactRowsV1(index,ids,hold,{signal,progress,chunk=P.verifyChunkRows,parallel=P.verifyParallelChunks,budget=P.bisectBudgetCalls}={}){
 const stale=new Set(),chunks=[],yielder=createYielderV1();let done=0,calls=0,next=0;
 for(let at=0;at<ids.length;at+=chunk)chunks.push(ids.slice(at,at+chunk));
 const check=async list=>{aborted(signal);calls++;return hold(list.map(id=>index.rows[id]),signal);};
 const find=async(list,state)=>{
  if(await check(list))return;
  if(list.length===1){stale.add(list[0]);return;}
  if(state.left<=0){for(const id of list)if(!await check([id]))stale.add(id);return;}
  state.left-=2;const mid=list.length>>1;await find(list.slice(0,mid),state);await find(list.slice(mid),state);
 };
 await progress?.({phase:"facts",done:0,total:ids.length});
 await Promise.all(Array.from({length:Math.min(parallel,chunks.length)},async()=>{
  while(next<chunks.length){const list=chunks[next++];await yielder();await find(list,{left:budget});done+=list.length;await progress?.({phase:"facts",done,total:ids.length});}
 }));
 return{stale,calls};
}

/** 🧭️ Selects the classes naming one of `only` and the distinct rows they list. */
export function selectFactClassesV1(index,only){
 const map=ownerIdMap(index),wanted=only?new Set([...only].map(name=>map.get(name)).filter(id=>id!==undefined)):undefined,classes=[],used=new Uint8Array(index.rows.length),ids=[];
 index.classes.forEach((cls,at)=>{if(wanted&&!cls.o.some(id=>wanted.has(id)))return;classes.push(at);for(const id of cls.r)if(!used[id]){used[id]=1;ids.push(id);}});
 return{classes,ids};
}

/** 🧱️ Consumers of the stale rows in the classes. */
export function staleConsumersV1(index,classes,stale){
 const names=new Set();
 for(const at of classes){const cls=index.classes[at];if(cls.r.some(id=>stale.has(id)))for(const owner of cls.o)names.add(index.owners[owner]);}
 return names;
}
