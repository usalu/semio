import {afterAll,expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {readEventsV1,readHeadV1,listOriginsV1,appendEventsV1,formatHlcV1,type FleetEventV1} from "../../📒️journal/🟦️.ts";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {gitSharedV1} from "../../🚧️boundary/🟦️.ts";
import {gitStateV1,gitViewV1,INDEX_BASE_V1,INDEX_PREFIX_V1,indexRealmV1,rebaseRequestedV1,requestRebaseV1} from "../../🌿️git/🟦️.ts";
import {fleetReportV1,readViewLinesV1} from "../../🩺️attest/🟦️.ts";
import {runReplicaCommandV1} from "../⌨️command/🟦️.ts";
import {closeReplicaV1,followOnceV1,joinV1,repairRequestedV1,repairV1,requestRepairV1,openReplicaV1,publishOnceV1,runV1,seedV1,statusV1,verifyV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../🟦️.ts";

type Machine={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];replica?:ReplicaV1};
const scratch:string[]=[],fast={pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`replica-git-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const environment={...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.test",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.test",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"};
function cli(who:Machine,args:string[],input?:string,extra:Record<string,string>={}):string{
 const result=Bun.spawnSync(["git",...args],{cwd:who.options.worktree,env:{...environment,...extra},stdin:input===undefined?undefined:Buffer.from(input),stdout:"pipe",stderr:"pipe"});
 if(result.exitCode!==0)throw Error(`git ${args.join(" ")}: ${result.stderr.toString()}`);
 return result.stdout.toString();
}
const tree=(who:Machine):string=>cli(who,["write-tree"]).trim();
const staged=(who:Machine):string[]=>cli(who,["diff","--cached","--name-only"]).split("\n").filter(Boolean);
const at=(who:Machine,path:string)=>join(who.options.worktree,...path.split("/"));
function write(who:Machine,path:string,text:string):void{const file=at(who,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);}
function machine(name:string,store:string):Machine{return {name,events:[],options:{worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",...fast}};}
async function open(who:Machine):Promise<void>{who.replica=await openReplicaV1(who.options,event=>who.events.push(event));}
async function shut(who:Machine):Promise<void>{if(who.replica){await closeReplicaV1(who.replica);who.replica=undefined;}}
async function converge(machines:readonly Machine[],done:()=>boolean,rounds:number=400):Promise<void>{
 for(let round=0;round<rounds;round++){
  for(const who of machines)await publishOnceV1(who.replica!);
  for(const who of machines)await followOnceV1(who.replica!);
  if(done())return;
  await Bun.sleep(15);
 }
 throw Error("The machines did not converge");
}
const same=(machines:readonly Machine[],read:(who:Machine)=>unknown):boolean=>machines.every(who=>JSON.stringify(read(who))===JSON.stringify(read(machines[0]!)));

async function fleet(names:readonly string[],git:boolean=true){
 const store=temp("store"),machines=names.map(name=>machine(name,store)),[first]=machines as [Machine,...Machine[]];
 write(first,".gitignore",fixture.gitignore);write(first,"a.txt","alpha\n");write(first,"b.txt","bravo\n");write(first,"dir/c.txt","charlie\n");
 if(git){cli(first,["init","-q","-b","main"]);cli(first,["add","-A"]);cli(first,["commit","-q","-m","initial"]);}
 await seedV1(first.options);
 for(const other of machines.slice(1))await joinV1(other.options);
 for(const who of machines)await open(who);
 return {store,machines};
}
async function journalPaths(store:string):Promise<string[]>{
 const paths:string[]=[];
 for(const origin of await listOriginsV1(store))for(const event of (await readEventsV1(store,origin,undefined)).events)paths.push(event.path);
 return paths;
}

posixTest("the group server publishes the base once, workers join onto it and have the same stage and a clean status",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01","g1-m02"]);
 try{
  const paths=await journalPaths(store);
  expect(paths.filter(path=>path===INDEX_BASE_V1)).toHaveLength(1);
  expect(same(machines,tree)).toBe(true);
  for(const who of machines){
   expect(cli(who,["status","--porcelain"])).toBe("");
   expect(cli(who,["rev-parse","HEAD"]).trim()).toBe(cli(machines[0]!,["rev-parse","HEAD"]).trim());
   expect((await gitStateV1(who.options.worktree)).indexTree).toBe(tree(who));
  }
  expect(readFileSync(at(machines[1]!,".git/config"),"utf8")).toContain("filemode");
  expect(await verifyV1(machines[1]!.options)).toMatchObject({ok:true});
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("a stage replicates, an unstage replicates, concurrent stages converge, the later hlc wins on one path, a commit replicates without a realm event, a status publishes nothing, and nothing private leaves the machine",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine],original=tree(a);
 try{
  write(a,"new.txt","n\n");cli(a,["add","new.txt"]);
  await converge(machines,()=>same(machines,tree)&&tree(b)!==original);
  expect(staged(b)).toEqual(["new.txt"]);expect(staged(c)).toEqual(["new.txt"]);expect(same(machines,staged)).toBe(true);
  cli(a,["reset","-q","--","new.txt"]);
  await converge(machines,()=>same(machines,tree)&&tree(c)===original);
  expect(staged(b)).toEqual([]);expect(staged(c)).toEqual([]);

  write(a,"x.txt","x\n");write(b,"y.txt","y\n");cli(a,["add","x.txt"]);cli(b,["add","y.txt"]);
  await converge(machines,()=>same(machines,tree)&&staged(c).length===2);
  expect(staged(a)).toEqual(["x.txt","y.txt"]);expect(staged(c)).toEqual(["x.txt","y.txt"]);

  const blobA=cli(a,["hash-object","-w","--stdin"],"from a\n").trim(),blobB=cli(b,["hash-object","-w","--stdin"],"from b\n").trim();
  cli(a,["update-index","--add","--cacheinfo",`100644,${blobA},s.txt`]);await publishOnceV1(a.replica!);
  await Bun.sleep(5);
  cli(b,["update-index","--add","--cacheinfo",`100644,${blobB},s.txt`]);await publishOnceV1(b.replica!);
  await converge(machines,()=>same(machines,tree)&&cli(c,["ls-files","-s","s.txt"]).includes(blobB));
  for(const who of machines)expect(cli(who,["ls-files","-s","s.txt"])).toContain(blobB);

  const before=(await journalPaths(store)).filter(path=>path.startsWith(INDEX_PREFIX_V1)).length;
  cli(a,["commit","-q","-m","second"]);
  await converge(machines,()=>same(machines,(who)=>cli(who,["rev-parse","HEAD"])));
  for(const who of machines){expect(cli(who,["diff","--cached","--name-only","HEAD"])).toBe("");expect(cli(who,["log","-1","--format=%s"]).trim()).toBe("second");}
  expect((await journalPaths(store)).filter(path=>path.startsWith(INDEX_PREFIX_V1)).length).toBe(before);
  expect(same(machines,tree)).toBe(true);

  const head=(await readHeadV1(store,"g1-m01"))!.events;
  utimesSync(at(b,"a.txt"),1_700_000_000,1_700_000_000);
  cli(b,["status","--porcelain"]);
  await publishOnceV1(b.replica!);await followOnceV1(a.replica!);
  expect((await readHeadV1(store,"g1-m01"))!.events).toBe(head);
  await converge(machines,()=>true);await converge(machines,()=>true);
  expect(await verifyV1(b.options)).toMatchObject({ok:true});

  for(const path of await journalPaths(store))if(path.startsWith(".git/"))expect(gitSharedV1(path),path).toBe(true);
  for(const who of machines)expect(who.events.filter(event=>event.event==="error")).toEqual([]);
  cli(b,["update-index","--force-remove","a.txt"]);
  const drift=(await verifyV1(b.options)).drift.filter(item=>item.path===".git/index");
  expect(drift).toHaveLength(1);
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("a rebase request publishes the stage of that machine as a new base, every machine adopts it, and the request is consumed",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[server]=machines as [Machine,Machine,Machine];
 try{
  write(server,"a.txt","alpha two\n");cli(server,["add","a.txt"]);
  await converge(machines,()=>same(machines,tree));
  await requestRebaseV1(server.options.worktree);
  expect(await rebaseRequestedV1(server.options.worktree)).toBe(true);
  await converge(machines,()=>!existsSync(join(server.options.worktree,".cache","f","index-rebase"))&&same(machines,tree));
  await converge(machines,()=>same(machines,tree),3);
  expect((await journalPaths(store)).filter(path=>path===INDEX_BASE_V1)).toHaveLength(2);
  for(const who of machines)expect(staged(who)).toEqual(["a.txt"]);
 }finally{for(const who of machines)await shut(who);}
});

posixTest("a repair re-creates the shared git files a machine holds as applied but does not have on disk, and consumes the request",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01"]),[server,worker]=machines as [Machine,Machine];
 try{
  await converge(machines,()=>same(machines,tree));
  rmSync(at(worker,".git/HEAD"));rmSync(at(worker,".git/refs"),{recursive:true});
  await requestRepairV1(worker.options.worktree);
  expect(await repairRequestedV1(worker.options.worktree)).toBe(true);
  expect(await repairV1(worker.replica!)).toBeGreaterThan(0);
  expect(await repairRequestedV1(worker.options.worktree)).toBe(false);
  await followOnceV1(worker.replica!);
  expect(readFileSync(at(worker,".git/HEAD"),"utf8")).toBe(readFileSync(at(server,".git/HEAD"),"utf8"));
  expect(cli(worker,["rev-parse","HEAD"]).trim()).toBe(cli(server,["rev-parse","HEAD"]).trim());
  expect(await repairV1(worker.replica!)).toBe(0);
  rmSync(at(worker,"a.txt"));
  worker.replica!.manifest.delete("a.txt");worker.replica!.files.delete("a.txt");
  expect(await repairV1(worker.replica!)).toBe(1);
  await followOnceV1(worker.replica!);
  expect(readFileSync(at(worker,"a.txt"),"utf8")).toBe(readFileSync(at(server,"a.txt"),"utf8"));
 }finally{for(const who of machines)await shut(who);}
});

posixTest("a base is published only for the first stage and for an explicit rebase: a stage of more than 5000 paths becomes entries and voids nothing a machine staged earlier, and a restarted replica keeps following",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine];
 try{
  write(b,"early1.txt","1\n");write(b,"early2.txt","2\n");write(b,"early3.txt","3\n");cli(b,["add","early1.txt","early2.txt","early3.txt"]);
  await publishOnceV1(b.replica!);
  const blob=cli(a,["hash-object","-w","--stdin"],"bulk\n").trim();
  cli(a,["update-index","--index-info"],Array.from({length:5001},(_,index)=>`100644 ${blob}\tbulk/f${index}.txt\n`).join(""));
  await publishOnceV1(a.replica!);
  const events=(await journalPaths(store));
  expect(events.filter(path=>path===INDEX_BASE_V1)).toHaveLength(1);
  expect(events.filter(path=>path.startsWith(`${INDEX_PREFIX_V1}bulk/`))).toHaveLength(5001);
  await converge(machines,()=>same(machines,tree)&&staged(c).length===5004);
  for(const who of machines)expect(staged(who)).toEqual(expect.arrayContaining(["early1.txt","early2.txt","early3.txt"]));
  for(const who of machines)expect(who.events.filter(event=>event.event==="overwritten"||event.event==="error")).toEqual([]);
  await shut(b);await open(b);
  write(a,"after.txt","z\n");cli(a,["add","after.txt"]);
  await converge(machines,()=>same(machines,tree)&&staged(b).length===5005);
  await converge(machines,()=>true);await converge(machines,()=>true);
  expect(await verifyV1(c.options)).toMatchObject({ok:true});
 }finally{for(const who of machines)await shut(who);}
},180_000);

posixTest("a lost local record does not matter: a stage made while the replica was stopped is published after the restart, a deleted .git/index is restored from the registers, and no delete is published for it",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine];
 try{
  await converge(machines,()=>same(machines,tree));
  await shut(b);
  write(b,"offline.txt","o\n");cli(b,["add","offline.txt"]);
  rmSync(at(b,".cache/f/git-index.json"),{force:true});
  await open(b);
  await converge(machines,()=>same(machines,tree)&&staged(c).includes("offline.txt"));
  expect(staged(a)).toEqual(["offline.txt"]);
  const before=(await journalPaths(store)).filter(indexRealmV1).length;
  rmSync(at(b,".git/index"));
  await publishOnceV1(b.replica!);
  await converge(machines,()=>same(machines,tree));
  expect(staged(b)).toEqual(["offline.txt"]);
  expect((await journalPaths(store)).filter(indexRealmV1)).toHaveLength(before);
  expect(b.events.some(event=>event.event==="repaired"&&event.stage==="git"&&event.reason==="index-missing")).toBe(true);
  for(const who of machines)expect(who.events.filter(event=>event.event==="error")).toEqual([]);
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("a rebase lists the registers of other machines it voids in an event and a file under lost, and every machine adopts the new base",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[server,worker] = machines as [Machine,Machine,Machine];
 try{
  write(worker,"y.txt","y\n");cli(worker,["add","y.txt"]);
  await converge(machines,()=>same(machines,tree)&&staged(server).includes("y.txt"));
  const blob=cli(worker,["rev-parse",":y.txt"]).trim();
  cli(server,["reset","-q","--","y.txt"]);
  await requestRebaseV1(server.options.worktree);
  await publishOnceV1(server.replica!);
  const event=server.events.find(item=>item.event==="published"&&item.reason==="rebase");
  expect(event).toMatchObject({stage:"git",path:INDEX_BASE_V1,count:1});
  expect(readFileSync(join(server.options.worktree,".cache","f",String(event!.lost)),"utf8")).toBe(`y.txt\tg1-m01\t100644 ${blob}\n`);
  await converge(machines,()=>same(machines,tree)&&staged(worker).length===0);
  for(const who of machines)expect(staged(who)).toEqual([]);
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("unmerged stage entries make status name the reason, nothing is published or reset meanwhile, and sharing resumes once they are resolved",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01"]),[a,b]=machines as [Machine,Machine];
 try{
  await converge(machines,()=>same(machines,tree));
  const blob=cli(b,["hash-object","-w","--stdin"],"u\n").trim(),before=tree(a);
  cli(b,["update-index","--index-info"],`100644 ${blob} 1\tu.txt\n100644 ${blob} 2\tu.txt\n100644 ${blob} 3\tu.txt\n`);
  await publishOnceV1(b.replica!);
  expect((await statusV1(b.options)).git).toMatchObject({shared:false,reason:"unmerged"});
  expect(b.events.filter(event=>event.event==="blocked"&&event.reason==="unmerged")).toHaveLength(1);
  await publishOnceV1(b.replica!);
  expect(b.events.filter(event=>event.event==="blocked"&&event.reason==="unmerged")).toHaveLength(1);
  expect(cli(b,["ls-files","-u"])).not.toBe("");
  expect(tree(a)).toBe(before);
  cli(b,["update-index","--add","--cacheinfo",`100644,${blob},u.txt`]);
  await converge(machines,()=>same(machines,tree));
  expect((await statusV1(b.options)).git).toMatchObject({shared:true,reason:"shared"});
  expect(staged(a)).toEqual(["u.txt"]);
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("workers publish no base, a replica without git keeps syncing files, and status says whether git state is shared",async()=>{
 const {store,machines}=await fleet(["w1","w2"]);
 try{
  expect((await journalPaths(store)).some(indexRealmV1)).toBe(false);
  write(machines[0]!,"f.txt","file\n");
  await converge(machines,()=>existsSync(at(machines[1]!,"f.txt")));
  expect((await journalPaths(store)).some(indexRealmV1)).toBe(false);
 }finally{for(const who of machines)await shut(who);}
 const bare=await fleet(["g1-server","g1-m01"],false);
 try{
  write(bare.machines[0]!,"f.txt","file\n");
  await converge(bare.machines,()=>existsSync(at(bare.machines[1]!,"f.txt")));
  for(const who of bare.machines)expect(who.events.filter(event=>event.event==="error")).toEqual([]);
  expect((await journalPaths(bare.store)).some(indexRealmV1)).toBe(false);
  expect((await statusV1(bare.machines[1]!.options)).git).toMatchObject({shared:false,reason:"no-repository"});
 }finally{for(const who of bare.machines)await shut(who);}
},120_000);

posixTest("an old state that tracks .git/index, the reflog and the config forgets them without publishing a delete, and incoming events for private paths are dropped",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01"]),[a,b]=machines as [Machine,Machine];
 try{
  const replica=a.replica!,legacy=[".git/index",".git/logs/HEAD",".git/config",".git/hooks/pre-commit"];
  write(a,".git/logs/HEAD","reflog\n");write(a,".git/hooks/pre-commit","#!/bin/sh\n");
  rmSync(at(a,".git/logs/HEAD"));
  for(const path of legacy)replica.files.set(path,{hlc:formatHlcV1({ms:1,counter:0},"g1-server"),operation:"put",sha256:"0".repeat(64),bytes:1,executable:false,mtimeMs:0});
  const countBefore=(await readHeadV1(store,"g1-server"))!.events;
  await publishOnceV1(replica);
  expect((await readHeadV1(store,"g1-server"))!.events).toBe(countBefore);
  for(const path of legacy)expect(replica.files.has(path)).toBe(false);
  expect(existsSync(at(a,".git/index"))).toBe(true);
  const index=readFileSync(at(b,".git/index"));
  const incoming:FleetEventV1[]=[".git/index",".git/config",".git/logs/HEAD"].flatMap((path,number)=>[{version:1,hlc:formatHlcV1({ms:Date.now()+number,counter:0},"g1-m02"),operation:"delete" as const,path,base:null},{version:1,hlc:formatHlcV1({ms:Date.now()+number+10,counter:1},"g1-m02"),operation:"put" as const,path,sha256:"1".repeat(64),bytes:1,executable:false,base:null}]);
  await appendEventsV1(store,"g1-m02",incoming);
  await followOnceV1(b.replica!);await followOnceV1(b.replica!);
  expect(readFileSync(at(b,".git/index")).equals(index)).toBe(true);
  expect(existsSync(at(b,".git/logs/HEAD"))).toBe(false);
  expect(b.replica!.queue.size).toBe(0);
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("an entry whose object has not arrived stays queued without an error and is applied once the object exists",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01"]),[, b] = machines as [Machine,Machine];
 try{
  const blob=Bun.spawnSync(["git","hash-object","--stdin"],{cwd:b.options.worktree,env:environment,stdin:Buffer.from("late\n"),stdout:"pipe"}).stdout.toString().trim();
  await appendEventsV1(store,"g1-m09",[{version:1,hlc:formatHlcV1({ms:Date.now()+50,counter:0},"g1-m09"),operation:"link",path:`${INDEX_PREFIX_V1}late.txt`,target:`100644 ${blob}`,base:null}]);
  await followOnceV1(b.replica!);
  expect(b.replica!.queue.size).toBe(1);
  expect(cli(b,["ls-files","-s","late.txt"])).toBe("");
  expect(b.events.filter(event=>event.event==="error")).toEqual([]);
  cli(b,["hash-object","-w","--stdin"],"late\n");
  await Bun.sleep(1100);
  await followOnceV1(b.replica!);
  expect(b.replica!.queue.size).toBe(0);
  expect(cli(b,["ls-files","-s","late.txt"])).toContain(blob);
  expect(await statusV1(b.options)).toMatchObject({git:{shared:true,reason:"shared"}});
 }finally{for(const who of machines)await shut(who);}
},120_000);

const python=testPythonV1(),oracle=join(import.meta.dirname,"🌿️git.py"),head=(who:Machine):string=>cli(who,["rev-parse","HEAD"]).trim();
const subjects=(who:Machine):string[]=>cli(who,["log","--format=%s","HEAD"]).split("\n").filter(Boolean);
const linearised=(machines:readonly Machine[]):ReplicaEventV1[]=>machines.flatMap(who=>who.events.filter(event=>event.event==="linearised"));
const clean=(who:Machine):boolean=>cli(who,["status","--porcelain"])==="";
function commit(who:Machine,file:string,message:string,extra:Record<string,string>={}):void{write(who,file,`${message}\n`);cli(who,["add",file]);cli(who,["commit","-q","-m",message],undefined,extra);}
function check(who:Machine,count:number,messages:readonly string[]):void{
 const ran=python===null?undefined:Bun.spawnSync([python,"-I",oracle,who.options.worktree,String(count),...messages],{stdout:"pipe",stderr:"pipe"});
 if(ran!==undefined)expect(ran.exitCode,ran.stderr.toString()).toBe(0);
 const rows=cli(who,["rev-list","--parents","HEAD"]).split("\n").filter(Boolean).map(row=>row.split(" "));
 expect(rows).toHaveLength(count);
 expect(rows.every(row=>row.length<=2)).toBe(true);
 for(const message of messages)expect(subjects(who).filter(subject=>subject===message)).toHaveLength(1);
}
const settled=(machines:readonly Machine[],count:number)=>()=>same(machines,head)&&same(machines,tree)&&machines.every(who=>subjects(who).length===count&&clean(who));

posixTest("two concurrent commits from one parent end as one linear history with both messages, no lost change, a clean stage and one linearised event",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine];
 try{
  commit(a,"a1.txt","from a");commit(b,"b1.txt","from b");
  const forks=[head(a),head(b)];
  expect(forks[0]).not.toBe(forks[1]);
  await converge(machines,settled(machines,3));
  await converge(machines,()=>true);await converge(machines,()=>true);
  expect(same(machines,head)).toBe(true);
  for(const who of machines){
   check(who,3,["initial","from a","from b"]);
   expect(cli(who,["show","HEAD:a1.txt"])).toBe("from a\n");expect(cli(who,["show","HEAD:b1.txt"])).toBe("from b\n");
   expect(cli(who,["cat-file","-t",forks[0]!]).trim()).toBe("commit");expect(cli(who,["cat-file","-t",forks[1]!]).trim()).toBe("commit");
   expect(who.events.filter(event=>event.event==="error")).toEqual([]);
  }
  const events=linearised(machines);
  expect(events).toHaveLength(1);
  expect(forks).toContain(events[0]!.loser);expect(forks).toContain(events[0]!.winner);
  expect(events[0]!.result).toBe(head(c));
  expect(cli(a,["rev-parse","HEAD~1"]).trim()).toBe(events[0]!.winner);
  expect(c.events.filter(event=>event.event==="linearised")).toEqual([]);
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("identical concurrent commits leave one commit and the same HEAD everywhere",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b]=machines as [Machine,Machine,Machine];
 try{
  commit(a,"same.txt","twin",{GIT_AUTHOR_DATE:"2001-01-01T00:00:00Z",GIT_COMMITTER_DATE:"2001-01-01T00:00:00Z"});
  commit(b,"same.txt","twin",{GIT_AUTHOR_DATE:"2002-02-02T00:00:00Z",GIT_COMMITTER_DATE:"2002-02-02T00:00:00Z"});
  expect(head(a)).not.toBe(head(b));
  await converge(machines,settled(machines,2));
  await converge(machines,()=>true);await converge(machines,()=>true);
  for(const who of machines)check(who,2,["initial","twin"]);
  const events=linearised(machines);
  expect(events).toHaveLength(1);
  expect(events[0]).toMatchObject({reason:"duplicate"});
  expect(events[0]!.result).toBe(events[0]!.winner);
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("three concurrent commits converge to one linear history with all messages",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine];
 try{
  commit(a,"a1.txt","from a");commit(b,"b1.txt","from b");commit(c,"c1.txt","from c");
  await converge(machines,settled(machines,4),800);
  await converge(machines,()=>true);await converge(machines,()=>true);
  expect(same(machines,head)).toBe(true);
  for(const who of machines){
   check(who,4,["initial","from a","from b","from c"]);
   for(const file of ["a1.txt","b1.txt","c1.txt"])expect(cli(who,["cat-file","-t",`HEAD:${file}`]).trim()).toBe("blob");
   expect(who.events.filter(event=>event.event==="error")).toEqual([]);
  }
  expect(linearised(machines).length).toBeGreaterThanOrEqual(2);
 }finally{for(const who of machines)await shut(who);}
},180_000);

posixTest("a commit on one machine while another is merely behind is a plain fast-forward without a rewrite",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine],parent=head(a);
 try{
  commit(a,"a1.txt","only a");
  await converge(machines,settled(machines,2));
  for(const who of machines)expect(cli(who,["rev-parse","HEAD~1"]).trim()).toBe(parent);
  commit(b,"b1.txt","then b");
  await converge(machines,settled(machines,3));
  await converge(machines,()=>true);
  for(const who of machines)check(who,3,["initial","only a","then b"]);
  expect(linearised(machines)).toEqual([]);
  expect(head(c)).toBe(head(b));
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("three real worktrees attest the same view, a local-only refused symlink makes the verdict name view.digest and fleet --diff list it, and removing it restores agreement",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01","g1-m02"]),controller=new AbortController();
 for(const who of machines)await shut(who);
 const options=(who:Machine):ReplicaOptionsV1=>({...who.options,attestMilliseconds:100,viewMilliseconds:300,viewDebounceMilliseconds:50}),runs=machines.map(who=>runV1(options(who),controller.signal,event=>who.events.push(event)));
 const until=async(condition:()=>Promise<boolean>):Promise<void>=>{const end=Date.now()+60_000;while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(50);}};
 const diff=async(who:Machine)=>{
  const notices:string[]=[];
  const result=await runReplicaCommandV1(["fleet","--diff","--worktree",who.options.worktree,"--store",store,"--lang","en"],{root:who.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>notices.push(line)});
  return {exitCode:result.exitCode,text:notices.join("\n")};
 };
 try{
  await until(async()=>{const report=await fleetReportV1(store);return report.ok&&report.rows.length===3&&report.rows.every(row=>row.view!==null);});
  const [server,worker,other]=machines as [Machine,Machine,Machine];
  const clean=(await fleetReportV1(store)).rows[0]!.view!;
  expect(clean).toMatchObject({staged:0,modified:0,deleted:0,untracked:0});
  expect((await diff(server)).exitCode).toBe(0);
  symlinkSync("/etc/hosts",at(worker,"local-link"));
  await until(async()=>(await fleetReportV1(store)).differing.length>0);
  const report=await fleetReportV1(store);
  expect(report.ok).toBe(false);
  expect(report.differing.map(item=>[item.originId,item.field])).toEqual([["g1-m01","view.digest"]]);
  expect(report.rows.find(row=>row.originId==="g1-m01")!.view).toMatchObject({untracked:1});
  const there=await diff(other);
  expect(there.exitCode).toBe(1);
  expect(there.text).toContain("g1-m01 differs from this machine: 1 lines only there, 0 lines only here");
  expect(there.text).toContain("only on g1-m01: ?? local-link");
  expect(there.text).not.toContain("g1-server differs");
  const here=await diff(worker);
  expect(here.exitCode).toBe(1);
  expect(here.text).toContain("only on g1-m01 (here): ?? local-link");
  expect(await readViewLinesV1(store,"g1-m01")).toEqual(["?? local-link"]);
  rmSync(at(worker,"local-link"));
  await until(async()=>(await fleetReportV1(store)).ok);
  expect((await diff(server)).exitCode).toBe(0);
  expect(await readViewLinesV1(store,"g1-m01")).toEqual([]);
 }finally{controller.abort();await Promise.all(runs);}
},120_000);

posixTest("untracked files created and deleted on one machine reach every attested view within seconds, without a complete status pass and without a diverged verdict in between",async()=>{
 const {store,machines}=await fleet(["g1-server","g1-m01","g1-m02"]),controller=new AbortController();
 for(const who of machines)await shut(who);
 const options=(who:Machine):ReplicaOptionsV1=>({...who.options,attestMilliseconds:100,viewMilliseconds:3_600_000,viewDebounceMilliseconds:50,deltaDebounceMilliseconds:50}),runs=machines.map(who=>runV1(options(who),controller.signal,event=>who.events.push(event)));
 const verdicts:string[]=[];
 const until=async(condition:(report:Awaited<ReturnType<typeof fleetReportV1>>)=>boolean):Promise<number>=>{const began=Date.now();for(;;){const report=await fleetReportV1(store);for(const row of report.rows)verdicts.push(row.verdict);if(condition(report))return Date.now()-began;if(Date.now()-began>60_000)throw Error(`Timed out: ${JSON.stringify(report.rows.map(row=>[row.originId,row.verdict,row.view]))}`);await Bun.sleep(25);}};
 try{
  await until(report=>report.ok&&report.rows.length===3&&report.rows.every(row=>row.view!==null&&row.view.stale!==true));
  const [,worker]=machines as [Machine,Machine,Machine];
  verdicts.length=0;
  for(let n=0;n<40;n++){mkdirSync(at(worker,`burst/${n%4}`),{recursive:true});writeFileSync(at(worker,`burst/${n%4}/f${n}.txt`),`${n}\n`);}
  const created=await until(report=>report.ok&&report.rows.every(row=>row.view!==null&&row.view.untracked===40&&row.view.stale!==true));
  rmSync(at(worker,"burst"),{recursive:true,force:true});
  const removed=await until(report=>report.ok&&report.rows.every(row=>row.view!==null&&row.view.untracked===0&&row.view.stale!==true));
  expect(created).toBeLessThan(20_000);
  expect(removed).toBeLessThan(20_000);
  expect(verdicts).not.toContain("diverged");
  for(const who of machines)expect((await gitViewV1(who.options.worktree))!.lines).toEqual([]);
 }finally{controller.abort();await Promise.all(runs);}
},120_000);

posixTest("a stage, a new file and a commit replicate without any watcher: the stat signature of .git/index, the directory sweep and the hot set find them",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01"]),[a,b]=machines as [Machine,Machine];
 for(const who of machines)await shut(who);
 const controller=new AbortController(),runs=machines.map(who=>runV1({...who.options,watch:false,scanSeconds:3600,hintMilliseconds:20,hotMilliseconds:30,sweepMilliseconds:30},controller.signal,event=>who.events.push(event)));
 const until=async(condition:()=>boolean):Promise<void>=>{const end=Date.now()+30_000;while(!condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(25);}};
 try{
  await Bun.sleep(600);
  write(a,"by-hand.txt","h\n");cli(a,["add","by-hand.txt"]);
  await until(()=>staged(b).includes("by-hand.txt"));
  expect(existsSync(at(b,"by-hand.txt"))).toBe(true);
  expect(tree(a)).toBe(tree(b));
  cli(a,["commit","-q","-m","by hand"]);
  await until(()=>cli(b,["rev-parse","HEAD"]).trim()===cli(a,["rev-parse","HEAD"]).trim());
  expect(staged(b)).toEqual([]);
 }finally{controller.abort();await Promise.all(runs);}
},120_000);

posixTest("concurrent commits that change one file differently and others differently end linear with every change, the file taking the later commit, and one linearised event naming the conflict",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine];
 try{
  write(a,"a1.txt","a1\n");cli(a,["add","a1.txt"]);commit(a,"a.txt","from a",{GIT_AUTHOR_DATE:"2030-01-01T00:00:00Z",GIT_COMMITTER_DATE:"2030-01-01T00:00:00Z"});
  write(b,"b1.txt","b1\n");cli(b,["add","b1.txt"]);commit(b,"a.txt","from b",{GIT_AUTHOR_DATE:"2030-06-01T00:00:00Z",GIT_COMMITTER_DATE:"2030-06-01T00:00:00Z"});
  await converge(machines,settled(machines,3));
  await converge(machines,()=>true);await converge(machines,()=>true);
  for(const who of machines){
   check(who,3,["initial","from a","from b"]);
   for(const file of ["a1.txt","b1.txt"])expect(cli(who,["cat-file","-t",`HEAD:${file}`]).trim()).toBe("blob");
   expect(cli(who,["show","HEAD:a.txt"])).toBe("from b\n");
   expect(who.events.filter(event=>event.event==="error")).toEqual([]);
  }
  const events=linearised(machines);
  expect(events).toHaveLength(1);
  expect(events[0]).toMatchObject({count:1});
  expect(head(c)).toBe(head(a));
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("new branches, tags and a detached HEAD replicate as plain last writer wins, concurrent commits on a detached HEAD are linearised, and every repository stays valid",async()=>{
 const {machines}=await fleet(["g1-server","g1-m01","g1-m02"]),[a,b,c]=machines as [Machine,Machine,Machine];
 try{
  cli(a,["branch","feature"]);cli(a,["tag","light"]);cli(a,["tag","-a","annotated","-m","note"]);
  await converge(machines,()=>["feature","light","annotated"].every(name=>existsSync(at(c,name==="feature"?".git/refs/heads/feature":`.git/refs/tags/${name}`))));
  for(const who of [b,c]){
   expect(cli(who,["rev-parse","feature"]).trim()).toBe(cli(a,["rev-parse","feature"]).trim());
   expect(cli(who,["rev-parse","annotated^{commit}"]).trim()).toBe(cli(a,["rev-parse","HEAD"]).trim());
  }
  cli(a,["checkout","-q","--detach"]);
  await converge(machines,()=>readFileSync(at(c,".git/HEAD"),"utf8")===readFileSync(at(a,".git/HEAD"),"utf8"));
  expect(readFileSync(at(c,".git/HEAD"),"utf8")).toMatch(/^[0-9a-f]{40}\s*$/u);
  commit(a,"a1.txt","detached a");commit(b,"b1.txt","detached b");
  await converge(machines,settled(machines,3));
  await converge(machines,()=>true);await converge(machines,()=>true);
  for(const who of machines){
   check(who,3,["initial","detached a","detached b"]);
   expect(cli(who,["fsck","--no-dangling"])).toBeDefined();
   expect(who.events.filter(event=>event.event==="error")).toEqual([]);
  }
  expect(linearised(machines).length).toBeGreaterThanOrEqual(1);
 }finally{for(const who of machines)await shut(who);}
},180_000);

const executableOn=(who:Machine,path:string):boolean=>(statSync(at(who,path)).mode&0o111)!==0;
const manifestExecutable=(who:Machine,path:string):boolean|undefined=>{const entry=who.replica!.files.get(path);return entry?.operation==="put"?entry.executable:undefined;};

posixTest("a file made on a machine that cannot hold the executable bit, made executable on a POSIX machine and edited on the first one stays executable everywhere, a new file takes the staged mode, and every status is clean",async()=>{
 const store=temp("store"),server=machine("g1-server",store),worker=machine("g1-m01",store),windows={...machine("g1-m02",store)};
 write(server,".gitignore",fixture.gitignore);write(server,"a.txt","alpha\n");cli(server,["init","-q","-b","main"]);cli(server,["add","-A"]);cli(server,["commit","-q","-m","initial"]);
 await seedV1(server.options);await joinV1(worker.options);await joinV1(windows.options);
 windows.options={...windows.options,modeless:true};
 const machines=[server,worker,windows];
 for(const who of machines)await open(who);
 cli(windows,["config","core.filemode","false"]);
 const clean=async():Promise<void>=>{
  for(const who of machines){expect(cli(who,["diff","--name-only"]),who.name).toBe("");expect(cli(who,["ls-files","--others","--exclude-standard"]),who.name).toBe("");expect((await gitViewV1(who.options.worktree))!.lines,who.name).toEqual([]);}
 };
 try{
  write(windows,"tool.sh","#!/bin/sh\necho one\n");
  await converge(machines,()=>existsSync(at(server,"tool.sh"))&&existsSync(at(worker,"tool.sh")));
  expect([executableOn(server,"tool.sh"),executableOn(worker,"tool.sh"),manifestExecutable(windows,"tool.sh")]).toEqual([false,false,false]);
  cli(server,["add","tool.sh"]);
  await converge(machines,()=>same(machines,tree));
  chmodSync(at(server,"tool.sh"),0o755);cli(server,["add","tool.sh"]);
  await converge(machines,()=>executableOn(worker,"tool.sh")&&manifestExecutable(windows,"tool.sh")===true&&same(machines,tree));
  expect(cli(worker,["ls-files","--stage","tool.sh"])).toContain("100755");
  expect(executableOn(windows,"tool.sh")).toBe(false);
  await clean();
  write(windows,"tool.sh","#!/bin/sh\necho two\n");cli(windows,["add","tool.sh"]);
  await converge(machines,()=>readFileSync(at(server,"tool.sh"),"utf8").includes("two")&&readFileSync(at(worker,"tool.sh"),"utf8").includes("two")&&same(machines,tree));
  expect([executableOn(server,"tool.sh"),executableOn(worker,"tool.sh"),manifestExecutable(server,"tool.sh"),manifestExecutable(worker,"tool.sh"),manifestExecutable(windows,"tool.sh")]).toEqual([true,true,true,true,true]);
  expect(cli(windows,["ls-files","--stage","tool.sh"])).toContain("100755");
  await clean();
  write(windows,"new.sh","#!/bin/sh\n");cli(windows,["add","--chmod=+x","new.sh"]);
  await converge(machines,()=>existsSync(at(server,"new.sh"))&&existsSync(at(worker,"new.sh"))&&same(machines,tree));
  expect([executableOn(server,"new.sh"),executableOn(worker,"new.sh"),manifestExecutable(windows,"new.sh")]).toEqual([true,true,true]);
  await clean();
  chmodSync(at(worker,"tool.sh"),0o644);cli(worker,["add","tool.sh"]);
  await converge(machines,()=>!executableOn(server,"tool.sh")&&manifestExecutable(windows,"tool.sh")===false&&same(machines,tree));
  await clean();
 }finally{for(const who of machines)await shut(who);}
},120_000);

posixTest("an executable file rewritten on a POSIX machine by an editor that creates it anew without the bit keeps the recorded bit everywhere, a bit-only change replicates both ways, and a rewrite that sets the bit wins",async()=>{
 const store=temp("store"),server=machine("g1-server",store),worker=machine("g1-m01",store);
 write(server,".gitignore",fixture.gitignore);write(server,"a.txt","alpha\n");write(server,"run.sh","#!/bin/sh\necho one\n");chmodSync(at(server,"run.sh"),0o755);cli(server,["init","-q","-b","main"]);cli(server,["add","-A"]);cli(server,["commit","-q","-m","initial"]);
 await seedV1(server.options);await joinV1(worker.options);
 const machines=[server,worker];
 for(const who of machines)await open(who);
 const recreate=(who:Machine,path:string,text:string,mode:number):void=>{const temporary=at(who,path)+".new";writeFileSync(temporary,text);chmodSync(temporary,mode);renameSync(temporary,at(who,path));};
 try{
  await converge(machines,()=>existsSync(at(worker,"run.sh")));
  expect([executableOn(server,"run.sh"),executableOn(worker,"run.sh"),manifestExecutable(server,"run.sh"),manifestExecutable(worker,"run.sh")]).toEqual([true,true,true,true]);
  recreate(worker,"run.sh","#!/bin/sh\necho two\n",0o644);
  await converge(machines,()=>readFileSync(at(server,"run.sh"),"utf8").includes("two")&&manifestExecutable(server,"run.sh")===true&&manifestExecutable(worker,"run.sh")===true);
  expect([executableOn(server,"run.sh"),executableOn(worker,"run.sh")]).toEqual([true,true]);
  chmodSync(at(server,"run.sh"),0o644);
  await converge(machines,()=>!executableOn(worker,"run.sh")&&manifestExecutable(worker,"run.sh")===false);
  recreate(server,"run.sh","#!/bin/sh\necho three\n",0o644);
  await converge(machines,()=>readFileSync(at(worker,"run.sh"),"utf8").includes("three"));
  expect([executableOn(server,"run.sh"),executableOn(worker,"run.sh"),manifestExecutable(worker,"run.sh")]).toEqual([false,false,false]);
  recreate(worker,"run.sh","#!/bin/sh\necho four\n",0o755);
  await converge(machines,()=>readFileSync(at(server,"run.sh"),"utf8").includes("four")&&executableOn(server,"run.sh"));
  expect(manifestExecutable(server,"run.sh")).toBe(true);
 }finally{for(const who of machines)await shut(who);}
},120_000);
