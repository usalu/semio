import Ajv from "ajv";
import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {READY_TREE_POLICY_V1 as P,readyDeadlineV1,readyScaledTreePolicyV1,readyStallRestartV1,readyForgetTreeV1,readyIdentityV1,readyKillPlanV1,readyKillTreeV1,readyLeaseVerdictV1,readyParseUnixTableV1,readyParseWindowsTableV1,readyProcessStartSyncV1,readyProgressKeyV1,readyNormaliseLineV1,readyStallTrackerV1,readyStalledV1,readyReadTableV1,readyReadTreeV1,readyTableCommandV1,readyHolderRestartV1,readyHolderWaitTrackerV1,readySemanticsChangedV1, readySemanticsV1,readyTreeMembersV1,readyWriteTreeV1,readyCpuMillisecondsV1,readyLivenessV1,readyTreeCpuV1,type ReadyProcessV1} from "../🟦️.ts";

/** 🌲️ Contract: a stage that is stopped is stopped with every process below it, including the orphans of exited parents, never a process that only has the same pid; a lease or an in-flight record names `{pid, startMs}` and a reused pid frees it; a stage that moves its counters is not stopped at the soft cap, one that stands still is. The fixture is language-agnostic; a Python oracle derives the same verdicts again. */
const scratch:string[]=[],ajv=new Ajv({strict:false});
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("the fixture and the policy are valid against the schema",()=>{
 expect(ajv.validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 for(const pattern of P.progressPatterns)expect(()=>new RegExp(pattern,"u")).not.toThrow();
});

test("kill plans, lease verdicts, deadlines and progress keys equal the fixture",()=>{
 for(const row of fixture.tree)expect(readyKillPlanV1({table:row.table,root:row.root,known:row.known,self:row.self,spare:new Set((row as {spare?:number[]}).spare??[])}),row.name).toEqual(row.kill);
 for(const row of fixture.cpu){const tree=readyTreeCpuV1(row.rows,row.root,row.extra),before=new Map(Object.entries(row.before).map(([pid,ms])=>[Number(pid),ms]));expect<unknown>(Object.fromEntries(tree),row.name).toEqual(row.tree);expect(readyLivenessV1(before,tree),row.name).toBe(row.alive);}
 expect(["0:01.50","1:02:03","2-03:04:05.50","12:34","x"].map(readyCpuMillisecondsV1)).toEqual([1500,3723000,183845500,754000,null]);
 for(const row of fixture.lease)expect<unknown>(readyLeaseVerdictV1({lease:(row as {lease?:{pid:number;startMs?:number;wallMs:number}|null}).lease,nowMs:row.nowMs,staleMs:row.staleMs,process:(row as {process:"alive"|"gone"|"unknown"}).process,processStartMs:(row as {processStartMs?:number}).processStartMs}),row.name).toEqual(row.expect);
 for(const row of fixture.deadline)expect<string>(readyDeadlineV1(row),row.name).toBe(row.verdict);
 for(const row of fixture.progress)expect(readyProgressKeyV1(row.lines),row.name).toBe(row.key);
});

test("a stage log that only counts seconds shows no progress: normalised lines, counters that move and the stop verdict equal the fixture",()=>{
 for(const row of fixture.stall){const tracker=readyStallTrackerV1();expect(row.rounds.map(lines=>tracker.observe(lines)),row.name).toEqual(row.fresh);}
 expect(readyNormaliseLineV1("Reading the project graph slice (4313 s): verification 3575 of 3575.")).toBe("Reading the project graph slice (#): verification 3575 of 3575.");
 expect(readyNormaliseLineV1("[repo] still working after 1850 s; last phase started: door.script")).toBe("[repo] still working after #; last phase started: door.script");
 expect(readyNormaliseLineV1("2026-11-16T03:14:51.238Z workspace")).toBe("# workspace");
 expect(P.stallStopMilliseconds).toBe(600000);
 expect(readyStalledV1(P.stallStopMilliseconds-1)).toBe(false);expect(readyStalledV1(P.stallStopMilliseconds)).toBe(true);
 const small=readyStallTrackerV1({...P,stallSeenMaximum:3});
 expect(small.observe(["a","b","c"])).toBe(true);expect(small.observe(["a"])).toBe(false);expect(small.observe(["d"])).toBe(true);expect(small.observe(["a"])).toBe(true);
 for(const pattern of P.normalisePatterns)expect(()=>new RegExp(pattern,"u")).not.toThrow();
});

test("the Python oracle derives the same verdicts",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {holder:{waitingMs:number;restart:boolean}[][];stall:boolean[][];tree:number[][];lease:unknown[];deadline:string[];progress:(string|null)[];cpu:{tree:Record<string,number>;alive:boolean}[]};
 expect(oracle.tree).toEqual(fixture.tree.map(row=>[...readyKillPlanV1({table:row.table,root:row.root,known:row.known,self:row.self,spare:new Set((row as {spare?:number[]}).spare??[])})]));
 expect(oracle.cpu).toEqual(fixture.cpu.map(row=>{const tree=readyTreeCpuV1(row.rows,row.root,row.extra);return{tree:Object.fromEntries(tree),alive:readyLivenessV1(new Map(Object.entries(row.before).map(([pid,ms])=>[Number(pid),ms])),tree)};}));
 expect(oracle.lease).toEqual(fixture.lease.map(row=>readyLeaseVerdictV1({lease:(row as {lease?:{pid:number;startMs?:number;wallMs:number}|null}).lease,nowMs:row.nowMs,staleMs:row.staleMs,process:(row as {process:"alive"|"gone"|"unknown"}).process,processStartMs:(row as {processStartMs?:number}).processStartMs})));
 expect(oracle.deadline).toEqual(fixture.deadline.map(row=>readyDeadlineV1(row)));
 expect(oracle.progress).toEqual(fixture.progress.map(row=>readyProgressKeyV1(row.lines)));
 expect(oracle.stall).toEqual(fixture.stall.map(row=>{const tracker=readyStallTrackerV1();return row.rounds.map(lines=>tracker.observe(lines));}));
 expect(oracle.holder).toEqual(fixture.holder.map(row=>{const tracker=readyHolderWaitTrackerV1();return row.rounds.map(round=>{const waitingMs=tracker.observe(round.lines,round.atMs);return{waitingMs,restart:readyHolderRestartV1(waitingMs,round.codeChanged)};});}));
});

test("progress is a change in what is worked on, never elapsed time in any spelling: a resource that only counts elapsedMs is stalled, a growing counter suffix is progress",()=>{
 const line=(resource:string,elapsed:string)=>`[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"${resource}","mode":"shared","elapsedMs":${elapsed}}}`;
 expect(readyNormaliseLineV1(line("dependencies:maintenance-parent-observation","1.191300000064075"))).toBe(readyNormaliseLineV1(line("dependencies:maintenance-parent-observation","1.8012999999336898")));
 expect(readyNormaliseLineV1(line("lease-sweep:resources","44"))).toBe(readyNormaliseLineV1(line("lease-sweep:resources","14")));
 expect(readyNormaliseLineV1(line("javascript-selected-payload:48022","5.5"))).not.toBe(readyNormaliseLineV1(line("javascript-selected-payload:48054","5.5")));
 expect(readyNormaliseLineV1(line("javascript-selected-payload:48022","5.5"))).toContain("javascript-selected-payload:48022");
 const tracker=readyStallTrackerV1(),seen:boolean[]=[];
 for(let round=0;round<40;round++)seen.push(tracker.observe([line("dependencies:maintenance-parent-observation",String(1+Math.random())),`[repo] still working after ${1220+round*30} s; last phase started: source.served`,`Process 48352 (x show projects) holds the dependency store since 2026-11-14T16:34:21.027Z; waiting for it (${1210+round*15} s of at most 21600 s)`]));
 expect(seen[0]).toBe(true);expect(seen.slice(1).every(row=>row===false),"after the first sight nothing is new in 39 further looks").toBe(true);
 expect(tracker.observe([line("javascript-selected-payload:48022","3.1")])).toBe(true);expect(tracker.observe([line("javascript-selected-payload:48022","9.9")])).toBe(false);expect(tracker.observe([line("javascript-selected-payload:48054","9.9")])).toBe(true);
});

test("semantics changed since the stage started: only a declared source whose value moved, never another file, an unreadable source is no change",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r181-semantics-"));scratch.push(root);
 const policy={...P,semanticsSources:[{file:"code/exchange.json",pointer:"/definitions/Policy/const/revision"},{file:"code/record.json",pointer:"/$defs/record/const/stamp"}]};
 const put=(relative:string,text:string)=>{const path=join(root,...relative.split("/"));mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,text);};
 put("code/exchange.json",JSON.stringify({definitions:{Policy:{const:{revision:1,other:1}}}}));put("code/record.json",JSON.stringify({"$defs":{record:{const:{stamp:"semio.graph-record/2"}}}}));
 const before=readySemanticsV1(root,policy);expect(Object.values(before)).toEqual(["1",JSON.stringify("semio.graph-record/2")]);
 put("code/exchange.json",JSON.stringify({definitions:{Policy:{const:{revision:1,other:2,claimTtlMilliseconds:30000}}}}));put("code/other.ts","x");put("code/a/🧪️tests/b.ts","x");
 expect(readySemanticsChangedV1(root,before,policy)).toBeNull();
 put("code/record.json",JSON.stringify({"$defs":{record:{const:{stamp:"semio.graph-record/3"}}}}));expect(readySemanticsChangedV1(root,before,policy)).toBe("code/record.json");
 put("code/exchange.json",JSON.stringify({definitions:{Policy:{const:{revision:2}}}}));expect(readySemanticsChangedV1(root,before,{...policy,semanticsSources:[policy.semanticsSources[0]!]})).toBe("code/exchange.json");
 put("code/exchange.json","{ half a landing");expect(readySemanticsChangedV1(root,before,{...policy,semanticsSources:[policy.semanticsSources[0]!]})).toBeNull();
 expect(readySemanticsChangedV1(join(root,"missing"),before,policy)).toBeNull();expect(readySemanticsChangedV1(root,readySemanticsV1(join(root,"missing"),policy),policy)).toBeNull();
});

test("the declared sources of the policy are the revision of the selected generation exchange and the stamp of the persisted record, both readable and in the checkout",()=>{
 const checkout=resolve(import.meta.dir,"../../../../../../../../../../..");
 expect(P.semanticsSources.length).toBe(2);
 const values=readySemanticsV1(checkout);for(const source of P.semanticsSources){expect(values[source.file+"#"+source.pointer],source.file).not.toBeNull();}
 expect(Object.values(values)[0]).toMatch(/^\d+$/u);expect(Object.values(values)[1]).toMatch(/^"semio\.graph-record\/\d+"$/u);
});

test("a stage that waited ten minutes on a named holder is restarted for changed code and for nothing else",()=>{
 expect(P.holderWaitRestartMilliseconds).toBe(600000);
 expect(readyHolderRestartV1(600000,true)).toBe(true);expect(readyHolderRestartV1(599999,true)).toBe(false);expect(readyHolderRestartV1(9e9,false)).toBe(false);
 for(const pattern of P.holderWaitPatterns)expect(()=>new RegExp(pattern,"u")).not.toThrow();
});

test("identity: the same process, a gone one and a reused pid are told apart",()=>{
 const table=[{pid:10,ppid:1,startMs:5000}];
 expect(readyIdentityV1({pid:10,startMs:5000},table)).toBe("same");expect(readyIdentityV1({pid:10,startMs:5000+P.toleranceMilliseconds},table)).toBe("same");
 expect(readyIdentityV1({pid:10,startMs:5000+P.toleranceMilliseconds+1},table)).toBe("reused");expect(readyIdentityV1({pid:11,startMs:5000},table)).toBe("gone");expect(readyIdentityV1({pid:10,startMs:0},table)).toBe("same");
});

test("the two process table formats parse, the unix one from the real ps of this machine",async()=>{
 expect(readyParseUnixTableV1("  10  1 Sat Nov  7 08:50:07 2026\n 11 10 Sat Nov  7 08:50:08 2026\n  garbage\n 12 10 Sat Nov  7 08:50:09 2026\n")).toEqual([{pid:10,ppid:1,startMs:Date.UTC(2026,10,7,8,50,7)},{pid:11,ppid:10,startMs:Date.UTC(2026,10,7,8,50,8)},{pid:12,ppid:10,startMs:Date.UTC(2026,10,7,8,50,9)}]);
 expect(readyParseWindowsTableV1("4\t0\t1794000000000\t639000000000000000\r\n812\t4\t1794000001500\t639000000015000000\r\nnot a row\r\n")).toEqual([{pid:4,ppid:0,startMs:1794000000000,windows:{birth:"639000000000000000"}},{pid:812,ppid:4,startMs:1794000001500,windows:{birth:"639000000015000000"}}]);
 expect(readyTableCommandV1("win32").command).toBe("powershell.exe");expect(readyTableCommandV1("linux").command).toBe("ps");
 if(process.platform!=="win32"){
  const table=await readyReadTableV1(),me=table.find(row=>row.pid===process.pid);
  expect(table.length).toBeGreaterThan(10);expect(me).toBeDefined();expect(Math.abs(me!.startMs-(Date.now()-process.uptime()*1000))).toBeLessThan(5000);expect(table.find(row=>row.pid===me!.ppid)).toBeDefined();
 }
});

test("the record of a tree is written, read back and forgotten",()=>{
 const directory=mkdtempSync(join(tmpdir(),"tree-"));scratch.push(directory);
 expect(readyReadTreeV1(directory,"dependencies")).toBeUndefined();
 readyWriteTreeV1(directory,"dependencies",{pid:5,startMs:1},[{pid:5,startMs:1},{pid:9,startMs:2}],10);
 expect(ajv.validate({$ref:"#/definitions/Tree",definitions:schema.definitions},JSON.parse(require("node:fs").readFileSync(join(directory,"dependencies.tree.json"),"utf8")))).toBe(true);
 expect(readyReadTreeV1(directory,"dependencies")).toEqual({root:{pid:5,startMs:1},processes:[{pid:5,startMs:1},{pid:9,startMs:2}]});
 readyForgetTreeV1(directory,"dependencies");expect(readyReadTreeV1(directory,"dependencies")).toBeUndefined();
});

test("members of the tree include the recorded orphans that are still there",()=>{
 const table:ReadyProcessV1[]=[{pid:100,ppid:1,startMs:1_000_000},{pid:102,ppid:101,startMs:1_000_300},{pid:103,ppid:102,startMs:1_000_400},{pid:300,ppid:1,startMs:5}];
 expect(readyTreeMembersV1(table,{pid:100,startMs:1_000_000},[{pid:102,startMs:1_000_300}]).map(row=>row.pid).sort()).toEqual([100,102,103]);
});

test("real processes: the grandchild of an exited parent is found by the sample and stopped, a process that only shares the pid is not",async()=>{
 if(process.platform==="win32")return;
 const child=spawn("sh",["-c","sleep 300 & echo $!; exit 0"],{stdio:["ignore","pipe","ignore"],detached:true});
 const orphan=await new Promise<number>(done=>{let out="";child.stdout.on("data",chunk=>{out+=String(chunk);if(out.includes("\n"))done(Number(out.trim()));});});
 try{
  const sampled=await readyReadTableV1(),root={pid:child.pid!,startMs:Date.now()};
  const seen=sampled.find(row=>row.pid===orphan);expect(seen).toBeDefined();
  await new Promise(done=>setTimeout(done,300));
  const later=await readyReadTableV1();
  expect(later.find(row=>row.pid===child.pid)).toBeUndefined();
  const orphanRow=later.find(row=>row.pid===orphan)!;expect(orphanRow.ppid).not.toBe(child.pid);
  const plan=readyKillPlanV1({table:later,root,known:[{pid:orphan,startMs:seen!.startMs}],self:process.pid});
  expect(plan).toEqual([orphan]);
  expect(readyKillPlanV1({table:later,root,known:[{pid:orphan,startMs:seen!.startMs+3_600_000}],self:process.pid})).toEqual([]);
  const stopped=await readyKillTreeV1({root,known:[{pid:orphan,startMs:seen!.startMs}],signal:"SIGKILL",table:later});
  expect(stopped).toEqual([orphan]);
  await new Promise(done=>setTimeout(done,300));
  expect((await readyReadTableV1()).find(row=>row.pid===orphan)).toBeUndefined();
 }finally{try{process.kill(orphan,"SIGKILL");}catch{}}
},30000);

test("the birth of one process matches its runtime clock",()=>{
 if(process.platform==="win32")return;
 const start=readyProcessStartSyncV1(process.pid);expect(start).not.toBeNull();expect(Math.abs(start!-(Date.now()-process.uptime()*1000))).toBeLessThan(5000);
 expect(readyProcessStartSyncV1(2_000_000_000)).toBeNull();expect(readyProcessStartSyncV1(-5)).toBeNull();
});

test("the pause before the restart of a silent stage doubles from 30 s up to 10 min, the sixth stop in a row is a failure, and an independent doubling loop agrees",()=>{
 let pause=P.stallRestartBackoffMilliseconds;const model:number[]=[];
 for(let stop=1;stop<=P.stallRestartMaximum;stop++){model.push(Math.min(P.stallRestartBackoffMaximumMilliseconds,pause));pause*=2;}
 expect(model).toEqual([30000,60000,120000,240000,480000]);
 for(let stop=1;stop<=P.stallRestartMaximum;stop++)expect(readyStallRestartV1({consecutive:stop})).toEqual({action:"restart",waitMilliseconds:model[stop-1]!});
 expect(readyStallRestartV1({consecutive:P.stallRestartMaximum+1})).toEqual({action:"fail",waitMilliseconds:0});
 expect(readyStallRestartV1({consecutive:0}).waitMilliseconds).toBe(30000);
 expect(readyStallRestartV1({consecutive:4,policy:{...P,stallRestartBackoffMaximumMilliseconds:100000}}).waitMilliseconds).toBe(100000);
});

test("the windows without progress are lengthened for a slower machine and nothing else changes",()=>{
 const scaled=readyScaledTreePolicyV1(2.83);
 expect(scaled.stallMilliseconds).toBe(Math.ceil(P.stallMilliseconds*2.83));expect(scaled.stallStopMilliseconds).toBe(1698000);expect(scaled.holderWaitRestartMilliseconds).toBe(1698000);expect(scaled.staleStageMilliseconds).toBe(Math.ceil(P.staleStageMilliseconds*2.83));
 expect(scaled.speedFactor).toBe(2.83);expect({...scaled,speedFactor:1,stallMilliseconds:0,stallStopMilliseconds:0,holderWaitRestartMilliseconds:0,holderSilentMilliseconds:0,holderYieldMilliseconds:0,staleStageMilliseconds:0}).toEqual({...P,stallMilliseconds:0,stallStopMilliseconds:0,holderWaitRestartMilliseconds:0,holderSilentMilliseconds:0,holderYieldMilliseconds:0,staleStageMilliseconds:0});
 expect(readyScaledTreePolicyV1(1)).toEqual(P);expect(readyScaledTreePolicyV1(0.5)).toEqual(P);
});
