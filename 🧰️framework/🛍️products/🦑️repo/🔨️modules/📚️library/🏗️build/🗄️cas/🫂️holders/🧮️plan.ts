/** 🫂️ One entry of a reference holder as the eviction plan sees it: a stable id, when it was last useful and the objects it names. */
export type CasHolderPlanEntryV1=Readonly<{id:string;access:number;digests:readonly string[];holder?:string}>;
/** 🫂️ One object the holders reference: its size and whether nothing but those holders keeps it (no pin, no lease, no hard link, older than the minimum age). */
export type CasHolderPlanBlobV1=Readonly<{size:number;freeable:boolean}>;
/** 🫂️ What the capacity eviction takes: the entries (in eviction order), the objects they alone referenced and the bytes those free. */
export type CasHolderPlanV1=Readonly<{entries:string[];digests:string[];freedBytes:number;protectedEntries:number}>;

/** 🫂️ The entries a holder keeps whatever the need: its most recently used live entries until the objects they name reach `floorBytes` (the entry that crosses the floor is kept whole); an entry that already lost an object is dead and never protected. Pure and deterministic. */
export function protectCasHolderEntriesV1(entries:readonly CasHolderPlanEntryV1[],blobs:ReadonlyMap<string,CasHolderPlanBlobV1>,floors:ReadonlyMap<string,number>):Set<string>{
 const kept=new Set<string>(),byHolder=new Map<string,CasHolderPlanEntryV1[]>();
 for(const entry of entries)if(entry.holder!==undefined&&(floors.get(entry.holder)??0)>0)byHolder.set(entry.holder,[...byHolder.get(entry.holder)??[],entry]);
 for(const [holder,rows] of byHolder){
  const seen=new Set<string>();let bytes=0;
  for(const entry of [...rows].sort((left,right)=>right.access-left.access||(left.id<right.id?-1:left.id>right.id?1:0))){
   if(bytes>=floors.get(holder)!)break;
   const own=[...new Set(entry.digests)];if(own.some(digest=>!blobs.has(digest)))continue;
   kept.add(entry.id);
   for(const digest of own)if(!seen.has(digest)){seen.add(digest);bytes+=blobs.get(digest)!.size;}
  }
 }
 return kept;
}

/** 🫂️ Plans the capacity eviction of reference holders, the last stage after every unreferenced object is gone and only for the store's own cap: least recently used entries first, ties by id; an entry is taken only when it frees an object nobody else references (or is already dead because one of its objects is gone), and the walk repeats until the `need` bytes are free or no entry qualifies, so entries that share objects fall together. Pure and deterministic.
 * @see ../🧬️schema/🔣️.json */
export function planCasHolderEvictionV1(entries:readonly CasHolderPlanEntryV1[],blobs:ReadonlyMap<string,CasHolderPlanBlobV1>,need:number,floors:ReadonlyMap<string,number>=new Map()):CasHolderPlanV1{
 const shielded=protectCasHolderEntriesV1(entries,blobs,floors);
 if(!(need>0))return{entries:[],digests:[],freedBytes:0,protectedEntries:shielded.size};
 const refs=new Map<string,number>();
 for(const entry of entries)for(const digest of new Set(entry.digests))refs.set(digest,(refs.get(digest)??0)+1);
 const order=[...entries].sort((left,right)=>left.access-right.access||(left.id<right.id?-1:left.id>right.id?1:0)),gone=new Set<string>(),chosen:string[]=[],digests:string[]=[];
 let freed=0;
 for(let progress=true;progress&&freed<need;){
  progress=false;
  for(const entry of order){
   if(freed>=need)break;
   if(gone.has(entry.id)||shielded.has(entry.id))continue;
   const own=[...new Set(entry.digests)];
   if(!own.some(digest=>!blobs.has(digest))&&!own.some(digest=>refs.get(digest)===1&&blobs.get(digest)?.freeable===true))continue;
   gone.add(entry.id);chosen.push(entry.id);progress=true;
   for(const digest of own){const left=(refs.get(digest)??1)-1;refs.set(digest,left);const blob=blobs.get(digest);if(left===0&&blob?.freeable===true){freed+=blob.size;digests.push(digest);}}
  }
 }
 return{entries:chosen,digests,freedBytes:freed,protectedEntries:shielded.size};
}
