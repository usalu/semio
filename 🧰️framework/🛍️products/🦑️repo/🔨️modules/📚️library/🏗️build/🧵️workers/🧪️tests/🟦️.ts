/** 🧵️ Contract: the worker pool returns exactly what a serial run returns (cross-checked against Python hashlib), keeps input order, bounds its threads, reports progress, cancels promptly as a pool and per task, and survives a failing or crashing task. The scenarios live in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import protocol from "../🧬️schema/🔣️.json";
import {createWorkerPoolV1} from "../🟨️.mjs";

const tasks=new URL("./🧰️tasks/🟨️.mjs",import.meta.url).href,task=(name:string)=>({module:tasks,export:name}),python=testPythonV1();
const files=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"worker-pool-"));mkdirSync(root,{recursive:true});for(const[name,row]of Object.entries(fixture.files))writeFileSync(join(root,name),row.text);return root;};

test("worker pool fixture and protocol are closed",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
 const ajv=new Ajv({strict:false}),request={type:"run",id:0,module:tasks,export:"square",input:3};
 expect(ajv.validate(protocol,request)).toBe(true);
 expect(ajv.validate(protocol,{...request,module:"https://example.com/x.mjs"})).toBe(false);
 expect(ajv.validate(protocol,{...request,extra:true})).toBe(false);
});

test("map keeps the input order and agrees with the serial result and with Python hashlib",async()=>{
 const pool=createWorkerPoolV1({workers:3}),root=files();
 try{
  expect(await pool.map(task("square"),fixture.square.inputs)).toEqual(fixture.square.expected);
  const paths=Object.keys(fixture.files).map(name=>join(root,name)),[digests]=await pool.map<string[]>(task("sha256Files"),[paths]);
  expect(digests).toEqual(Object.values(fixture.files).map(row=>row.sha256));
  expect(digests).toEqual(Object.values(fixture.files).map(row=>createHash("sha256").update(row.text).digest("hex")));
  if(python){const result=Bun.spawnSync([python,"-I","-c","import hashlib,sys\nfor p in sys.argv[1:]:\n    print(hashlib.sha256(open(p,'rb').read()).hexdigest())",...paths]);expect(result.stdout.toString().trim().split("\n")).toEqual(digests);}
 }finally{await pool.close();rmSync(root,{recursive:true,force:true});}
});

test("the pool never runs on more threads than it was given and reports finished tasks",async()=>{
 const progress:number[]=[],pool=createWorkerPoolV1({workers:fixture.bounded.workers,onProgress:row=>progress.push(row.completed)});
 try{
  const threads=await pool.map<number>(task("thread"),Array.from({length:fixture.bounded.tasks},()=>null));
  expect(new Set(threads).size).toBeLessThanOrEqual(fixture.bounded.maximumDistinctThreads);
  expect(progress.length).toBe(fixture.bounded.tasks);
  expect(progress).toEqual([...progress].sort((a,b)=>a-b));expect(progress.at(-1)).toBe(fixture.bounded.tasks);
 }finally{await pool.close();}
});

test("a task reports monotonic progress up to its total",async()=>{
 const rows:{completed:number;total:number}[]=[],pool=createWorkerPoolV1({workers:1,onTaskProgress:row=>rows.push(row)});
 try{
  expect(await pool.run(task("count"),fixture.count.total)).toBe(fixture.count.total);
  expect(rows.length).toBe(fixture.count.total);
  expect(rows.map(row=>row.completed)).toEqual(Array.from({length:fixture.count.total},(_,index)=>index+1));
  expect(rows.every(row=>row.total===fixture.count.total)).toBe(true);
 }finally{await pool.close();}
});

test("aborting the pool signal rejects running and queued tasks promptly with the caller's reason",async()=>{
 const controller=new AbortController(),pool=createWorkerPoolV1({workers:fixture.cancel.workers,signal:controller.signal}),started=performance.now();
 const pending=Array.from({length:fixture.cancel.tasks},()=>pool.run(task("spin"),fixture.cancel.spinMilliseconds).then(()=>"finished",error=>error));
 await new Promise(accept=>setTimeout(accept,300));
 const reason=new Error("stop the fixture");controller.abort(reason);
 const outcomes=await Promise.all(pending);
 expect(outcomes.every(outcome=>outcome===reason)).toBe(true);
 expect(performance.now()-started).toBeLessThan(fixture.cancel.deadlineMilliseconds);
 await expect(pool.run(task("square"),2)).rejects.toBe(reason);
});

test("cancelling one task leaves the pool and the other tasks running",async()=>{
 const pool=createWorkerPoolV1({workers:2}),own=new AbortController();
 try{
  const slow=pool.run(task("spin"),fixture.cancel.spinMilliseconds,{signal:own.signal}).then(()=>"finished",error=>error),fast=pool.run(task("square"),9);
  await new Promise(accept=>setTimeout(accept,200));own.abort();
  expect(((await slow) as Error).name).toBe("AbortError");
  expect(await fast).toBe(81);expect(await pool.run(task("square"),5)).toBe(25);
 }finally{await pool.close();}
});

test("a failing task rejects with its own message and a crashing worker is replaced",async()=>{
 const pool=createWorkerPoolV1({workers:1});
 try{
  await expect(pool.run(task("fail"),fixture.failure.message)).rejects.toThrow(fixture.failure.message);
  await expect(pool.run(task("crash"),null)).rejects.toThrow();
  expect(await pool.run(task("square"),7)).toBe(49);
  await expect(pool.run({module:"https://example.com/x.mjs",export:"square"},1)).rejects.toThrow("Closed worker task required");
 }finally{await pool.close();}
});
