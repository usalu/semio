const YIELD_OWNERS_V1=64;

/** 🧭️ Whether a row names one of `paths` or the listing of the directory a path lives in; the one rule by which a moved path finds the owners that read it (the owner facts memo of the loader and the taken memo of the owner loop evict by it).
 * @see ../../../../📥️load/🟦️.ts `evictOwnerFactsReferencingV1` */
export function createPathRowMatcherV1(paths){
 const named=new Set(paths),listings=new Set(paths.map(path=>path.slice(0,Math.max(path.lastIndexOf("/"),path.lastIndexOf("\\")))));
 return row=>{const{kind,path}=row;return named.has(path)||(kind==="readdir"||kind==="readdirNames")&&listings.has(path);};
}

/** ♻️ The owners one read took in this process, kept from one pass over the owners to the next: an owner is answered from here only while the key of its job is the same and every row its record consumed still holds, which is the question a full look answers by reading its stored record again; the rows are retained into the running capture exactly as a look retains them, so the final verification of the pass covers them.
 * A trusted entry (every survivor of a pass that ended complete, after the moved paths evicted theirs) is answered without verifying its rows again until `distrust()` (any pass that did not end complete, or an attempt that carries the memo over) clears the marks, the same rule as the trust set of the inventory.
 * The answer of an owner is kept as its JSON text, so a pass never shares a project object another pass may have changed.
 * @see ./🧪️tests/🟦️.ts */
export function createTakenMemoV1({holds,retain}){
 if(typeof holds!=="function"||typeof retain!=="function")throw Error("Row verification and row retention ports required");
 const entries=new Map(),counters={remembered:0,reused:0,evictedNamed:0,evictedChanged:0,evictedKey:0,passes:0,trusted:0};
 return Object.freeze({
  remember(name,{key,project,rows}){
   if(typeof key!=="string"||!Array.isArray(rows)||rows.length===0)return false;
   entries.set(name,{key,text:JSON.stringify(project),rows,trusted:false});counters.remembered++;return true;
  },
  evictReferencing(paths){
   if(paths.length===0)return 0;
   const matches=createPathRowMatcherV1(paths);let evicted=0;
   for(const[name,entry]of[...entries])if(entry.rows.some(matches)){entries.delete(name);evicted++;}
   counters.evictedNamed+=evicted;return evicted;
  },
  async replay(wanted,{signal}={}){
   counters.passes++;const reused=[];let visited=0;
   for(const{name,key}of wanted){
    const entry=entries.get(name);if(entry===undefined)continue;
    if(visited++%YIELD_OWNERS_V1===YIELD_OWNERS_V1-1){signal?.throwIfAborted();await new Promise(done=>setImmediate(done));}
    if(entry.key!==key){entries.delete(name);counters.evictedKey++;continue;}
    if(entry.trusted)counters.trusted++;else if(!holds(entry.rows)){entries.delete(name);counters.evictedChanged++;continue;}
    retain(entry.rows);counters.reused++;reused.push({name,project:JSON.parse(entry.text),rows:entry.rows});
   }
   return reused;
  },
  trustSurvivors(){let marked=0;for(const entry of entries.values())if(!entry.trusted){entry.trusted=true;marked++;}return marked;},
  distrust(){let cleared=0;for(const entry of entries.values())if(entry.trusted){entry.trusted=false;cleared++;}return cleared;},
  forget(name){return entries.delete(name);},
  clear(){entries.clear();},
  get size(){return entries.size;},
  stats(){return{entries:entries.size,...counters};},
 });
}
