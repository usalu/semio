import {afterAll,expect,mock,test} from "bun:test";
import * as real from "node:fs/promises";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {appendEventsV1,formatHlcV1,readHeadV1} from "../../📒️journal/🟦️.ts";
import type {ReplicaEventV1} from "../🟦️.ts";

type Fault={op:string;match:string;code:string;left:number};
let faults:Fault[]=[],hook:((op:string,path:string)=>void|Promise<void>)|undefined;
const originals={...real};
const wrap=(op:string)=>async(...args:unknown[])=>{
 const path=args.map(String).join("\0"),fault=faults.find(item=>item.op===op&&item.left>0&&path.includes(item.match));
 if(fault){fault.left--;throw Object.assign(Error(`${fault.code}: injected, ${op}`),{code:fault.code});}
 await hook?.(op,path);
 return (originals as unknown as Record<string,(...inner:unknown[])=>unknown>)[op]!(...args);
};
mock.module("node:fs/promises",()=>({...originals,...Object.fromEntries(["rename","unlink","copyFile","symlink","stat","lstat"].map(op=>[op,wrap(op)]))}));
const {closeReplicaV1,compactV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,seedV1,verifyV1}=await import("../🟦️.ts");

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`replica-race-${label}-`));scratch.push(path);return path;};
afterAll(()=>{faults=[];hook=undefined;for(const path of scratch)rmSync(path,{recursive:true,force:true});});

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
  expect(events.filter(event=>event.event==="blocked"&&event.path==="held.txt")).toEqual([{version:1,event:"blocked",path:"held.txt",reason:"sharing-violation"}]);
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

test("an apply whose rename onto a held file fails keeps the old content, leaves no staging file behind and succeeds after the release",async()=>{
 const {a,b,clock,events}=await pair();
 put(a,"doc.txt","one");
 const first=await openReplicaV1(a);await publishOnceV1(first);
 const second=await openReplicaV1(b,event=>events.push(event));await followOnceV1(second);
 put(a,"doc.txt","two, a bit longer");await publishOnceV1(first);
 faults=[{op:"rename",match:"doc.txt",code:"EPERM",left:1000}];
 expect(await followOnceV1(second)).toMatchObject({applied:0,queued:1});
 expect(readFileSync(join(b.worktree,"doc.txt"),"utf8")).toBe("one");
 expect(readdirSync(join(b.worktree,".cache","f","tmp"))).toEqual([]);
 expect(events.some(event=>event.event==="blocked"&&event.path==="doc.txt")).toBe(true);
 faults=[];clock.ms+=60_000;
 expect(await followOnceV1(second)).toMatchObject({applied:1,queued:0});
 expect(readFileSync(join(b.worktree,"doc.txt"),"utf8")).toBe("two, a bit longer");
 await closeReplicaV1(first);await closeReplicaV1(second);
},60000);

test("a write into the staging file that fails with EACCES is reported as blocked, not as an error, and is retried",async()=>{
 const {a,b,clock,events}=await pair();
 put(a,"new.txt","fresh");
 const first=await openReplicaV1(a);await publishOnceV1(first);
 const second=await openReplicaV1(b,event=>events.push(event));
 faults=[{op:"copyFile",match:`${join(".cache","f","tmp")}`,code:"EACCES",left:1000}];
 expect(await followOnceV1(second)).toMatchObject({applied:0,queued:1});
 expect(events.filter(event=>event.event==="blocked"&&event.path==="new.txt").map(event=>event.reason)).toEqual(["EACCES"]);
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
 faults=[{op:"stat",match:"busy.txt",code:"EBUSY",left:1000}];
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
 expect(result).toMatchObject({dropped:1,queued:0});
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
