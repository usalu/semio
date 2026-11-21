import {afterAll,expect,mock,test} from "bun:test";
import {Database} from "bun:sqlite";
import sharingQueue from "./🕰️sharing-queue.json" with {type:"json"};
import * as real from "node:fs/promises";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {appendEventsV1,blobPathV1,formatHlcV1,readEventsV1,readHeadV1,storeBlobV1} from "../../📒️journal/🟦️.ts";
import spellingFixture from "./🧫️fixtures/🧺️spelling.json" with {type:"json"};
import outboundFixture from "./🧫️fixtures/⏩️outbound.json" with {type:"json"};
import {writeHintV1} from "../../🎣️tiers/🟦️.ts";
import {createStoreGuardV1} from "../../🛡️resilience/🟦️.ts";
import {createHash} from "node:crypto";
import type {ReplicaEventV1} from "../🟦️.ts";
import fixture from "../../🛡️resilience/🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};

type Fault={op:string;match:string;code:string;left:number};
let faults:Fault[]=[],hook:((op:string,path:string)=>void|Promise<void>)|undefined,readDelay:((path:string)=>number)|undefined;
const originals={...real};
const wrap=(op:string)=>async(...args:unknown[])=>{
 const path=args.map(String).join("\0"),fault=faults.find(item=>item.op===op&&item.left>0&&path.includes(item.match));
 if(fault){fault.left--;throw Object.assign(Error(`${fault.code}: injected, ${op}`),{code:fault.code});}
 await hook?.(op,path);
 const result=await (originals as unknown as Record<string,(...inner:unknown[])=>unknown>)[op]!(...args);
 const delay=op==="open"?readDelay?.(String(args[0])):undefined;
 if(delay){
  const file=result as Awaited<ReturnType<typeof real.open>>,read=file.read.bind(file) as (...inner:unknown[])=>unknown;
  Object.defineProperty(file,"read",{value:async (...inner:unknown[])=>{await Bun.sleep(delay);return read(...inner);}});
 }
 return result;
};
mock.module("node:fs/promises",()=>({...originals,...Object.fromEntries(["rename","unlink","copyFile","symlink","stat","lstat","open","readdir"].map(op=>[op,wrap(op)]))}));
const {applyQueue,closeReplicaV1,compactV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,runV1,seedV1,verifyV1}=await import("../🟦️.ts");

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`replica-race-${label}-`));scratch.push(path);return path;};
afterAll(()=>{faults=[];hook=undefined;readDelay=undefined;for(const path of scratch)rmSync(path,{recursive:true,force:true});});

for(const code of ["EINVAL","ENOENT"])test(`an apply whose rename races with a directory removal (${code}) is retried in place and reported as no error`,async()=>{
 const store=temp("store"),events:ReplicaEventV1[]=[],options=(name:string)=>({worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0});
 const a=options("a"),b=options("b");
 writeFileSync(join(a.worktree,".gitignore"),"ignored\n");
 await seedV1(a);await joinV1(b);
 const first=await openReplicaV1(a),second=await openReplicaV1(b,event=>events.push(event));
 const file=join(a.worktree,"race-target","deep","x.txt");
 mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"arrived");
 await publishOnceV1(first);
 faults=[{op:"rename",match:"race-target",code,left:2}];
 await followOnceV1(second);
 expect(events.filter(event=>event.event==="error")).toEqual([]);
 expect(faults[0]!.left).toBe(0);
 expect(existsSync(join(b.worktree,"race-target","deep","x.txt"))&&readFileSync(join(b.worktree,"race-target","deep","x.txt"),"utf8")).toBe("arrived");
 await closeReplicaV1(first);await closeReplicaV1(second);
},60000);

const settings=(store:string,name:string,clock:{ms:number})=>({worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0,now:()=>clock.ms});
async function pair(){
 const store=temp("store"),events:ReplicaEventV1[]=[],clock={ms:Date.now()},a=settings(store,"a",clock),b=settings(store,"b",clock);
 writeFileSync(join(a.worktree,"seed.txt"),"seed");
 await seedV1(a);await joinV1(b);
 return {store,a,b,clock,events};
}
const put=(who:{worktree:string},path:string,text:string)=>{const file=join(who.worktree,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};

for(const code of ["EBUSY","EPERM","EACCES"]){
 test(`a delete that fails with ${code} (file held open by another process) is queued, reported once as blocked, survives a restart and is applied after the release`,async()=>{
  const {store,a,b,clock,events}=await pair();
  put(a,"held.txt","content");
  const first=await openReplicaV1(a);await publishOnceV1(first);
  let second=await openReplicaV1(b,event=>events.push(event));await followOnceV1(second);
  expect(readFileSync(join(b.worktree,"held.txt"),"utf8")).toBe("content");
  rmSync(join(a.worktree,"held.txt"));await publishOnceV1(first);
  faults=[{op:"unlink",match:"held.txt",code,left:1000}];
  const result=await followOnceV1(second);
  expect(result).toMatchObject({events:1,applied:0,queued:1});
  expect(faults[0]!.left).toBe(999);
  expect(events.filter(event=>event.event==="blocked"&&event.path==="held.txt")).toEqual([{version:1,event:"blocked",path:"held.txt",reason:"sharing-violation"}]);
  expect(second.failures.diagnostics()).toContainEqual({kind:"blocked-sharing-violation",count:1,lastMs:clock.ms});
  expect(second.failures.diagnostics()).toContainEqual({kind:"blocked-incoming-worktree-sharing",count:1,lastMs:clock.ms});
  expect(events.filter(event=>event.event==="error")).toEqual([]);
  expect(existsSync(join(b.worktree,"held.txt"))).toBe(true);
  const head=(await readHeadV1(store,"a"))!;
  expect(second.cursors.a).toEqual({segment:head.segment,bytes:head.bytes});
  await closeReplicaV1(second);
  second=await openReplicaV1(b,event=>events.push(event));
  expect(second.queue.size).toBe(1);
  faults=[];clock.ms+=60_000;
  expect(await followOnceV1(second)).toMatchObject({events:0,applied:1,queued:0});
  expect(existsSync(join(b.worktree,"held.txt"))).toBe(false);
  await closeReplicaV1(first);await closeReplicaV1(second);
 },60000);
}

test("a sharing-blocked put spends one rename attempt per queue turn and does not delay unrelated work",async()=>{
 const {a,b,clock,store}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b);
 try{
  put(a,"busy.txt","arriving");put(a,"unrelated.txt","ready");
  await publishOnceV1(first);
  const db=new Database(":memory:"),started=clock.ms,source=blobPathV1(store,createHash("sha256").update("arriving").digest("hex"));
  let reads=0;hook=(op,path)=>{if(op==="open"&&path.startsWith(source+"\0"))reads++;};
  try{
   faults=[{op:"rename",match:"busy.txt",code:"EBUSY",left:sharingQueue.refusedAttempts}];
   const rows=db.query(`with rounds as (
     select key number,json_extract(value,'$.at') elapsed,json_extract(value,'$.due') ready,
       json_extract(value,'$.other') other from json_each(?1)
    ), progress as (
     select *,sum(elapsed>=ready) over(order by number) attempts from rounds
    )
    select elapsed,max(0,?2-attempts) remaining,other+(attempts>?2) applied,attempts<=?2 queued,
      min(attempts,?2) sharingCount,max(case when elapsed>=ready and attempts<=?2 then elapsed end) over(order by number) sharingAt
    from progress order by number`).all(JSON.stringify(sharingQueue.rounds),sharingQueue.refusedAttempts) as {elapsed:number;remaining:number;applied:number;queued:number;sharingCount:number;sharingAt:number}[];
   const expected=db.query("select count(distinct value) reads from json_each(?)").get(JSON.stringify(["arriving", "arriving", "arriving"])) as {reads:number};
   for(const row of rows){
    clock.ms=started+row.elapsed;
    expect(await followOnceV1(second)).toMatchObject({applied:row.applied,queued:row.queued});
    expect(faults[0]!.left).toBe(row.remaining);
    const details=second.failures.diagnostics();
    expect(details).toContainEqual({kind:"blocked-incoming-worktree-sharing",count:row.sharingCount,lastMs:started+row.sharingAt});
    expect(second.failures.summary()).toContainEqual({kind:"blocked",count:row.sharingCount,lastMs:started+row.sharingAt});
    expect(second.failures.summary()).not.toContainEqual({kind:"blocked-incoming-worktree-sharing",count:row.sharingCount,lastMs:started+row.sharingAt});
    expect(second.failures.diagnostics(2)).toEqual(details.slice(0,2));
    expect(readFileSync(join(b.worktree,"unrelated.txt"),"utf8")).toBe("ready");
   }
   expect(reads).toBe(expected.reads);
  }finally{db.close();}
  expect(readFileSync(join(b.worktree,"busy.txt"),"utf8")).toBe("arriving");
 }finally{hook=undefined;faults=[];await closeReplicaV1(first);await closeReplicaV1(second);}
},60000);


test("a Git input rename sharing violation stays outside the ordinary worktree diagnostic",async()=>{
 const {a,b,clock,store}=await pair(),second=await openReplicaV1(b),path=".git/refs/tags/fixture";
 try{
  const text="a".repeat(40)+"\n",source=join(a.worktree,"seed.txt");
  writeFileSync(source,text);
  const {sha256,bytes}=await storeBlobV1(store,"a",source);
  await appendEventsV1(store,"a",[{version:1,hlc:formatHlcV1({ms:++clock.ms,counter:0},"a"),operation:"put",path,sha256,bytes,executable:false,base:null}]);
  faults=[{op:"rename",match:"fixture",code:"EBUSY",left:1}];
  await followOnceV1(second);
  expect(faults[0]!.left).toBe(0);
  expect(second.queue.has(path)).toBe(true);
  const details=second.failures.diagnostics();
  expect(details).toContainEqual({kind:"blocked-sharing-violation",count:1,lastMs:clock.ms});
  expect(details.some(item=>item.kind==="blocked-incoming-worktree-sharing")).toBe(false);
 }finally{faults=[];await closeReplicaV1(second);}
},60000);

test("a new winning PUT does not inherit the prior sharing cooldown or its temporary bytes",async()=>{
 const {a,b,clock}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b),path="new-winner.txt";
 try{
  put(a,path,"first");await publishOnceV1(first);
  faults=[{op:"rename",match:path,code:"EBUSY",left:1}];
  expect((await followOnceV1(second)).queued).toBe(1);
  expect(readdirSync(second.paths.tmp).filter(name=>readFileSync(join(second.paths.tmp,name),"utf8")==="first")).toHaveLength(1);
  clock.ms++;put(a,path,"second");await publishOnceV1(first);
  expect(await followOnceV1(second)).toMatchObject({applied:1,queued:0});
  expect(readFileSync(join(b.worktree,path),"utf8")).toBe("second");
  expect(readdirSync(second.paths.tmp)).toEqual([]);
 }finally{faults=[];await closeReplicaV1(first);await closeReplicaV1(second);}
},60000);


for(const kind of ["private replacement","CAS replacement","abort","close"] as const)test("a sharing-blocked PUT revalidates or releases its staged file after "+kind,async()=>{
 const {a,b,clock,store}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b),path="exact.txt",body="source";
 let closed=false;
 try{
  put(a,path,body);await publishOnceV1(first);
  const source=blobPathV1(store,createHash("sha256").update(body).digest("hex"));
  faults=[{op:"rename",match:path,code:"EBUSY",left:1}];
  expect((await followOnceV1(second)).queued).toBe(1);
  const staged=readdirSync(second.paths.tmp);
  expect(staged).toHaveLength(1);
  if(kind==="close"){
   await closeReplicaV1(second);closed=true;expect(readdirSync(second.paths.tmp)).toEqual([]);return;
  }
  if(kind==="abort"){
   const controller=new AbortController();
   controller.abort(Error("fixture abort"));
   await expect(applyQueue(second,controller.signal)).rejects.toThrow("fixture abort");
   expect(readdirSync(second.paths.tmp)).toEqual([]);return;
  }
  const sourceChanged=kind==="CAS replacement",file=sourceChanged?source:join(second.paths.tmp,staged[0]!);
  const before=await real.stat(file);
  const swap=file+".next";writeFileSync(swap,"forged");utimesSync(swap,before.atime,before.mtime);await real.rename(swap,file);
  let reads=0;hook=(op,path)=>{if(op==="open"&&path.startsWith(source+"\0"))reads++;};
  clock.ms+=60_000;
  expect((await followOnceV1(second)).applied).toBe(sourceChanged?0:1);
  expect(reads).toBe(1);
  expect(readdirSync(second.paths.tmp)).toEqual([]);
  if(sourceChanged)expect(existsSync(join(b.worktree,path))).toBe(false);
  else expect(readFileSync(join(b.worktree,path),"utf8")).toBe(body);
 }finally{hook=undefined;faults=[];await closeReplicaV1(first);if(!closed)await closeReplicaV1(second);}
},60000);


test("concurrent queue turns cannot spend the same retained PUT twice",async()=>{
 const {a,b,clock}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b),path="one-owner.txt";
 let release!:()=>void,entered!:()=>void,attempts=0,active:Promise<unknown>|undefined;
 const barrier=new Promise<void>(done=>{release=done;}),intoRename=new Promise<void>(done=>{entered=done;});
 try{
  put(a,path,"exact");await publishOnceV1(first);faults=[{op:"rename",match:path,code:"EBUSY",left:1}];
  expect((await followOnceV1(second)).queued).toBe(1);
  hook=async(op,args)=>{if(op==="rename"&&args.endsWith(path)){attempts++;entered();await barrier;}};
  clock.ms+=60_000;
  active=applyQueue(second,undefined);await intoRename;
  expect(await applyQueue(second,undefined)).toMatchObject({applied:0,dropped:0});
  expect(attempts).toBe(1);release();expect(await active).toMatchObject({applied:1,dropped:0});
  expect(readFileSync(join(b.worktree,path),"utf8")).toBe("exact");expect(readdirSync(second.paths.tmp)).toEqual([]);
 }finally{release();await active?.catch(()=>{});hook=undefined;faults=[];await closeReplicaV1(first);await closeReplicaV1(second);}
},60000);

test("incoming items renew the store guard on journal, byte and materialisation progress",async()=>{
 const {a,b,clock}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b);
 try{
  for(const {bytes,observations} of fixture.intakeProgress){
   const name="incoming-"+bytes+".bin",content=Buffer.alloc(bytes,111);
   writeFileSync(join(a.worktree,name),content);await publishOnceV1(first);
   let beats=0;second.beat=()=>{beats++;};
   expect(await followOnceV1(second)).toMatchObject({applied:1,queued:0});
   expect(readFileSync(join(b.worktree,name))).toEqual(content);
   expect(beats,String(bytes)).toBe(observations+2);
   expect(await followOnceV1(second)).toMatchObject({applied:0,queued:0});
   expect(beats,String(bytes)).toBe(observations+2);clock.ms+=1000;
  }
 }finally{await closeReplicaV1(first);await closeReplicaV1(second);}
},60000);

for(const {name,deadlineMs,completionMs,stallMs,status} of fixture.materialisationProgress)test(`the store guard follows completed small materialisations: ${name}`,async()=>{
 const {a,b}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1({...b,concurrency:1});
 const probe=Promise.withResolvers<boolean>(),probing=Promise.withResolvers<void>(),release=Promise.withResolvers<void>();
 let now=0,beats=0,renames=0;
 const guard=createStoreGuardV1({probe:()=>{probing.resolve();return probe.promise;},deadlineMs,now:()=>now,changed:()=>{}});
 second.beat=()=>{beats++;guard.beat();};
 try{
  const count=completionMs.length+(stallMs===null?0:1);
  for(let i=0;i<count;i++)put(a,`small-${i}.txt`,"");
  await publishOnceV1(first);
  hook=async(op,path)=>{
   if(op!=="rename"||!path.includes(`\0${join(b.worktree,"small-")}`))return;
   await probing.promise;
   const completedAt=completionMs[renames++];
   if(completedAt===undefined&&stallMs!==null){now=stallMs;await release.promise;return;}
   now=completedAt!;
   await Bun.sleep(60);
  };
  const result=await guard.turn(signal=>followOnceV1(second,signal));
  expect(result.status).toBe(status);
  expect(beats).toBe(1+completionMs.length);
  if(status==="done"){
   expect(result).toMatchObject({value:{applied:count,queued:0}});
   await followOnceV1(second);
   expect(beats).toBe(1+completionMs.length);
  }
 }finally{
  release.resolve();probe.resolve(true);
  const until=Date.now()+2000;
  while(guard.outstanding()>0&&Date.now()<until)await Bun.sleep(10);
  hook=undefined;
  await closeReplicaV1(first);await closeReplicaV1(second);
 }
 expect(guard.outstanding()).toBe(0);
 expect(beats).toBe(1+completionMs.length);
 if(stallMs!==null)expect(existsSync(join(b.worktree,`small-${completionMs.length}.txt`))).toBe(true);
},60000);

test("a slow verified download survives a stalled probe while an idle download expires",async()=>{
 const {store,a,b,clock}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b),deadline=400;
 try{
  for(const [moving,delay] of [[true,90],[false,850]] as const){
   const name=`${moving?"moving":"idle"}.bin`,content=Buffer.alloc(6*1024*1024,moving?51:52),blob=blobPathV1(store,createHash("sha256").update(content).digest("hex"));
   writeFileSync(join(a.worktree,name),content);await publishOnceV1(first);
   const probe=Promise.withResolvers<boolean>(),guard=createStoreGuardV1({probe:()=>probe.promise,deadlineMs:deadline,now:Date.now,changed:()=>{}});
   let beats=0;second.beat=()=>{beats++;guard.beat();};readDelay=path=>path===blob?delay:0;
   try{
    const result=await guard.turn(signal=>followOnceV1(second,signal));
    expect(result.status).toBe(moving?"done":"stalled");
    if(moving){expect(result).toMatchObject({value:{applied:1,queued:0}});expect(readFileSync(join(b.worktree,name))).toEqual(content);}
    else expect(existsSync(join(b.worktree,name))).toBe(false);
   }finally{
    probe.resolve(true);
    const until=Date.now()+2000;
    while(guard.outstanding()>0&&Date.now()<until)await Bun.sleep(10);
    expect(guard.outstanding()).toBe(0);
    expect(beats).toBe(moving?8:1);readDelay=undefined;
   }
   clock.ms+=1000;
  }
 }finally{await closeReplicaV1(first);await closeReplicaV1(second);}
},60000);

test("an apply whose rename onto a held file fails keeps the old content, keeps one verified private staging file until the release",async()=>{
 const {a,b,clock,events}=await pair();
 put(a,"doc.txt","one");
 const first=await openReplicaV1(a);await publishOnceV1(first);
 const second=await openReplicaV1(b,event=>events.push(event));await followOnceV1(second);
 const visible=readdirSync(b.worktree);
 put(a,"doc.txt","two, a bit longer");await publishOnceV1(first);
 faults=[{op:"rename",match:"doc.txt",code:"EPERM",left:1000}];
 expect(await followOnceV1(second)).toMatchObject({applied:0,queued:1});
 expect(readFileSync(join(b.worktree,"doc.txt"),"utf8")).toBe("one");
 expect(readdirSync(b.worktree)).toEqual(visible);
 const staged=readdirSync(second.paths.tmp);
 expect(staged).toHaveLength(1);
 expect(readFileSync(join(second.paths.tmp,staged[0]!),"utf8")).toBe("two, a bit longer");
 expect(events.some(event=>event.event==="blocked"&&event.path==="doc.txt")).toBe(true);
 faults=[];clock.ms+=60_000;
 expect(await followOnceV1(second)).toMatchObject({applied:1,queued:0});
 expect(readFileSync(join(b.worktree,"doc.txt"),"utf8")).toBe("two, a bit longer");
 expect(readdirSync(second.paths.tmp)).toEqual([]);
 await closeReplicaV1(first);await closeReplicaV1(second);
},60000);

test("a write into the staging file that fails with EACCES is reported as blocked, not as an error, and is retried",async()=>{
 const {a,b,clock,events}=await pair();
 put(a,"new.txt","fresh");
 const first=await openReplicaV1(a);await publishOnceV1(first);
 const second=await openReplicaV1(b,event=>events.push(event));
 faults=[{op:"open",match:`${join(".cache","f","tmp")}`,code:"EACCES",left:1000}];
 expect(await followOnceV1(second)).toMatchObject({applied:0,queued:1});
 expect(events.filter(event=>event.event==="blocked"&&event.path==="new.txt").map(event=>event.reason)).toEqual(["EACCES"]);
 expect(second.failures.summary()).toContainEqual({kind:"blocked",count:1,lastMs:clock.ms});
 expect(second.failures.diagnostics()).toContainEqual({kind:"blocked-eacces",count:1,lastMs:clock.ms});
 expect(second.failures.diagnostics().some(item=>item.kind==="blocked-incoming-worktree-sharing")).toBe(false);
 expect(events.filter(event=>event.event==="error")).toEqual([]);
 faults=[];clock.ms+=60_000;
 expect(await followOnceV1(second)).toMatchObject({applied:1,queued:0});
 expect(readFileSync(join(b.worktree,"new.txt"),"utf8")).toBe("fresh");
 await closeReplicaV1(first);await closeReplicaV1(second);
},60000);

test("a local file that cannot be read because another process holds it is retried by the next publish and is neither published half nor lost",async()=>{
 const {a,b}=await pair();
 put(a,"busy.txt","held then released");
 const events:ReplicaEventV1[]=[],first=await openReplicaV1(a,event=>events.push(event)),second=await openReplicaV1(b);
 faults=[{op:"open",match:"busy.txt",code:"EBUSY",left:1000}];
 const held=await publishOnceV1(first,undefined,["busy.txt"]);
 expect(held).toMatchObject({published:0,retry:["busy.txt"]});
 expect(first.files.get("busy.txt")).toBeUndefined();
 faults=[];
 expect(await publishOnceV1(first,undefined,held.retry)).toMatchObject({published:1,retry:[]});
 await followOnceV1(second);
 expect(readFileSync(join(b.worktree,"busy.txt"),"utf8")).toBe("held then released");
 expect(events.filter(event=>event.event==="blocked"||event.event==="published"||event.event==="error")).toEqual([{version:1,event:"blocked",path:"busy.txt",reason:"EBUSY"},{version:1,event:"published",count:1,bytes:18}]);
 await closeReplicaV1(first);await closeReplicaV1(second);
},60000);

test("a journal head that cannot be replaced (EBUSY after every retry) neither crashes the publish nor loses the event: the paths are returned for retry and published later",async()=>{
 const {a,b}=await pair();
 put(a,"kept.txt","journal busy");put(a,"also.txt","two");
 const events:ReplicaEventV1[]=[],first=await openReplicaV1(a,event=>events.push(event)),second=await openReplicaV1(b),before={...first.cursors.a};
 faults=[{op:"rename",match:"head.json",code:"EBUSY",left:5}];
 const held=await publishOnceV1(first,undefined,["kept.txt","also.txt"]);
 expect(held.published).toBe(0);expect([...held.retry].sort()).toEqual(["also.txt","kept.txt"]);
 expect(first.files.get("kept.txt")).toBeUndefined();expect(first.cursors.a).toEqual(before);
 expect((await followOnceV1(second)).events).toBe(0);
 expect(await publishOnceV1(first,undefined,held.retry)).toMatchObject({published:2,retry:[]});
 await followOnceV1(second);
 expect(readFileSync(join(b.worktree,"kept.txt"),"utf8")).toBe("journal busy");expect(readFileSync(join(b.worktree,"also.txt"),"utf8")).toBe("two");
 expect(events.filter(event=>event.event==="blocked")).toEqual([{version:1,event:"blocked",reason:"EBUSY"}]);expect(events.filter(event=>event.event==="error")).toEqual([]);
 await closeReplicaV1(first);await closeReplicaV1(second);
},60000);

test("a symlink that cannot be created for lack of privilege is skipped with a visible event, the other files of the round are applied, nothing is published for it and verify lists it",async()=>{
 const {a,b,events}=await pair();
 put(a,"target.txt","t");symlinkSync("target.txt",join(a.worktree,"link"));put(a,"next.txt","n");
 const first=await openReplicaV1(a);await publishOnceV1(first);
 const second=await openReplicaV1(b,event=>events.push(event));
 faults=[{op:"symlink",match:join(b.worktree,"link"),code:"EPERM",left:1000}];
 const result=await followOnceV1(second);
 expect(result).toMatchObject({dropped:0,queued:0});
 expect(events.filter(event=>event.event==="skipped")).toEqual([{version:1,event:"skipped",path:"link",reason:"symlink-unsupported"}]);
 expect(events.filter(event=>event.event==="error")).toEqual([]);
 expect(readFileSync(join(b.worktree,"next.txt"),"utf8")).toBe("n");
 expect(existsSync(join(b.worktree,"link"))).toBe(false);
 expect((await publishOnceV1(second)).published).toBe(0);
 await closeReplicaV1(second);await closeReplicaV1(first);
 faults=[];
 const report=await verifyV1(b);
 expect(report.drift).toEqual([{path:"link",kind:"missing-local",expected:"target.txt"}]);
},60000);

test("a path that the operating system refuses as too long is skipped with a visible event when applied and when observed, and the walk does not fail",async()=>{
 const {a,b,events}=await pair();
 put(a,"long.txt","l");
 const first=await openReplicaV1(a);await publishOnceV1(first);
 const second=await openReplicaV1(b,event=>events.push(event));
 faults=[{op:"lstat",match:join(b.worktree,"long.txt"),code:"ENAMETOOLONG",left:1}];
 expect(await followOnceV1(second)).toMatchObject({dropped:1,queued:0});
 expect(events.filter(event=>event.event==="skipped")).toEqual([{version:1,event:"skipped",path:"long.txt",reason:"too-long"}]);
 put(b,"deep.txt","d");events.length=0;
 faults=[{op:"lstat",match:join(b.worktree,"deep.txt"),code:"ENAMETOOLONG",left:1}];
 expect((await publishOnceV1(second,undefined,["deep.txt"])).published).toBe(0);
 expect(events).toEqual([{version:1,event:"skipped",path:"deep.txt",reason:"too-long"}]);
 await closeReplicaV1(second);await closeReplicaV1(first);
},60000);

const aged=(file:string,seconds:number)=>{const stamp=Date.now()/1000-seconds;utimesSync(file,stamp,stamp);};

const blobNames=(count:number)=>Array.from({length:count},(_,index)=>`ab${index.toString(16).padStart(62,"0")}`);
function strew(store:string,names:readonly string[]):string{
 const folder=join(store,"v1","blobs","ab");
 mkdirSync(folder,{recursive:true});
 for(const name of names){writeFileSync(join(folder,name),"x");aged(join(folder,name),86400*30);}
 return folder;
}

test("compact --collect-blobs streams: one batch of doomed blobs per directory waits at most, and cancelling stops between files",async()=>{
 const {store}=await pair(),folder=strew(store,blobNames(5000)),controller=new AbortController();
 let statCalls=0,unlinks=0,statsAtCancel=-1;
 hook=(op,path)=>{
  if(!path.includes(folder))return;
  if(op==="stat")statCalls++;
  if(op==="unlink"&&++unlinks===100){statsAtCancel=statCalls;controller.abort();}
 };
 await expect(compactV1(store,"a",14,controller.signal,()=>{},Date.now,{collectBlobs:true})).rejects.toMatchObject({name:"AbortError"});
 hook=undefined;
 expect(statsAtCancel).toBeGreaterThan(0);expect(statsAtCancel).toBeLessThanOrEqual(2048+100+8);
 const left=readdirSync(folder).length;
 expect(left).toBeGreaterThan(4800);expect(left).toBeLessThan(5000);
},120000);

test("compact --collect-blobs reports progress with the removed count and spares a blob that an event references after the scan began",async()=>{
 const {store}=await pair(),names=blobNames(60),late=names[30]!,folder=strew(store,names),progress:ReplicaEventV1[]=[];
 let appended=false;
 hook=async(op,path)=>{
  if(op!=="stat"||!path.endsWith(late)||appended)return;
  appended=true;
  await appendEventsV1(store,"a",[{version:1,hlc:formatHlcV1({ms:Date.now()+5000,counter:0},"a"),operation:"put",path:"late.txt",sha256:late,bytes:1,executable:false,base:null}]);
 };
 const result=await compactV1(store,"a",14,undefined,event=>progress.push(event),Date.now,{collectBlobs:true});
 hook=undefined;
 expect(appended).toBe(true);expect(result.blobs).toBe(59);expect(readdirSync(folder)).toEqual([late]);
 const last=progress.filter(event=>event.stage==="compact").at(-1)!;
 expect(last).toMatchObject({done:last.total,count:59});
},120000);

test("compact removes staging files older than six hours, also those of other origins and of the relay, and keeps younger ones",async()=>{
 const {store}=await pair(),tmp=join(store,"v1","tmp"),hex="0123456789abcdef";
 mkdirSync(tmp,{recursive:true});
 for(const name of [`relay-${hex}`,`b-4242-${hex}`,`a-${hex}`,".nfs0001"])writeFileSync(join(tmp,name),"stale");
 const result=await compactV1(store,"a",14,undefined,()=>{},()=>Date.now()+7*3600*1000);
 expect(result.staging).toBe(3);expect(readdirSync(tmp)).toEqual([".nfs0001"]);
 writeFileSync(join(tmp,`relay-${hex}`),"fresh");
 expect((await compactV1(store,"a",14,undefined,()=>{},()=>Date.now()+5*3600*1000)).staging).toBe(0);
},60000);

/** ⏩ Replays an explicit hint while a separate large local preparation is waiting on its first source probe. */
for(const {replace} of outboundFixture.cases)test("an ordinary large outbound preparation cannot delay a fresh exact-path hint"+(replace?" or publish replaced bytes":""),async()=>{
 const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifactRoot)throw Error("Original ticket or process-test artifact root required");
 const base=mkdtempSync(join(artifactRoot,"lr-"));scratch.push(base);
 const store=join(base,"s"),worktree=join(base,"w");
 mkdirSync(worktree);writeFileSync(join(worktree,"anchor.txt"),"initial");
 const options={worktree,storeRoot:store,originId:"g1-server",groupId:"g1",watch:false,lock:false,pollMilliseconds:10,scanSeconds:3600,debounceMilliseconds:5,persistMilliseconds:0,hintMilliseconds:15};
 await seedV1(options);
 const position=await readHeadV1(store,options.originId),database=new Database(":memory:"),controller=new AbortController();
 const gate=Promise.withResolvers<void>(),probe=Promise.withResolvers<void>();
 const wait=async(predicate:()=>Promise<boolean>,milliseconds:number)=>{const end=Date.now()+milliseconds;do{if(await predicate())return true;await Bun.sleep(30);}while(Date.now()<end);return false;};
 const {large,hint,sequence,replacement}=outboundFixture,lookup=()=>readEventsV1(store,options.originId,position??undefined);
 const contentSha=(text:string)=>{const chunk=Buffer.alloc(1<<20),hash=createHash("sha256");chunk.write(text);for(let remaining=large.bytes;remaining>0;remaining-=chunk.length){hash.update(chunk.subarray(0,Math.min(remaining,chunk.length)));if(remaining===large.bytes)chunk.fill(0);}return hash.digest("hex");};
 const original=contentSha(""),latest=replace?contentSha(replacement):original;
 const daemon=runV1(options,controller.signal);
 try{
  expect(await wait(async()=>existsSync(join(worktree,".cache","f","health.json")),10000)).toBe(true);
  const source=join(worktree,large.path);
  writeFileSync(source,"");
  await real.truncate(source,large.bytes);
  const oldBlob=blobPathV1(store,original);
  hook=async(op,path)=>{if(replace?op==="rename"&&path.endsWith("\0"+oldBlob):op==="stat"&&path===source){probe.resolve();await gate.promise;}};
  await writeHintV1(worktree,[large.path]);
  let held=false;void probe.promise.then(()=>{held=true;});
  expect(await wait(async()=>held,10000)).toBe(true);
  writeFileSync(join(worktree,hint.path),hint.text);
  await writeHintV1(worktree,[hint.path]);
  const reached=await wait(async()=>(await lookup()).events.some(row=>row.path===hint.path),3000);
  const paths=(await lookup()).events.filter(item=>item.path===large.path||item.path===hint.path).map(item=>item.path);
  const oracle=database.query("select value path from json_each(?1) where key<?2").all(JSON.stringify(sequence),1).map(row=>(row as {path:string}).path);
  expect(reached).toBe(true);
  expect(paths).toEqual(oracle);
  const event=(await lookup()).events.find(row=>row.path===hint.path);
  expect(event?.operation==="put"?event.sha256:null).toBe(createHash("sha256").update(hint.text).digest("hex"));
  if(replace){const handle=await real.open(source,"r+");try{await handle.write(replacement,0,"utf8");}finally{await handle.close();}}
  gate.resolve();
  expect(await wait(async()=>(await lookup()).events.some(row=>row.path===large.path),10000)).toBe(true);
  const resulting=(await lookup()).events.filter(row=>sequence.includes(row.path));
  expect(resulting.map(row=>row.path)).toEqual(sequence);
  const largeEvent=resulting.find(row=>row.path===large.path);
  expect(largeEvent?.operation==="put"?largeEvent.sha256:null).toBe(latest);
 }finally{
  hook=undefined;gate.resolve();controller.abort();await daemon;database.close();
 }
},35000);

/** 🧺 Enumerates exact Unicode spellings once per physical directory version during targeted publication. */
for(const row of spellingFixture.cases)test("targeted publication shares an exact spelling snapshot for "+row.id,async()=>{
 const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifactRoot)throw Error("Original ticket or process-test artifact root required");
 const base=mkdtempSync(join(artifactRoot,"ls-"));scratch.push(base);
 const worktree=join(base,"w"),storeRoot=join(base,"s"),dir=join(worktree,spellingFixture.directory);
 mkdirSync(worktree);writeFileSync(join(worktree,"anchor.txt"),"initial");
 const options={worktree,storeRoot,originId:"g1-server",groupId:"g1",persistMilliseconds:0};
 await seedV1(options);
 const replica=await openReplicaV1(options),head=await readHeadV1(storeRoot,options.originId),database=new Database(":memory:");
 let reads=0,created=false;
 const files=Array.from({length:row.files},(_unused,i)=>spellingFixture.namePrefix+i+".txt");
 const paths=files.map(file=>spellingFixture.directory+"/"+file),last=join(dir,files.at(-1)!);
 try{
  mkdirSync(dir);
  for(const file of row.createLast?files.slice(0,-1):files)writeFileSync(join(dir,file),file);
  hook=(op,path)=>{
   if(op==="readdir"&&path===dir){
    reads++;
    if(row.firstReadFails&&reads===1)throw Object.assign(Error("Fixture listing refused"),{code:"EACCES"});
   }
   if(row.createLast&&op==="lstat"&&path.split("\0")[0]===last&&!created){created=true;writeFileSync(last,files.at(-1)!);}
  };
  const published=await publishOnceV1(replica,undefined,paths);
  const oracle=database.query("select count(*) scans, sum(value) files from json_each(?1)").get(JSON.stringify(row.epochs)) as {scans:number;files:number};
  expect(published.published).toBe(oracle.files);
  const events=(await readEventsV1(storeRoot,options.originId,head??undefined)).events;
  expect(events.map(event=>event.path).sort()).toEqual(paths.slice(row.firstReadFails?1:0).sort());
  expect(events.every(event=>event.operation==="put"&&event.sha256===createHash("sha256").update(files[paths.indexOf(event.path)]!).digest("hex"))).toBe(true);
  expect(reads).toBe(oracle.scans);
 }finally{hook=undefined;await closeReplicaV1(replica);database.close();}
},20000);

/** 🧮️ Keeps an unchanged cooling queue's structural proof until a new register changes its authority. */
test("a cooling incoming queue reuses a settled clash pass and rechecks a new winner",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Original ticket or process-test artifact root required");
 const root=mkdtempSync(join(artifacts,"qc-"));scratch.push(root);
 const storeRoot=join(root,"s"),clock={ms:Date.now()},a={worktree:join(root,"a"),storeRoot,originId:"a",groupId:"g1",now:()=>clock.ms},b={...a,originId:"b",worktree:join(root,"b")};
 mkdirSync(a.worktree);
 put(a,"seed.txt","seed");
 await seedV1(a);await joinV1(b);
 const first=await openReplicaV1(a),second=await openReplicaV1(b),oracle=new Database(":memory:");
 const sample=sharingQueue.settledClash,path=sample.path;
 let queries=0;
 const find=second.folds.find.bind(second.folds);
 second.folds.find=(path)=>{queries++;return find(path);};
 try{
  put(a,path,sample.first);
  await publishOnceV1(first,undefined,[path]);
  faults=[{op:"rename",match:path.split("/").at(-1)!,code:"EBUSY",left:1000}];
  expect(await followOnceV1(second)).toMatchObject({events:1,applied:0,queued:1});
  expect(queries).toBeGreaterThan(0);
  const kept=faults[0]!.left;
  const frozen=oracle.query("select value as queued, 0 as reads from json_each(?1)").all(JSON.stringify(sample.cooling)) as {queued:number;reads:number}[];
  for(const state of frozen){
   queries=0;
   expect(await followOnceV1(second)).toMatchObject({events:0,applied:0,queued:state.queued});
   expect(queries).toBe(state.reads);
   expect(faults[0]!.left).toBe(kept);
  }
  faults=[];clock.ms++;
  put(a,path,sample.replacement);
  await publishOnceV1(first,undefined,[path]);
  queries=0;
  expect(await followOnceV1(second)).toMatchObject({events:1,applied:1,queued:0});
  expect(queries).toBeGreaterThan(0);
  expect(readFileSync(join(b.worktree,path),"utf8")).toBe(sample.replacement);
 }finally{faults=[];oracle.close();await closeReplicaV1(first);await closeReplicaV1(second);}
},30000);
