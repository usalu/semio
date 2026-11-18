import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,truncateSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {widen,settleHeap} from "../📇️book/🟦️.ts";
import {closeReplicaV1,openReplicaV1,publishOnceV1,repairV1,seedV1} from "../🟦️.ts";
import {writeSnapshotV1,type FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";

const scratch:string[]=[],watchdogHeartbeatMilliseconds=1000;
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

function turns(){
 let last=performance.now(),longest=0,count=0,active=true;
 const tick=():void=>{if(!active)return;const now=performance.now();longest=Math.max(longest,now-last);last=now;count++;setImmediate(tick);};
 setImmediate(tick);
 return {stop:()=>{active=false;return {longest,count};}};
}

const logicalEntryMilliseconds=0.02;

/** ⏱️ Runs `body` with `performance.now` replaced by a logical clock that advances by a fixed cost for every manifest entry the repair loops name, so the yield budget and the measured turn length depend on the work done and never on how long a loaded host takes to do it. */
async function underLogicalClock<T>(replica:Awaited<ReturnType<typeof openReplicaV1>>,body:()=>Promise<T>):Promise<T>{
 const original=performance.now,book=replica.book as unknown as {name:(id:number)=>string};
 let logical=0;
 const name=book.name.bind(book);
 book.name=(id:number)=>{logical+=logicalEntryMilliseconds;return name(id);};
 performance.now=()=>logical;
 try{return await body();}finally{performance.now=original;delete (book as Partial<typeof book>).name;}
}

test("publishing a large sparse file never holds the event loop for longer than the watchdog's heartbeat interval",async()=>{
 const root=mkdtempSync(join(tmpdir(),"replica-turns-"));
 scratch.push(root);
 const worktree=join(root,"tree"),store=join(root,"store");
 mkdirSync(worktree);mkdirSync(store);
 writeFileSync(join(worktree,"a.txt"),"a\n");
 const options={worktree,storeRoot:store,originId:"g1-server",groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
 await seedV1(options);
 const replica=await openReplicaV1(options,()=>{});
 try{
  const size=1.5*2**30;
  writeFileSync(join(worktree,"big.bin"),"");truncateSync(join(worktree,"big.bin"),size);
  const meter=turns(),result=await publishOnceV1(replica),measured=meter.stop();
  expect(result.published).toBe(1);
  expect(result.bytes).toBe(size);
  expect(measured.count).toBeGreaterThan(100);
  expect(measured.longest).toBeLessThan(watchdogHeartbeatMilliseconds/2);
 }finally{await closeReplicaV1(replica);}
},180_000);

test("forced full collections are rate limited: thousands of settle requests and growths of large tables in one turn cost at most one collection",async()=>{
 const holder=globalThis as unknown as {Bun:{gc:(force:boolean)=>void}},original=holder.Bun.gc;
 let calls=0,logical=performance.now()+3_600_000;
 const realNow=performance.now;
 holder.Bun.gc=(force:boolean)=>{calls++;original.call(holder.Bun,force);};
 performance.now=()=>logical;
 try{
  const before=calls;
  for(let index=0;index<5000;index++)settleHeap();
  let array=new Uint8Array(1<<20);
  for(let index=0;index<40;index++)array=widen(array,array.length+(3<<20));
  await Promise.resolve();await new Promise(done=>setImmediate(done));
  expect(calls-before).toBe(1);
  logical+=1100;settleHeap();
  expect(calls-before).toBe(2);
 }finally{holder.Bun.gc=original;performance.now=realNow;}
});

test("a drift repair over a 300k-entry manifest and a 300k-entry store snapshot never holds the event loop for longer than 50 ms in one turn",async()=>{
 const root=mkdtempSync(join(tmpdir(),"replica-drift-"));
 scratch.push(root);
 const worktree=join(root,"tree"),store=join(root,"store");
 mkdirSync(worktree);mkdirSync(store);
 const options={worktree,storeRoot:store,originId:"g1-server",groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
 await seedV1(options);
 const total=300_000,hlc="1790000000000-000001-g1-m01",table=new Map<string,FleetManifestEntryV1>();
 for(let index=0;index<total;index++)table.set(`d${index%997}/e${index%31}/f${index}.rs`,{hlc,operation:"put",sha256:(index.toString(16).padStart(8,"0")).repeat(8),bytes:index%4096,executable:false});
 await writeSnapshotV1(store,"g1-m01","1790000000000-000002-g1-m01",table,{});
 const replica=await openReplicaV1(options,()=>{});
 try{
  for(const [path,entry] of table)if(!path.endsWith("0"))replica.manifest.set(path,entry);
  const measured=await underLogicalClock(replica,async()=>{const meter=turns(),queued=await repairV1(replica),result=meter.stop();expect(queued).toBeGreaterThan(total/2);return result;});
  expect(measured.count).toBeGreaterThan(100);
  expect(measured.longest).toBeLessThan(50);
 }finally{await closeReplicaV1(replica);}
},600_000);
