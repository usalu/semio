/** 🧠️ Contract: the dense bitset reach sets give exactly the members, the union and the membership of Python sets, equal sets share one buffer, the guard fails loudly, and a set of 22 thousand modules costs at most 2.75 KB. */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{createReachSetsV1}from"../🟨️.mjs";

const oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py"),python=process.env.SEMIO_PYTHON??"python3";
const cases=(()=>{let state=7;const draw=(limit:number)=>{state=(state*1103515245+12345)%2147483648;return(state>>8)%limit;};
 return Array.from({length:12},(_,at)=>({sets:Array.from({length:1+draw(8)},()=>Array.from({length:draw(60)},()=>draw(at<6?200:22000)))}));})();

test("members, union and membership equal the Python sets, equal sets share one buffer",()=>{
 const run=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify({cases}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const model=JSON.parse(run.stdout.toString()).cases as{sets:number[][];union:number[];distinct:number}[];
 for(const answer of model){
  const store=createReachSetsV1(),built=answer.sets.map(values=>store.of(values));
  built.forEach((set,at)=>{expect([...store.members(set)]).toEqual(answer.sets[at]);for(const value of answer.sets[at]!)expect(store.has(set,value)).toBe(true);expect(store.has(set,22001)).toBe(false);});
  expect([...store.members(store.union(built))]).toEqual(answer.union);
  expect(new Set(built).size).toBe(answer.distinct);
 }
});

test("a set over 22 thousand modules costs at most 2.75 KB and the guard fails loudly",()=>{
 const store=createReachSetsV1();expect(store.of(Array.from({length:22000},(_,at)=>at)).byteLength).toBeLessThanOrEqual(2752);
 const small=createReachSetsV1(1024);expect(()=>{for(let at=0;at<100;at++)small.of([at*64]);}).toThrow(/guard/);
});
