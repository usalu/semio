import {afterAll,expect,mock,test} from "bun:test";
import {Database} from "bun:sqlite";
import ts from "typescript";
import sharingQueue from "./🕰️sharing-queue.json" with {type:"json"};
import renameEpermFixture from "./🧫️fixtures/🧾️rename-eperm.json" with {type:"json"};
import windowsReplaceFixture from "./🧫️fixtures/🪟️windows-replace.json" with {type:"json"};
import * as real from "node:fs/promises";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {appendEventsV1,blobPathV1,formatHlcV1,readEventsV1,readHeadV1,storeBlobV1} from "../../📒️journal/🟦️.ts";
import spellingFixture from "./🧫️fixtures/🧺️spelling.json" with {type:"json"};
import outboundFixture from "./🧫️fixtures/⏩️outbound.json" with {type:"json"};
import durableFixture from "./🧫️fixtures/📬️durable-hints.json" with {type:"json"};
import {hintsPathV1,writeHintV1} from "../../🎣️tiers/🟦️.ts";
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
 test(`a delete that fails with ${code} is queued, reported once as blocked, survives a restart and is applied after the release`,async()=>{
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
  expect(second.failures.diagnostics()).toContainEqual({kind:`blocked-target-sharing-${code.toLowerCase()}`,count:1,lastMs:clock.ms});
  expect(second.failures.diagnostics()).toContainEqual({kind:"blocked-incoming-worktree-sharing",count:1,lastMs:clock.ms});
  expect(second.failures.diagnostics()).toContainEqual({kind:"blocked-target-sharing-unlink",count:1,lastMs:clock.ms});
  expect(second.failures.diagnostics().some(row=>row.kind==="blocked-target-sharing-rename")).toBe(false);
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
    expect(details).toContainEqual({kind:"blocked-target-sharing-ebusy",count:row.sharingCount,lastMs:started+row.sharingAt});
    expect(details).toContainEqual({kind:"blocked-target-sharing-rename",count:row.sharingCount,lastMs:started+row.sharingAt});
    expect(details.some(item=>item.kind==="blocked-target-sharing-unlink")).toBe(false);
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



test("ordinary sharing retry cohorts yield to the next follow and keep overdue paths eligible",async()=>{
 const {a,b,clock}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b),db=new Database(":memory:"),fixture=sharingQueue.cohort;
 try{
  const paths=Array.from({length:fixture.blockedPaths},(_,at)=>"cohort-held/"+at+".txt");
  for(const path of paths)put(a,path,"held");
  await publishOnceV1(first);
  faults=[{op:"rename",match:"cohort-held",code:"EBUSY",left:fixture.blockedPaths*3}];
  expect(await followOnceV1(second)).toMatchObject({events:fixture.blockedPaths,applied:0,queued:fixture.blockedPaths});
  expect(faults[0]!.left).toBe(fixture.blockedPaths*2);
  clock.ms+=fixture.firstRetryMilliseconds;
  put(a,fixture.unrelatedPath,"fresh");await publishOnceV1(first,undefined,[fixture.unrelatedPath]);
  const expected=db.query<{handled:number},[number,number]>(`with recursive runs(i,remaining) as (
    values(0,?1) union all select i+1,max(0,remaining-?2) from runs where i<3
  ) select ?1-remaining handled from runs where i>0 order by i`).all(fixture.blockedPaths,fixture.maximumRetriesPerTurn);
  for(const [at,row] of expected.entries()){
   expect(await followOnceV1(second)).toMatchObject({applied:at===0?1:0,queued:fixture.blockedPaths});
   expect(fixture.blockedPaths*2-faults[0]!.left).toBe(row.handled);
  }
  expect(readFileSync(join(b.worktree,fixture.unrelatedPath),"utf8")).toBe("fresh");
  faults=[];clock.ms+=2*fixture.firstRetryMilliseconds;
  for(const [at,row] of expected.entries()){
   expect(await followOnceV1(second)).toMatchObject({applied:row.handled-(expected[at-1]?.handled??0),queued:fixture.blockedPaths-row.handled});
  }
  expect(paths.every(path=>readFileSync(join(b.worktree,path),"utf8")==="held")).toBe(true);
 }finally{faults=[];db.close();await closeReplicaV1(first);await closeReplicaV1(second);}
},60000);

for(const row of renameEpermFixture.cases){
 const nativeTest=row.target==="link"&&process.platform==="win32"?test.skip:test;
 nativeTest(`a staged rename ${row.code} records only observed target metadata (${row.target}/${row.mode})`,async()=>{
  const {a,b,clock}=await pair(),first=await openReplicaV1(a),targetName="metadata-target.txt",target=join(b.worktree,targetName);
  if(row.target==="file"){put(a,targetName,"old content");await publishOnceV1(first);}
  const second=await openReplicaV1(b);
  const db=new Database(":memory:");
  try{
   await followOnceV1(second);
   if(row.target==="file")chmodSync(target,row.mode!);
   if(row.target==="link"){symlinkSync("seed.txt",target);clock.ms+=1_000;}
   put(a,targetName,"new content");await publishOnceV1(first);
   const expected=db.query(`select case when ?1<>'EPERM' then null
     when ?2='unseen' then 'unseen' when ?2<>'file' then 'other'
     when (?3 & 128)>0 then 'write' else 'no-write' end as bucket`)
     .get(row.code,row.target,row.mode) as {bucket:string|null};
   expect(expected.bucket===null?null:`blocked-rename-eperm-target-${expected.bucket}`).toBe(row.kind);
   let denied=false,targetReadsAfterDeny=0;
   hook=(op,paths)=>{
    const [from,to]=paths.split("\0");
    if(op==="rename"&&to===target&&!from!.includes("displaced-")){denied=true;throw Object.assign(Error("Fixture target rename refusal"),{code:row.code});}
    if(denied&&op==="lstat"&&from===target)targetReadsAfterDeny++;
   };
   expect(await followOnceV1(second)).toMatchObject({applied:0,queued:1});
   expect(denied).toBe(true);expect(targetReadsAfterDeny).toBe(0);
   expect(second.failures.diagnostics().filter(item=>item.kind.startsWith("blocked-rename-eperm-target-")))
    .toEqual(row.kind===null?[]:[{kind:row.kind,count:1,lastMs:clock.ms}]);
   expect(existsSync(target)).toBe(row.target!=="unseen");
  }finally{
   hook=undefined;db.close();
   if(row.target==="file")chmodSync(target,0o644);
   await closeReplicaV1(first);await closeReplicaV1(second);
  }
 },60_000);
}

for(const row of windowsReplaceFixture.cases){
 test(`a staged put over a Windows-semantics target replaces it or keeps the original intact (${row.name})`,async()=>{
  const {a,b,clock}=await pair(),first=await openReplicaV1(a),name="replace-target.exe",target=join(b.worktree,name);
  put(a,name,"old content");await publishOnceV1(first);
  const second=await openReplicaV1(b),db=new Database(":memory:");
  try{
   await followOnceV1(second);
   if(row.readOnly)chmodSync(target,0o444);
   let late=row.lateFailures;
   hook=(op,paths)=>{
    if(op!=="rename")return;
    const [from,to]=paths.split("\0");
    if(from!.includes("displaced-"))return;
    if(from===target&&to!.includes("displaced-")){if(row.asideBlocked)throw Object.assign(Error("EPERM: held open without delete sharing"),{code:"EPERM"});return;}
    if(to!==target)return;
    if(existsSync(target)&&(row.running||(lstatSync(target).mode&0o200)===0))throw Object.assign(Error("EPERM: access denied"),{code:"EPERM"});
    if(!existsSync(target)&&late>0){late--;throw Object.assign(Error("EPERM: access denied"),{code:"EPERM"});}
   };
   put(a,name,"new content, longer");await publishOnceV1(first);
   const expected=db.query(`select case when (?1=1 and ?2=1) or (?1=1 and ?3>0) then 'blocked' else 'installed' end as outcome`)
    .get(row.running?1:0,row.asideBlocked?1:0,row.lateFailures) as {outcome:string};
   const installed=expected.outcome==="installed";
   expect(await followOnceV1(second)).toMatchObject({applied:installed?1:0,queued:installed?0:1});
   expect(readFileSync(target,"utf8")).toBe(installed?"new content, longer":"old content");
   expect((lstatSync(target).mode&0o200)!==0).toBe(installed||!row.readOnly);
   expect(readdirSync(second.paths.tmp).filter(file=>file.startsWith("displaced-"))).toEqual([]);
   if(installed)expect(readdirSync(second.paths.tmp)).toEqual([]);
   else expect(readdirSync(second.paths.tmp)).toHaveLength(1);
   if(!installed){
    hook=undefined;chmodSync(target,0o644);clock.ms+=60_000;
    expect(await followOnceV1(second)).toMatchObject({applied:1,queued:0});
    expect(readFileSync(target,"utf8")).toBe("new content, longer");
   }
  }finally{hook=undefined;db.close();await closeReplicaV1(first);await closeReplicaV1(second);}
 },60_000);
}

test("a delete of a read-only target clears the attribute and removes it, and a refused removal puts the attribute back",async()=>{
 const {a,b,clock}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b),target=join(b.worktree,"gone.txt");
 try{
  put(a,"gone.txt","bye");await publishOnceV1(first);await followOnceV1(second);
  chmodSync(target,0o444);
  rmSync(join(a.worktree,"gone.txt"));await publishOnceV1(first);
  hook=(op,paths)=>{if(op==="unlink"&&paths===target&&(lstatSync(target).mode&0o200)===0)throw Object.assign(Error("EPERM: read-only"),{code:"EPERM"});};
  expect(await followOnceV1(second)).toMatchObject({applied:1,queued:0});
  expect(existsSync(target)).toBe(false);
  put(a,"gone2.txt","bye");await publishOnceV1(first);await followOnceV1(second);
  const other=join(b.worktree,"gone2.txt");
  chmodSync(other,0o444);rmSync(join(a.worktree,"gone2.txt"));await publishOnceV1(first);
  hook=(op,paths)=>{if(op==="unlink"&&paths===other)throw Object.assign(Error("EPERM: held"),{code:"EPERM"});};
  clock.ms+=1000;
  expect(await followOnceV1(second)).toMatchObject({applied:0,queued:1});
  expect(lstatSync(other).mode&0o200).toBe(0);
 }finally{hook=undefined;await closeReplicaV1(first);await closeReplicaV1(second);}
},60_000);

test("the files of the daemon's own module are applied before a backlog of ordinary files",async()=>{
 const {a,b}=await pair(),first=await openReplicaV1(a),module="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree";
 for(let n=0;n<30;n++)put(a,`a-backlog/${String(n).padStart(2,"0")}.txt`,`backlog ${n}`);
 put(a,`${module}/🔁️replica/🟦️.ts`,"export const code=1;\n");put(a,`${module}/🚦️canary/🟦️.ts`,"export const canary=1;\n");
 await publishOnceV1(first);
 const second=await openReplicaV1({...b,concurrency:1}),order:string[]=[];
 hook=(op,paths)=>{if(op!=="rename")return;const to=paths.split("\0")[1]!;if(to.startsWith(b.worktree))order.push(to.slice(b.worktree.length+1));};
 try{
  await followOnceV1(second);
  const codeAt=order.map((name,at)=>name.startsWith(module)?at:-1).filter(at=>at>=0),backlogAt=order.map((name,at)=>name.startsWith("a-backlog")?at:-1).filter(at=>at>=0);
  expect(codeAt).toHaveLength(2);expect(backlogAt.length).toBeGreaterThanOrEqual(30);
  expect(Math.max(...codeAt)).toBeLessThan(Math.min(...backlogAt));
  const independent=Bun.spawnSync(["python3","-I","-c","import json,sys; order=json.loads(sys.argv[1]); code=[i for i,n in enumerate(order) if n.startswith(sys.argv[2])]; rest=[i for i,n in enumerate(order) if n.startswith('a-backlog')]; print(max(code)<min(rest))",JSON.stringify(order),module]).stdout.toString().trim();
  expect(independent).toBe("True");
 }finally{hook=undefined;await closeReplicaV1(first);await closeReplicaV1(second);}
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
  expect(details).toContainEqual({kind:"blocked-target-sharing-ebusy",count:1,lastMs:clock.ms});
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
 expect(second.failures.diagnostics().some(item=>item.kind.startsWith("blocked-target-sharing-"))).toBe(false);
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


for(const scope of durableFixture.scopes)test("an incomplete "+scope+" hint survives a replica restart",async()=>{
 const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifactRoot)throw Error("Original ticket or process-test artifact root required");
 const base=mkdtempSync(join(artifactRoot,"lh-"));scratch.push(base);
 const worktree=join(base,"w"),storeRoot=join(base,"s"),options={worktree,storeRoot,originId:"g1-server",groupId:"g1",watch:false,lock:false,pollMilliseconds:10,scanSeconds:3600,debounceMilliseconds:5,persistMilliseconds:0,hintMilliseconds:15};
 const {blocked,published,unchanged,directory}=durableFixture;
 for(const path of [blocked.path,published.path,unchanged])put(options,path,"seed");
 await seedV1(options);
 const head=await readHeadV1(storeRoot,options.originId),lookup=async()=>(await readEventsV1(storeRoot,options.originId,head??undefined)).events;
 const until=async(check:()=>boolean|Promise<boolean>)=>{const end=Date.now()+5000;do{if(await check())return true;await Bun.sleep(20);}while(Date.now()<end);return false;};
 const database=new Database(":memory:"),first=new AbortController(),second=new AbortController();
 let running=runV1(options,first.signal),failed=false;
 try{
  expect(await until(()=>existsSync(join(worktree,".cache","f","health.json")))).toBe(true);
  hook=(operation,path)=>{if(operation==="open"&&path.split("\0")[0]===join(worktree,blocked.path)){failed=true;throw Object.assign(Error("Fixture sharing conflict"),{code:"EBUSY"});}};
  put(options,blocked.path,blocked.text);put(options,published.path,published.text);
  const pending=join(hintsPathV1(worktree),await writeHintV1(worktree,scope==="directory"?[directory]:[blocked.path,published.path]));
  const settled=join(hintsPathV1(worktree),await writeHintV1(worktree,[unchanged]));
  expect(await until(async()=>failed&&!existsSync(settled)&&(await lookup()).some(row=>row.path===published.path))).toBe(true);
  expect((await lookup()).some(row=>row.path===blocked.path)).toBe(false);
  first.abort();await running;
  expect(existsSync(pending)).toBe(true);
  hook=undefined;
  running=runV1(options,second.signal);
  expect(await until(()=>!existsSync(pending))).toBe(true);
  const expected=database.query("select json_extract(value, '$.path') path, json_extract(value, '$.text') body from json_each(?1) order by key").all(JSON.stringify([published,blocked])) as {path:string;body:string}[];
  expect((await lookup()).map(row=>[row.path,row.operation==="put"?row.sha256:null])).toEqual(expected.map(row=>[row.path,createHash("sha256").update(row.body).digest("hex")]));
 }finally{hook=undefined;first.abort();second.abort();await running;database.close();}
},18000);



test("only committed outbound event batches renew the store guard",async()=>{
 const {a,store}=await pair(),replica=await openReplicaV1(a),database=new Database(":memory:");
 const head=await readHeadV1(store,a.originId),path=durableFixture.published.path;
 const beats:number[]=[];
 replica.beat=()=>{beats.push(replica.cursors[a.originId]!.bytes);};
 try{
  put(a,path,durableFixture.published.text);
  faults=[{op:"rename",match:join(a.originId,"head.json"),code:"EBUSY",left:5}];
  expect((await publishOnceV1(replica,undefined,[path])).retry).toEqual([path]);
  expect(beats).toEqual([]);
  await publishOnceV1(replica,undefined,[path]);
  await publishOnceV1(replica,undefined,[path]);
  rmSync(join(a.worktree,path));
  await publishOnceV1(replica,undefined,[path]);
  const events=(await readEventsV1(store,a.originId,head??undefined)).events;
  const proof=database.query("select value from json_each(?1) order by key").all(JSON.stringify(durableFixture.committed)).map(row=>(row as {value:string}).value);
  expect(events.map(row=>row.operation)).toEqual(proof);
  expect(beats).toHaveLength(proof.length);
  expect(beats[0]!).toBeGreaterThan(head!.bytes);
  expect(beats[1]!).toBe((await readHeadV1(store,a.originId))!.bytes);
 }finally{faults=[];await closeReplicaV1(replica);database.close();}
},20000);

test("a directory hint remains pending for an incoming descendant missing from disk",async()=>{
 const {a,b,clock}=await pair(),first=await openReplicaV1(a),second=await openReplicaV1(b),database=new Database(":memory:");
 const {blocked,published,directory}=durableFixture;
 const confirm=(paths:readonly string[])=>publishOnceV1(second,undefined,paths,undefined,undefined,undefined,undefined,true);
 try{
  put(a,published.path,published.text);
  await publishOnceV1(first,undefined,[published.path]);
  expect((await followOnceV1(second)).applied).toBe(1);
  put(a,blocked.path,blocked.text);
  await publishOnceV1(first,undefined,[blocked.path]);
  faults=[{op:"rename",match:join(b.worktree,blocked.path),code:"EBUSY",left:1}];
  expect((await followOnceV1(second)).queued).toBe(1);
  expect(existsSync(join(b.worktree,blocked.path))).toBe(false);
  const requested=[directory,blocked.path,durableFixture.unchanged];
  const expected=database.query("select value path from json_each(?1) where value=?2 or substr(?2,1,length(value)+1)=value||'/' order by value").all(JSON.stringify(requested),blocked.path).map(row=>(row as {path:string}).path);
  expect([...(new Set((await confirm(requested)).retry))].sort()).toEqual(expected);
  clock.ms+=60_000;
  expect((await followOnceV1(second)).queued).toBe(0);
  expect((await confirm(requested)).retry).toEqual([]);
 }finally{faults=[];await closeReplicaV1(first);await closeReplicaV1(second);database.close();}
},20000);

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
test("a cooling incoming queue reuses its clash pass and decodes only materialisable entries",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Original ticket or process-test artifact root required");
 const root=mkdtempSync(join(artifacts,"qc-"));scratch.push(root);
 const storeRoot=join(root,"s"),clock={ms:Date.now()},a={worktree:join(root,"a"),storeRoot,originId:"a",groupId:"g1",now:()=>clock.ms},b={...a,originId:"b",worktree:join(root,"b")};
 mkdirSync(a.worktree);
 put(a,"seed.txt","seed");
 await seedV1(a);await joinV1(b);
 const first=await openReplicaV1(a),second=await openReplicaV1(b),oracle=new Database(":memory:");
 const sample=sharingQueue.settledClash,path=sample.path;
 let queries=0,decodes=0;
 const find=second.folds.find.bind(second.folds),entryAt=second.manifest.entryAt.bind(second.manifest);
 second.manifest.entryAt=(id)=>{if(second.book.name(id)===path)decodes++;return entryAt(id);};
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
   queries=0;decodes=0;
   expect(await followOnceV1(second)).toMatchObject({events:0,applied:0,queued:state.queued});
   expect(queries).toBe(state.reads);expect(decodes).toBe(state.reads);
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

test("ordinary file sweeps leave incoming paths to the retry that owns them",()=>{
 const source=ts.createSourceFile("replica.ts",readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
 const declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="hotChanged");
 if(declaration===undefined)throw Error("Actual replica file sweep required");
 const script=ts.transpileModule(declaration.getText(source),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const rows=JSON.parse(readFileSync(new URL("./🧫️fixtures/🧹️sweep-queue.json",import.meta.url),"utf8"));
 const db=new Database(":memory:"),workspace=mkdtempSync(join(tmpdir(),"sweep-queue-"));
 let probes=0;
 const changed=new Function("lstatSync","absolute","gone","linkless","sharedV1",script+"\nreturn hotChanged;")(
  (path:string)=>{probes++;return lstatSync(path);},join,(error:NodeJS.ErrnoException)=>error.code==="ENOENT",()=>false,()=>true);
 try{
  for(const [index,row] of rows.entries()){
   const path="path-"+index+".txt",target=join(workspace,path);
   if(row.present)writeFileSync(target,"physical value");
   const stamp=row.present?lstatSync(target):undefined,id=row.known?0:undefined,owner={settings:{worktree:workspace,modeless:true},book:{find:()=>id},
    queue:{has:()=>row.queued,hasId:()=>row.queued},files:{live:()=>row.known,operationAt:()=>"put",bytes:[(stamp?.size??0)+(row.settled?0:1)],mtime:[stamp?.mtimeMs],racyAt:()=>false}};
   const expected=db.query<{probes:number;changed:number},[number,number,number,number]>(
    "select (not ?2) as probes, (not ?2 and case when ?1 then (not ?3 or not ?4) else ?3 end) as changed"
   ).get(Number(row.known),Number(row.queued),Number(row.present),Number(row.settled))!;
   const before=probes;
   expect(changed(owner,path,Date.now(),target),row.name).toBe(Boolean(expected.changed));
   expect(probes-before,row.name).toBe(expected.probes);
  }
 }finally{db.close();rmSync(workspace,{recursive:true,force:true});}
});
