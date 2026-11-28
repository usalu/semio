import {afterAll,expect,test} from "bun:test";
import {lstatSync,mkdtempSync,rmSync,writeFileSync,type Stats} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {StatBudgetV1,isStatsV1} from "../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const stampOf=(size:number):Stats=>({size,mtimeMs:size} as unknown as Stats);
const wait=(milliseconds:number):void=>{const until=performance.now()+milliseconds;while(performance.now()<until);};

let busy=0,blocked=0;
const spend=(milliseconds:number):void=>{wait(milliseconds);busy+=milliseconds;};

function turns(){
 busy=0;blocked=0;
 let active=true;
 const tick=():void=>{if(!active)return;blocked=Math.max(blocked,busy);busy=0;setImmediate(tick);};
 setImmediate(tick);
 return {stop:()=>{active=false;blocked=Math.max(blocked,busy);return blocked;}};
}

async function sweep(budget:StatBudgetV1,paths:readonly string[],sliceMilliseconds:number):Promise<number[]>{
 const sizes:number[]=[];
 for(let from=0;from<paths.length;){
  const upto=Math.min(paths.length,from+budget.chunk(sliceMilliseconds));
  for(const result of await budget.many(paths.slice(from,upto)))sizes.push(isStatsV1(result)?result.size:-1);
  from=upto;
 }
 return sizes;
}

test("a filesystem whose every stat takes 5 ms never blocks the loop for 100 ms, sync or async",async()=>{
 const paths=Array.from({length:600},(_,index)=>`/slow/${index}`);
 const budget=new StatBudgetV1({lstatSync:path=>{spend(5);return stampOf(Number(path.split("/").pop()));},lstat:path=>new Promise(done=>setTimeout(()=>done(stampOf(Number(path.split("/").pop()))),5))});
 const watch=turns(),started=performance.now(),sizes=await sweep(budget,paths,6),longest=watch.stop(),took=performance.now()-started;
 expect(sizes).toEqual(paths.map((_,index)=>index));
 expect(longest).toBeLessThan(100);
 expect(budget.mode().parallel).toBe(true);
 expect(took).toBeLessThan(600*5);
});

test("the old synchronous loop on the same slow filesystem blocks the loop far beyond 100 ms",async()=>{
 const watch=turns();
 for(let index=0;index<64;index++)spend(5);
 await new Promise<void>(done=>setImmediate(done));
 expect(watch.stop()).toBeGreaterThan(100);
});

test("a fast filesystem stays synchronous and returns real stats equal to a Python os.lstat oracle",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"slow-fs-"));scratch.push(directory);
 const paths=Array.from({length:50},(_,index)=>{const path=join(directory,`f${index}`);writeFileSync(path,"x".repeat(index));return path;});
 paths.push(join(directory,"missing"));
 const budget=new StatBudgetV1(),results=await budget.many(paths);
 const oracle=spawnSync("python3",["-I","-c","import os,sys\nfor p in sys.argv[1:]:\n  try: print(os.lstat(p).st_size)\n  except OSError: print(-1)",...paths],{encoding:"utf8"}).stdout.trim().split("\n").map(Number);
 expect(results.map(result=>isStatsV1(result)?result.size:-1)).toEqual(oracle);
 expect(results.map(result=>isStatsV1(result)?result.size:-1)).toEqual(paths.map(path=>{try{return lstatSync(path).size;}catch{return -1;}}));
 expect((results[50] as NodeJS.ErrnoException).code).toBe("ENOENT");
 expect(budget.mode().parallel).toBe(false);
 expect(budget.mode().switches).toBe(0);
});

test("a slow phase moves to async stats and a fast probe moves back to sync",async()=>{
 let slow=true;
 const budget=new StatBudgetV1({probeEvery:2,lstatSync:()=>{if(slow)spend(3);return stampOf(1);},lstat:async()=>stampOf(1)});
 await budget.many(["a","b","c"]);
 expect(budget.mode().parallel).toBe(true);
 slow=false;
 for(let round=0;round<4;round++)await budget.many(["a","b"]);
 expect(budget.mode().parallel).toBe(false);
 expect(budget.mode().switches).toBe(2);
});
