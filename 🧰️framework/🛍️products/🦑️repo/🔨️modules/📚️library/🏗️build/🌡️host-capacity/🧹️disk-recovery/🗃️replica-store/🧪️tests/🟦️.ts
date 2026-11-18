/** 🧪️ Contract: the replica store is compacted by the machine that is the group server and by no other (a deletion in the store is never published, the workers read the store itself), through the store's own compaction under its store-wide lock; on a synthetic store the unreferenced old blobs go and every referenced blob keeps its content (digest law), a worker or a held lock removes nothing and says why in both languages, and a compaction killed at any moment leaves every referenced blob intact so that the next one finishes the work (kill law). */
import {afterAll,describe,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {appendEventsV1,blobPathV1,formatHlcV1,storeBlobV1,storePathsV1,type FleetEventV1} from "../../../../../../🤝️shared-worktree/📒️journal/🟦️.ts";
import {snapshotV1} from "../../../../../../🤝️shared-worktree/🔁️replica/🟦️.ts";
import {compactionDefaultsV1,storeReportV1,withCompactionLockV1} from "../../../../../../🤝️shared-worktree/🗜️compaction/🟦️.ts";
import topology from "../../../../../../🤝️shared-worktree/🗺️topology/🔣️.json" with {type:"json"};
import {DISK_WRITERS_V1,requireDiskWritersV1} from "../../🟦️.ts";
import {REPLICA_STORE_V1,collectReplicaStoreV1,readReplicaStoreRoleV1,replicaStoreRoleV1,type ReplicaCompactionV1} from "../🟦️.ts";
import {compactReplicaStoreV1,realDiskWritersV1} from "../../🔌️real/🟦️.ts";

setDefaultTimeout(240_000);
const roots:string[]=[],fresh=(label:string)=>{const root=mkdtempSync(join(tmpdir(),"replica-store-"+label+"-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const hour=3_600_000,day=24*hour,sharedWorktree=process.env.R172_MIRROR?"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree":join(import.meta.dirname,"../../../../../../🤝️shared-worktree");
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");
const put=(origin:string,counter:number,path:string,sha256:string,bytes:number):FleetEventV1=>({version:1,hlc:formatHlcV1({ms:Date.now()-5*day+counter,counter},origin),operation:"put",path,sha256,bytes,executable:false,base:null});
const sizeOf=(directory:string):number=>{let total=0;for(const name of readdirSync(directory,{withFileTypes:true})){const path=join(directory,name.name);total+=name.isDirectory()?sizeOf(path):statSync(path).size;}return total;};
const world=async(live:number,garbage:number,bytes:number=20_000)=>{
 const store=fresh("store"),source=join(fresh("source"),"file"),kept:string[]=[],gone:string[]=[],events:FleetEventV1[]=[];
 const make=async(text:string)=>{writeFileSync(source,text.repeat(Math.ceil(bytes/text.length)).slice(0,bytes));const stored=await storeBlobV1(store,"g1-server",source),stamp=new Date(Date.now()-5*day);utimesSync(blobPathV1(store,stored.sha256),stamp,stamp);return stored.sha256;};
 for(let index=0;index<live;index++){const digest=await make("live-"+index+"-");kept.push(digest);events.push(put("g1-server",index+1,"p/"+index,digest,bytes));}
 for(let index=0;index<garbage;index++)gone.push(await make("garbage-"+index+"-"));
 await appendEventsV1(store,"g1-server",events);await snapshotV1(store,"g1-server");
 return{store,kept,gone};
};

describe("who may compact the store",()=>{
 test("the committed topology names the settings the real compaction reads, and the registry rows are consistent",()=>{
  expect(topology.replica.compaction).toMatchObject({safetyHours:expect.any(Number),staleHours:expect.any(Number)});
  expect(REPLICA_STORE_V1.topology).toBe("replica.compaction");
  requireDiskWritersV1(DISK_WRITERS_V1);
  const row=DISK_WRITERS_V1.find(item=>item.id==="replica-store")!;
  expect(row.command).toContain("repo ./📜️script.ts compaction --apply");expect(row.relief).toBe("host");
 });
 test("the machine file decides: origin group-server is the server, any other origin a worker, anything unreadable none",()=>{
  expect(replicaStoreRoleV1({version:1,originId:"g1-server",groupId:"g1",storeRoot:"/Users/Shared/semio-store"})).toEqual({kind:"server",originId:"g1-server",groupId:"g1",storeRoot:"/Users/Shared/semio-store"});
  expect(replicaStoreRoleV1({originId:"g1-m01",groupId:"g1",storeRoot:"/mnt/semio-store"}).kind).toBe("worker");
  expect(replicaStoreRoleV1({originId:"g2-server",groupId:"g1",storeRoot:"/mnt/s"}).kind).toBe("worker");
  expect(replicaStoreRoleV1({originId:"g1-server",groupId:"g1",storeRoot:"relative/store"}).kind).toBe("none");
  expect(replicaStoreRoleV1(null).kind).toBe("none");expect(replicaStoreRoleV1({originId:"g1-server"}).kind).toBe("none");
  const workspace=fresh("ws");expect(readReplicaStoreRoleV1(workspace).kind).toBe("none");
  mkdirSync(join(workspace,".cache/f"),{recursive:true});writeFileSync(join(workspace,".cache/f/machine.json"),JSON.stringify({version:1,originId:"g1-server",groupId:"g1",storeRoot:"/x/store",key:"k"}));
  expect(readReplicaStoreRoleV1(workspace)).toMatchObject({kind:"server",storeRoot:"/x/store"});
 });
});

describe("the collection with a fake store",()=>{
 const server={kind:"server" as const,originId:"g1-server",groupId:"g1",storeRoot:"/s"};
 const run=async(role:Parameters<typeof collectReplicaStoreV1>[0]["role"],answer:ReplicaCompactionV1,language:"en"|"de"="en")=>{const lines:string[]=[],calls:string[]=[];const result=await collectReplicaStoreV1({role,language,report:line=>lines.push(line),compact:async(root,origin)=>{calls.push(root+"@"+origin);return answer;}});return{result,lines,calls};};
 test("a worker never compacts and names the group server in both languages",async()=>{
  const worker={kind:"worker" as const,originId:"g1-m01",groupId:"g1",storeRoot:"/mnt/s"};
  const english=await run(worker,{skipped:null,bytes:1});expect(english.calls).toEqual([]);expect(english.result.skipped).toBe("not-server");expect(english.lines[0]).toContain("g1-server");expect(english.lines[0]).toContain("g1-m01");
  expect((await run(worker,{skipped:null,bytes:1},"de")).lines[0]).toContain("Gruppenserver");
  expect((await run({kind:"none"},{skipped:null,bytes:1})).calls).toEqual([]);
 });
 test("the server compacts its own store under its own origin; a held lock, a refusal and an abort are named skips",async()=>{
  const done=await run(server,{skipped:null,bytes:3*1024**3});expect(done.calls).toEqual(["/s@g1-server"]);expect(done.result).toMatchObject({compacted:true,bytes:3*1024**3});expect(done.lines[0]).toContain("3 GiB");
  const locked=await run(server,{skipped:"locked",bytes:0});expect(locked.result).toMatchObject({compacted:false,skipped:"locked"});expect(locked.lines[0]).toContain("lock");
  expect((await run(server,{skipped:"locked",bytes:0},"de")).lines[0]).toContain("Verdichtungssperre");
  expect((await run(server,{skipped:"refused",bytes:0})).result.skipped).toBe("refused");expect((await run(server,{skipped:"aborted",bytes:5})).result.compacted).toBe(false);
 });
 test("cancellation before the work stops it",async()=>{
  const controller=new AbortController();controller.abort();let called=false;
  await expect(collectReplicaStoreV1({role:server,language:"en",signal:controller.signal,compact:async()=>{called=true;return{skipped:null,bytes:0};}})).rejects.toBeDefined();expect(called).toBe(false);
 });
});

describe("a synthetic store with the store's own compaction",()=>{
 test("the unreferenced old blobs go, every referenced blob keeps its bytes, the bytes the report promised are the bytes the directory lost",async()=>{
  const {store,kept,gone}=await world(30,40);
  const before=sizeOf(store),report=await storeReportV1(store,compactionDefaultsV1,Date.now(),undefined,true);
  const promised=report.reclaimable.blobs!.bytes;expect(report.reclaimable.blobs!.count).toBe(40);
  const hashes=kept.map(digest=>sha(blobPathV1(store,digest)));expect(hashes).toEqual(kept);
  const outcome=await collectReplicaStoreV1({role:{kind:"server",originId:"g1-server",groupId:"g1",storeRoot:store},language:"en",compact:compactReplicaStoreV1});
  expect(outcome.compacted).toBe(true);expect(outcome.bytes).toBeGreaterThan(promised*0.99);
  for(const digest of gone)expect(existsSync(blobPathV1(store,digest))).toBe(false);
  for(const digest of kept)expect(sha(blobPathV1(store,digest))).toBe(digest);
  expect(before-sizeOf(store)).toBeGreaterThan(promised*0.99);
  expect(readdirSync(join(storePathsV1(store).root,"trash")).length).toBe(0);
 });
 test("a worker role removes nothing from the store, the tree is byte for byte the same",async()=>{
  const {store,gone}=await world(5,8),listing=()=>JSON.stringify(readdirSync(storePathsV1(store).blobs,{recursive:true}).sort());
  const before=listing();let called=false;
  await collectReplicaStoreV1({role:{kind:"worker",originId:"g1-m01",groupId:"g1",storeRoot:store},language:"en",compact:async()=>{called=true;return{skipped:null,bytes:0};}});
  expect(called).toBe(false);expect(listing()).toBe(before);for(const digest of gone)expect(existsSync(blobPathV1(store,digest))).toBe(true);
 });
 test("while another machine holds the store-wide lock nothing is removed and the skip says locked",async()=>{
  const {store,gone}=await world(5,8),lines:string[]=[];
  const held=await withCompactionLockV1(store,"g1-m07",async()=>collectReplicaStoreV1({role:{kind:"server",originId:"g1-server",groupId:"g1",storeRoot:store},language:"en",report:line=>lines.push(line),compact:compactReplicaStoreV1}));
  expect(held!.value).toMatchObject({compacted:false,skipped:"locked"});for(const digest of gone)expect(existsSync(blobPathV1(store,digest))).toBe(true);expect(lines[0]).toContain("lock");
 });
 test("the registered writer of the real binding measures what a compaction would return and compacts only on the server, honouring a dry run",async()=>{
  const {store,kept,gone}=await world(10,12),workspace=fresh("ws");mkdirSync(join(workspace,".cache/f"),{recursive:true});
  const machine=(origin:string)=>writeFileSync(join(workspace,".cache/f/machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"k"}));
  const bind=(dryRun:boolean)=>realDiskWritersV1({workspace,home:fresh("home"),tmp:fresh("tmp"),uid:1,environment:{},dryRun,compact:compactReplicaStoreV1}).find(item=>item.row.id==="replica-store")!;
  machine("g1-m01");expect(await bind(false).measure()).toBe(0);
  machine("g1-server");const writer=bind(false),promised=await writer.measure();expect(promised).toBeGreaterThanOrEqual(12*20_000);
  expect((await writer.describe!())).toMatchObject({root:store});
  await bind(true).collect!({needBytes:1});for(const digest of gone)expect(existsSync(blobPathV1(store,digest))).toBe(true);
  await writer.collect!({needBytes:1});for(const digest of gone)expect(existsSync(blobPathV1(store,digest))).toBe(false);for(const digest of kept)expect(existsSync(blobPathV1(store,digest))).toBe(true);
 });
 test("a compaction killed in the middle of its blob reclaim, twice, leaves every referenced blob intact; the dead owner holds the store-wide lock (named as locked) until it was not refreshed for two hours, then the next one finishes the work",async()=>{
  const {store,kept,gone}=await world(40,4600,300),script=join(fresh("child"),"compact.ts"),replica=join(sharedWorktree,"🔁️replica/🟦️.ts");
  writeFileSync(script,`import {fleetCompactV1} from ${JSON.stringify(replica)};import topology from ${JSON.stringify(join(sharedWorktree,"🗺️topology/🔣️.json"))} with {type:"json"};await fleetCompactV1(process.argv[2],"g1-server",topology.replica.compaction,"apply",undefined,event=>console.log(JSON.stringify(event)),Date.now,{blobs:true});`);
  let killed=0;
  for(let round=0;round<2;round++){
   const child=Bun.spawn([process.execPath,script,store],{stdout:"pipe",stderr:"ignore"});let seen="",did=false;
   for await(const chunk of child.stdout){seen+=Buffer.from(chunk).toString();if(!did&&seen.includes('"stage":"reclaim"')){did=true;child.kill("SIGKILL");killed++;}}
   await child.exited;expect(did).toBe(true);
   for(const digest of kept)expect(sha(blobPathV1(store,digest))).toBe(digest);
   expect(gone.some(digest=>existsSync(blobPathV1(store,digest)))).toBe(true);
   const lock=join(storePathsV1(store).root,"compaction.lock");expect(existsSync(lock)).toBe(true);
   expect((await compactReplicaStoreV1(store,"g1-server")).skipped).toBe("locked");
   const old=new Date(Date.now()-3*hour);utimesSync(lock,old,old);
  }
  const finish=await compactReplicaStoreV1(store,"g1-server");expect(finish.skipped).toBeNull();
  for(const digest of gone)expect(existsSync(blobPathV1(store,digest))).toBe(false);
  for(const digest of kept)expect(sha(blobPathV1(store,digest))).toBe(digest);
  expect(readdirSync(join(storePathsV1(store).root,"trash")).length).toBe(0);
  expect(killed).toBe(2);
 });
});
