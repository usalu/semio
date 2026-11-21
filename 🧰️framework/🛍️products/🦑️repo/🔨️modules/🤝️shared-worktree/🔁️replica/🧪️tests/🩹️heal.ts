import {afterAll,expect,spyOn,test} from "bun:test";
import {Database} from "bun:sqlite";
import * as fs from "node:fs/promises";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import healthyHeal from "./🧫️fixtures/🩹️healthy-heal.json" with {type:"json"};
import {appendEventsV1,formatHlcV1,readEventsV1,listOriginsV1,storeBlobV1,type FleetEventV1} from "../../📒️journal/🟦️.ts";
import {gitBrokenV1,gitStateV1} from "../../🌿️git/🟦️.ts";
import {authorisePacksV1} from "../🧷️packgate/🟦️.ts";
import {readAttestationV1} from "../../🩺️attest/🟦️.ts";
import {closeReplicaV1,followOnceV1,healGitV1,joinV1,openReplicaV1,publishOnceV1,runV1,seedV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../🟦️.ts";

type Machine={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];replica?:ReplicaV1};
const scratch:string[]=[],fast={pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`replica-heal-${label}-`));scratch.push(path);return path;};
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
async function converge(machines:readonly Machine[],done:()=>boolean,rounds:number=400):Promise<void>{
 for(let round=0;round<rounds;round++){
  for(const who of machines)await publishOnceV1(who.replica!);
  for(const who of machines)await followOnceV1(who.replica!);
  if(done())return;
  await Bun.sleep(15);
 }
 throw Error("The machines did not converge");
}
const packs=(who:Machine):string[]=>readdirSync(at(who,".git/objects/pack")).filter(name=>name.endsWith(".pack")).sort();
const sha=(file:string):string=>createHash("sha256").update(readFileSync(file)).digest("hex");

async function pair(){
 const store=temp("store"),server=machine("g1-server",store),worker=machine("g1-m01",store);
 write(server,".gitignore",fixture.gitignore);write(server,"a.txt","alpha\n");write(server,"dir/c.txt","charlie\n");
 cli(server,["init","-q","-b","main"]);cli(server,["add","-A"]);cli(server,["commit","-q","-m","initial"]);
 cli(server,["repack","-a","-d","-q"]);
 await seedV1(server.options);await joinV1(worker.options);
 await open(server);await open(worker);
 await converge([server,worker],()=>existsSync(at(worker,".git/objects/pack"))&&packs(worker).length===packs(server).length&&ok(worker,["cat-file","-e","HEAD^{commit}"]));
 return {store,server,worker};
}

async function repack(who:Machine):Promise<{old:string[];fresh:string[]}>{
 const old=readdirSync(at(who,".git/objects/pack")).filter(name=>name.startsWith("pack-")).map(name=>`.git/objects/pack/${name}`);
 write(who,"b.txt",`bravo ${Date.now()}\n`);cli(who,["add","-A"]);cli(who,["commit","-q","-m","second"]);
 cli(who,["repack","-a","-d","-q"]);
 const now=readdirSync(at(who,".git/objects/pack")).filter(name=>name.startsWith("pack-")).map(name=>`.git/objects/pack/${name}`);
 const result={old:old.filter(path=>!now.includes(path)),fresh:now.filter(path=>!old.includes(path))},stem=(path:string)=>path.slice(path.lastIndexOf("/")+1).replace(/\.[a-z]+$/u,"");
 await authorisePacksV1(who.options.worktree,{publish:result.fresh.map(stem),retire:[...result.old.map(stem),"multi-pack-index"]});
 return result;
}

posixTest("a repack publishes no deletion and no index of the new pack before the new pack itself, so a worker never loses the objects of its old packs",async()=>{
 const {store,server,worker}=await pair();
 try{
  const {old,fresh}=await repack(server),pack=fresh.find(path=>path.endsWith(".pack"))!;
  expect(old.length).toBeGreaterThan(0);
  const result=await publishOnceV1(server.replica!,undefined,[...old,...fresh.filter(path=>path!==pack),".git/objects/pack/multi-pack-index"]);
  const paths:{operation:string;path:string}[]=[];
  for(const origin of await listOriginsV1(store))for(const event of (await readEventsV1(store,origin,undefined)).events)paths.push({operation:event.operation,path:event.path});
  expect(paths.filter(item=>item.path.startsWith(".git/objects/pack/")&&(item.operation==="delete"||fresh.includes(item.path)))).toEqual([]);
  expect(result.retry).toEqual(expect.arrayContaining([...old,...fresh.filter(path=>path!==pack)]));
  await followOnceV1(worker.replica!);
  expect(ok(worker,["cat-file","-e","HEAD^{commit}"])).toBe(true);
  await converge([server,worker],()=>ok(worker,["cat-file","-e",`${cli(server,["rev-parse","HEAD"]).trim()}^{commit}`])&&old.every(path=>!existsSync(at(worker,path))));
  expect(fresh.every(path=>existsSync(at(worker,path)))).toBe(true);
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
 }finally{await shut(server);await shut(worker);}
},120_000);

posixTest("a worker keeps its old packs while the fleet holds an index whose pack has not arrived, and drops them once it has",async()=>{
 const {store,server,worker}=await pair();
 try{
  const {old,fresh}=await repack(server),pack=fresh.find(path=>path.endsWith(".pack"))!;
  const put=async(path:string,number:number):Promise<FleetEventV1>=>{const stored=await storeBlobV1(store,"g1-m09",at(server,path));return {version:1,hlc:formatHlcV1({ms:Date.now()+number,counter:0},"g1-m09"),operation:"put",path,sha256:stored.sha256,bytes:stored.bytes,executable:false,base:null};};
  const removals:FleetEventV1[]=old.map((path,number)=>({version:1,hlc:formatHlcV1({ms:Date.now()+100+number,counter:0},"g1-m09"),operation:"delete",path,base:null}));
  await appendEventsV1(store,"g1-m09",[...await Promise.all(fresh.filter(path=>path!==pack).map((path,number)=>put(path,number))),...removals]);
  await followOnceV1(worker.replica!);
  expect(old.every(path=>existsSync(at(worker,path)))).toBe(true);
  expect(ok(worker,["cat-file","-e","HEAD^{commit}"])).toBe(true);
  await appendEventsV1(store,"g1-m09",[await put(pack,200)]);
  await followOnceV1(worker.replica!);await followOnceV1(worker.replica!);
  expect(existsSync(at(worker,pack))).toBe(true);
  expect(old.some(path=>existsSync(at(worker,path)))).toBe(false);
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
 }finally{await shut(server);await shut(worker);}
},120_000);


posixTest("a winning tombstone preserves an unheld private pack and its index",async()=>{
 const {store,server,worker}=await pair();
 const db=new Database(":memory:");
 try{
  const baseline=new Set(readdirSync(at(worker,".git/objects/pack")));
  write(worker,"local-pack-source.txt","exact unshared content\n");
  cli(worker,["add","--","local-pack-source.txt"]);
  cli(worker,["commit","-q","-m","private packed commit"]);
  cli(worker,["repack","-d","-q"]);
  const privatePaths=readdirSync(at(worker,".git/objects/pack")).filter(name=>!baseline.has(name)&&/^pack-[0-9a-f]+\.(?:pack|idx)$/u.test(name)).map(name=>".git/objects/pack/"+name);
  expect(privatePaths).toHaveLength(2);
  const tip=cli(worker,["rev-parse","HEAD"]).trim(),before=new Map(privatePaths.map(path=>[path,sha(at(worker,path))])),now=Date.now();
  const deletes:FleetEventV1[]=privatePaths.map((path,index)=>({version:1,hlc:formatHlcV1({ms:now,counter:index},"g1-m09"),operation:"delete",path,base:null}));
  const expected=db.query("select value as path from json_each(?1) where ?2 <> 'put' order by value").all(JSON.stringify(privatePaths),"absent").map(row=>(row as {path:string}).path);
  expect(privatePaths.every(path=>worker.replica!.files.get(path)===undefined)).toBe(true);
  await appendEventsV1(store,"g1-m09",deletes);
  await followOnceV1(worker.replica!);
  expect(privatePaths.filter(path=>existsSync(at(worker,path))&&sha(at(worker,path))===before.get(path)).sort()).toEqual(expected);
  expect(privatePaths.map(path=>worker.replica!.manifest.get(path)?.operation)).toEqual(["delete","delete"]);
  expect(ok(worker,["cat-file","-e",tip+"^{commit}"])).toBe(true);
 }finally{db.close();await shut(server);await shut(worker);}
},20000);

posixTest("the check names why HEAD does not resolve, the heal brings the shared git files and the stage back from the store, and nothing is reported once it is sound",async()=>{
 const {server,worker}=await pair();
 try{
  const tip=cli(server,["rev-parse","HEAD"]).trim(),stage=cli(worker,["write-tree"]).trim(),head=readFileSync(at(worker,".git/HEAD"),"utf8");
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
  const idx=packs(worker).map(name=>`.git/objects/pack/${name.replace(/\.pack$/u,".idx")}`);
  rmSync(at(worker,idx[0]!));
  rmSync(at(worker,".git/index"));
  rmSync(at(worker,".git/HEAD"));
  expect(await gitBrokenV1(worker.options.worktree)).toBe("head-unreadable");
  writeFileSync(at(worker,".git/HEAD"),head);
  expect(await gitBrokenV1(worker.options.worktree)).toBe("tip-missing");
  const queued=await healGitV1(worker.replica!);
  expect(queued).toBeGreaterThanOrEqual(1);
  await followOnceV1(worker.replica!);await publishOnceV1(worker.replica!);
  expect(existsSync(at(worker,idx[0]!))).toBe(true);
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
  expect(cli(worker,["rev-parse","HEAD"]).trim()).toBe(tip);
  expect(cli(worker,["write-tree"]).trim()).toBe(stage);
  expect((await gitStateV1(worker.options.worktree)).commit).toBe(tip);
 }finally{await shut(server);await shut(worker);}
},120_000);

posixTest("a config git cannot parse is named, an absent config and a sound repository are not",async()=>{
 const {server,worker}=await pair();
 try{
  const config=readFileSync(at(worker,".git/config"),"utf8");
  writeFileSync(at(worker,".git/config"),"[core\n\tbare = =\n");
  expect(await gitBrokenV1(worker.options.worktree)).toBe("config-unreadable");
  rmSync(at(worker,".git/config"));
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
  writeFileSync(at(worker,".git/config"),config);
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
 }finally{await shut(server);await shut(worker);}
},120_000);

posixTest("a running daemon finds a repository it cannot read at its first round, repairs it from the store and attests the head again",async()=>{
 const {store,server,worker}=await pair();
 const controller=new AbortController();
 try{
  const tip=cli(server,["rev-parse","HEAD"]).trim(),stage=cli(server,["write-tree"]).trim();
  await shut(worker);
  rmSync(at(worker,".git/HEAD"));rmSync(at(worker,".git/index"));
  for(const name of packs(worker)){rmSync(at(worker,`.git/objects/pack/${name}`));rmSync(at(worker,`.git/objects/pack/${name.replace(/\.pack$/u,".idx")}`));}
  expect(await gitBrokenV1(worker.options.worktree)).not.toBeNull();
  const events:ReplicaEventV1[]=[],done=runV1({...worker.options,gitCheckMilliseconds:50,attestMilliseconds:50},controller.signal,event=>events.push(event));
  for(let round=0;round<400&&(await gitBrokenV1(worker.options.worktree)!==null||(await readAttestationV1(store,"g1-m01"))?.head?.commit!==tip);round++)await Bun.sleep(50);
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
  expect((await readAttestationV1(store,"g1-m01"))?.head?.commit).toBe(tip);
  expect(events.some(event=>event.event==="repaired"&&event.stage==="git")).toBe(true);
  await converge([server],()=>true,3);
  expect(cli(worker,["status","--short"]).trim()).toBe("");
  expect(cli(worker,["write-tree"]).trim()).toBe(stage);
  expect(cli(server,["write-tree"]).trim()).toBe(stage);
  controller.abort();await done.catch(()=>{});
 }finally{controller.abort();await shut(server);await shut(worker);}
},120_000);

posixTest("a healthy Git heal yields to an ordinary journal turn between ordered slices",async()=>{
 const {store,server,worker}=await pair(),controller=new AbortController(),database=new Database(":memory:"),trace:{kind:string;id:number}[]=[],delayed=new Map<string,number>();
 const stat=fs.lstat,statSpy=spyOn(fs,"lstat").mockImplementation((async (path:Parameters<typeof fs.lstat>[0],options?:any)=>{
  const id=typeof path==="string"?delayed.get(path):undefined;
  if(id!==undefined&&/\bhealGit(?:Path)?V1\b/u.test(new Error().stack??"")){trace.push({kind:"probe",id});await Bun.sleep(healthyHeal.latencyMilliseconds);}
  return stat(path,options);
 }) as typeof fs.lstat);
 let running:ReturnType<typeof runV1>|undefined;
 try{
  await shut(worker);
  expect(await gitBrokenV1(worker.options.worktree)).toBeNull();
  const options={...worker.options,watch:false,gitCheckMilliseconds:50,gitHealMilliseconds:0,attestMilliseconds:50,storeGate:async()=>{trace.push({kind:"turn",id:0});},opened(replica:ReplicaV1){
   for(let id=0;id<healthyHeal.probes;id++){
    const path=".git/objects/aa/"+id.toString(16).padStart(38,"0");
    write(worker,path,"fixture "+id);
    const row={hlc:formatHlcV1({ms:Date.now(),counter:id},"g1-server"),operation:"put" as const,sha256:sha(at(worker,path)),bytes:Buffer.byteLength("fixture "+id),executable:false};
    replica.manifest.set(path,row);replica.files.set(path,{...row,mtimeMs:statSync(at(worker,path)).mtimeMs});
    delayed.set(at(worker,path),id);
   }
  }};
  write(worker,healthyHeal.worktreePath,"ordinary healthy-heal proof");
  running=runV1(options,controller.signal,event=>{if(event.event==="published"&&event.stage!=="git")trace.push({kind:"published",id:0});});
  const deadline=performance.now()+8000;
  while(performance.now()<deadline&&(!trace.some(row=>row.kind==="probe"&&row.id===healthyHeal.probes-1)||!trace.some(row=>row.kind==="published")))await Bun.sleep(10);
  controller.abort();await running;
  const expected=database.query(`with rows as (
   select key position,json_extract(value,'$.kind') kind,json_extract(value,'$.id') id from json_each(?1)
  ), first_round as (select min(position) first,max(position) last,count(*) n from (select * from rows where kind='probe' order by position limit ?2))
  select n,(select count(*) from rows where kind in ('turn','published') and position > first and position < last) between_count from first_round`).get(JSON.stringify(trace),healthyHeal.probes) as {n:number;between_count:number};
  expect(expected.n).toBe(healthyHeal.probes);
  expect(expected.between_count).toBeGreaterThan(0);
  expect(trace.filter(row=>row.kind==="probe").slice(0,healthyHeal.probes).map(row=>row.id)).toEqual(Array.from({length:healthyHeal.probes},(_,id)=>id));
  expect((await readEventsV1(store,"g1-m01",undefined)).events.some(event=>event.path===healthyHeal.worktreePath)).toBe(true);
 }finally{controller.abort();await running?.catch(()=>{});statSpy.mockRestore();database.close();await shut(server);await shut(worker);}
},30000);

test("Git heal discards an absent leaf observed after its turn was cancelled",async()=>{
 const worker=machine("g1-m01",temp("cancel-store")),controller=new AbortController(),reason=Error("fixture cancelled"),path=".git/HEAD",target=at(worker,path);
 await open(worker);
 const replica=worker.replica!,row={hlc:formatHlcV1({ms:Date.now(),counter:0},"g1-server"),operation:"put" as const,sha256:"a".repeat(64),bytes:42,executable:false};
 replica.manifest.set(path,row);replica.files.set(path,{...row,mtimeMs:0});
 const entered=Promise.withResolvers<void>(),release=Promise.withResolvers<void>(),stat=fs.lstat;
 const mock=spyOn(fs,"lstat").mockImplementation((async (name:Parameters<typeof fs.lstat>[0],options?:any)=>{
  if(name===target){entered.resolve();await release.promise;}
  return stat(name,options);
 }) as typeof fs.lstat);
 try{
  const pending=healGitV1(replica,controller.signal);
  await Promise.race([entered.promise,pending.then(()=>{throw Error("no Git leaf probe");})]);
  controller.abort(reason);release.resolve();
  await expect(pending).rejects.toBe(reason);
  expect(replica.files.get(path)).toEqual({...row,mtimeMs:0});
  expect(replica.queue.has(path)).toBe(false);
 }finally{release.resolve();mock.mockRestore();await shut(worker);}
});
