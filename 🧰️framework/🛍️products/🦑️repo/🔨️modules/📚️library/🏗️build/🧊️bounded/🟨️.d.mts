/** 🧊️ The declared constants of the bounded caches. */
export const BOUNDED_POLICY_V1:Readonly<{version:1;maximumRegistered:number}>;
/** 📊️ The public statistics of one cache. */
export interface CacheStatsV1{readonly name:string;readonly entries:number;readonly bytes:number;readonly maxEntries:number;readonly maxBytes:number;readonly hits:number;readonly misses:number;readonly evictions:number;readonly rejected:number;readonly loads:number}
/** 🧊️ What a bounded cache is built from: both bounds are finite, `sizeOf` defaults to one byte per entry. */
export interface BoundedCacheOptionsV1<K,V>{name?:string;maxEntries:number;maxBytes:number;sizeOf?:(value:V,key:K)=>number;onEvict?:(key:K,value:V)=>void}
/** 🧊️ A least recently used cache that cannot grow beyond its bounds. */
export interface BoundedCacheV1<K,V>{get(key:K):V|undefined;peek(key:K):V|undefined;has(key:K):boolean;set(key:K,value:V):boolean;delete(key:K):boolean;clear():void;shrink(fraction:number):number;keys():K[];load(key:K,loader:(key:K)=>V|Promise<V>,signal?:AbortSignal):Promise<V>;stats():CacheStatsV1}
export function createBoundedCacheV1<K,V>(options:BoundedCacheOptionsV1<K,V>):BoundedCacheV1<K,V>;
export function boundedCacheStatsV1():CacheStatsV1[];
export function shrinkBoundedCachesV1(fraction:number):number;
