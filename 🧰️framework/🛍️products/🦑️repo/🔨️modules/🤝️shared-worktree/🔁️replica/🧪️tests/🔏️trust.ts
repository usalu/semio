import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {decideTrustV1,type TrustV1} from "../../🔏️trust/🟦️.ts";
import {readHealthV1,writeHealthV1} from "../../🛡️resilience/🟦️.ts";
import {closeReplicaV1,openReplicaV1,publishOnceV1,requireReplicaShapeV1,runV1,seedV1,statusV1,sweptDriftV1,trustWalkV1,walkLocalV1,type ReplicaEventV1,type ReplicaOptionsV1} from "../🟦️.ts";

setDefaultTimeout(120_000);
const scratch:string[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`trust-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const until=async(condition:()=>boolean|Promise<boolean>,milliseconds=30000):Promise<void>=>{const end=Date.now()+milliseconds;while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(20);}};
const put=(root:string,path:string,text:string,seconds?:number):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);if(seconds!==undefined){const when=Date.now()/1000-seconds;utimesSync(file,when,when);}};
const git={present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)};
const options=(store:string,worktree:string):ReplicaOptionsV1=>({worktree,storeRoot:store,originId:"t1",groupId:"g1",pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,viewMilliseconds:300,watch:false,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true})});
const tier=(health:{wallMs:number;pid:number;store:string;trust?:TrustV1}):string=>decideTrustV1(health,health.wallMs,()=>true);

test("a running daemon writes a trust statement that grants the trusted tier after its first file sweep and directory sweep, whatever its failure counters say",async()=>{
 const store=temp("store"),worktree=temp("tree"),ticket=".🧬semio/🦑️repo/🎫️tickets/t/bad*name.md";
 put(worktree,"a.txt","alpha\n");put(worktree,ticket,"refused by the boundary\n");
 const settings=options(store,worktree),events:ReplicaEventV1[]=[],controller=new AbortController();
 await seedV1(settings);
 const done=runV1(settings,controller.signal,event=>events.push(event));
 try{
  await until(async()=>{const health=await readHealthV1(worktree);return health?.trust.verifiedMs!=null&&health.failures.some(item=>item.kind==="refused");});
  const health=(await readHealthV1(worktree))!;
  expect(health.trust.scope).toBe("shared-source");
  expect(health.failures.some(item=>item.kind==="refused")).toBe(true);
  expect(health.trust.lostMs).toBeNull();
  expect(tier(health)).toBe("trusted");
  const raw=JSON.parse(readFileSync(join(worktree,".cache","f","health.json"),"utf8")) as {trust:TrustV1};
  expect(raw.trust).toEqual(health.trust);
 }finally{controller.abort();await done.catch(()=>{});}
});

test("a walk of a tree that equals the manifest verifies it, a file that differs and is older than the grace without the hot set knowing it is drift, a young one is not, and a clean walk returns the trust",async()=>{
 const store=temp("store"),worktree=temp("tree"),settings=options(store,worktree);
 put(worktree,"a.txt","alpha\n",600);put(worktree,"b.txt","beta\n",600);
 await seedV1(settings);
 const replica=await openReplicaV1(settings,()=>{});
 try{
  replica.hot.expire(Date.now()+86_400_000);
  const clean=await walkLocalV1(replica);
  trustWalkV1(replica,clean.startedMs,clean.changed,[]);
  expect(replica.trust.record()).toMatchObject({verifiedMs:clean.startedMs,lostMs:null});
  put(worktree,"a.txt","changed behind the back of every tier\n",300);
  const drift=await walkLocalV1(replica);
  expect(drift.changed.length).toBe(1);
  trustWalkV1(replica,drift.startedMs,drift.changed,[]);
  expect(replica.trust.record().lostKind).toBe("sweep-drift");
  const lost=replica.trust.record().lostMs!;
  expect(decideTrustV1({wallMs:1,pid:1,store:"ok",trust:replica.trust.record()},1,()=>true)).toBe("verify");
  await publishOnceV1(replica);
  await Bun.sleep(5);
  const again=await walkLocalV1(replica);
  expect(again.changed.length).toBe(0);
  trustWalkV1(replica,again.startedMs,again.changed,[]);
  expect(replica.trust.record().verifiedMs).toBe(again.startedMs);
  expect(again.startedMs).toBeGreaterThanOrEqual(lost);
  put(worktree,"b.txt","edited just now\n");
  const young=await walkLocalV1(replica);
  expect(young.changed.length).toBe(1);
  const before=replica.trust.record();
  trustWalkV1(replica,young.startedMs,young.changed,[]);
  expect(replica.trust.record().lostMs).toBe(before.lostMs);
  expect(replica.trust.record().verifiedMs).toBe(young.startedMs);
  expect(replica.trust.record().verifiedMs!).toBeGreaterThan(replica.trust.record().lostMs!);
  trustWalkV1(replica,young.startedMs+1,young.changed,["b.txt"]);
  expect(replica.trust.record().lostMs!).toBeGreaterThanOrEqual(before.lostMs!);
  expect(replica.trust.record().lostKind).toBe("sweep-drift");
 }finally{await closeReplicaV1(replica);}
});

test("the file sweep judges a changed file: hot files and young files are no drift, old cold files and vanished cold files are",async()=>{
 const store=temp("store"),worktree=temp("tree"),settings=options(store,worktree);
 put(worktree,"old.txt","old\n",600);put(worktree,"new.txt","new\n");
 await seedV1(settings);
 const replica=await openReplicaV1(settings,()=>{});
 try{
  const now=Date.now();
  replica.hot.expire(now+86_400_000);
  expect(sweptDriftV1(replica,"old.txt",now)).toBe(true);
  expect(sweptDriftV1(replica,"new.txt",now)).toBe(false);
  expect(sweptDriftV1(replica,"gone.txt",now)).toBe(true);
  replica.hot.note("old.txt",now);
  expect(sweptDriftV1(replica,"old.txt",now)).toBe(false);
 }finally{await closeReplicaV1(replica);}
});

test("a failure on a source path loses the trust of the tracker of an open replica and a failure in git objects does not",async()=>{
 const store=temp("store"),worktree=temp("tree"),settings=options(store,worktree);
 put(worktree,"a.txt","alpha\n");
 await seedV1(settings);
 const replica=await openReplicaV1(settings,()=>{});
 try{
  replica.trust.verified("walk",10);
  replica.trust.failure("blocked:objects:pack-incomplete");replica.trust.failure("refused:.🧬semio/🦑️repo/🎫️tickets/x/y:illegal-character");
  expect(replica.trust.record().lostMs).toBeNull();
  replica.trust.failure("unstable:src/a.rs");
  expect(replica.trust.record()).toMatchObject({lostKind:"unstable"});
 }finally{await closeReplicaV1(replica);}
});

test("the status reader accepts the health document of every state the daemon writes: pending none, waiting and refused, trust held, never verified and lost, with and without sidecars and failures",async()=>{
 const store=temp("store"),worktree=temp("tree"),settings=options(store,worktree),controller=new AbortController();
 put(worktree,"a.txt","alpha\n");
 await seedV1(settings);
 const done=runV1(settings,controller.signal,()=>{});
 try{
  await until(async()=>(await readHealthV1(worktree))?.trust.verifiedMs!=null);
  const report=await statusV1(settings),health=(await readHealthV1(worktree))!;
  expect(report.health).not.toBeNull();
  requireReplicaShapeV1("Status",report,"live status");
  controller.abort();await done.catch(()=>{});
  const digest="a".repeat(64),pendings=[null,{code:digest,ok:false as const,reason:"module x does not load"},{code:digest,ok:false as const,reason:"not adopted automatically: refused before"},{code:digest,ok:false as const,reason:"waiting: no origin runs this code yet"}];
  const trusts=[{scope:"shared-source" as const,verifiedMs:1_794_649_664_692,lostMs:null,lostKind:null},{scope:"shared-source" as const,verifiedMs:null,lostMs:null,lostKind:null},{scope:"shared-source" as const,verifiedMs:1_794_649_664_692,lostMs:1_794_649_700_000,lostKind:"blocked"},{scope:"shared-source" as const,verifiedMs:null,lostMs:1_794_649_700_000,lostKind:"sweep-drift"}];
  const failures=[[],[{kind:"loop-blocked-restart",count:3,lastMs:1_794_651_965_324},{kind:"unstable",count:2,lastMs:1_794_652_678_046}]];
  let count=0;
  for(const pending of pendings)for(const trust of trusts)for(const list of failures)for(const sidecars of [undefined,[{name:"build-agent",state:"running" as const,pid:1,restarts:0,sinceMs:1,lastExit:null,nextStartMs:null}]]){
   const variant={...health,pending,trust,failures:list,...sidecars===undefined?{sidecars:undefined}:{sidecars}};
   const document=JSON.parse(JSON.stringify(variant)) as typeof health;
   await writeHealthV1(worktree,document);
   const read=await readHealthV1(worktree);
   expect(read).toEqual(document);
   const shown={wallMs:read!.wallMs,store:read!.store,storeSinceMs:read!.storeSinceMs,code:read!.code,pending:read!.pending,failures:read!.failures,trust:read!.trust,...read!.sidecars===undefined?{}:{sidecars:read!.sidecars}};
   requireReplicaShapeV1("Status",{...report,health:shown},`state ${count}`);
   count++;
  }
  expect(count).toBe(64);
  await expect((async()=>{const {trust:_,...bare}=health;await writeHealthV1(worktree,bare as never);})()).rejects.toThrow();
 }finally{controller.abort();await done.catch(()=>{});}
});
