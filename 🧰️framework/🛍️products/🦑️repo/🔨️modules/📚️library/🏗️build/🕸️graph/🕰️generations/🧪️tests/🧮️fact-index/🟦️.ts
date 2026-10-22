/** 🧮️ Contract: the inverted fact index of a cold build equals the independent partition model (rows by first appearance, classes of identical consumers) for seeded random corpora with shared, duplicated and empty owners, and 3,000 owners are indexed without the per-row consumer strings of the first design. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import{canonicalJsonV1,createFactIndexV1}from"../../🟨️.mjs";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle.py");
type Row={kind:string;path:string;fact:unknown};

/** 🎲️ A deterministic generator (mulberry32) so the corpus is the same in every run and every language of the model. */
function random(seed:number){return()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};}

function corpus(seed:number,owners:number,pool:number,perOwner:number):Map<string,Row[]>{
 const next=random(seed),rows=Array.from({length:pool},(_,i)=>({kind:["file","readdir","exists"][i%3]!,path:"p/"+i,fact:{size:i,hash:"h"+(i%17)}})),result=new Map<string,Row[]>();
 for(let owner=0;owner<owners;owner++){
  const list:Row[]=[],count=owner%11===0?0:Math.floor(next()*perOwner);
  for(let item=0;item<count;item++){const base=rows[Math.floor(next()*pool*(next()<0.5?1:0.1))]!;list.push(next()<0.2?{...base,fact:{...(base.fact as object)}}:base);}
  if(owner%7===0)list.push(...list.slice(0,5));
  result.set("o"+((owner*7919)%owners).toString().padStart(5,"0"),list);
 }
 return result;
}

function model(rows:Map<string,Row[]>){
 const directory=mkdtempSync(join(tmpdir(),"fact-index-"));
 try{const path=join(directory,"corpus.json");writeFileSync(path,JSON.stringify({owners:Object.fromEntries(rows)}));const done=Bun.spawnSync([python!,"-I",oracle,"index",path]);expect(done.exitCode).toBe(0);return JSON.parse(done.stdout.toString());}
 finally{rmSync(directory,{recursive:true,force:true});}
}

for(const[seed,owners,pool,perOwner]of[[1,12,30,20],[2,60,200,80],[3,400,2000,300]] as const)test(`the fact index equals the partition model for seed ${seed} (${owners} owners, ${pool} distinct rows)`,()=>{
 const rows=corpus(seed,owners,pool,perOwner),actual=createFactIndexV1(rows);
 expect(actual.version).toBe(1);
 if(python){const{version,...rest}=actual;expect(JSON.parse(canonicalJsonV1(rest))).toEqual(JSON.parse(canonicalJsonV1(model(rows))));}
});

test("3,000 owners with 200 rows each are indexed in bounded time and every owner keeps its rows",()=>{
 const rows=corpus(9,3000,40000,400),started=performance.now(),index=createFactIndexV1(rows),seconds=(performance.now()-started)/1000;
 const consumers=new Map<string,Set<string>>();
 for(const cls of index.classes)for(const row of cls.r)for(const owner of cls.o){const key=canonicalJsonV1(index.rows[row]);(consumers.get(key)??consumers.set(key,new Set()).get(key)!).add(index.owners[owner]!);}
 for(const[name,list]of[...rows].filter((_,i)=>i%97===0))for(const row of list)expect(consumers.get(canonicalJsonV1(row))!.has(name)).toBe(true);
 expect(index.owners.length).toBe(3000);expect(seconds).toBeLessThan(30);
});
