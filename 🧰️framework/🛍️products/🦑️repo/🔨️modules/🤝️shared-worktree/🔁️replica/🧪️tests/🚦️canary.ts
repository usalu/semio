import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {publishRefusalV1,readRefusalsV1,transientBackoffMsV1,transientRefusalCountV1,transientStartV1} from "../../🚦️canary/🟦️.ts";
import {appendEventsV1,formatHlcV1,storeBlobV1} from "../../📒️journal/🟦️.ts";
import {codeDigestV1,readAttestationV1,writeAttestationV1} from "../../🩺️attest/🟦️.ts";
import {silentLatencyV1} from "../../🎣️tiers/🟦️.ts";
import {readRunHistoryV1,recordRunV1,sealCodeV1} from "../../🛡️resilience/🟦️.ts";
import {admitCodeV1,applyQueue,closeReplicaV1,createAdmissionStateV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,runV1,seedV1,type ReplicaEventV1,type ReplicaOptionsV1} from "../🟦️.ts";

setDefaultTimeout(120_000);
const scratch:string[]=[],module_="mod";
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`canary-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const until=async(condition:()=>boolean|Promise<boolean>,milliseconds=20_000):Promise<void>=>{const end=Date.now()+milliseconds;while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(20);}};
const git={present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)};
const fast={pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,viewMilliseconds:300,viewDebounceMilliseconds:50,logFiles:[] as string[]};
const attest=(store:string,origin:string,code:string,wallMs:number,extra:{sync?:boolean}={}):Promise<void>=>writeAttestationV1(store,{version:1,originId:origin,groupId:"g1",platform:"linux",wallMs,hlc:`0000001000000-000000-${origin}`,sync:extra.sync??true,code,cursors:{},files:1,manifest:"1".repeat(64),head:null,indexTree:null,view:null,guards:null,latency:silentLatencyV1(),queue:{incoming:0}});
const bounded=<T>(work:()=>Promise<T>):Promise<T>=>work();

test("the deadlock: new code whose files were written on three origins is not waited on for ever, the first candidate goes after two minutes, the next one a turn later, and nobody goes before",async()=>{
 const store=temp("store"),roots={a1:temp("a"),b1:temp("b"),c1:temp("c")};
 const options=(who:keyof typeof roots):ReplicaOptionsV1=>({worktree:roots[who],storeRoot:store,originId:who,groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false});
 for(const root of Object.values(roots)){put(root,`${module_}/x/🟦️.ts`,"export const x=1;\n");put(root,`${module_}/y/🟦️.ts`,"export const y=1;\n");put(root,`${module_}/z/🟦️.ts`,"export const z=1;\n");}
 await seedV1(options("a1"));
 for(const who of ["b1","c1"] as const)await joinV1(options(who));
 const running=await codeDigestV1(join(roots.a1,module_)),replicas={a1:await openReplicaV1(options("a1")),b1:await openReplicaV1(options("b1")),c1:await openReplicaV1(options("c1"))};
 for(const who of ["a1","b1","c1"] as const)await sealCodeV1({root:roots[who],moduleRoot:join(roots[who],module_),digest:running,digestOf:codeDigestV1});
 for(const [who,name] of [["a1","x"],["b1","y"],["c1","z"]] as const){put(roots[who],`${module_}/${name}/🟦️.ts`,`export const ${name}=2;\n`);await publishOnceV1(replicas[who]);}
 for(const who of ["a1","b1","c1"] as const){for(let n=0;n<2;n++){await followOnceV1(replicas[who]);await applyQueue(replicas[who],undefined);}}
 const mixed=await codeDigestV1(join(roots.a1,module_));
 for(const who of ["b1","c1"] as const)expect(await codeDigestV1(join(roots[who],module_))).toBe(mixed);
 let clock=Date.now();
 const states={a1:createAdmissionStateV1(),b1:createAdmissionStateV1(),c1:createAdmissionStateV1()},ask=(who:keyof typeof roots)=>admitCodeV1(replicas[who],{digest:mixed,running,moduleRoot:join(roots[who],module_),bounded,state:states[who],now:()=>clock});
 const heartbeat=async():Promise<void>=>{for(const who of ["a1","b1","c1"] as const)await attest(store,who,running,clock);};
 await heartbeat();
 for(const who of ["a1","b1","c1"] as const)expect(await ask(who)).toMatchObject({ok:false,reason:expect.stringContaining("no origin runs this code yet")});
 const first=clock;
 for(let second=5;second<120;second+=5){clock=first+second*1000;await heartbeat();for(const who of ["a1","b1","c1"] as const)expect((await ask(who)).ok,`${who} at ${second} s`).toBe(false);}
 clock=first+120_000;await heartbeat();
 expect(await ask("a1")).toEqual({ok:true});
 expect(await ask("b1")).toMatchObject({ok:false,reason:expect.stringContaining("a1 goes before this machine")});
 expect(await ask("c1")).toMatchObject({ok:false,reason:expect.stringContaining("goes before this machine")});
 clock=first+419_000;await heartbeat();
 expect((await ask("b1")).ok).toBe(false);
 clock=first+420_000;await heartbeat();
 expect((await ask("b1")).ok).toBe(true);
 expect((await ask("c1")).ok).toBe(false);
 await attest(store,"a1",mixed,clock);
 expect(await ask("c1")).toMatchObject({ok:false,reason:expect.stringContaining("a1 runs this code and has to stay healthy")});
 for(const who of ["a1","b1","c1"] as const)await closeReplicaV1(replicas[who]);
});

test("the group server goes first when it is fresh, and the lowest fresh id when it is silent",async()=>{
 const store=temp("store"),root=temp("w"),other=temp("o");
 put(other,`${module_}/x/🟦️.ts`,"export const x=1;\n");
 await seedV1({worktree:other,storeRoot:store,originId:"g1-m09",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false});
 const options:ReplicaOptionsV1={worktree:root,storeRoot:store,originId:"g1-m02",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false};
 mkdirSync(root,{recursive:true});await joinV1(options);
 const replica=await openReplicaV1(options),sender=await openReplicaV1({worktree:other,storeRoot:store,originId:"g1-m09",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false});
 await followOnceV1(replica);await applyQueue(replica,undefined);
 const running=await codeDigestV1(join(root,module_));
 await sealCodeV1({root,moduleRoot:join(root,module_),digest:running,digestOf:codeDigestV1});
 put(other,`${module_}/x/🟦️.ts`,"export const x=2;\n");await publishOnceV1(sender);
 await followOnceV1(replica);await applyQueue(replica,undefined);
 const digest=await codeDigestV1(join(root,module_));
 expect(digest).not.toBe(running);
 let clock=Date.now();
 const state=createAdmissionStateV1(),ask=()=>admitCodeV1(replica,{digest,running,moduleRoot:join(root,module_),bounded,state,now:()=>clock});
 await attest(store,"g1-server",running,clock);await attest(store,"g1-m01",running,clock);
 const first=clock;
 expect((await ask()).ok).toBe(false);
 clock=first+120_000;await attest(store,"g1-server",running,clock);await attest(store,"g1-m01",running,clock);
 expect(await ask()).toMatchObject({ok:false,reason:expect.stringContaining("g1-m01 goes before this machine")});
 clock=first+420_000;await attest(store,"g1-m01",running,clock);
 expect((await ask()).ok).toBe(true);
 await closeReplicaV1(replica);await closeReplicaV1(sender);
});

test("churn does not restart the wait: a digest that changes every minute is adopted after fifteen minutes of continuous waiting, a refused one never, and the wait starts again only after the running digest was reached",async()=>{
 const store=temp("store"),root=temp("w"),other=temp("o");
 put(other,`${module_}/x/🟦️.ts`,"export const x=1;\n");
 await seedV1({worktree:other,storeRoot:store,originId:"g1-m09",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false});
 const options:ReplicaOptionsV1={worktree:root,storeRoot:store,originId:"g1-m12",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false};
 mkdirSync(root,{recursive:true});await joinV1(options);
 const replica=await openReplicaV1(options),sender=await openReplicaV1({worktree:other,storeRoot:store,originId:"g1-m09",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false});
 await followOnceV1(replica);await applyQueue(replica,undefined);
 const running=await codeDigestV1(join(root,module_));
 await sealCodeV1({root,moduleRoot:join(root,module_),digest:running,digestOf:codeDigestV1});
 let clock=Date.now();
 const state=createAdmissionStateV1(),beat=async():Promise<void>=>{for(const who of ["g1-server","g1-m01","g1-m02","g1-m05"])await attest(store,who,running,clock);await attest(store,"g1-m12",running,clock);};
 const digests:string[]=[],started=clock;
 const edit=async(n:number):Promise<string>=>{put(other,`${module_}/x/🟦️.ts`,`export const x=${n+10};\n`);await publishOnceV1(sender);await followOnceV1(replica);await applyQueue(replica,undefined);const digest=await codeDigestV1(join(root,module_));digests.push(digest);return digest;};
 const ask=(digest:string)=>admitCodeV1(replica,{digest,running,moduleRoot:join(root,module_),bounded,state,now:()=>clock});
 let verdictAt=-1;
 for(let minute=0;minute<=16;minute++){
  clock=started+minute*60_000;await beat();
  const digest=await edit(minute),verdict=await ask(digest);
  if(verdict.ok){verdictAt=minute;break;}
 }
 expect(new Set(digests).size).toBe(digests.length);
 expect(verdictAt).toBe(15);
 const fresh=createAdmissionStateV1();
 clock=started+16*60_000;await beat();
 const refusedDigest=digests.at(-1)!;
 await publishRefusalV1(store,"g1-m01",{digest:refusedDigest,reason:"self-check exited with 1: SyntaxError"},clock);
 const refusedState=createAdmissionStateV1();
 for(const minute of [0,20,40]){clock=started+16*60_000+minute*60_000;await beat();expect((await admitCodeV1(replica,{digest:refusedDigest,running,moduleRoot:join(root,module_),bounded,state:refusedState,now:()=>clock})).ok).toBe(false);}
 expect(fresh.since).toBeUndefined();
 expect((await admitCodeV1(replica,{digest:running,running,moduleRoot:join(root,module_),bounded,state,now:()=>clock})).ok).toBe(true);
 expect(state.since).toBeUndefined();
 await closeReplicaV1(replica);await closeReplicaV1(sender);
},120_000);

test("a refusal for a silent watchdog or a blocked loop is transient and expires with a growing back-off, a deterministic one stays",async()=>{
 const store=temp("store"),digest="d".repeat(64),other="e".repeat(64);
 expect(transientStartV1("the worktree code does not start: the watchdog of the daemon fell silent")).toBe(true);
 expect(transientStartV1("the worktree code does not start: the daemon was not ready after 120 s")).toBe(true);
 expect(transientStartV1("the worktree code does not start: exit 1")).toBe(false);
 expect(transientStartV1("self-check exited with 1: SyntaxError")).toBe(false);
 const independent=Bun.spawnSync(["python3","-I","-c","print([min(1800000,300000*2**max(0,n-1)) for n in (1,2,3,4,5,9)])"]).stdout.toString().trim();
 expect(JSON.stringify([1,2,3,4,5,9].map(transientBackoffMsV1)).replaceAll(",",", ")).toBe(independent);
 const silent="the worktree code does not start: the watchdog of the daemon fell silent";
 let clock=1_000_000;
 expect(await publishRefusalV1(store,"g1-m05",{digest,reason:silent},clock)).toBe(true);
 expect(await publishRefusalV1(store,"g1-m05",{digest,reason:silent},clock+1000)).toBe(false);
 expect(await publishRefusalV1(store,"g1-m05",{digest:other,reason:"self-check exited with 1: SyntaxError"},clock)).toBe(true);
 expect(await publishRefusalV1(store,"g1-m05",{digest:other,reason:silent},clock+10*3_600_000)).toBe(false);
 expect((await readRefusalsV1(store)).find(item=>item.digest===digest)!.reason.endsWith("[transient 1]")).toBe(true);
 const peers:{originId:string;groupId:string;code:string;wallMs:number;sync:boolean;fallback:boolean}[]=[];
 const {decideCanaryV1}=await import("../../🚦️canary/🟦️.ts");
 const verdictAt=async(at:number)=>decideCanaryV1({digest,running:"r".repeat(64),originId:"g1-m05",groupId:"g1",now:at,waitedMs:0,authored:false,peers,retired:[],snapshots:[],refusals:await readRefusalsV1(store),healthy:{},limits:{freshMs:60_000,waitMs:120_000,turnMs:300_000,canaryMs:60_000}});
 expect(await verdictAt(clock+299_000)).toMatchObject({ok:false,why:"refused"});
 expect(await verdictAt(clock+300_000)).toMatchObject({ok:false,why:"mixed"});
 clock+=300_000;
 expect(await publishRefusalV1(store,"g1-m05",{digest,reason:silent},clock)).toBe(true);
 expect(transientRefusalCountV1((await readRefusalsV1(store)).find(item=>item.digest===digest)!.reason)).toBe(2);
 expect(await verdictAt(clock+599_000)).toMatchObject({ok:false,why:"refused"});
 expect(await verdictAt(clock+600_000)).toMatchObject({ok:false,why:"mixed"});
});

test("a self-check that fails is refused for the whole fleet, one that only timed out is not, and a peer then no longer waits for the code but refuses it",async()=>{
 const store=temp("store"),check:{ok:false;reason:string}={ok:false,reason:"self-check exited with 1: SyntaxError"};
 const root=temp("code");put(root,"x/🟦️.ts","export const a=1;\n");
 const events:ReplicaEventV1[]=[],controller=new AbortController(),worktree=temp("a1");
 put(worktree,"a.txt","alpha\n");
 const options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"a1",groupId:"g1",...fast,codeRoot:root,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>check};
 await seedV1(options);
 const done=runV1(options,controller.signal,event=>events.push(event));
 await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
 put(root,"x/🟦️.ts","export const a=2;\n");
 const broken=await codeDigestV1(root);
 await until(()=>events.some(event=>event.event==="restart-refused"&&event.reason===broken));
 await until(async()=>(await readRefusalsV1(store)).some(item=>item.digest===broken));
 expect((await readRefusalsV1(store)).find(item=>item.digest===broken)).toMatchObject({originId:"a1",reason:"self-check exited with 1: SyntaxError"});
 check.reason="self-check timed out after 180000 ms";
 put(root,"x/🟦️.ts","export const a=3;\n");
 const slow=await codeDigestV1(root);
 await until(()=>events.some(event=>event.event==="restart-refused"&&event.reason===slow));
 await Bun.sleep(200);
 expect((await readRefusalsV1(store)).some(item=>item.digest===slow)).toBe(false);
 controller.abort();await done;
 const peerRoot=temp("p");
 const peerOptions:ReplicaOptionsV1={worktree:peerRoot,storeRoot:store,originId:"b1",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false};
 await joinV1(peerOptions);
 const peer=await openReplicaV1(peerOptions);
 expect(await admitCodeV1(peer,{digest:broken,running:"1".repeat(64),moduleRoot:join(peerRoot,"mod"),bounded})).toMatchObject({ok:false,reason:expect.stringContaining("refused by a1")});
 await closeReplicaV1(peer);
});

test("a daemon on the fallback code because the worktree code did not start refuses that code, for itself and for the fleet, and one that fell back because of a revert does not",async()=>{
 for(const [reason,refused] of [["the worktree code does not start: exit 1",true],["the code on disk ran before and was superseded",false]] as const){
  const store=temp("store"),running=temp("run"),watch=temp("watch"),worktree=temp("w");
  put(running,"x/🟦️.ts","export const a=1;\n");put(watch,"x/🟦️.ts","export const a=2;\n");put(worktree,"a.txt","alpha\n");
  const controller=new AbortController(),events:ReplicaEventV1[]=[],options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"a1",groupId:"g1",...fast,codeRoot:running,watchRoot:watch,fallback:{guard:"code-fallback",source:"snapshot",reason},gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true}),admit:async()=>({ok:false,reason:"waiting"})};
  await seedV1(options);
  const done=runV1(options,controller.signal,event=>events.push(event)),failed=await codeDigestV1(watch);
  await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
  await Bun.sleep(400);
  expect((await readRefusalsV1(store)).some(item=>item.digest===failed&&item.originId==="a1")).toBe(refused);
  expect((await readRunHistoryV1(worktree)).refused.includes(failed)).toBe(refused);
  controller.abort();await done;
 }
});

test("a code that the watchdog ended while it was new is refused and the daemon steps aside, an end before its first run or long after it is not blamed",async()=>{
 for(const [first,ended,blamed] of [[-120_000,-30_000,true],[-3_000_000,-30_000,false],[-30_000,-120_000,false]] as const){
  const store=temp("store"),running=temp("run"),worktree=temp("w"),now=Date.now();
  put(running,"x/🟦️.ts","export const a=1;\n");put(worktree,"a.txt","alpha\n");
  const digest=await codeDigestV1(running);
  await recordRunV1(worktree,digest,now+first);
  put(worktree,".cache/f/boot/loop-blocked-restart.json",JSON.stringify({version:1,count:1,pid:1,wallMs:now+ended,lagMs:130_000,phase:"run",reason:"blocked",activity:"sweep",sample:null}));
  const controller=new AbortController(),events:ReplicaEventV1[]=[],options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"a1",groupId:"g1",...fast,codeRoot:running,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true})};
  await seedV1(options);
  const done=runV1(options,controller.signal,event=>events.push(event));
  if(blamed){
   await done;
   expect(events.some(event=>event.event==="restarting")).toBe(true);
   expect((await readRefusalsV1(store)).some(item=>item.digest===digest)).toBe(true);
    expect((await readRunHistoryV1(worktree)).refused).not.toContain(digest);expect((await readRefusalsV1(store)).find(item=>item.digest===digest)!.reason).toMatch(/\[transient 1\]$/u);
  }else{
   await until(async()=>(await readAttestationV1(store,"a1"))!==undefined);
   await Bun.sleep(400);
   expect(events.some(event=>event.event==="restarting")).toBe(false);
   expect((await readRefusalsV1(store)).length).toBe(0);
   controller.abort();await done;
  }
 }
});

const idle=async(name:string,store:string,now:()=>number)=>{
 const root=temp(name),options:ReplicaOptionsV1={worktree:root,storeRoot:store,originId:name,groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false,now};
 mkdirSync(root,{recursive:true});await joinV1(options);
 return {root,replica:await openReplicaV1(options)};
};
const blob=async(store:string,origin:string,text:string)=>{const file=join(temp("blob"),"b");writeFileSync(file,text);return storeBlobV1(store,origin,file);};
const event=(ms:number,counter:number,origin:string,operation:"put"|"delete",path:string,sha256?:string,bytes?:number)=>({version:1 as const,hlc:formatHlcV1({ms,counter},origin),operation,path,...operation==="put"?{sha256,bytes,executable:false}:{},base:null});

test("a pack index whose pack was never published no longer holds back the deletions under .git/objects once it is ten minutes old, and a young one still does",async()=>{
 for(const [age,applied] of [[3_600_000,true],[30_000,false]] as const){
  const store=temp("store"),clock={ms:Date.now()};
  await seedV1({worktree:temp("seed"),storeRoot:store,originId:"seed",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false});
  const idx=await blob(store,"seed","idx\n"),object=await blob(store,"seed","loose\n"),path=".git/objects/12/34567890abcdef1234567890abcdef12345678";
  const {root,replica}=await idle("w1",store,()=>clock.ms);
  await appendEventsV1(store,"seed",[event(clock.ms-age,0,"seed","put",".git/objects/pack/pack-aa.idx",idx.sha256,4),event(clock.ms-age,1,"seed","put",path,object.sha256,6)]);
  await followOnceV1(replica);
  expect(existsSync(join(root,path))).toBe(true);expect(replica.queue.size).toBe(0);
  await appendEventsV1(store,"seed",[event(clock.ms-5000,2,"seed","delete",path)]);
  await followOnceV1(replica);await applyQueue(replica,undefined);
  expect(replica.queue.size===0,`age ${age}`).toBe(applied);
  expect(existsSync(join(root,path)),`age ${age}`).toBe(!applied);
  await closeReplicaV1(replica);
 }
});

test("a blocked apply pass backs off while nothing changes, doubling up to 30 s, and runs again at once when the queue changes",async()=>{
 const store=temp("store"),clock={ms:Date.now()};
 await seedV1({worktree:temp("seed"),storeRoot:store,originId:"seed",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false});
 const idx=await blob(store,"seed","idx\n"),object=await blob(store,"seed","loose\n");
 const paths=[1,2,3].map(n=>`.git/objects/1${n}/34567890abcdef1234567890abcdef12345678`);
 const {root,replica}=await idle("w1",store,()=>clock.ms);
 await appendEventsV1(store,"seed",[event(clock.ms-5000,0,"seed","put",".git/objects/pack/pack-aa.idx",idx.sha256,4),...paths.map((path,n)=>event(clock.ms-5000,n+1,"seed","put",path,object.sha256,6))]);
 await followOnceV1(replica);
 for(const path of paths)expect(existsSync(join(root,path))).toBe(true);
 await appendEventsV1(store,"seed",paths.map((path,n)=>event(clock.ms-4000,n+10,"seed","delete",path)));
 const blocked=():number=>replica.failures.summary().find(item=>item.kind==="blocked")?.count??0;
 await followOnceV1(replica);
 expect(replica.queue.size).toBe(3);
 const first=blocked();
 expect(first).toBeGreaterThanOrEqual(1);
 for(let n=0;n<50;n++)await applyQueue(replica,undefined);
 expect(blocked()).toBe(first);
 const steps:number[]=[];
 for(const wait of [499,1,1000,2000,4000,8000,16000,30000,30000]){
  clock.ms+=wait;
  const before=blocked();
  await applyQueue(replica,undefined);
  steps.push(blocked()-before);
 }
 expect(steps).toEqual([0,1,1,1,1,1,1,1,1]);
 for(const path of paths)expect(existsSync(join(root,path))).toBe(true);
 const before=blocked();
 await appendEventsV1(store,"seed",[event(clock.ms,0,"seed","put",".git/objects/pack/pack-bb.idx",idx.sha256,4)]);
 await followOnceV1(replica);
 expect(blocked()).toBe(before+1);
 await closeReplicaV1(replica);
});
