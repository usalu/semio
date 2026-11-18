import Ajv from "ajv";
import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import treeSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {readyStallTrackerV1,readyParseUnixTableV1,readyParseWindowsTableV1,readyReadTableV1,readyReadTreeV1,readyTaskkillArgumentsV1,readyWriteTreeV1,type ReadyKnownV1,type ReadyProcessV1} from "../../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {LEASE_OWNER_ENVIRONMENT_V1,leaseRegistryRootV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import ready from "../../../🧬️schema/🔣️.json" with {type:"json"};
import {READY_CONTAINMENT_POLICY_V1 as C,readyRegistryRootV1,readyContainmentLineV1,readyContainmentStatusV1,readyHeirsV1,readyKillRecordLineV1,readyKillRecorderV1,readyOrphanPlanV1,readyOwnerTokenV1,readyPathKeyV1,readySharedPidsV1,readyReapOrphansV1,readyRegistryEntriesV1,readySampleDelayV1,readyStopV1,readyStoppedWindowsV1,readySweepRostersV1,type ReadyRegistryEntryV1} from "../🟦️.ts";

/** 🧹️ Contract: every process a stage starts is stopped as one unit and the stop is proven by a second look at the process table; what survives is named; the orphans of a stopped try, found through the lease registry by pid and start time, are stopped before the next stage and while a stage waits on them, and nobody else's process ever is. The fixture is language-agnostic; a Python oracle derives the same verdicts again. */
const scratch:string[]=[],ajv=new Ajv({strict:false}),posix=process.platform==="win32"?test.skip:test;
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`r85-${label}-`));scratch.push(path);return path;};
const wait=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}};
const orphanInput=(row:(typeof fixture.orphan)[number])=>({entries:row.entries as ReadyRegistryEntryV1[],table:row.table as ReadyProcessV1[],windows:row.windows,root:row.root,human:row.human===null?null:row.human as {pid?:number;startedMs:number},self:row.self,stage:row.stage===null?null:row.stage as {root:ReadyKnownV1;known:ReadyKnownV1[]}});

test("the fixture, the policy and the texts are valid: both languages say the same things with the same placeholders",()=>{
 expect(ajv.validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture),JSON.stringify(ajv.errors)).toBe(true);
 expect(Object.keys(schema.definitions.Texts.const.en).sort()).toEqual(Object.keys(schema.definitions.Texts.const.de).sort());
 for(const kind of Object.keys(schema.definitions.Texts.const.en)){
  const slots=(language:"en"|"de")=>[...(schema.definitions.Texts.const[language] as Record<string,string>)[kind]!.matchAll(/\{(\w+)\}/gu)].map(found=>found[1]).sort();
  expect(slots("de"),kind).toEqual(slots("en"));
 }
 expect(()=>new RegExp(C.registryFileName,"u")).not.toThrow();expect(C.stoppedOutcomes).toEqual(["stalled","stage-timeout","interrupted"]);
 expect(C.fastSampleMilliseconds).toBe(1000);expect(C.fastWindowMilliseconds).toBe(60000);expect(C.slowSampleMilliseconds).toBeGreaterThan(1000);expect(C.slowSampleMilliseconds).toBeLessThanOrEqual(10000);
});

test("sample delays, windows of stopped tries, the Windows taskkill arguments and the orphan plans equal the authored fixture",()=>{
 for(const row of fixture.delay)expect(readySampleDelayV1(row.elapsedMs),String(row.elapsedMs)).toBe(row.delayMs);
 for(const row of fixture.window)expect(readyStoppedWindowsV1(row.runs),row.name).toEqual(row.windows);
 for(const row of fixture.taskkill)expect(readyTaskkillArgumentsV1(row.pid),String(row.pid)).toEqual(row.arguments);
 for(const row of fixture.orphan)expect(readyOrphanPlanV1(orphanInput(row)).map(orphan=>orphan.pid),row.name).toEqual(row.orphans);
 for(const row of fixture.heir)expect(readyHeirsV1({table:row.table as ReadyProcessV1[],previous:row.previous as ReadyKnownV1[],sinceMs:row.sinceMs}).map(member=>member.pid),row.name).toEqual(row.heirs);
 for(const row of fixture.owner)expect(readyOwnerTokenV1(row.root,row.stage,row.startedMs),row.stage).toBe(row.token);
 expect(readyPathKeyV1("C:\\Users\\W\\Semio")).toBe("c:/users/w/semio");
 const found=readyOrphanPlanV1(orphanInput(fixture.orphan[0]!))[0]!;
 expect(found).toMatchObject({pid:91672,name:"bun.exe"});expect(found.command).toContain("show projects");expect(found.held).toEqual(["native-fixed-image:C:/x"]);
});

test("the Python oracle derives the same verdicts",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("../../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as {delay:number[];window:unknown[];taskkill:string[][];orphan:number[][];heir:number[][];owner:string[]};
 expect(oracle.delay).toEqual(fixture.delay.map(row=>readySampleDelayV1(row.elapsedMs)));
 expect(oracle.window).toEqual(fixture.window.map(row=>readyStoppedWindowsV1(row.runs)));
 expect(oracle.taskkill).toEqual(fixture.taskkill.map(row=>[...readyTaskkillArgumentsV1(row.pid)]));
 expect(oracle.orphan).toEqual(fixture.orphan.map(row=>readyOrphanPlanV1(orphanInput(row)).map(orphan=>orphan.pid)));
 expect(oracle.owner).toEqual(fixture.owner.map(row=>readyOwnerTokenV1(row.root,row.stage,row.startedMs)));
 expect(oracle.heir).toEqual(fixture.heir.map(row=>readyHeirsV1({table:row.table as ReadyProcessV1[],previous:row.previous as ReadyKnownV1[],sinceMs:row.sinceMs}).map(member=>member.pid)));
});

test("Windows branch: the table row carries the program name, every recorded member is ended with taskkill /PID <pid> /T /F only while its creation time matches, and the stop is proven by a second table",async()=>{
 expect(readyParseWindowsTableV1("812,4,1794000001500,node.exe\r\n900,812,1794000002000\r\nnot a row\r\n")).toEqual([{pid:812,ppid:4,startMs:1794000001500,name:"node.exe"},{pid:900,ppid:812,startMs:1794000002000}]);
 expect(readyParseUnixTableV1("  10     1    05:30 /usr/bin/sleep\n 11 10 1-02:03:04 node\n 12 10 03:04\n",1_000_000_000_000).map(row=>row.name)).toEqual(["sleep","node",undefined]);
 const start=1_794_000_000_000;
 let rows:ReadyProcessV1[]=[{pid:500,ppid:4,startMs:start,name:"wrapper.exe"},{pid:501,ppid:500,startMs:start+100,name:"repo.exe"},{pid:600,ppid:9999,startMs:start+5000,name:"bun.exe"},{pid:601,ppid:9999,startMs:start+9_000_000,name:"unrelated.exe"},{pid:99,ppid:4,startMs:start-1000,name:"self.exe"}];
 const calls:{command:string;args:readonly string[]}[]=[],ports={platform:"win32" as const,self:99,sleep:async()=>undefined,table:async()=>rows,run:async(command:string,args:readonly string[])=>{calls.push({command,args});rows=rows.filter(row=>String(row.pid)!==args[1]);}};
 const result=await readyStopV1({...ports,root:{pid:500,startMs:start},known:[{pid:600,startMs:start+5000},{pid:601,startMs:start+5000}]});
 expect(calls.every(call=>call.command==="taskkill")).toBe(true);
 expect(calls.map(call=>[...call.args]).sort()).toEqual([["/PID","500","/T","/F"],["/PID","501","/T","/F"],["/PID","600","/T","/F"]].sort());
 expect(result.survivors).toEqual([]);expect(result.verified).toBe(true);expect(result.passes).toBe(1);
 expect(result.stopped.map(member=>[member.pid,member.name]).sort()).toEqual([[500,"wrapper.exe"],[501,"repo.exe"],[600,"bun.exe"]]);
 expect(rows.map(row=>row.pid).sort()).toEqual([601,99]);
});

test("Windows branch: a member that refuses to die is ended again on every pass, then reported by name and kept for the next start",async()=>{
 const start=1_794_000_000_000,rows:ReadyProcessV1[]=[{pid:500,ppid:4,startMs:start,name:"wrapper.exe"},{pid:610,ppid:500,startMs:start+10,name:"stubborn.exe"},{pid:611,ppid:500,startMs:start+20,name:"gone-soon.exe"}];
 let live=[...rows],calls=0,sleeps=0;
 const result=await readyStopV1({platform:"win32",self:99,sleep:async()=>{sleeps++;},table:async()=>live,run:async(_command,args)=>{calls++;if(args[1]!=="610")live=live.filter(row=>String(row.pid)!==args[1]);},root:{pid:500,startMs:start}});
 expect(result.survivors).toEqual([{pid:610,startMs:start+10,name:"stubborn.exe"}]);expect(result.passes).toBe(C.stopAttempts);expect(result.verified).toBe(true);
 expect(calls).toBe(3+(C.stopAttempts-1));expect(sleeps).toBe(C.stopAttempts);
 const status=readyContainmentStatusV1({stage:"tools",reaped:[],survivors:result.survivors})!;
 expect(ajv.validate({$ref:"#/definitions/Containment",definitions:schema.definitions},status),JSON.stringify(ajv.errors)).toBe(true);
 expect(status.en).toContain("stubborn.exe (610)");expect(status.en).toContain("tools");expect(status.de).toContain("stubborn.exe (610)");expect(status.de).toContain("Stufe tools");expect(status.de).toContain("erneut gestoppt");
 expect((await readyStopV1({platform:"win32",self:99,sleep:async()=>undefined,table:async()=>[],run:async()=>undefined,root:{pid:500,startMs:start}})).verified,"an unreadable table claims nothing").toBe(false);
});

test("the status of a run says what was reaped and what survived, in English and German, and nothing when there is nothing",()=>{
 expect(readyContainmentStatusV1({stage:"tools",reaped:[],survivors:[]})).toBeNull();
 const status=readyContainmentStatusV1({stage:"tools",reaped:[{pid:91672,name:"bun.exe",kind:"registry"}],survivors:[]})!;
 expect(ajv.validate({$ref:"#/definitions/Containment",definitions:schema.definitions},status),JSON.stringify(ajv.errors)).toBe(true);
 expect(status.en).toContain("bun.exe (91672)");expect(status.de).toContain("bun.exe (91672)");expect(status.en).not.toBe(status.de);expect(status.survivors).toEqual([]);
 const line=readyContainmentLineV1("reaped-registry",{pid:91672,name:"bun.exe",stage:"tools",started:"2026-11-16T12:18:49.000Z"});
 expect(line).toContain("Stopped orphan 91672 (bun.exe)");expect(line).toContain(" / Verwaisten Prozess 91672 (bun.exe)");expect(line).not.toContain("{");
});

posix("real processes: a root with a detached grandchild whose parent exited is ended, the second table proves it, and a member that cannot be ended is named by its program",async()=>{
 const child=spawn("sh",["-c","sleep 300 & echo $!; exit 0"],{stdio:["ignore","pipe","ignore"],detached:true});
 const orphan=await new Promise<number>(done=>{let out="";child.stdout.on("data",chunk=>{out+=String(chunk);if(out.includes("\n"))done(Number(out.trim()));});});
 try{
  await wait(300);
  const table=await readyReadTableV1(),row=table.find(each=>each.pid===orphan)!;
  expect(row.name).toBe("sleep");
  const stuck=await readyStopV1({root:{pid:orphan,startMs:row.startMs},kill:()=>undefined,sleep:async()=>undefined});
  expect(stuck.survivors.map(member=>[member.pid,member.name])).toEqual([[orphan,"sleep"]]);expect(stuck.passes).toBe(C.stopAttempts);expect(alive(orphan)).toBe(true);
  const real=await readyStopV1({root:{pid:orphan,startMs:row.startMs},policy:{...C,settleMilliseconds:0}});
  expect(real.survivors).toEqual([]);expect(real.verified).toBe(true);expect(real.stopped.map(member=>member.pid)).toEqual([orphan]);
  for(let waited=0;waited<3000&&alive(orphan);waited+=50)await wait(50);
  expect(alive(orphan)).toBe(false);
  const again=await readyStopV1({root:{pid:orphan,startMs:row.startMs}});
  expect(again.stopped).toEqual([]);expect(again.verified).toBe(true);
 }finally{try{process.kill(orphan,"SIGKILL");}catch{}}
},60000);

posix("real processes: the roster an earlier try left behind is stopped before the next stage, forgotten when clean, and kept with only its survivors when a member cannot be ended",async()=>{
 const directory=temp("roster"),sleeper=spawn("sleep",["300"],{stdio:"ignore",detached:true});sleeper.unref();
 try{
  await wait(200);
  const row=(await readyReadTableV1()).find(each=>each.pid===sleeper.pid)!;
  readyWriteTreeV1(directory,"dependencies",{pid:1_999_999,startMs:1},[{pid:row.pid,startMs:row.startMs,name:"sleep"}],Date.now());
  const said:string[]=[],kept=await readySweepRostersV1({directory,kill:()=>undefined,sleep:async()=>undefined,say:line=>said.push(line)});
  expect(kept.survivors.map(member=>member.pid)).toEqual([sleeper.pid!]);expect(readyReadTreeV1(directory,"dependencies")!.processes.map(member=>member.pid)).toEqual([sleeper.pid!]);expect(alive(sleeper.pid!)).toBe(true);
  const clean=await readySweepRostersV1({directory,say:line=>said.push(line)});
  expect(clean.survivors).toEqual([]);expect(clean.reaped).toEqual([{pid:sleeper.pid!,name:"sleep",kind:"roster"}]);
  expect(readyReadTreeV1(directory,"dependencies")).toBeUndefined();
  for(let waited=0;waited<3000&&alive(sleeper.pid!);waited+=50)await wait(50);
  expect(alive(sleeper.pid!)).toBe(false);expect(said.some(line=>line.includes("Stopped process")&&line.includes(" / Prozess "))).toBe(true);
 }finally{try{process.kill(sleeper.pid!,"SIGKILL");}catch{}}
},60000);

posix("real processes: only the registry orphan that carries the owner token of a stopped try is reaped, by pid and start time; another agent's process, a resident service, another stage's token, a reused pid and a person's window are left alone, and the kill record names the killer",async()=>{
 const registry=temp("registry"),root=temp("checkout"),folder=join(registry,"0123456789abcdef01234567"),directory=temp("ready");mkdirSync(folder,{recursive:true});
 const spawned:number[]=[],started=Date.now(),token=readyOwnerTokenV1(root,"tools",started-30000),other=readyOwnerTokenV1(root,"workspace",started-30000);
 const launch=async():Promise<{pid:number;startMs:number}>=>{
  const child=spawn("sleep",["300"],{stdio:"ignore",detached:true});child.unref();spawned.push(child.pid!);await wait(150);
  const row=(await readyReadTableV1()).find(each=>each.pid===child.pid)!;return{pid:row.pid,startMs:row.startMs};
 };
 const repo="fed0aa7c7c00cedaa8abc3ae7c9a032a8655b5ef9d0d37095feddf0e08b75442";
 const register=(who:{pid:number;startMs:number},options:{owner?:string;command?:string;startMs?:number}={})=>writeFileSync(join(folder,`${who.pid}-${who.startMs}.json`),JSON.stringify({version:1,pid:who.pid,startedAtMs:options.startMs??who.startMs,updatedAtMs:Date.now(),command:options.command??`${repo} show projects`,directory:join(root,".cache","b","l","m","data"),...options.owner===undefined?{}:{owner:options.owner},holds:[{resource:"native-fixed-image:x",mode:"shared",sinceMs:who.startMs,task:1}],waits:[]}));
 try{
  const orphan=await launch(),second=await launch(),agent=await launch(),service=await launch(),elsewhere=await launch(),reused=await launch();
  register(orphan,{owner:token});register(second,{owner:token});register(agent);register(service,{owner:token,command:`${repo} service host run`});register(elsewhere,{owner:other});register(reused,{owner:token,startMs:reused.startMs-3_600_000});
  const windows=[{stage:"tools",startedMs:started-30000,endedMs:Date.now()+1000}],say:string[]=[],explained:string[]=[];
  expect(readySharedPidsV1(readyRegistryEntriesV1(registry),await readyReadTableV1())).toEqual(new Set([service.pid]));
  const none=await readyReapOrphansV1({registryRoot:registry,root,windows:[],human:null,stage:null,stageName:"tools",kind:"registry"});
  expect(none.reaped,"no stopped try, nothing is reaped").toEqual([]);
  const human=await readyReapOrphansV1({registryRoot:registry,root,windows,human:{startedMs:started-1000},stage:null,stageName:"tools",kind:"registry"});
  expect(human.reaped,"inside the working window of a person nothing is reaped").toEqual([]);
  const result=await readyReapOrphansV1({registryRoot:registry,root,windows,human:{startedMs:started-9_000_000},stage:null,stageName:"tools",kind:"registry",directory,say:line=>say.push(line),explain:lines=>explained.push(...lines)});
  expect(result.reaped.map(row=>row.pid).sort((a,b)=>a-b)).toEqual([orphan.pid,second.pid].sort((a,b)=>a-b));
  for(let waited=0;waited<3000&&alive(orphan.pid);waited+=50)await wait(50);
  expect(alive(orphan.pid)).toBe(false);expect(alive(second.pid)).toBe(false);
  for(const pid of[agent.pid,service.pid,elsewhere.pid,reused.pid])expect(alive(pid),String(pid)).toBe(true);
  expect(say.some(line=>line.includes(`Stopped orphan ${orphan.pid} (sleep)`)&&line.includes(" / Verwaisten Prozess ")&&line.includes(token))).toBe(true);
  const record=readFileSync(join(directory,C.killRecordName),"utf8");
  expect(record).toContain(`Process ${orphan.pid} (sleep) received SIGTERM from the ready stage runner`);expect(record).toContain(` / Prozess ${orphan.pid} (sleep) erhielt SIGTERM vom Stufen-Läufer`);expect(record).toContain(token);expect(record).not.toContain("SIGKILL");expect(record).not.toContain(`Process ${agent.pid} `);
  expect(say.some(line=>line.includes("received SIGTERM"))).toBe(true);
  expect(readyRegistryEntriesV1(registry).length).toBe(6);
  expect(explained[0]).toContain("6 entries");expect(explained[0]).toContain("orphan 2");expect(explained[0]).toContain("not-ours 2");expect(explained[0]).toContain("shared 1");expect(explained[0]).toContain("identity 1");expect(explained.some(line=>line.includes(`${reused.pid} identity`)&&line.includes("directory"))).toBe(true);
 }finally{for(const pid of spawned)try{process.kill(pid,"SIGKILL");}catch{}}
},60000);

posix("real processes: a stop asks with SIGTERM and waits, a process that ends by itself is never sent SIGKILL, one that ignores SIGTERM is killed after the grace, and every signal is recorded with its killer",async()=>{
 const directory=temp("grace"),spawned:number[]=[];
 const launch=async(script:string):Promise<{pid:number;startMs:number}>=>{
  const child=spawn("sh",["-c",script],{stdio:"ignore",detached:true});child.unref();spawned.push(child.pid!);await wait(300);
  const row=(await readyReadTableV1()).find(each=>each.pid===child.pid)!;return{pid:row.pid,startMs:row.startMs};
 };
 try{
  const polite=await launch("exec sleep 300"),stubborn=await launch('trap "" TERM; while :; do sleep 1; done');
  const calls:string[]=[],record=readyKillRecorderV1({directory,stage:"tools",owner:"ready:x:tools:1",reason:"orphan",killer:4242});
  const polished=await readyStopV1({root:polite,grace:5000,record:(row,signal)=>{calls.push(`${row.pid}:${signal}`);record(row,signal);}});
  expect(polished.survivors).toEqual([]);expect(calls).toEqual([`${polite.pid}:SIGTERM`]);
  const tough=await readyStopV1({root:stubborn,grace:700,policy:{...C,settleMilliseconds:0},record:(row,signal)=>{calls.push(`${row.pid}:${signal}`);record(row,signal);}});
  expect(tough.survivors).toEqual([]);expect(calls.filter(call=>call.startsWith(`${stubborn.pid}:`)).slice(0,2)).toEqual([`${stubborn.pid}:SIGTERM`,`${stubborn.pid}:SIGKILL`]);
  for(let waited=0;waited<3000&&alive(stubborn.pid);waited+=50)await wait(50);
  expect(alive(stubborn.pid)).toBe(false);
  const text=readFileSync(join(directory,C.killRecordName),"utf8");
  expect(text).toContain(`Process ${stubborn.pid} (sh) received SIGKILL from the ready stage runner of this worker (runner process 4242, stage tools, owner ready:x:tools:1)`);expect(text).toContain("Stufen-Läufer");
 }finally{for(const pid of spawned)try{process.kill(pid,"SIGKILL");}catch{}}
},60000);

posix("real processes: a spared service and everything below it survives a stop of the tree that contains it",async()=>{
 const child=spawn("sh",["-c","sleep 300 & sleep 301 & wait"],{stdio:"ignore",detached:true}),spawned:number[]=[child.pid!];child.unref();
 try{
  await wait(400);
  const rows=await readyReadTableV1(),root=rows.find(each=>each.pid===child.pid)!,below=rows.filter(each=>each.ppid===root.pid);
  expect(below.length).toBe(2);spawned.push(...below.map(each=>each.pid));
  const result=await readyStopV1({root:{pid:root.pid,startMs:root.startMs},spare:()=>new Set([below[0]!.pid]),policy:{...C,settleMilliseconds:0},group:root.pid});
  expect(alive(below[0]!.pid),"the spared child lives, and the group is not signalled").toBe(true);
  expect(result.stopped.map(member=>member.pid).sort()).toEqual([root.pid,below[1]!.pid].sort());
 }finally{for(const pid of spawned)try{process.kill(pid,"SIGKILL");}catch{}}
},60000);

test("an unreadable or missing registry reaps nothing and never throws",async()=>{
 expect(readyRegistryEntriesV1(join(tmpdir(),"r85-does-not-exist"))).toEqual([]);
 const registry=temp("damaged"),folder=join(registry,"aaaa");mkdirSync(folder,{recursive:true});writeFileSync(join(folder,"12-34.json"),"{not json");writeFileSync(join(folder,"directory.json"),"{}");writeFileSync(join(folder,"7-8.json"),JSON.stringify({pid:7,startedAtMs:8,command:"c",directory:"d"}));
 expect(readyRegistryEntriesV1(registry).map(row=>row.pid)).toEqual([7]);
 expect(existsSync(join(folder,"12-34.json"))).toBe(true);expect(readFileSync(join(folder,"directory.json"),"utf8")).toBe("{}");
 expect(treeSchema.definitions.Policy.const.treeName).toBe("tree.json");
});

test("the registry root is the registry's own answer on every platform and the Containment of ready.json is the one of this schema",()=>{
 const home=temp("home");
 for(const platform of["darwin","linux","win32"] as const)for(const environment of[{},{LOCALAPPDATA:"C:\\Users\\w\\AppData\\Local",XDG_CACHE_HOME:"/xdg"},{SEMIO_LEASE_REGISTRY_DIRECTORY:"/override"}])expect(readyRegistryRootV1(environment,platform,home),`${platform} ${JSON.stringify(environment)}`).toBe(leaseRegistryRootV1(environment,platform,home));
 expect(ready.definitions.Containment).toEqual(schema.definitions.Containment);
});

posix("the lease registry writes the owner token of its environment into the entry of a process that holds a lease, and none without one",async()=>{
 expect(LEASE_OWNER_ENVIRONMENT_V1).toBe(C.ownerEnvironment);
 const module=fileURLToPath(new URL("../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts",import.meta.url));
 const script=`const {leaseRegistryV1}=await import(${JSON.stringify(module)});const directory="/x/.cache/b/l/m/data";leaseRegistryV1.attach({snapshot:()=>({holds:[{directory,resource:"r",mode:"exclusive",atMs:Date.now(),task:1}],waits:[]}),cycle:()=>undefined});leaseRegistryV1.changed(directory,{registry:{flushMilliseconds:0,maximumEntryBytes:65536}},true);await new Promise(done=>setTimeout(done,60000));`;
 for(const owner of["ready:abc:tools:1",undefined]){
  const registry=temp("owner"),environment:Record<string,string>={...process.env as Record<string,string>,SEMIO_LEASE_REGISTRY_DIRECTORY:registry};delete environment[C.ownerEnvironment];
  if(owner!==undefined)environment[C.ownerEnvironment]=owner;
  const child=spawn(process.execPath,["-e",script],{env:environment,stdio:"ignore"});
  try{
   let entries=readyRegistryEntriesV1(registry);
   for(let waited=0;entries.length===0&&waited<10000;waited+=100){await wait(100);entries=readyRegistryEntriesV1(registry);}
   expect(entries.length).toBe(1);expect(entries[0]!.owner).toBe(owner);
  }finally{child.kill("SIGKILL");}
 }
},30000);

test("the input staging and verification of a native artifact publish a growing step counter, so the stall rule of a stage sees progress where it saw a standing line (g1-m05 was cut every 10 minutes inside native-artifact-input-stage)",()=>{
 const source=readFileSync(join(import.meta.dir,"..","..","..","..","..","..","..","..","..","..","..","🔨️modules","🏃️process","📦️artifacts","🏗️native-build","💾️storage","📤️publication","🌐️platform","🟦️.ts"),"utf8");
 for(const name of["stage","verification","read"])expect(source,name).not.toMatch(new RegExp(`resource:"native-artifact-input-${name}",`,"u"));
 const sites=source.match(/resource:"native-artifact-input-(?:stage|verification|read)[:"]/gu)??[],stepped=source.match(/resource:"native-artifact-input-(?:stage|verification|read):"\+nativeArtifactInputStepV1\(\)/gu)??[];
 expect(stepped.length,"every staging, verification and read site of an input carries the counter").toBe(sites.length);expect(stepped.length).toBeGreaterThanOrEqual(5);
 const line=(step:number)=>`[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-stage:${step}","mode":"exclusive","elapsedMs":0}}`;
 const tracker=readyStallTrackerV1(),fresh=[1,2,3,4].map(step=>tracker.observe([line(step)]));
 expect(fresh).toEqual([true,true,true,true]);
 const standing=readyStallTrackerV1(),old=(k:number)=>`[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-stage","mode":"exclusive","elapsedMs":${k}}}`;
 expect([1,2,3].map(k=>standing.observe([old(k)]))).toEqual([true,false,false]);
});
