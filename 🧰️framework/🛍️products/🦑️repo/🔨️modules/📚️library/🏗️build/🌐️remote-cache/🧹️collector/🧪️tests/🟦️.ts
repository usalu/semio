import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {readdir} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import policy from "../../../🔣️policy.json" with {type:"json"};
import {cacheLedgerV1,cacheAreaBudgetBytesV1,cacheRootDirectoryV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1,collectCacheV1} from "../../../🧹️cache-gc/🏃️runner/🟦️.ts";
import {collectRemoteCacheForLedgerV1} from "../../⌨️command/🟦️.ts";
import {RemoteCacheStoreV1} from "../../💾️store/🟦️.ts";
import {acquireRemoteCacheLockV1,collectRemoteCacheDirectoryV1,findDanglingRemoteCacheResultsV1,planRemoteCacheCollectionV1,REMOTE_CACHE_COLLECTOR_LOCK_V1,scanRemoteCacheDirectoryV1,type RemoteCacheDropV1,type RemoteCacheRowV1} from "../🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dir,"../../🔬️action-result/🧪️tests/🐍️oracle/🐍️.py"),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-gc-${label}-`));scratch.push(path);return path;};
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const body=(bytes:Uint8Array)=>(async function*(){yield bytes;})();
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

test.skipIf(!python)("the collection plan equals an independent Python model on 300 seeded stores, ties and reservations included",()=>{
 const next=seeded(20261021),requests:{rows:RemoteCacheRowV1[];bounds:Record<string,number>;dangling:string[]}[]=[];
 for(let round=0;round<300;round++){
  const count=Math.floor(next()*40),rows:RemoteCacheRowV1[]=[];
  for(let index=0;index<count;index++)rows.push({key:`main/nx/${String(index).padStart(2,"0")}/k${Math.floor(next()*1000)}-${index}`,size:1+Math.floor(next()*5000),accessed:1_000_000+Math.floor(next()*(round%3===0?5:2000))});
  const total=rows.reduce((sum,row)=>sum+row.size,0),maximumBytes=Math.max(1,Math.floor(total*(0.3+next()*1.2))),bounds={maximumBytes,maximumAgeMilliseconds:round%4===0?Math.floor(next()*1500):0,now:1_002_000,need:round%5===0?Math.floor(next()*1000):0,reserved:round%7===0?Math.floor(next()*1000):0};
  requests.push({rows,bounds,dangling:rows.filter(()=>next()<0.08).map(row=>row.key)});
 }
 const result=Bun.spawnSync([python!,"-I",oracle,"plan"],{stdin:new TextEncoder().encode(JSON.stringify(requests)),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const model=JSON.parse(result.stdout.toString()) as RemoteCacheDropV1[][];let reasons=new Set<string>(),removed=0;
 requests.forEach((request,index)=>{
  const ours=planRemoteCacheCollectionV1(request.rows,request.bounds as never,new Set(request.dangling));
  expect(ours,`round ${index}`).toEqual(model[index]!);for(const drop of ours){reasons.add(drop.reason);removed++;}
  const dropped=new Set(ours.map(drop=>drop.key)),kept=request.rows.filter(row=>!dropped.has(row.key)).reduce((sum,row)=>sum+row.size,0);
  if(kept>0&&ours.some(drop=>drop.reason==="budget"))expect(kept+(request.bounds.reserved??0)).toBeLessThanOrEqual(Math.floor(request.bounds.maximumBytes!*9/10)-(request.bounds.need??0));
 });
 expect([...reasons].sort()).toEqual(["age","budget","dangling"]);expect(removed).toBeGreaterThan(500);
});

async function seed(folder:string,count:number,size:number,start=Date.now()-count*10_000):Promise<RemoteCacheStoreV1>{
 const store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0});
 for(let index=0;index<count;index++){
  const outcome=await store.put("main","nx",`key${String(index).padStart(3,"0")}`,body(new Uint8Array(size).fill(index&255)),{mode:"keep",declaredLength:size});expect(outcome).toBe("created");
  const at=(start+index*10_000)/1000;utimesSync(join(folder,"v1","main","nx",`ke`,`key${String(index).padStart(3,"0")}`),at,at);
 }
 return store;
}

test("a dry run plans what an application removes, nothing else; the newest entries survive and the bound holds",async()=>{
 const folder=directory("plan"),store=await seed(folder,40,10_000);await store.close();const root=join(folder,"v1");
 const request={root,maximumBytes:300_000,maximumAgeMilliseconds:0,verifyResults:true};
 const plan=await collectRemoteCacheDirectoryV1({...request,dryRun:true});expect(plan.locked).toBe(false);
 const before=await scanRemoteCacheDirectoryV1(root);expect(before.size,"a plan removes nothing").toBe(40);
 const planned=plan.locked?undefined:plan.value;expect(planned).toMatchObject({dryRun:true,scanned:{entries:40,bytes:400_000},removed:{entries:13,bytes:130_000,budget:13,age:0,dangling:0},remaining:{entries:27,bytes:270_000}});
 const progress:string[]=[],applied=await collectRemoteCacheDirectoryV1({...request,dryRun:false,onProgress:stage=>progress.push(stage)});
 expect(applied.locked).toBe(false);expect(applied.locked?undefined:{...applied.value,dryRun:true}).toEqual(planned);expect(progress).toEqual(expect.arrayContaining(["scan","remove"]));
 const after=await scanRemoteCacheDirectoryV1(root),names=[...after.keys()].map(key=>key.split("/")[3]!).sort();
 expect(names).toEqual(Array.from({length:27},(_,index)=>`key${String(index+13).padStart(3,"0")}`));
 expect(existsSync(join(root,REMOTE_CACHE_COLLECTOR_LOCK_V1)),"the lock is released").toBe(false);
 const again=await collectRemoteCacheDirectoryV1({...request,dryRun:false});expect(again.locked?undefined:again.value.removed.entries,"collecting twice is idempotent").toBe(0);
});

test("idle entries go by age before the bound is looked at",async()=>{
 const folder=directory("age"),store=await seed(folder,10,1000,Date.now()-10*86_400_000);await store.close();
 const outcome=await collectRemoteCacheDirectoryV1({root:join(folder,"v1"),maximumBytes:2**30,maximumAgeMilliseconds:5*86_400_000,dryRun:false,verifyResults:false});
 expect(outcome.locked?undefined:outcome.value.removed).toMatchObject({age:10,budget:0,entries:10});
});

test("action results whose blobs left the store, or that were never valid, are collected as dangling",async()=>{
 const folder=directory("dangling"),store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0});
 const blob=new TextEncoder().encode("alpha\n"),hash=sha(blob);
 const pointer=(value:string)=>{const digest=new Uint8Array([0x0a,0x40,...new TextEncoder().encode(value),0x10,0x06]);return new Uint8Array([0x12,digest.length+5,0x0a,0x01,0x70,0x12,digest.length,...digest]);};
 expect(await store.put("main","cas",hash,body(blob),{mode:"content",verify:true,declaredLength:blob.length})).toBe("created");
 expect(await store.put("main","ac",sha("good"),body(pointer(hash)),{mode:"replace"})).toBe("created");
 expect(await store.put("main","ac",sha("orphan"),body(pointer(sha("elsewhere"))),{mode:"replace"})).toBe("created");
 expect(await store.put("main","ac",sha("broken"),body(new Uint8Array([0xff,0xff,0xff])),{mode:"replace"})).toBe("created");
 expect(await store.put("other","ac",sha("good"),body(pointer(hash)),{mode:"replace"}),"a blob of another namespace does not count").toBe("created");
 await store.close();const root=join(folder,"v1"),index=await scanRemoteCacheDirectoryV1(root),dangling=await findDanglingRemoteCacheResultsV1(root,index);
 expect([...dangling].map(key=>{const [namespace,kind,,name]=key.split("/");return `${namespace}/${kind}/${name}`;}).sort()).toEqual([`main/ac/${sha("broken")}`,`main/ac/${sha("orphan")}`,`other/ac/${sha("good")}`].sort());
 const outcome=await collectRemoteCacheDirectoryV1({root,maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:true});
 expect(outcome.locked?undefined:outcome.value.removed).toMatchObject({dangling:3,entries:3});
 const kept=await scanRemoteCacheDirectoryV1(root);expect([...kept.keys()].sort()).toEqual([`main/ac/${sha("good").slice(0,2)}/${sha("good")}`,`main/cas/${hash.slice(0,2)}/${hash}`].sort());
});

test("only one collector works on a store at a time; a lock of a dead process or an old lock is taken over",async()=>{
 const folder=directory("lock"),store=await seed(folder,5,1000);await store.close();const root=join(folder,"v1"),request={root,maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:false};
 const mine=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_COLLECTOR_LOCK_V1);expect(mine.held).toBe(false);
 const refused=await collectRemoteCacheDirectoryV1(request);expect(refused).toEqual({locked:true,lock:join(root,REMOTE_CACHE_COLLECTOR_LOCK_V1)});
 expect((await collectRemoteCacheDirectoryV1({...request,dryRun:true})).locked,"a plan needs no lock").toBe(false);
 if(!mine.held)await mine.release();expect(existsSync(join(root,REMOTE_CACHE_COLLECTOR_LOCK_V1))).toBe(false);
 writeFileSync(join(root,REMOTE_CACHE_COLLECTOR_LOCK_V1),`2147483646\n${Date.now()}\n`);
 expect((await collectRemoteCacheDirectoryV1(request)).locked,"the owner is dead").toBe(false);
 writeFileSync(join(root,REMOTE_CACHE_COLLECTOR_LOCK_V1),`${process.pid}\n${Date.now()-7*3600_000}\n`);
 expect((await collectRemoteCacheDirectoryV1(request)).locked,"older than six hours").toBe(false);
 writeFileSync(join(root,REMOTE_CACHE_COLLECTOR_LOCK_V1),"garbage");
 expect((await collectRemoteCacheDirectoryV1(request)).locked,"unreadable lock content").toBe(false);
 const racers=await Promise.all(Array.from({length:12},()=>acquireRemoteCacheLockV1(root,"race.lock")));
 expect(racers.filter(row=>!row.held).length,"exactly one of twelve simultaneous takers wins").toBe(1);
 for(const row of racers)if(!row.held)await row.release();
});

test("cancelling a collection leaves a consistent store and the next run finishes the job",async()=>{
 const folder=directory("cancel"),store=await seed(folder,600,500);await store.close();const root=join(folder,"v1"),controller=new AbortController();
 const request={root,maximumBytes:150_000,maximumAgeMilliseconds:0,verifyResults:false,dryRun:false};
 await expect(collectRemoteCacheDirectoryV1({...request,signal:controller.signal,onProgress:(stage,done)=>{if(stage==="remove"&&done>=256)controller.abort();}})).rejects.toThrow();
 const partial=await scanRemoteCacheDirectoryV1(root);expect(partial.size).toBeLessThan(600);expect(partial.size).toBeGreaterThan(200);
 expect(existsSync(join(root,REMOTE_CACHE_COLLECTOR_LOCK_V1)),"the lock is released after a cancel").toBe(false);
 const rest=await collectRemoteCacheDirectoryV1(request);expect(rest.locked).toBe(false);
 const done=await scanRemoteCacheDirectoryV1(root);let bytes=0;for(const row of done.values())bytes+=row.size;expect(bytes).toBeLessThanOrEqual(135_000);
 const names=await readdir(join(root,"main","nx"));expect(names.length).toBeGreaterThan(0);
 const cancelled=new AbortController();cancelled.abort();await expect(scanRemoteCacheDirectoryV1(root,{signal:cancelled.signal})).rejects.toThrow();
});

test("the ledger declares the remote cache home, the build policy points at it and cache gc hands it to the same collector",async()=>{
 const ledger=cacheLedgerV1(),area=ledger.areas.find(row=>row.id==="remote-cache")!,server=policy.remoteCache.server;
 expect(area).toMatchObject({root:"repository",kind:"tree",directory:"remote-cache",managed:{owner:"remote-cache"}});
 const workspace=directory("ledger"),home=join(cacheRootDirectoryV1(workspace,ledger,"repository"),"remote-cache");
 expect(join(workspace,server.directory),"the policy directory is the ledger home").toBe(home);
 const budget=cacheAreaBudgetBytesV1(ledger,area)!;expect(server.maximumBytes).toBeLessThanOrEqual(Math.floor(budget*0.9));expect(server.maximumBytes).toBeGreaterThanOrEqual(1048576);
 mkdirSync(home,{recursive:true});const store=await RemoteCacheStoreV1.open({directory:home,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0});
 const blob=new Uint8Array(1000);for(let index=0;index<8;index++)await store.put("main","nx",`k${index}`,body(blob),{mode:"keep",declaredLength:1000});await store.close();
 const controller=new AbortController(),asked:string[]=[],ports=cacheRunnerPortsV1({signal:controller.signal,progress:async()=>{},exclusive:async()=>undefined,delegate:async(owner,id,dryRun)=>{asked.push(`${owner}:${id}:${dryRun}`);return owner==="remote-cache"?collectRemoteCacheForLedgerV1(workspace,dryRun,controller.signal):undefined;}});
 const lines:string[]=[],result=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["remote-cache"])},ports,{language:"en",write:line=>lines.push(line)});
 expect(asked).toEqual(["remote-cache:remote-cache:true"]);expect(result.delegated).toEqual([{owner:"remote-cache",text:"would remove 0 of 8 entries (0 B)"}]);
 expect(await collectRemoteCacheForLedgerV1(join(workspace,"nowhere"),true,controller.signal)).toBe("absent");
});
