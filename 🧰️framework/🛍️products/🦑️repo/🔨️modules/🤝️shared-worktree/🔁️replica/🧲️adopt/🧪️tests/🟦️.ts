import {afterAll,expect,test} from "bun:test";
import {chmodSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,sep} from "node:path";
import {createHash} from "node:crypto";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import replicaFixture from "../../🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {compileJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {listOriginsV1,storePathsV1} from "../../../📒️journal/🟦️.ts";
import {adoptV1,AdoptRefusedV1,classifyV1,planAdoptV1,readAdoptStepV1,reasonV1,type AdoptReportV1,type AdoptStepV1} from "../🟦️.ts";
import {closeReplicaV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,seedV1,verifyV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../../🟦️.ts";

type Machine={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];replica?:ReplicaV1};
const scratch:string[]=[],fast={pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0},python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py");
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`adopt-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const environment={...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.test",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.test",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"};
function cli(directory:string,args:string[]):string{
 const result=Bun.spawnSync(["git",...args],{cwd:directory,env:environment,stdout:"pipe",stderr:"pipe"});
 if(result.exitCode!==0)throw Error(`git ${args.join(" ")}: ${result.stderr.toString()}`);
 return result.stdout.toString();
}
const git=(who:Machine,...args:string[]):string=>cli(who.options.worktree,args);
const at=(who:Machine,path:string)=>join(who.options.worktree,...path.split("/"));
const read=(who:Machine,path:string):string=>readFileSync(at(who,path),"utf8");
function write(who:Machine,path:string,text:string,when?:number):void{const file=at(who,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);if(when!==undefined)utimesSync(file,when/1000,when/1000);}
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
const same=(machines:readonly Machine[],value:(who:Machine)=>unknown):boolean=>machines.every(who=>JSON.stringify(value(who))===JSON.stringify(value(machines[0]!)));
const sha=(text:string|Buffer):string=>createHash("sha256").update(text).digest("hex");
const HOUR=3_600_000,inside=(who:Machine,source:string):string=>source.slice(who.options.worktree.length);

function snapshot(root:string):string[]{
 const lines:string[]=[];
 const walk=(directory:string):void=>{
  for(const item of readdirSync(directory,{withFileTypes:true}).sort((left,right)=>left.name<right.name?-1:1)){
   const path=join(directory,item.name),info=statSync(path);
   if(item.isDirectory()){lines.push(`d ${path.slice(root.length)} ${info.mtimeMs}`);walk(path);}
   else lines.push(`f ${path.slice(root.length)} ${info.size} ${info.mtimeMs} ${sha(readFileSync(path))}`);
  }
 };
 walk(root);
 return lines;
}

async function build(options:{unrelated?:boolean}={}){
 const store=temp("store"),server=machine("g1-server",store),worker=machine("g1-m01",store),adopter=machine("g1-m02",store);
 write(server,".gitignore",replicaFixture.gitignore);write(server,"m.txt","mike v0\n");
 git(server,"init","-q","-b","main");git(server,"add","-A");git(server,"commit","-q","-m","zero");
 write(server,"a.txt","alpha\n");write(server,"b.txt","bravo\n");write(server,"dir/c.txt","charlie\n");write(server,"d.txt","delta\n");write(server,"e.txt","echo\n");write(server,"g.txt","golf\n");write(server,"h.txt","hotel\n");write(server,"i.txt","india\n");write(server,"k.txt","kilo\n");write(server,"m.txt","mike v1\n");
 git(server,"add","-A");git(server,"commit","-q","-m","initial");
 git(server,"tag","fleet-old");git(server,"pack-refs","--all");
 await seedV1(server.options);
 await joinV1(worker.options);
 if(options.unrelated){
  mkdirSync(adopter.options.worktree,{recursive:true});
  cpSync(server.options.worktree,adopter.options.worktree,{recursive:true,filter:source=>!inside(server,source).includes(`${sep}.cache`)&&!inside(server,source).includes(`${sep}.git`)});
  write(adopter,"z.txt","zulu old\n");
  git(adopter,"init","-q","-b","main");git(adopter,"add","-A");git(adopter,"commit","-q","-m","unrelated root");
 }else cpSync(server.options.worktree,adopter.options.worktree,{recursive:true,filter:source=>!inside(server,source).includes(`${sep}.cache`)});
 await open(server);await open(worker);
 write(server,"g.txt","golf mid\n");
 await converge([server,worker],()=>read(worker,"g.txt")==="golf mid\n");
 write(server,"b.txt","bravo fleet\n");rmSync(at(server,"d.txt"));rmSync(at(server,"e.txt"));write(server,"g.txt","golf fleet\n");write(server,"h.txt","hotel fleet\n");write(server,"k.txt","kilo fleet\n");write(server,"m.txt","mike v2\n");write(server,"z.txt","zulu new\n");
 git(server,"add","-A");git(server,"commit","-q","-m","fleet commit");
 write(server,"f.txt","foxtrot\n");git(server,"add","f.txt");
 await converge([server,worker],()=>same([server,worker],who=>git(who,"write-tree"))&&git(worker,"log","-1","--format=%s").trim()==="fleet commit"&&git(worker,"diff","--cached","--name-only","HEAD").trim()==="f.txt");
 const now=Date.now();
 write(adopter,"a.txt","alpha local\n",now-30*24*HOUR);
 write(adopter,"b.txt","bravo\n",now+HOUR);
 write(adopter,"d.txt","delta\n",now+HOUR);
 write(adopter,"e.txt","echo local\n",now+HOUR);
 write(adopter,"g.txt","golf mid\n",now+HOUR);
 write(adopter,"h.txt","hotel local\n",now+HOUR);
 write(adopter,"k.txt","kilo\n",now-30*24*HOUR);
 write(adopter,"m.txt","mike v0\n",now+HOUR);
 write(adopter,"local-only.txt","only\n");
 write(adopter,"ignored/junk.bin","junk");write(adopter,"x.log","log");
 mkdirSync(at(adopter,"nested"),{recursive:true});cli(at(adopter,"nested"),["init","-q","-b","main"]);write(adopter,"nested/readme.txt","nested\n");
 if(!options.unrelated){git(adopter,"add","a.txt","local-only.txt");git(adopter,"commit","-q","-m","local commit");}
 write(adopter,"staged-local.txt","s\n");git(adopter,"add","staged-local.txt");
 git(adopter,"branch","side");git(adopter,"tag","v-local");
 git(adopter,"stash","store","-m","local stash",git(adopter,"stash","create").trim());
 git(adopter,"pack-refs","--all");utimesSync(at(adopter,".git/packed-refs"),(now+HOUR)/1000,(now+HOUR)/1000);
 return {store,server,worker,adopter,localCommit:options.unrelated?"":git(adopter,"rev-parse","HEAD").trim()};
}

const run=(who:Machine,extra:Partial<{force:boolean;fault:(step:AdoptStepV1)=>void}>={})=>adoptV1(who.options,{dryRun:false,force:false,sample:20,...extra},undefined,event=>who.events.push(event));

async function verifyAdoption(fleetMachines:{store:string;server:Machine;worker:Machine;adopter:Machine;localCommit:string},report:AdoptReportV1):Promise<void>{
 const {server,worker,adopter,localCommit}=fleetMachines;
 const lost=(path:string):string=>join(adopter.options.worktree,".cache","f",path);
 expect(read(adopter,"a.txt")).toBe("alpha local\n");
 for(const [path,text] of [["b.txt","bravo fleet\n"],["g.txt","golf fleet\n"],["k.txt","kilo fleet\n"],["m.txt","mike v2\n"],["h.txt","hotel fleet\n"],["f.txt","foxtrot\n"],["i.txt","india\n"],["dir/c.txt","charlie\n"]] as const)expect(read(adopter,path)).toBe(text);
 expect(existsSync(at(adopter,"d.txt"))).toBe(false);expect(existsSync(at(adopter,"e.txt"))).toBe(false);
 expect(read(adopter,"local-only.txt")).toBe("only\n");expect(read(adopter,"staged-local.txt")).toBe("s\n");
 expect(read(adopter,"ignored/junk.bin")).toBe("junk");expect(read(adopter,"x.log")).toBe("log");
 expect(read(adopter,"nested/readme.txt")).toBe("nested\n");expect(existsSync(at(adopter,"nested/.git"))).toBe(true);
 const staleOf=(path:string)=>report.stale.find(item=>item.path===path)!,conflictOf=(path:string)=>report.conflicts.find(item=>item.path===path)!;
 for(const [path,text,reason] of [["b.txt","bravo\n","journal"],["d.txt","delta\n","journal"],["g.txt","golf mid\n","journal"],["k.txt","kilo\n","journal"],["m.txt","mike v0\n","git"]] as const){
  expect(staleOf(path).reason).toBe(reason);expect(readFileSync(lost(staleOf(path).lost),"utf8")).toBe(text);
 }
 expect(conflictOf("e.txt").reason).toBe("deleted-by-fleet");expect(conflictOf("e.txt").fleet).toBeNull();expect(readFileSync(lost(conflictOf("e.txt").lost),"utf8")).toBe("echo local\n");
 expect(conflictOf("h.txt").reason).toBe("both-changed");expect(readFileSync(lost(conflictOf("h.txt").lost),"utf8")).toBe("hotel local\n");expect(conflictOf("h.txt").fleet).toBe(sha("hotel fleet\n"));expect(conflictOf("h.txt").local).toBe(sha("hotel local\n"));
 expect(report.edited).toEqual(["a.txt"]);expect(report.published).toEqual(expect.arrayContaining(["local-only.txt","staged-local.txt"]));
 expect(report.counts.stale).toBe(5);expect(report.counts.conflict).toBe(2);expect(report.counts.edit).toBe(1);expect(report.counts.nested).toBe(1);expect(report.counts.publish).toBeGreaterThanOrEqual(2);expect(report.counts.materialise).toBeGreaterThanOrEqual(1);
 expect(report.stage.lost).not.toBeNull();expect(existsSync(lost(report.stage.lost!))).toBe(true);
 expect(report.stage.paths).toContain("staged-local.txt");
 expect(report.pending).toEqual([]);
 expect(existsSync(join(adopter.options.worktree,".cache","f","lost",`adopt-${report.stamp}.md`))).toBe(true);
 const markdown=readFileSync(join(adopter.options.worktree,".cache","f","lost",`adopt-${report.stamp}.md`),"utf8");
 expect(markdown).toContain("Conflicts");expect(markdown).toContain("e.txt (deleted-by-fleet)");expect(markdown).toContain("h.txt (both-changed)");
 const compiled=compileJsonSchemaSubset({$ref:"#/definitions/Report"},schema);
 expect(compiled.validate(report)).toEqual([]);
 if(python){
  const file=join(temp("report"),"report.json");writeFileSync(file,JSON.stringify(report));
  const check=Bun.spawnSync([python,"-I",oracle,"report",file],{stdout:"pipe",stderr:"pipe"});
  expect(check.stdout.toString()+check.stderr.toString()).toBe("");
 }
 const subjects=git(adopter,"log","--format=%s","main").trim().split("\n");
 expect(subjects).toEqual(["local commit","fleet commit","initial","zero"]);
 expect(git(adopter,"rev-list","--merges","main").trim()).toBe("");
 expect(readFileSync(lost(`lost/adopt-${report.stamp}/git/.git/packed-refs`),"utf8")).toContain("refs/heads/side");
 expect(git(adopter,"write-tree").trim()).toBe(git(server,"write-tree").trim());
 await open(adopter);
 await converge([server,worker,adopter],()=>same([server,worker,adopter],who=>who.replica!.files.manifestHex())&&same([server,worker,adopter],who=>git(who,"rev-parse","HEAD"))&&same([server,worker,adopter],who=>git(who,"write-tree")));
 for(const who of [server,worker,adopter]){
  expect(git(who,"log","--format=%s","main").trim().split("\n")).toEqual(["local commit","fleet commit","initial","zero"]);
  expect(git(who,"diff","--cached","--name-only","HEAD")).toBe(git(server,"diff","--cached","--name-only","HEAD"));
  expect(read(who,"a.txt")).toBe("alpha local\n");expect(read(who,"local-only.txt")).toBe("only\n");
  for(const [path,text] of [["b.txt","bravo fleet\n"],["g.txt","golf fleet\n"],["k.txt","kilo fleet\n"],["m.txt","mike v2\n"],["h.txt","hotel fleet\n"]] as const)expect(read(who,path)).toBe(text);
  expect(existsSync(at(who,"d.txt"))).toBe(false);expect(existsSync(at(who,"e.txt"))).toBe(false);
  expect(git(who,"rev-parse","v-local^{commit}").trim()).toBe(localCommit);expect(git(who,"rev-parse","refs/heads/side^{commit}").trim()).toBe(localCommit);expect(git(who,"rev-parse","fleet-old").trim()).toHaveLength(40);
  expect(git(who,"for-each-ref","--format=%(refname:lstrip=3)","refs/adopt").trim()).toBe("stash-0");expect(git(who,"cat-file","-t",git(who,"for-each-ref","--format=%(objectname)","refs/adopt").trim()).trim()).toBe("commit");
 }
 expect(readFileSync(at(server,".git/packed-refs"),"utf8")).not.toContain("refs/heads/side");
 expect(existsSync(at(server,"ignored/junk.bin"))).toBe(false);expect(existsSync(at(server,"x.log"))).toBe(false);
 expect(await verifyAdoptionState(adopter)).toBe(true);
 for(const who of [server,worker,adopter])expect(who.events.filter(event=>event.event==="error")).toEqual([]);
}

async function verifyAdoptionState(who:Machine):Promise<boolean>{
 const report=await verifyV1(who.options);
 return report.drift.filter(item=>!item.path.startsWith("nested/")).length===0;
}

const vectors=fixture.cases as unknown as readonly {local:never;remote:never;evidence:never;expect:string|null;reason:string|null}[];

test("the classification of every vector follows the rule, never reads a modification time, and the Python oracle decides the same",()=>{
 const results=vectors.map(item=>{const category=classifyV1((item.local??undefined) as never,(item.remote??undefined) as never,item.evidence);return [category??null,reasonV1(category,(item.remote??undefined) as never,item.evidence)??null];});
 expect(results).toEqual(vectors.map(item=>[item.expect,item.reason]));
 for(const item of vectors){
  if(item.local===null)continue;
  const local=item.local as {mtimeMs:number};
  for(const mtimeMs of [0,1,4102444800000]){
   const moved=classifyV1({...local,mtimeMs} as never,(item.remote??undefined) as never,item.evidence)??null;
   expect(moved).toBe(item.expect);
  }
 }
 if(python){
  const file=join(import.meta.dirname,"🧫️fixtures","🔣️.json"),run=Bun.spawnSync([python,"-I",oracle,"classify",file],{stdout:"pipe",stderr:"pipe"});
  expect(JSON.parse(run.stdout.toString())).toEqual(results);
 }
});

posixTest("a diverged existing repository is adopted loss-free by content: old versions with fresh modification times and resurrected deletes lose to the fleet and publish nothing, local work is published, changes on both sides are conflicts that keep both versions, and all three replicas converge",async()=>{
 const fleet=await build();
 try{
  await shut(fleet.adopter);
  const before=new Map(["b.txt","d.txt","e.txt","g.txt","h.txt","k.txt","m.txt"].map(path=>[path,existsSync(at(fleet.server,path))?read(fleet.server,path):null]));
  const result=await run(fleet.adopter);
  expect(result.queued).toBe(0);
  expect(await readAdoptStepV1(fleet.adopter.options.worktree)).toBe("reported");
  await verifyAdoption(fleet,result.report!);
  for(const [path,text] of before)expect(existsSync(at(fleet.server,path))?read(fleet.server,path):null).toBe(text);
 }finally{for(const who of [fleet.server,fleet.worker,fleet.adopter])await shut(who);}
},180_000);

posixTest("the dry run prints the counts per class, every conflict with its reason and the evidence, and changes nothing in the worktree, its repository or the store",async()=>{
 const fleet=await build();
 try{
  const treeBefore=git(fleet.adopter,"write-tree"),before=snapshot(fleet.adopter.options.worktree),storeBefore=snapshot(fleet.store),indexBefore=sha(readFileSync(at(fleet.adopter,".git/index")));
  const plan=await planAdoptV1(fleet.adopter.options,{sample:3});
  expect(plan.dryRun).toBe(true);
  expect(plan.counts.stale).toBe(5);expect(plan.counts.conflict).toBe(2);expect(plan.counts.edit).toBe(1);expect(plan.counts.nested).toBe(1);expect(plan.counts.publish).toBeGreaterThanOrEqual(2);expect(plan.counts.materialise).toBeGreaterThanOrEqual(1);expect(plan.counts.same).toBeGreaterThanOrEqual(3);
  expect(plan.decisions.filter(item=>item.category==="conflict").map(item=>[item.path,item.reason])).toEqual([["e.txt","deleted-by-fleet"],["h.txt","both-changed"]]);
  expect(plan.decisions.find(item=>item.path==="b.txt")).toMatchObject({category:"stale",reason:"journal",localSha256:sha("bravo\n"),fleetSha256:sha("bravo fleet\n")});
  expect(plan.decisions.find(item=>item.path==="d.txt")).toMatchObject({category:"stale",reason:"journal",fleetSha256:null});
  expect(plan.decisions.find(item=>item.path==="m.txt")).toMatchObject({category:"stale",reason:"git"});
  expect(plan.decisions.find(item=>item.path==="a.txt")).toMatchObject({category:"edit",reason:"fleet-unchanged"});
  for(const category of Object.keys(plan.samples) as (keyof typeof plan.samples)[])expect(plan.samples[category].length).toBeLessThanOrEqual(3);
  expect(plan.nested).toEqual(["nested"]);
  expect(plan.evidence.events).toBeGreaterThan(0);expect(plan.evidence.commits).toBeGreaterThan(0);
  expect(plan.git.fleetTip).not.toBeNull();expect(plan.git.stagedPaths).toBe(1);
  expect(["diverged","unknown"]).toContain(plan.git.relation);
  const dry=await adoptV1(fleet.adopter.options,{dryRun:true,force:false,sample:3});
  expect(dry.report).toBeUndefined();expect(dry.plan.counts).toEqual(plan.counts);
  expect(snapshot(fleet.adopter.options.worktree)).toEqual(before);
  expect(snapshot(fleet.store)).toEqual(storeBefore);
  expect(existsSync(at(fleet.adopter,".cache"))).toBe(false);
  expect(sha(readFileSync(at(fleet.adopter,".git/index")))).toBe(indexBefore);expect(git(fleet.adopter,"write-tree")).toBe(treeBefore);
  expect(await listOriginsV1(fleet.store)).not.toContain("g1-m02");
 }finally{for(const who of [fleet.server,fleet.worker,fleet.adopter])await shut(who);}
},180_000);

for(const step of ["planned","saved","objects","files","applied","published","staged"] as const){
 posixTest(`an adoption that crashes after the step ${step} continues from its marker and ends in the same converged state`,async()=>{
  const fleet=await build();
  try{
   await shut(fleet.adopter);
   let crashed=false;
   await expect(run(fleet.adopter,{fault:reached=>{if(reached===step&&!crashed){crashed=true;throw Error(`[DEBUG] injected crash after ${reached}`);}}})).rejects.toThrow("injected crash");
   expect(await readAdoptStepV1(fleet.adopter.options.worktree)).toBe(step);
   const result=await run(fleet.adopter);
   expect(result.queued).toBe(0);
   expect(await readAdoptStepV1(fleet.adopter.options.worktree)).toBe("reported");
   await verifyAdoption(fleet,result.report!);
  }finally{for(const who of [fleet.server,fleet.worker,fleet.adopter])await shut(who);}
 },180_000);
}

posixTest("a repository without a common commit is refused before anything visible changes unless forced, a forced one decides by content (a version unchanged since its own base is stale), and a worktree that has joined already is refused",async()=>{
 const fleet=await build({unrelated:true});
 try{
  await shut(fleet.adopter);
  const before=readFileSync(at(fleet.adopter,"a.txt"),"utf8"),tip=git(fleet.adopter,"rev-parse","HEAD");
  const plan=await planAdoptV1(fleet.adopter.options,{sample:3});
  expect(plan.decisions.find(item=>item.path==="z.txt")).toMatchObject({category:"stale",reason:"base"});
  expect(plan.decisions.find(item=>item.path==="b.txt")).toMatchObject({category:"stale",reason:"journal"});
  await expect(run(fleet.adopter)).rejects.toBeInstanceOf(AdoptRefusedV1);
  expect(readFileSync(at(fleet.adopter,"a.txt"),"utf8")).toBe(before);
  expect(readFileSync(at(fleet.adopter,"z.txt"),"utf8")).toBe("zulu old\n");
  expect(git(fleet.adopter,"rev-parse","HEAD")).toBe(tip);
  const forced=await run(fleet.adopter,{force:true});
  expect(forced.queued).toBe(0);
  expect(read(fleet.adopter,"z.txt")).toBe("zulu new\n");
  expect(forced.report!.stale.find(item=>item.path==="z.txt")!.reason).toBe("base");
  const subjects=git(fleet.adopter,"log","--format=%s","main").trim().split("\n");
  expect(subjects).toContain("unrelated root");expect(subjects).toContain("initial");expect(subjects).toContain("fleet commit");
  await shut(fleet.worker);
  await expect(run(fleet.worker)).rejects.toBeInstanceOf(AdoptRefusedV1);
 }finally{for(const who of [fleet.server,fleet.worker,fleet.adopter])await shut(who);}
},180_000);

const COMPOSED="präsentation",UMLAUT="übung",decomposed=(path:string):string=>path.normalize("NFD"),executable=(who:Machine,path:string):boolean=>(statSync(at(who,path)).mode&0o111)!==0;
const spellings=(directory:string):string[]=>readdirSync(directory,{recursive:true,encoding:"utf8"}).filter(name=>!name.startsWith(".git")&&!name.startsWith(".cache")).map(name=>name.split(sep).join("/")).sort();

async function extracted(){
 const store=temp("store"),server=machine("g1-server",store),adopter=machine("g1-m02",store),named=[[`${COMPOSED}/a.txt`,"alpha\n"],[`${COMPOSED}/${UMLAUT}/b.txt`,"bravo\n"],[`${UMLAUT}.txt`,"top\n"]] as const;
 write(server,".gitignore",replicaFixture.gitignore);write(server,"plain.txt","plain\n");
 for(const [path,text] of named)write(server,path,text);
 for(const path of ["tool.sh","mine.sh"])write(server,path,`#!/bin/sh\necho ${path}\n`);
 chmodSync(at(server,"tool.sh"),0o755);
 git(server,"init","-q","-b","main");git(server,"add","-A");git(server,"commit","-q","-m","initial");
 write(server,"loose.sh","#!/bin/sh\necho loose\n");
 await seedV1(server.options);
 cpSync(server.options.worktree,adopter.options.worktree,{recursive:true,filter:source=>{const inner=inside(server,source).normalize("NFC");return !inner.startsWith(`${sep}.cache`)&&!inner.includes(COMPOSED)&&!inner.includes(UMLAUT);}});
 for(const [path,text] of named)write(adopter,decomposed(path),text);
 chmodSync(at(adopter,"tool.sh"),0o755);chmodSync(at(adopter,"mine.sh"),0o755);chmodSync(at(adopter,"loose.sh"),0o755);
 chmodSync(at(server,"tool.sh"),0o644);
 await open(server);
 try{await publishOnceV1(server.replica!);}finally{await shut(server);}
 return {store,server,adopter,kept:readdirSync(adopter.options.worktree).includes(decomposed(COMPOSED)),renamed:[COMPOSED,`${COMPOSED}/${UMLAUT}`,`${UMLAUT}.txt`].sort(),paths:named.map(([path])=>path)};
}

posixTest("the dry run of a tree as macOS tar extracts it reads every decomposed name as its NFC name, counts the executable bit as content, and changes nothing",async()=>{
 const fleet=await extracted();
 if(!fleet.kept)return;
 const before=snapshot(fleet.adopter.options.worktree),storeBefore=snapshot(fleet.store);
 const plan=await planAdoptV1(fleet.adopter.options,{sample:1000});
 expect(plan.renamed).toEqual(fleet.renamed);
 expect(plan.samples.same).toEqual(expect.arrayContaining([...fleet.paths,"plain.txt"]));
 for(const category of ["materialise","publish"] as const)expect(plan.samples[category].filter(path=>!path.startsWith(".git/"))).toEqual([]);
 expect(plan.decisions.map(item=>[item.path,item.category,item.reason,item.modeOnly])).toEqual([["loose.sh","conflict","both-changed",true],["mine.sh","edit","fleet-unchanged",true],["tool.sh","stale","journal",true]]);
 expect(plan.decisions.every(item=>item.localSha256===item.fleetSha256)).toBe(true);
 expect(snapshot(fleet.adopter.options.worktree)).toEqual(before);expect(snapshot(fleet.store)).toEqual(storeBefore);
 expect(executable(fleet.adopter,"tool.sh")).toBe(true);
 expect(spellings(fleet.adopter.options.worktree)).toEqual(expect.arrayContaining(fleet.paths.map(decomposed)));
},180_000);

posixTest("adopting a tree as macOS tar extracts it renames every decomposed name to NFC before it plans, never adopts a differing executable bit silently (the bit the fleet cleared loses, the bit set here is published, the undecidable one is a conflict) and publishes no extracted file again",async()=>{
 const fleet=await extracted(),{server,adopter}=fleet;
 if(!fleet.kept)return;
 try{
  const result=await run(adopter),report=result.report!,lost=(path:string):string=>join(adopter.options.worktree,".cache","f",path);
  expect(spellings(adopter.options.worktree).filter(name=>name!==name.normalize("NFC"))).toEqual([]);
  expect(spellings(adopter.options.worktree)).toEqual(expect.arrayContaining(fleet.paths));
  expect(result.plan.renamed).toEqual(fleet.renamed);expect(report.renamed).toEqual(fleet.renamed);
  expect(report.issues.filter(issue=>issue.includes("renamed-to-nfc"))).toEqual([]);
  expect(report.published).toEqual([]);expect(report.edited).toEqual(["mine.sh"]);
  expect(report.stale.map(item=>[item.path,item.reason])).toEqual([["tool.sh","journal"]]);
  expect(report.conflicts.map(item=>[item.path,item.reason,item.local===item.fleet])).toEqual([["loose.sh","both-changed",true]]);
  expect(existsSync(lost(report.stale[0]!.lost))).toBe(true);expect(existsSync(lost(report.conflicts[0]!.lost))).toBe(true);
  expect([executable(adopter,"tool.sh"),executable(adopter,"mine.sh"),executable(adopter,"loose.sh")]).toEqual([false,true,false]);
  expect(compileJsonSchemaSubset({$ref:"#/definitions/Report"},schema).validate(report)).toEqual([]);
  if(python){
   const file=join(temp("report"),"report.json");writeFileSync(file,JSON.stringify(report));
   const check=Bun.spawnSync([python,"-I",oracle,"report",file],{stdout:"pipe",stderr:"pipe"});
   expect(check.stdout.toString()+check.stderr.toString()).toBe("");
  }
  await open(server);await open(adopter);
  await converge([server,adopter],()=>same([server,adopter],who=>who.replica!.files.manifestHex())&&executable(server,"mine.sh"));
  expect([executable(server,"tool.sh"),executable(server,"mine.sh"),executable(server,"loose.sh")]).toEqual([false,true,false]);
  const segments=join(storePathsV1(fleet.store).origins,"g1-m02","segments"),published=readdirSync(segments).flatMap(name=>readFileSync(join(segments,name),"utf8").split("\n").filter(line=>line!=="").map(line=>(JSON.parse(line) as {path:string}).path)).filter(path=>!path.startsWith(".git/"));
  expect([...new Set(published)]).toEqual(["mine.sh"]);
  expect((await verifyV1(adopter.options)).drift).toEqual([]);
  for(const who of [server,adopter])expect(who.events.filter(event=>event.event==="error")).toEqual([]);
 }finally{for(const who of [server,adopter])await shut(who);}
},180_000);

function random(seed:number):()=>number{
 let state=seed>>>0;
 return ()=>{state=(state+0x6D2B79F5)>>>0;let value=Math.imul(state^(state>>>15),1|state);value=(value+Math.imul(value^(value>>>7),61|value))^value;return ((value^(value>>>14))>>>0)/4294967296;};
}

function lostBytes(root:string):Set<string>{
 const found=new Set<string>(),walk=(directory:string):void=>{
  for(const item of readdirSync(directory,{withFileTypes:true})){
   const path=join(directory,item.name);
   if(item.isDirectory())walk(path);else found.add(readFileSync(path,"utf8"));
  }
 };
 if(existsSync(root))walk(root);
 return found;
}

for(const seed of [1,2,3,4,5,6,7,8,9,10,11,12]){
 posixTest(`property, seed ${seed}: adopting an old snapshot of the fleet with random local edits and random modification times never changes a fleet path the repository did not edit and never loses a local edit`,async()=>{
  const draw=random(seed),store=temp("store"),server=machine("g1-server",store),adopter=machine("g1-m02",store),follower=machine("g1-m03",store),names=Array.from({length:24},(_,index)=>`${index%3===0?"dir/":""}f${index}.txt`);
  const text=(label:string):string=>`${label} ${seed} ${draw().toString(16)}\n`;
  write(server,".gitignore",replicaFixture.gitignore);
  for(const name of names.slice(0,18))write(server,name,text("first"));
  await seedV1(server.options);await open(server);
  const steps=2+Math.floor(draw()*4),snapshotAfter=Math.floor(draw()*steps);
  let copied=false;
  const copy=():void=>{
   mkdirSync(adopter.options.worktree,{recursive:true});
   cpSync(server.options.worktree,adopter.options.worktree,{recursive:true,filter:source=>!inside(server,source).includes(`${sep}.cache`)});
   git(adopter,"init","-q","-b","main");git(adopter,"add","-A");git(adopter,"commit","-q","-m","snapshot");
   copied=true;
  };
  if(snapshotAfter===0)copy();
  for(let step=1;step<=steps;step++){
   for(const name of names){
    const roll=draw();
    if(roll<0.2)write(server,name,text(`fleet ${step}`));
    else if(roll<0.28&&existsSync(at(server,name)))rmSync(at(server,name));
    else if(roll<0.34&&existsSync(at(server,name))){const old=read(server,name);write(server,name,text(`fleet ${step}`));if(draw()<0.5)write(server,name,old);}
   }
   await publishOnceV1(server.replica!);
   if(step===snapshotAfter&&!copied)copy();
  }
  if(!copied)copy();
  const snapshotOf=(who:Machine,name:string):string|null=>existsSync(at(who,name))?read(who,name):null;
  const fleetNow=new Map(names.map(name=>[name,snapshotOf(server,name)])),base=new Map(names.map(name=>[name,snapshotOf(adopter,name)])),edits=new Map<string,string>(),now=Date.now();
  for(const name of names){
   const roll=draw(),when=now+(draw()<0.5?-1:1)*Math.floor(draw()*60)*24*HOUR;
   if(roll<0.25){const value=text("local");write(adopter,name,value,when);edits.set(name,value);}
   else if(roll<0.35&&existsSync(at(adopter,name)))rmSync(at(adopter,name));
   else if(existsSync(at(adopter,name))){const file=at(adopter,name);utimesSync(file,when/1000,when/1000);}
  }
  for(let index=0;index<3;index++){const name=`new${index}.txt`,value=text("new");write(adopter,name,value,now+Math.floor(draw()*20-10)*HOUR);edits.set(name,value);if(draw()<0.4&&!fleetNow.has(name)){write(server,name,text("fleet new"));await publishOnceV1(server.replica!);fleetNow.set(name,read(server,name));}else fleetNow.set(name,null);}
  await shut(server);
  const result=await run(adopter);
  expect(result.queued).toBe(0);
  await open(server);await open(adopter);
  await joinV1(follower.options);
  await converge([server,adopter],()=>same([server,adopter],who=>who.replica!.files.manifestHex()));
  const all=[...names,"new0.txt","new1.txt","new2.txt"],lost=lostBytes(join(adopter.options.worktree,".cache","f","lost"));
  const after=(name:string):string|null=>snapshotOf(server,name);
  try{
   for(const name of all){
    const edit=edits.get(name),fleet=fleetNow.get(name)??null,old=base.get(name)??null;
    if(edit===undefined){expect([name,after(name)]).toEqual([name,fleet]);continue;}
    if(fleet===old||old===null&&fleet===null)expect([name,after(name)]).toEqual([name,edit]);
    else{expect([name,after(name)]).toEqual([name,fleet]);expect(lost.has(edit)).toBe(true);}
   }
   for(const [name,edit] of edits)expect(after(name)===edit||lost.has(edit)).toBe(true);
  }finally{await shut(follower);}
 },180_000);
}
