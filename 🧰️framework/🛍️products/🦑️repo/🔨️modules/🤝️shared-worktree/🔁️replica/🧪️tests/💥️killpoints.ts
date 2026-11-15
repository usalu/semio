import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {installKillPointsV1,KillPointV1} from "../../🛡️resilience/🧪️tests/💥️killpoints.ts";

setDefaultTimeout(280_000);
const kill=await installKillPointsV1();
const {appendEventsV1,readEventSpansV1,readHeadV1,formatHlcV1}=await import("../../📒️journal/🟦️.ts");
const {closeReplicaV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,seedV1,verifyV1}=await import("../🟦️.ts");
type ReplicaOptionsV1=import("../🟦️.ts").ReplicaOptionsV1;
type ReplicaV1=import("../🟦️.ts").ReplicaV1;

const scratch:string[]=[],fast={pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,logFiles:[] as string[]};
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`kill-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
let tick=1_700_000_000;
type Who={name:string;options:ReplicaOptionsV1;replica?:ReplicaV1};
const at=(who:Who,path:string):string=>join(who.options.worktree,...path.split("/"));
function write(who:Who,path:string,text:string):void{const file=at(who,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);utimesSync(file,tick,tick);tick++;}
const walk=(directory:string):string[]=>existsSync(directory)?readdirSync(directory,{withFileTypes:true}).flatMap(item=>item.isDirectory()?walk(join(directory,item.name)):[join(directory,item.name)]):[];
function shared(who:Who):Record<string,string>{
 const root=who.options.worktree,rows:Record<string,string>={};
 for(const file of walk(root)){const path=file.slice(root.length+1);if(path.startsWith(".cache")||path===".git"||path.startsWith(".git/"))continue;rows[path]=lstatSync(file).isSymbolicLink()?"link":readFileSync(file,"utf8");}
 return rows;
}
const open=async(who:Who):Promise<void>=>{who.replica=await openReplicaV1(who.options);};
async function die(who:Who):Promise<void>{
 who.replica=undefined;
 writeFileSync(join(who.options.worktree,".cache","f","daemon.lock"),JSON.stringify({version:1,pid:2147483646,startedMs:1,originId:who.name}));
}
async function settle(all:readonly Who[]):Promise<void>{for(let round=0;round<4;round++){for(const who of all)await publishOnceV1(who.replica!);for(const who of all)await followOnceV1(who.replica!);}}

async function world():Promise<{store:string;a:Who;b:Who}>{
 const store=temp("store"),make=(name:string):Who=>({name,options:{worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",...fast}}),a=make("a"),b=make("b");
 write(a,".gitignore","ignored\n");write(a,"keep.txt","keep\n");write(a,"edit.txt","edit v1\n");write(a,"gone.txt","gone\n");
 await seedV1(a.options);await joinV1(b.options);
 await open(a);await open(b);await settle([a,b]);
 return {store,a,b};
}

async function sweep(label:string,act:(w:{store:string;a:Who;b:Who})=>Promise<void>,prepare:(w:{store:string;a:Who;b:Who})=>void|Promise<void>,expectedAfter:(w:{store:string;a:Who;b:Who})=>Record<string,string>):Promise<number>{
 kill.reset();
 const dry=await world();await prepare(dry);kill.reset();await act(dry);
 const total=kill.count();
 expect(total,`${label} must have mutations`).toBeGreaterThan(3);
 let killed=0;
 for(const mode of ["before","after"] as const)for(let index=0;index<total;index++){
  kill.reset();
  const w=await world();await prepare(w);
  const expected=expectedAfter(w);
  kill.reset();kill.arm(index,mode);
  let died=false;
  try{await act(w);}catch(error){if(!(error instanceof KillPointV1))throw error;died=true;}
  finally{await kill.disarm();}
  if(died)killed++;
  await die(w.a);await die(w.b);
  await open(w.a);await open(w.b);
  await settle([w.a,w.b]);
  const context=`${label} ${mode} ${index} (${kill.last()[index]??"?"})`;
  expect(shared(w.a),context).toEqual(expected);
  expect(shared(w.b),context).toEqual(expected);
  for(const who of [w.a,w.b]){const report=await verifyV1(who.options);expect(report.drift,context).toEqual([]);}
 }
 return killed;
}

test("a kill at any filesystem mutation of a publish round leaves state that converges to the same tree on both machines and verifies clean",async()=>{
 const edit=(w:{a:Who}):void=>{write(w.a,"new/added.txt","added\n");write(w.a,"edit.txt","edit v2\n");rmSync(at(w.a,"gone.txt"));};
 const killed=await sweep("publish",async w=>{await publishOnceV1(w.a.replica!);},edit,w=>({".gitignore":"ignored\n","keep.txt":"keep\n","edit.txt":"edit v2\n","new/added.txt":"added\n"}));
 expect(killed).toBeGreaterThan(10);
});

test("a kill at any mutation of a follow round that applies remote edits leaves state that converges and verifies clean",async()=>{
 const killed=await sweep("follow",async w=>{await followOnceV1(w.b.replica!);},async w=>{write(w.a,"remote/new.txt","from a\n");write(w.a,"edit.txt","edit v3\n");rmSync(at(w.a,"gone.txt"));await publishOnceV1(w.a.replica!);},w=>({".gitignore":"ignored\n","keep.txt":"keep\n","edit.txt":"edit v3\n","remote/new.txt":"from a\n"}));
 expect(killed).toBeGreaterThan(5);
});

test("a kill at any mutation while the state is persisted on close leaves a state that loads or is recovered and converges",async()=>{
 const killed=await sweep("close",async w=>{await publishOnceV1(w.a.replica!);await closeReplicaV1(w.a.replica!);},w=>{write(w.a,"closed/one.txt","one\n");write(w.a,"edit.txt","edit v4\n");},w=>({".gitignore":"ignored\n","keep.txt":"keep\n","edit.txt":"edit v4\n","gone.txt":"gone\n","closed/one.txt":"one\n"}));
 expect(killed).toBeGreaterThan(5);
});

test("a kill at any mutation of a journal append leaves a head that readers trust and an append that can be repeated without a corrupt line",async()=>{
 const store=temp("journal"),origin="a",event=(n:number)=>({version:1 as const,hlc:formatHlcV1({ms:1_700_000_000_000+n,counter:0},origin),operation:"delete" as const,path:`p${n}.txt`,base:null});
 await appendEventsV1(store,origin,[event(1),event(2)]);
 const copy=(from:string,to:string):void=>{for(const file of walk(from)){const target=join(to,file.slice(from.length+1));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,readFileSync(file));}};
 kill.reset();await appendEventsV1(store,origin,[event(3),event(4)]);const total=kill.count();
 expect(total).toBeGreaterThan(3);
 for(const mode of ["before","after"] as const)for(let index=0;index<total;index++){
  const trial=temp("trial");copy(store,trial);
  rmSync(join(trial,"v1","origins",origin,"segments","000000000000.jsonl"),{force:true});
  await appendEventsV1(trial,origin,[event(1),event(2)]);
  kill.reset();kill.arm(index,mode);
  try{await appendEventsV1(trial,origin,[event(3),event(4)]);}catch(error){if(!(error instanceof KillPointV1))throw error;}
  finally{await kill.disarm();}
  const seen=await readEventSpansV1(trial,origin,undefined),paths=seen.events.map(item=>item.path);
  expect(paths.slice(0,2),`${mode} ${index}`).toEqual(["p1.txt","p2.txt"]);
  expect(paths.length===2||paths.length===4,`${mode} ${index}: ${paths.join(",")}`).toBe(true);
  await appendEventsV1(trial,origin,[event(3),event(4)].slice(paths.length-2));
  const again=await readEventSpansV1(trial,origin,undefined);
  expect(again.events.map(item=>item.path),`${mode} ${index}`).toEqual(["p1.txt","p2.txt","p3.txt","p4.txt"]);
  expect((await readHeadV1(trial,origin))!.events).toBe(4);
 }
});
