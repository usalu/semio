/** 🧵️ Contract: two worker threads of one process (the slice pool of the graph service) never hold the same exclusive resource lease at once, and a shared hold of one thread blocks an exclusive request of another: the lease is a lock between holders, not between process ids, so a shared pid is no re-entrancy. Different resources (different roots) do not wait for each other. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {Worker} from "node:worker_threads";

type Span={mode:string;acquired:number;released:number;pid:number};
const run=(directory:string,resource:string,mode:"shared"|"exclusive",holdMilliseconds:number)=>new Promise<Span>((resolve,reject)=>{
 const worker=new Worker(new URL("./👷️worker.ts",import.meta.url),{workerData:{directory,resource,mode,holdMilliseconds}});
 worker.once("message",resolve);worker.once("error",reject);
});
const overlap=(a:Span,b:Span)=>a.acquired<b.released&&b.acquired<a.released;

test("two threads of one process serialize on one exclusive resource and an exclusive thread waits for a shared one, while another resource does not wait",async()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"lease-threads-")));
 try{
  const[first,second]=await Promise.all([run(directory,"facts:a","exclusive",250),run(directory,"facts:a","exclusive",250)]);
  expect(first.pid).toBe(second.pid);expect(overlap(first,second),"exclusive holds of two threads overlap").toBe(false);
  const shared=run(directory,"facts:b","shared",300),exclusive=run(directory,"facts:b","exclusive",100),other=run(directory,"facts:c","exclusive",100);
  const[s,e,o]=await Promise.all([shared,exclusive,other]);
  expect(overlap(s,e),"an exclusive thread must wait for a shared one").toBe(false);
  expect(Math.min(s.acquired,e.acquired)).toBeLessThanOrEqual(o.acquired+250);
 }finally{rmSync(directory,{recursive:true,force:true});}
},120000);
