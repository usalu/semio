import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
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
