import {afterAll,expect,spyOn,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import rescueCases from "./🧫️fixtures/🧷️pack-rescue.json" with {type:"json"};
import membership from "./🧫️fixtures/🧷️pack-membership.json" with {type:"json"};
import retirement from "./🧫️fixtures/🧷️pack-retirement.json" with {type:"json"};
import rescueSchema from "../🧷️packgate/🧬️schema/🔣️.json" with {type:"json"};
import {authorisePacksV1,packDirectoryV1,rescueLocalPacksV1} from "../🧷️packgate/🟦️.ts";
import * as packNames from "../🥾️bootstrap/🧺️packs/🟦️.ts";
import {formatHlcV1,listOriginsV1,readEventsV1} from "../../📒️journal/🟦️.ts";
import {consolidateGitV1} from "../🧱️consolidate/🟦️.ts";
import {applyQueue,checkpointReplicaV1,closeReplicaV1,enqueueId,followOnceV1,joinV1,openReplicaV1,publishOnceV1,runV1,seedV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../🟦️.ts";

type Machine={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];replica?:ReplicaV1};
const scratch:string[]=[],fast={pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0,gitCheckMilliseconds:50,rescueMilliseconds:50,rescueAgeMilliseconds:0,attestMilliseconds:50};
const temp=(label:string)=>{const path=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),`replica-gate-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const environment={...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.test",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.test",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"};
function cli(who:Machine,args:string[]):string{
 const result=Bun.spawnSync(["git",...args],{cwd:who.options.worktree,env:environment,stdout:"pipe",stderr:"pipe"});
 if(result.exitCode!==0)throw Error(`git ${args.join(" ")}: ${result.stderr.toString()}`);
 return result.stdout.toString();
}
const ok=(who:Machine,args:string[]):boolean=>Bun.spawnSync(["git",...args],{cwd:who.options.worktree,env:environment,stdout:"pipe",stderr:"pipe"}).exitCode===0;
const at=(who:Machine,path:string)=>join(who.options.worktree,...path.split("/"));
function write(who:Machine,path:string,text:string):void{const file=at(who,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);}
const machine=(name:string,store:string):Machine=>({name,events:[],options:{worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",...fast}});
const open=async(who:Machine):Promise<void>=>{who.replica=await openReplicaV1(who.options,event=>who.events.push(event));};
const shut=async(who:Machine):Promise<void>=>{if(who.replica){await closeReplicaV1(who.replica);who.replica=undefined;}};
const packs=(who:Machine):string[]=>readdirSync(at(who,".git/objects/pack")).filter(name=>name.endsWith(".pack")).sort();
async function converge(machines:readonly Machine[],done:()=>boolean,rounds:number=400):Promise<void>{
 for(let round=0;round<rounds;round++){
  for(const who of machines)await publishOnceV1(who.replica!);
  for(const who of machines)await followOnceV1(who.replica!);
  if(done())return;
  await Bun.sleep(15);
 }
 throw Error("The machines did not converge");
}

async function pair(){
 const store=temp("store"),server=machine("g1-server",store),worker=machine("g1-m01",store);
 write(server,".gitignore",fixture.gitignore);write(server,"a.txt","alpha\n");write(server,"dir/c.txt","charlie\n");
 cli(server,["init","-q","-b","main"]);cli(server,["add","-A"]);cli(server,["commit","-q","-m","initial"]);
 cli(server,["repack","-a","-d","-q"]);
 await seedV1(server.options);await joinV1(worker.options);
 await open(server);await open(worker);
 await converge([server,worker],()=>existsSync(at(worker,".git/objects/pack"))&&packs(worker).length===packs(server).length&&ok(worker,["cat-file","-e","HEAD^{commit}"]));
 return {store,server,worker,fleetPack:packs(server)[0]!};
}

async function packEvents(store:string,origin:string):Promise<string[]>{
 const found:string[]=[];
 for(const name of await listOriginsV1(store))if(name===origin)for(const event of (await readEventsV1(store,name,undefined)).events)if(event.path.startsWith(".git/objects/pack/")||event.path.startsWith(".git/objects/info/"))found.push(`${event.operation}:${event.path}`);
 return found;
}

async function settle(server:Machine,worker:Machine,commit:string,pause:()=>Promise<void>):Promise<void>{
 const controller=new AbortController(),runs=[runV1(server.options,controller.signal,event=>server.events.push(event)),runV1(worker.options,controller.signal,event=>worker.events.push(event))];
 try{
  for(let round=0;round<600&&!ok(server,["cat-file","-e",`${commit}^{commit}`]);round++)await Bun.sleep(50);
  await pause();
 }finally{controller.abort();await Promise.all(runs.map(run=>run.catch(()=>{})));}
}

for(const [label,command] of [["git repack -a -d",["repack","-a","-d","-q"]],["git gc --aggressive --prune=now",["gc","--aggressive","--prune=now","-q"]]] as const)
posixTest(`${label} on a worker publishes no pack byte and no pack delete, the commit made there still reaches the fleet and both repositories pass fsck`,async()=>{
 const {store,server,worker,fleetPack}=await pair();
 try{
  await shut(server);await shut(worker);
  write(worker,"worker.txt",`from the worker ${label}\n`);cli(worker,["add","-A"]);cli(worker,["commit","-q","-m","worker commit"]);
  const commit=cli(worker,["rev-parse","HEAD"]).trim();
  cli(worker,["config","gc.auto","0"]);
  cli(worker,command as unknown as string[]);
  expect(packs(worker).some(name=>name!==fleetPack)).toBe(true);
  await settle(server,worker,commit,async()=>{
   for(let round=0;round<200&&packs(worker).some(name=>name!==fleetPack);round++)await Bun.sleep(50);
  });
  expect(ok(server,["cat-file","-e",`${commit}^{commit}`])).toBe(true);
  expect(await packEvents(store,"g1-m01")).toEqual([]);
  expect((await packEvents(store,"g1-server")).filter(item=>item.startsWith("delete:"))).toEqual([]);
  cli(server,["fsck","--full"]);cli(worker,["fsck","--full"]);
  expect(packs(worker).filter(name=>name!==fleetPack)).toEqual([]);
  expect(statSync(at(server,`.git/objects/pack/${fleetPack}`)).size).toBeGreaterThan(0);
  expect(packs(server).length).toBe(1);
 }finally{await shut(server);await shut(worker);}
},180_000);

posixTest("the consolidator publishes its pack with a keep marker, a worker repack leaves that pack alone, and only authorised stems are retired for the fleet",async()=>{
 const {store,server,worker,fleetPack}=await pair();
 try{
  await shut(server);await shut(worker);
  write(server,"later.txt","later\n");cli(server,["add","-A"]);cli(server,["commit","-q","-m","later"]);cli(server,["repack","-d","-q"]);
  const controller=new AbortController(),run=runV1(server.options,controller.signal,()=>{}),runWorker=runV1(worker.options,controller.signal,()=>{});
  let result;
  try{result=await consolidateGitV1({worktree:server.options.worktree,apply:true,storeRoot:store,waitMilliseconds:60_000,pollMilliseconds:100,needBytes:1,space:async()=>undefined});}
  finally{
   for(let round=0;round<400&&!(existsSync(at(worker,`.git/objects/pack/${result?.pack}.pack`))&&ok(worker,["cat-file","-e",`${cli(server,["rev-parse","HEAD"]).trim()}^{commit}`])&&!existsSync(at(worker,`.git/objects/pack/${fleetPack}`)));round++)await Bun.sleep(50);
   controller.abort();await Promise.all([run.catch(()=>{}),runWorker.catch(()=>{})]);
  }
  expect(result?.outcome).toBe("consolidated");
  const fresh=`${result!.pack}.pack`;
  expect(existsSync(at(worker,`.git/objects/pack/${fresh}`))).toBe(true);
  expect(existsSync(at(worker,`.git/objects/pack/${result!.pack}.keep`))).toBe(true);
  expect(existsSync(at(worker,`.git/objects/pack/${fleetPack}`))).toBe(false);
  cli(worker,["repack","-a","-d","-q"]);
  expect(existsSync(at(worker,`.git/objects/pack/${fresh}`))).toBe(true);
  cli(worker,["fsck","--full"]);cli(server,["fsck","--full"]);
 }finally{await shut(server);await shut(worker);}
},180_000);

test("pack rescue touches index tables only for eligible local objects",async()=>{
 const database=new Database(":memory:");
 try{
  expect(new Ajv({strict:true}).validate(rescueSchema.definitions.RescueProbeFixture,rescueCases)).toBe(true);
  for(const row of rescueCases.cases){
   const worktree=temp(row.id),directory=join(worktree,".git","objects","pack");
   mkdirSync(directory,{recursive:true});
   const now=Date.now(),fleet=new Set<string>(),authorised:string[]=[];
   for(const pack of row.rows){
    if(pack.role==="fleet")fleet.add(packDirectoryV1+pack.stem+".pack");
    if(pack.role==="authorised")authorised.push(pack.stem);
    for(const extension of pack.files){
     const file=join(directory,pack.stem+"."+extension),age=(now-pack.ageMilliseconds)/1000;
     writeFileSync(file,"invalid fixture data");utimesSync(file,age,age);
    }
   }
   await authorisePacksV1(worktree,{publish:authorised},now);
   const expected=database.query(`with candidates as (
    select json_extract(value,'$.stem') stem,json_extract(value,'$.role') role,json_array_length(value,'$.files') fileCount,json_extract(value,'$.ageMilliseconds') age from json_each(?1)
   ), locals as (select stem from candidates where role='local' and fileCount=2 and age>=?2)
   select stem,'read' action from candidates where exists(select 1 from locals) and fileCount=2 and (role in ('fleet','authorised') or stem in locals)
   union all select stem,'retire' action from candidates where role='local' and fileCount=1 and age>=3600000`).all(JSON.stringify(row.rows),rescueCases.minimumAgeMilliseconds) as {stem:string;action:string}[];
   const filesRead=spyOn(packNames,"readPackNamesV1");
   try{
    const controller=new AbortController(),cancelled=Error("fixture cancelled");
    controller.abort(cancelled);
    await expect(rescueLocalPacksV1(worktree,path=>fleet.has(path),{now,signal:controller.signal})).rejects.toBe(cancelled);
    expect(filesRead.mock.calls).toEqual([]);
    for(const pack of row.rows)for(const extension of pack.files)expect(existsSync(join(directory,pack.stem+"."+extension))).toBe(true);
    const result=await rescueLocalPacksV1(worktree,path=>fleet.has(path),{now,minAgeMilliseconds:rescueCases.minimumAgeMilliseconds});
    const read=expected.filter(value=>value.action==="read").map(value=>join(directory,value.stem+".idx")).sort();
    expect(filesRead.mock.calls.map(([index])=>index).sort(),row.id).toEqual(read);
    const retired=expected.filter(value=>value.action==="retire").map(value=>value.stem);
    const examined=read.length===0?0:row.rows.filter(value=>value.role==="local"&&read.includes(join(directory,value.stem+".idx"))).length;
    expect(result,row.id).toEqual({examined,objects:0,retired:retired.length,kept:examined});
    for(const pack of row.rows)for(const extension of pack.files)expect(existsSync(join(directory,pack.stem+"."+extension)),pack.stem).toBe(!retired.includes(pack.stem));
   }finally{filesRead.mockRestore();}
  }
 }finally{database.close();}
});

test("the replica pack gate requires a live fleet put or explicit publication authority",async()=>{
 const store=temp("membership-store"),worker=machine("g1-m01",store),db=new Database(":memory:");
 try{
  await open(worker);
  const replica=worker.replica!,workspace=worker.options.worktree,now=Date.now(),hlc=formatHlcV1({ms:now,counter:0},worker.name);
  const put={hlc,operation:"put" as const,sha256:"a".repeat(64),bytes:1,executable:false},remove={hlc,operation:"delete" as const};
  mkdirSync(at(worker,".git"),{recursive:true});
  replica.manifest.set(packDirectoryV1+"pack-sentinel.pack",put);
  await authorisePacksV1(workspace,{
   publish:membership.cases.filter(row=>row.grant==="publish").map(row=>row.stem),
   retire:membership.cases.filter(row=>row.grant==="retire").map(row=>row.stem),
  },now);
  const oracle=db.query("select ?1 <> 'put' and ?2 <> 'publish' as local");
  const actual:Record<string,boolean|undefined>={},expected:Record<string,boolean>={};
  for(const row of membership.cases){
   const pack=packDirectoryV1+row.stem+".pack";
   if(row.mode==="interned")replica.book.intern(pack);
   if(row.mode==="put")replica.manifest.set(pack,put);
   if(row.mode==="delete")replica.manifest.set(pack,remove);
   expect(Boolean((oracle.get(row.mode,row.grant) as {local:number}).local),row.name).toBe(row.local);
   expected[row.name]=row.local;
   actual[row.name]=replica.ignore.local?.(packDirectoryV1+row.stem+"."+row.extension);
  }
  expect(actual).toEqual(expected);
 }finally{db.close();await shut(worker);}
});

for(const scenario of retirement.cases)test(`the replica preserves an old pack until ${scenario.name} without enumerating the manifest`,async()=>{
 const store=temp("retirement-store"),db=new Database(":memory:");
 let now=1_800_000_000_000;
 const created=now,worker=machine("g1-m01",store);
 const who={...worker,options:{...worker.options,now:()=>now}};
 const old=packDirectoryV1+"pack-old.pack",index=packDirectoryV1+"pack-new.idx",pack=packDirectoryV1+"pack-new.pack";
 const hlc=(ms:number)=>formatHlcV1({ms,counter:0},who.name);
 const put=(content:string,ms=created)=>({hlc:hlc(ms),operation:"put" as const,sha256:createHash("sha256").update(content).digest("hex"),bytes:Buffer.byteLength(content),executable:false});
 const remove={hlc:hlc(created),operation:"delete" as const,mtimeMs:0};
 const oracle=db.query("select ?1 = 'pending' or (?1 = 'missing' and ?2 < 600000) as retained");
 try{
  await open(who);
  let replica=who.replica!;
  write(who,old,"old");replica.files.set(old,put("old",created-1));
  replica.manifest.set(old,remove);enqueueId(replica,replica.book.find(old)!);
  write(who,index,"index");replica.files.set(index,put("index"));replica.manifest.set(index,put("index"));
  const irrelevant=put("irrelevant");
  for(let id=0;id<1024;id++){
   const path=`worktree/${id}.idx`;
   replica.files.set(path,irrelevant);replica.manifest.set(path,irrelevant);
  }
  for(const path of [".git/objects/pack-sibling/pack-similar.idx",packDirectoryV1+"pack-extra.idx.rev"]){
   replica.files.set(path,irrelevant);replica.manifest.set(path,irrelevant);
  }
  const linked=packDirectoryV1+"pack-linked.idx",link={hlc:hlc(created),operation:"link" as const,target:"pack-new.idx"};
  replica.manifest.set(linked,link);replica.files.set(linked,link);
  replica.manifest.set(packDirectoryV1+"pack-deleted.idx",remove);replica.files.set(packDirectoryV1+"pack-deleted.idx",remove);
  replica.book.intern(packDirectoryV1+"pack-interned.idx");
  const sentinel=packDirectoryV1+"pack-sentinel.pack";
  replica.files.set(sentinel,put("sentinel"));replica.manifest.set(sentinel,put("sentinel"));
  if(scenario.checkpoint){
   await checkpointReplicaV1(replica);
   const generation=replica.generation;
   await shut(who);await open(who);replica=who.replica!;
   expect(replica.generation).toBe(generation);
   expect(replica.manifest.has(index)).toBe(true);
  }
  expect(replica.ignore.local?.(old)).toBe(true);
  const scans=spyOn(replica.manifest,"ids");
  try{
   for(const step of scenario.steps){
    now=created+step.elapsed;
    if(step.pack==="pending"){
     replica.manifest.set(pack,put("new"));enqueueId(replica,replica.book.find(pack)!);
    }else if(step.pack==="applied")write(who,pack,"new");
    await applyQueue(replica,undefined);
    expect(Boolean((oracle.get(step.pack,step.elapsed) as {retained:number}).retained)).toBe(step.retained);
    expect(existsSync(at(who,old)),step.pack+` at ${step.elapsed}`).toBe(step.retained);
    expect(replica.queue.hasId(replica.book.find(old)!)).toBe(step.retained);
    if(step.pack==="pending")expect(replica.queue.hasId(replica.book.find(pack)!)).toBe(true);
    if(step.pack==="applied")expect(replica.files.get(pack)?.operation).toBe("put");
   }
   expect(scans).not.toHaveBeenCalled();
  }finally{scans.mockRestore();}
 }finally{db.close();await shut(who);}
});
