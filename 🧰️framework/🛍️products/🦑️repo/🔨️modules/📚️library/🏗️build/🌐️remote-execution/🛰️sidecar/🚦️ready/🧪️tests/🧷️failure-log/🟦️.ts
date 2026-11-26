import{afterAll,expect,test}from"bun:test";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawn,spawnSync}from"node:child_process";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import boundary from"./🔣️.json"with{type:"json"};
import{READY_STAGES_V1 as STAGES,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,runReadyStageV1,runReadyV1,type ReadyLocalStateV1,type ReadyPortsV1}from"../../🟦️.ts";

/** 🧷️ A stage that fails after a supervisor restart leaves its whole story: the stage writes its own log file, a second supervisor adopts the running stage, and the failure text reaches the log mirror and `ready.json` although a flood of machine rows follows it; a stage ended by a signal says which one; the log of the failed try stays on the machine. */
const scratch:string[]=[],alive:number[]=[];afterAll(()=>{for(const pid of alive)try{process.kill(pid,"SIGKILL");}catch{}for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`failure-${label}-`));scratch.push(path);return path;};
const posix=process.platform==="win32"?test.skip:test;
const fakeHome=(script:string)=>{const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const stageOf=(name:string)=>({...STAGES.find(stage=>stage.name===name)!,timeoutMilliseconds:600000});
const live=process.env.R186_MODULE??join(import.meta.dir,"..","..","🟦️.ts"),preload=process.env.R186_PRELOAD;
const rows=(count:number,from:number)=>`i=${from}; while [ $i -lt ${from+count} ]; do i=$((i+1)); echo '[script] {"owner":"repository:build","command":"cargo:build","stage":"running","activity":{"resource":"cargo:artifact-plan:{\\"bytes\\":'$i',\\"units\\":756}","mode":"shared","elapsedMs":4681.4}}'; done`;
const FAILING=`#!/bin/sh\necho "stage starts"\n${rows(1500,0)}\necho "waiting for the build"\nsleep 4\necho "error: task norm-din4108-rs:build failed (KNOWN-TEXT-ONE)"\necho "error[E0599]: no method named KNOWN-TEXT-TWO"\n${rows(600,2000)}\necho "[repository:command] still running after 12 s"\necho "[repo] still working after 40 s; last phase started: source.served"\nexit 1\n`;
const SUPERVISOR=`const[mod,root,home]=process.argv.slice(2);const{runReadyStageV1,READY_STAGES_V1}=await import(mod);await runReadyStageV1({home,root,environment:{},stage:{...READY_STAGES_V1.find(stage=>stage.name==="workspace"),timeoutMilliseconds:600000},signal:new AbortController().signal,jobs:1,onLine:()=>{}});`;
const until=async(check:()=>boolean,milliseconds:number):Promise<boolean>=>{for(let waited=0;waited<milliseconds;waited+=50){if(check())return true;await new Promise<void>(done=>setTimeout(done,50));}return false;};
const world=(adopt:(options:Parameters<ReadyPortsV1["run"]>[1])=>ReturnType<typeof runReadyStageV1>)=>{
 const shared={state:emptyReadyStateV1() as ReadyLocalStateV1,now:1_000_000},controller=new AbortController(),mirrors=new Map<string,string[]>(),directory=temp("lease");
 const digest=readyDigestsV1({platform:"linux",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)});
 const ports:ReadyPortsV1={
  server:false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{shared.now+=milliseconds;await new Promise<void>(done=>setTimeout(done,1));if(shared.state.failure!==null)controller.abort();if(signal.aborted)return;},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>false,write:()=>{}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory,owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  run:async(stage,options)=>stage.name==="workspace"?adopt(options):{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:1,tail:[]},
  status:{update:()=>{}},say:()=>{},
  mirror:(stage,lines)=>{mirrors.set(stage,[...lines]);},
 };
 return{ports,controller,mirrors,shared};
};

posix("a stage adopted by a second supervisor after the first was killed fails with its whole text in the log mirror and in ready.json, the flood of machine rows after it is no cover",async()=>{
 const root=temp("root"),home=fakeHome(FAILING),file=join(temp("supervisor"),"supervisor.mjs");writeFileSync(file,SUPERVISOR);
 const first=spawn(process.execPath,[...preload===undefined?[]:["--preload",preload],file,live,root,home],{stdio:"ignore",env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}});alive.push(first.pid!);
 const directory=join(root,".cache","f","ready"),log=join(directory,"workspace.log"),running=join(directory,"running.json");
 expect(await until(()=>existsSync(running)&&existsSync(log)&&readFileSync(log,"utf8").includes("waiting for the build"),20000),"the stage runs under the first supervisor").toBe(true);
 const record=JSON.parse(readFileSync(running,"utf8")) as {supervisor?:number};
 expect(validateJsonSchemaSubset(schema.definitions.Inflight,record,schema),"running.json follows its schema").toEqual([]);
 expect(record.supervisor).toBe(first.pid);
 process.kill(first.pid!,"SIGKILL");expect(await until(()=>{try{process.kill(first.pid!,0);return false;}catch{return true;}},5000)).toBe(true);
 const w=world(options=>runReadyStageV1({home,root,environment:{},stage:stageOf("workspace"),...options}));
 await runReadyV1(w.ports,w.controller.signal);
 const failure=w.shared.state.failure!;
 expect(failure.stage).toBe("workspace");expect(failure.exit).toBe(1);
 expect(failure.tail.join("\n"),"ready.json tail").toContain("KNOWN-TEXT-ONE");expect(failure.tail.join("\n")).toContain("KNOWN-TEXT-TWO");
 expect(failure.tail.some(line=>line.startsWith("[script] {")),"no machine row in the tail").toBe(false);
 const mirror=w.mirrors.get("workspace-failed")!;
 expect(mirror.join("\n")).toContain("KNOWN-TEXT-ONE");expect(mirror.join("\n")).toContain("KNOWN-TEXT-TWO");
 expect(mirror.some(line=>line.startsWith("[script] {"))).toBe(false);expect(mirror.length).toBeLessThanOrEqual(200);
 expect(mirror[0]).toContain(`supervisor ${first.pid}`);expect(mirror[0]).toContain(`supervisor ${process.pid}`);expect(mirror[0]).toContain(" / ");
 expect(readFileSync(log,"utf8").split("\n").length,"the stage log itself holds everything").toBeGreaterThan(2000);
},90000);

posix("a stage ended by a signal says which one in both languages, and the log of the failed try stays next to the log of the next",async()=>{
 const killed=fakeHome(`#!/bin/sh\necho "before the kill"\nkill -9 $$\n`),root=temp("root"),directory=join(root,".cache","f","ready");
 const result=await runReadyStageV1({home:killed,root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>{}});
 expect(result.exit).toBeNull();expect(result.tail.at(-1)).toContain("SIGKILL");expect(result.tail.at(-1)).toContain(" / ");expect(result.tail.at(-1)).toContain("Signal");
 const next=fakeHome(`#!/bin/sh\necho "second try"\nexit 0\n`);
 const second=await runReadyStageV1({home:next,root,environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>{}});
 expect(second.exit).toBe(0);
 expect(readFileSync(join(directory,"workspace.previous.log"),"utf8")).toContain("before the kill");expect(readFileSync(join(directory,"workspace.log"),"utf8")).not.toContain("before the kill");
},60000);

posix("every failed Nx task's own output tail reaches the failure log and ready.json although a flood of rows and the rerun summary follow it, and equals the independent Python cut",async()=>{
 const tasks=["@semio-tech/ui-styling-tokens:generate:prototype","@semio-tech/framework-actor-rs:typegen:prototype"],body=(task:string)=>Array.from({length:80},(_,index)=>`\u001b[2mdetail ${task} ${index}\u001b[0m`).concat(`\u001b[31merror\u001b[0m: DEEP-CAUSE ${task}`),script=["#!/bin/sh",...tasks.flatMap(task=>[`echo '> nx run ${task}'`,...body(task).map(line=>`printf '%s\\n' '${line}'`),"echo ''"]),rows(3000,0),"echo ' NX   Running target generate failed'","echo 'Failed tasks:'",...tasks.map(task=>`echo '- ${task}'`),"echo 'Output of 7 successful tasks were not shown.'","exit 1",""].join("\n"),home=fakeHome(script);
 const result=await runReadyStageV1({home,root:temp("tails"),environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>{}});
 expect(result.exit).toBe(1);
 const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
 for(const task of tasks){
  const own=body(task).map(line=>line.replace(/\u001b\[[0-9;]*[A-Za-z]/gu,"")).slice(-60);
  for(const lines of [result.failure!,result.tail]){const at=lines.findIndex(line=>line.startsWith(`[nx] failed task ${task}`));expect(at,task).toBeGreaterThanOrEqual(0);expect(lines.slice(at+1,at+1+(lines===result.failure?60:10))).toEqual(own.slice(-(lines===result.failure?60:10)));}
  if(python){const oracle=spawnSync(python,["-I","-c","import sys,re;t=sys.argv[1];rows=[re.sub(r'\\x1b\\[[0-9;]*[A-Za-z]','',l) for l in sys.stdin.read().split(chr(10))];print(chr(10).join([l for l in rows if l.strip() and not l.startswith('[')][-60:]))",task],{input:body(task).join("\n"),encoding:"utf8"});expect(oracle.stdout.trimEnd().split("\n")).toEqual(own);}
 }
},60000);

test("the Nx child that ends by a signal is said, not reported as a plain exit 1 (source shape)",()=>{
 const source=readFileSync(process.env.R186_SCRIPT??join(import.meta.dir,"..","..","..","..","..","..","⚡️caching","🚀️bootstrap","📜️script.ts"),"utf8");
 expect(source).toContain("function nxCloseCodeV1(");expect(source.match(/nxCloseCodeV1\(/gu)!.length).toBeGreaterThanOrEqual(5);
 expect(source).not.toContain('"close", value => accept(value ?? 1)');
});

test("the exit record of the wrapper and the running record follow their schemas",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Exit,{exit:null,signal:"SIGKILL",spawnFailed:false,endedMs:1},schema)).toEqual([]);
 expect(validateJsonSchemaSubset(schema.definitions.Exit,{exit:1,signal:null,spawnFailed:false,endedMs:1},schema)).toEqual([]);
 expect(validateJsonSchemaSubset(schema.definitions.Exit,{exit:"1",signal:null,spawnFailed:false,endedMs:1},schema).length).toBeGreaterThan(0);
});

for(const [killed,ending] of [["$$","SIGKILL"],["$PPID","left no exit file"]] as const) posix(`an early tools refusal survives later output and ${killed==="$$"?"a killed command":"a lost wrapper"}`,async()=>{
 const evidence='[tools:setup-refusal] {"id":"node","phase":"verify","status":"failed","pin":"24","found":null,"location":null,"primary":"known early tool refusal"}';
 const home=fakeHome(`#!/bin/sh\nprintf '%s\n' '${evidence}'\n${rows(10000,0)}\nkill -9 ${killed}\n`);
 const result=await runReadyStageV1({home,root:temp("early"),environment:{},stage:stageOf("tools"),signal:new AbortController().signal,jobs:1,onLine:()=>{}});
 expect(result.exit).toBeNull();
 expect(result.tail).toContain(evidence);
 expect(result.failure?.filter(line=>line===evidence)).toEqual([evidence]);
 expect(result.tail.at(-1)).toContain(ending);
},60000);

posix("a partial leading heartbeat cannot displace the complete current failure tail",async()=>{
 const {heartbeat,latest,repeat}=boundary;
 const home=fakeHome(`#!/bin/sh\ni=0; while [ $i -lt ${repeat} ]; do echo "${heartbeat}"; i=$((i+1)); done\necho "${latest}"\nexit 1\n`);
 const result=await runReadyStageV1({home,root:temp("heartbeat-boundary"),environment:{},stage:stageOf("workspace"),signal:new AbortController().signal,jobs:1,onLine:()=>{}});
 expect(result.exit).toBe(1);
 expect(result.tail.at(-1)).toBe(latest);
 expect(result.failure?.at(-1)).toBe(latest);
 expect(result.tail.every(line=>line.startsWith("Reading the project graph slice ("))).toBe(true);
},60000);
