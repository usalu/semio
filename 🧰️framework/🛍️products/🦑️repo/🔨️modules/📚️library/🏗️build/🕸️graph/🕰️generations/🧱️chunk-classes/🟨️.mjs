/** 🧱️ Builds the classes of the fact index from runs of row numbers instead of from the pairs (owner, row): owners that share a long block of rows (the command closure that thousands of projects carry) cut it at the same rows, the runs are interned once and only the owners that hold each run are kept, so the construction costs memory by distinct run and not by owner times row. */
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";

/** 🗺️ The declared constants of the chunked construction; the schema file is the single source and a law keeps both equal. */
export const INDEX_CHUNK_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
const P=INDEX_CHUNK_POLICY_V1;

/** 🧩️ Interns runs of row numbers and remembers which owners hold each: an owner's list is cut after every row whose number hashes to zero in `chunkCutBits` bits, so owners that share a long block cut it at the same rows and hold the same runs; the pairs of (owner, row) of thousands of owners that share a block are then never expanded. */
export function createChunkInternerV1(){
 const index=new Map(),chunks=[],holders=[],shift=32-P.chunkCutBits;
 return{
  chunks,holders,
  cuts:number=>Math.imul(number,0x9e3779b1)>>>shift===0,
  add(numbers,from,to,owner){
   const length=to-from;let hash=length;for(let at=from;at<to;at++)hash=(Math.imul(hash,0x9e3779b1)^numbers[at])>>>0;
   const bucket=index.get(hash);let found=-1;
   if(bucket!==undefined)for(const candidate of typeof bucket==="number"?[bucket]:bucket){const held=chunks[candidate];if(held.length!==length)continue;let same=true;for(let at=0;at<length;at++)if(held[at]!==numbers[from+at]){same=false;break;}if(same){found=candidate;break;}}
   if(found<0){found=chunks.length;chunks.push(numbers.slice(from,to));holders.push([]);if(bucket===undefined)index.set(hash,found);else if(typeof bucket==="number")index.set(hash,[bucket,found]);else bucket.push(found);}
   holders[found].push(owner);
  },
 };
}

/** ✂️ Cuts the first `count` row numbers of `numbers` (one owner's distinct rows in the order they were met) into runs and interns them for `owner`. */
export function addOwnerRunsV1(interner,numbers,count,owner){
 let from=0;
 for(let at=0;at<count;at++)if(at===count-1||interner.cuts(numbers[at])){interner.add(numbers,from,at+1,owner);from=at+1;}
}

/** 🗃️ The classes of an index over `rowCount` rows from the interned runs (with `typed`, the consumers of a class that unites several runs are merged in typed lists, four bytes per consumer, instead of a set and a list per class): rows -> runs -> classes (rows with equal owner sets merge, sorted by their first row), the same partition the expanded construction gives. */
export function classesOfChunksV1(rowCount,interner,{typed=false}={}){
 const{chunks,holders}=interner,first=new Uint32Array(rowCount+1);
 for(const ids of chunks)for(const id of ids)first[id+1]++;
 for(let id=0;id<rowCount;id++)first[id+1]+=first[id];
 const held=new Uint32Array(first[rowCount]),fill=first.slice(0,rowCount);
 chunks.forEach((ids,chunk)=>{for(const id of ids)held[fill[id]++]=chunk;});
 const buckets=new Map(),groups=[];
 for(let id=0;id<rowCount;id++){
  const from=first[id],to=first[id+1];let hash=to-from;
  for(let at=from;at<to;at++)hash=(Math.imul(hash,0x9e3779b1)^held[at])>>>0;
  let chain=buckets.get(hash),group;
  if(chain===undefined)buckets.set(hash,chain=[]);
  for(const candidate of chain){const other=candidate.span;if(other[1]-other[0]!==to-from)continue;let same=true;for(let at=0;at<to-from;at++)if(held[other[0]+at]!==held[from+at]){same=false;break;}if(same){group=candidate;break;}}
  if(group===undefined){group={r:[],span:[from,to]};chain.push(group);groups.push(group);}
  group.r.push(id);
 }
 const merged=new Map(),classes=[];
 for(const group of groups){
  const[from,to]=group.span;let o;
  if(to-from===1)o=holders[held[from]];
  else if(typed){let total=0;for(let at=from;at<to;at++)total+=holders[held[at]].length;const all=new Uint32Array(total);let fill=0;for(let at=from;at<to;at++){all.set(holders[held[at]],fill);fill+=holders[held[at]].length;}all.sort();let unique=0;for(let at=0;at<total;at++)if(at===0||all[at]!==all[at-1])all[unique++]=all[at];o=all.subarray(0,unique);}
  else{const set=new Set();for(let at=from;at<to;at++)for(const owner of holders[held[at]])set.add(owner);o=[...set].sort((a,b)=>a-b);}
  let hash=o.length;for(const owner of o)hash=(Math.imul(hash,0x9e3779b1)^owner)>>>0;
  let chain=merged.get(hash),found;
  if(chain===undefined)merged.set(hash,chain=[]);
  for(const candidate of chain){const other=candidate.o;if(other.length!==o.length)continue;let same=true;for(let at=0;at<o.length;at++)if(other[at]!==o[at]){same=false;break;}if(same){found=candidate;break;}}
  if(found===undefined){found={r:group.r,o:o.slice()};chain.push(found);classes.push(found);}else for(const id of group.r)found.r.push(id);
 }
 for(const cls of classes)if(cls.r.length>1)cls.r.sort((a,b)=>a-b);
 classes.sort((a,b)=>a.r[0]-b.r[0]);
 return classes;
}

/** 📥️ The classes of a patch that upserts many pairs: each upserted owner's rows (`idOfRow` finds or adds the row number) are cut into runs like a whole index, and one class is pushed per run, so the patch never builds a class per owner out of a shared block. */
export function bulkUpsertClassesV1({upserts,ownerIds,idOfRow,rowCount,classes}){
 const interner=createChunkInternerV1();let seen=new Int32Array(rowCount+P.scratchSlackRows),scratch=new Uint32Array(P.scratchInitialEntries),stamp=0;
 for(const name of[...upserts.keys()].sort(compareCodePointsV1)){
  const list=upserts.get(name),owner=ownerIds.get(name);stamp++;if(scratch.length<list.length)scratch=new Uint32Array(Math.max(list.length,scratch.length*2));
  let count=0;
  for(const row of list){const id=idOfRow(row);if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+P.scratchSlackRows));grown.set(seen);seen=grown;}if(seen[id]!==stamp){seen[id]=stamp;scratch[count++]=id;}}
  addOwnerRunsV1(interner,scratch,count,owner);
  if(!count)classes.push({r:[],o:[owner]});
 }
 interner.chunks.forEach((ids,chunk)=>classes.push({r:Array.from(ids).sort((a,b)=>a-b),o:interner.holders[chunk].slice().sort((a,b)=>a-b)}));
}

/** 🚿️ The classes of a streamed upsert, pushed as typed arrays: every owner's rows arrive as batches (`rowBatches(name)`, an async iterable of arrays that may load them on demand, or of `Uint32Array`s that already hold the row numbers of the index), so only one owner's distinct row numbers and the interned runs are held, never the rows of all owners at once. With `bulk` the rows are cut into runs like `bulkUpsertClassesV1`, otherwise every owner gets one class of all its rows. */
export async function streamUpsertClassesV1({names,rowBatches,ownerIds,idOfRow,rowCount,classes,bulk,pause}){
 const interner=createChunkInternerV1();let seen=new Int32Array(rowCount+P.scratchSlackRows),scratch=new Uint32Array(P.scratchInitialEntries),stamp=0;
 for(const name of names){
  const owner=ownerIds.get(name);stamp++;let count=0;
  for await(const batch of rowBatches(name)){
   if(scratch.length<count+batch.length){const grown=new Uint32Array(Math.max(count+batch.length,scratch.length*2));grown.set(scratch.subarray(0,count));scratch=grown;}
   const numbered=batch instanceof Uint32Array;
   for(const item of batch){const id=numbered?item:idOfRow(item);if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+P.scratchSlackRows));grown.set(seen);seen=grown;}if(seen[id]!==stamp){seen[id]=stamp;scratch[count++]=id;}}
   await pause?.();
  }
  if(!count){classes.push({r:new Uint32Array(0),o:Uint32Array.of(owner)});continue;}
  if(bulk)addOwnerRunsV1(interner,scratch,count,owner);else classes.push({r:scratch.slice(0,count).sort(),o:Uint32Array.of(owner)});
 }
 interner.chunks.forEach((ids,chunk)=>classes.push({r:ids.sort(),o:Uint32Array.from(interner.holders[chunk]).sort()}));
}
