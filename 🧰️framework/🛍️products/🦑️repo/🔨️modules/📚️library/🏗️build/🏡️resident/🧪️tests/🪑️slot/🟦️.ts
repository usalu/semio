import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {EventEmitter} from "node:events";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import fixture from "./🔣️.json" with {type:"json"};

test("cancelling a transferred slice slot makes the slot available to the next owner",async()=>{
 const controls=new Map<string,ReturnType<typeof Promise.withResolvers<EventEmitter>>>(),signals=new Map<string,AbortController>(),db=new Database(":memory:");
 for(const root of fixture.roots){controls.set(root,Promise.withResolvers());signals.set(root,new AbortController());}
 db.exec("CREATE TABLE transitions (id INTEGER PRIMARY KEY, kind TEXT NOT NULL)");
 const record=db.query("INSERT INTO transitions (kind) VALUES (?)");
 const active=()=>db.query<{count:number},[]>("SELECT sum(CASE kind WHEN 'start' THEN 1 WHEN 'done' THEN -1 ELSE 0 END) AS count FROM transitions").get()?.count??0;
 const workers=new Map<string,EventEmitter>(),queue=Promise.withResolvers<void>();
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused"),maximum:fixture.maximum,heartbeatMilliseconds:10,hostMegabytes:8192,
  pressure:()=>({sample:()=>({tripped:false})}),reaper:{reap:async()=>0,killNow:()=>undefined},
  createWorker:(_url:URL,{root}:{root:string})=>{
   const worker=Object.assign(new EventEmitter(),{terminate:async()=>undefined});
   record.run("start");workers.set(root,worker);controls.get(root)!.resolve(worker);
   return worker;
  }
 } as never);
 const [leader,departing,follower]=fixture.roots,reason=new Error(fixture.reason),pending:Promise<unknown>[]=[];
 const run=(root:string,onProgress?:(_:number,detail?:{decision?:{key:string}})=>void)=>{
  const done=pool.run({root},{signal:signals.get(root)!.signal,onProgress});
  void done.catch(()=>undefined);pending.push(done);return done;
 };
 const complete=(worker:EventEmitter)=>{record.run("done");worker.emit("message",{type:"done",graph:{}});};
 try{
  const first=run(leader!);
  const worker=await controls.get(leader!)!.promise;
  const second=run(departing!,(_ms,detail)=>{if(detail?.decision?.key==="queued-pool")queue.resolve();});
  await queue.promise;
  expect(pool.stats()).toMatchObject({active:active(),queued:1,started:1});
  complete(worker);
  queueMicrotask(()=>signals.get(departing!)!.abort(reason));
  await first;
  await expect(second).rejects.toBe(reason);
  expect(pool.stats()).toMatchObject({active:active(),queued:0,started:1});
  const third=run(follower!);
  complete(await controls.get(follower!)!.promise);
  await third;
  expect(pool.stats()).toMatchObject({active:active(),queued:0,started:2});
  expect(workers.has(departing!)).toBe(false);
 }finally{
  for(const abort of signals.values())abort.abort();
  await Promise.allSettled(pending);await pool.close();db.close();
 }
},10000);

test("cancellation during a replay detaches only the late slice subscriber",async()=>{
 const began=Promise.withResolvers<EventEmitter>(),seen=Promise.withResolvers<void>(),reason=new Error(fixture.replay.reason),first=new AbortController(),late=new AbortController();
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused"),maximum:fixture.maximum,heartbeatMilliseconds:10,hostMegabytes:8192,
  pressure:()=>({sample:()=>({tripped:false})}),reaper:{reap:async()=>0,killNow:()=>undefined},
  createWorker:()=>{
   const worker=Object.assign(new EventEmitter(),{terminate:async()=>undefined});began.resolve(worker);return worker;
  }
 } as never);
 const request={root:fixture.roots[0]},step=fixture.replay.progress,pending:Promise<unknown>[]=[];
 try{
  const leading=pool.run(request,{signal:first.signal,onProgress:(_ms:number,detail?:{phase:string})=>{if(detail?.phase===step.phase)seen.resolve();}});
  pending.push(leading);
  const worker=await began.promise;
  worker.emit("message",{type:"progress",...step});
  await seen.promise;
  let replayed;
  const joined=pool.run(request,{signal:late.signal,onProgress:(_ms:number,detail:unknown)=>{replayed=detail;late.abort(reason);}});
  void joined.catch(()=>undefined);pending.push(joined);
  expect(replayed).toEqual(step);
  worker.emit("message",{type:"done",graph:{}});
  await expect(joined).rejects.toBe(reason);
  await leading;
  expect(pool.stats()).toMatchObject({active:0,completed:1,started:1});
 }finally{
  first.abort();late.abort();await Promise.allSettled(pending);await pool.close();
 }
},10000);
