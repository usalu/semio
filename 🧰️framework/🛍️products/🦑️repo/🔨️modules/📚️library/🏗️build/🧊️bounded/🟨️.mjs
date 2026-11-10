import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🧊️ The declared constants of the bounded caches; the schema file is the single source. */
export const BOUNDED_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
const fail=message=>Error(message),registry=new Map();

/** 📊️ The statistics of every registered cache, ordered by name: what the daemon status and the bench read. */
export function boundedCacheStatsV1(){return[...registry.values()].map(cache=>cache.stats()).sort((a,b)=>a.name<b.name?-1:a.name>b.name?1:0);}

/** 🗜️ Evicts the oldest entries of every registered cache down to `fraction` of its entries (0 empties them) and answers the freed bytes: what the memory governor calls under pressure so memory shrinks instead of growing. */
export function shrinkBoundedCachesV1(fraction){let freed=0;for(const cache of registry.values())freed+=cache.shrink(fraction);return freed;}

/**
 * 🧊️ Creates a least recently used cache with a finite entry bound and a finite byte bound: growth is impossible, the oldest entries leave first and `onEvict` sees each one. A value larger than `maxBytes` is rejected, never stored. `load` shares one loader call between concurrent callers of a key and honours cancellation. A named cache is listed by `boundedCacheStatsV1`.
 * @param {{name?:string;maxEntries:number;maxBytes:number;sizeOf?:(value:any,key:any)=>number;onEvict?:(key:any,value:any)=>void}} options
 */
export function createBoundedCacheV1({name,maxEntries,maxBytes,sizeOf=()=>1,onEvict}){
 if(!Number.isSafeInteger(maxEntries)||maxEntries<1||!Number.isSafeInteger(maxBytes)||maxBytes<1)throw fail("Finite entry and byte bounds required");
 const entries=new Map(),flights=new Map();let oldest,newest,bytes=0,hits=0,misses=0,evictions=0,rejected=0,loads=0;
 const unlink=node=>{if(node.older)node.older.newer=node.newer;else oldest=node.newer;if(node.newer)node.newer.older=node.older;else newest=node.older;node.older=node.newer=undefined;};
 const append=node=>{node.older=newest;node.newer=undefined;if(newest)newest.newer=node;else oldest=node;newest=node;};
 const touch=node=>{if(node!==newest){unlink(node);append(node);}};
 const drop=key=>{const node=entries.get(key);if(!node)return false;entries.delete(key);unlink(node);bytes-=node.size;return node;};
 const evict=()=>{while(entries.size>maxEntries||bytes>maxBytes){const node=drop(oldest.key);evictions++;onEvict?.(node.key,node.value);}};
 const cache={
  get(key){const node=entries.get(key);if(!node){misses++;return undefined;}hits++;touch(node);return node.value;},
  peek(key){return entries.get(key)?.value;},
  has(key){return entries.has(key);},
  set(key,value){
   const size=sizeOf(value,key);if(!Number.isSafeInteger(size)||size<0)throw fail("Non negative integer size required");
   drop(key);if(size>maxBytes){rejected++;return false;}
   const node={key,value,size,older:undefined,newer:undefined};entries.set(key,node);append(node);bytes+=size;evict();return true;
  },
  delete(key){return drop(key)!==false;},
  clear(){while(oldest){const node=drop(oldest.key);evictions++;onEvict?.(node.key,node.value);}},
  shrink(fraction){
   if(!(fraction>=0&&fraction<=1))throw fail("Fraction between 0 and 1 required");
   const keep=Math.floor(entries.size*fraction);let freed=0;
   while(entries.size>keep){const node=drop(oldest.key);freed+=node.size;evictions++;onEvict?.(node.key,node.value);}
   return freed;
  },
  keys(){const out=[];for(let node=oldest;node;node=node.newer)out.push(node.key);return out;},
  async load(key,loader,signal){
   signal?.throwIfAborted();
   const node=entries.get(key);if(node){hits++;touch(node);return node.value;}
   misses++;let flight=flights.get(key);
   if(!flight){loads++;flight=Promise.resolve().then(()=>loader(key)).then(value=>{cache.set(key,value);return value;}).finally(()=>flights.delete(key));flights.set(key,flight);}
   if(!signal)return flight;
   return new Promise((resolve,reject)=>{const stop=()=>reject(signal.reason);signal.addEventListener("abort",stop,{once:true});flight.then(resolve,reject).finally(()=>signal.removeEventListener("abort",stop));});
  },
  stats(){return{name:name??"",entries:entries.size,bytes,maxEntries,maxBytes,hits,misses,evictions,rejected,loads};},
 };
 if(name!==undefined){
  if(!registry.has(name)&&registry.size>=BOUNDED_POLICY_V1.maximumRegistered)registry.delete(registry.keys().next().value);
  registry.delete(name);registry.set(name,cache);
 }
 return Object.freeze(cache);
}
