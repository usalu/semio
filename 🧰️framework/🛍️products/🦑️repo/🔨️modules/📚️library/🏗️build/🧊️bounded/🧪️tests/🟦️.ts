/** 🧊️ Contract: a bounded cache never exceeds its entry or byte bound, evicts least recently used first, rejects oversized values, shares concurrent loads and honours cancellation; its counters equal the independent Python model and the language-agnostic fixture on every operation trace. */
import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{boundedCacheStatsV1,createBoundedCacheV1,shrinkBoundedCachesV1}from"../🟨️.mjs";

type Operation=["set",string,number]|["get",string]|["peek",string]|["delete",string];
interface Case{id:string;maxEntries:number;maxBytes:number;operations:Operation[];expected:{hits:number;misses:number;evictions:number;rejected:number;bytes:number;keys:string[]}}
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),oracle=join(here,"🐍️oracle","🐍️.py"),fixture=JSON.parse(readFileSync(join(here,"🔣️.json"),"utf8")) as{cases:Case[]};
const replay=(row:{maxEntries:number;maxBytes:number;operations:Operation[]},check?:(cache:ReturnType<typeof createBoundedCacheV1<string,number>>)=>void)=>{
 const cache=createBoundedCacheV1<string,number>({maxEntries:row.maxEntries,maxBytes:row.maxBytes,sizeOf:value=>value});
 for(const operation of row.operations){
  if(operation[0]==="set")cache.set(operation[1],operation[2]);else if(operation[0]==="get")cache.get(operation[1]);else if(operation[0]==="peek")cache.peek(operation[1]);else cache.delete(operation[1]);
  check?.(cache);
 }
 const stats=cache.stats();return{hits:stats.hits,misses:stats.misses,evictions:stats.evictions,rejected:stats.rejected,bytes:stats.bytes,keys:cache.keys()};
};

test("every fixture trace ends in the expected counters and order, and the bounds hold after every operation",()=>{
 for(const row of fixture.cases)expect(replay(row,cache=>{const stats=cache.stats();expect(stats.entries).toBeLessThanOrEqual(row.maxEntries);expect(stats.bytes).toBeLessThanOrEqual(row.maxBytes);}),row.id).toEqual(row.expected);
});

test("random traces equal the independent Python least recently used model",()=>{
 if(!python)return;
 let seed=12345;const next=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const cases=Array.from({length:12},(_,index)=>({maxEntries:1+index%7,maxBytes:5+index*9,operations:Array.from({length:600},()=>{const key=String(Math.floor(next()*30)),pick=next();return(pick<.5?["set",key,Math.floor(next()*14)]:pick<.9?["get",key]:["delete",key]) as Operation;})}));
 const directory=mkdtempSync(join(tmpdir(),"bounded-"));
 try{const path=join(directory,"corpus.json");writeFileSync(path,JSON.stringify({cases}));const done=Bun.spawnSync([python,"-I",oracle,"replay",path]);expect(done.exitCode).toBe(0);const expected=JSON.parse(done.stdout.toString()) as Case["expected"][];for(const[index,row]of cases.entries())expect(replay(row),String(index)).toEqual(expected[index]);}
 finally{rmSync(directory,{recursive:true,force:true});}
});

test("get promotes, peek and has do not, onEvict sees every eviction and clear",()=>{
 const evicted:string[]=[],cache=createBoundedCacheV1<string,number>({maxEntries:2,maxBytes:100,onEvict:key=>{evicted.push(key);}});
 cache.set("a",1);cache.set("b",2);cache.peek("a");cache.has("a");cache.set("c",3);
 expect(evicted).toEqual(["a"]);cache.get("b");cache.set("d",4);expect(evicted).toEqual(["a","c"]);cache.clear();expect(evicted).toEqual(["a","c","b","d"]);expect(cache.stats().entries).toBe(0);expect(cache.stats().bytes).toBe(0);
});

test("unbounded or fractional bounds and negative sizes are refused",()=>{
 expect(()=>createBoundedCacheV1({maxEntries:Infinity,maxBytes:1})).toThrow();
 expect(()=>createBoundedCacheV1({maxEntries:1,maxBytes:1.5})).toThrow();
 expect(()=>createBoundedCacheV1({maxEntries:0,maxBytes:1})).toThrow();
 expect(()=>createBoundedCacheV1<string,number>({maxEntries:1,maxBytes:5,sizeOf:()=>-1}).set("a",1)).toThrow();
});

test("concurrent loads of a key share one loader call, a failure is not cached and an abort rejects only its caller",async()=>{
 const cache=createBoundedCacheV1<string,number>({maxEntries:4,maxBytes:10});let calls=0;
 const loader=async()=>{calls++;await Bun.sleep(5);return 7;};
 expect(await Promise.all([cache.load("k",loader),cache.load("k",loader),cache.load("k",loader)])).toEqual([7,7,7]);expect(calls).toBe(1);expect(cache.stats().loads).toBe(1);expect(await cache.load("k",loader)).toBe(7);expect(calls).toBe(1);
 await expect(cache.load("bad",async()=>{throw Error("boom");})).rejects.toThrow("boom");expect(cache.has("bad")).toBe(false);
 const controller=new AbortController(),pending=cache.load("slow",async()=>{await Bun.sleep(20);return 1;},controller.signal);controller.abort(Error("stop"));
 await expect(pending).rejects.toThrow("stop");expect(await cache.load("slow",async()=>2)).toBe(1);
 const early=new AbortController();early.abort();await expect(cache.load("x",loader,early.signal)).rejects.toThrow();
});

test("named caches are listed by name and the registry is bounded",()=>{
 createBoundedCacheV1({name:"test:b",maxEntries:1,maxBytes:1});createBoundedCacheV1({name:"test:a",maxEntries:2,maxBytes:2});
 const names=boundedCacheStatsV1().map(row=>row.name);expect(names.indexOf("test:a")).toBeLessThan(names.indexOf("test:b"));
 for(let index=0;index<400;index++)createBoundedCacheV1({name:"flood:"+index,maxEntries:1,maxBytes:1});expect(boundedCacheStatsV1().length).toBeLessThanOrEqual(256);
});

test("shrink evicts the oldest entries down to a fraction, the registry shrink reaches every named cache",()=>{
 const evicted:number[]=[],cache=createBoundedCacheV1<number,number>({name:"test:shrink",maxEntries:10,maxBytes:1000,sizeOf:value=>value,onEvict:key=>{evicted.push(key);}});
 for(let key=0;key<10;key++)cache.set(key,10);
 expect(cache.shrink(0.5)).toBe(50);expect(evicted).toEqual([0,1,2,3,4]);expect(cache.keys()).toEqual([5,6,7,8,9]);
 expect(()=>cache.shrink(2)).toThrow();expect(shrinkBoundedCachesV1(0)).toBeGreaterThanOrEqual(50);expect(cache.stats().entries).toBe(0);expect(cache.stats().bytes).toBe(0);
});
