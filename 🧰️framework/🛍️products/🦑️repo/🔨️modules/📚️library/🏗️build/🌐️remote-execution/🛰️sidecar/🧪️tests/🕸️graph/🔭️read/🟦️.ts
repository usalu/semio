import{Database}from"bun:sqlite";
import handoff from"./🔣️.json"with{type:"json"};
import Ajv from"ajv";
import{afterAll,expect,test}from"bun:test";
import{appendFileSync,chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readReadRecordV1}from"../../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import schema from"../../../../../🪪️graph-state/🙋️wanted/🧬️schema/🔣️.json"with{type:"json"};
import{queuedGraphRebuildDecisionV1}from"../../../🕸️graph/🔭️read/⏳️wait/📜️script.ts";
import{adoptGraphReadV1,GRAPH_READ_FILES_V1,inferGraphReadV1,liveGraphReadV1,rotateGraphReadLogV1}from"../../../🕸️graph/🔭️read/🟦️.ts";

/** 🔭️ Contract: a graph read is never thrown away for a code change. It runs under a detached runner with a record, a log and a watch file; the process that started it, the build agent or a launcher that restarts for new code may die at any moment, the next process attaches to the same read, tells nothing twice and never starts a second read for the generation; a runner nobody watches stops its read after the orphan time; a read that ended while nobody watched is answered from its record; a read that crashed is not attached to, the next one starts anew. */
const made:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`graph-read-${label}-`));made.push(path);return path;};
afterAll(()=>{for(const path of made)rmSync(path,{recursive:true,force:true});});
const sleep=(milliseconds:number)=>new Promise<void>(done=>{setTimeout(done,milliseconds);});
const until=async(what:string,check:()=>boolean,milliseconds=20000)=>{const started=Date.now();while(!check()){if(Date.now()-started>milliseconds)throw Error("timed out: "+what);await sleep(25);}};
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const fake=(label:string,script:string)=>{const dir=temp(label),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\n"+script+"\n");chmodSync(path,0o755);return{dir,path,starts:join(dir,"starts")};};
const policy={...wantedServerPolicyV1,inferStallMilliseconds:600000,watchMilliseconds:40,readBeatMilliseconds:50,readPollMilliseconds:40};
const wave=(n:number)=>`echo "graph: generation ${n} published, 48 projects refreshed in 80 s (0 kept their edges)" >&2`;
const slowRead=(label:string,waves:number)=>{const repo=fake(label,`echo start >> "$(dirname "$0")/starts"\ni=1\nwhile [ $i -le ${waves} ]; do ${wave(0).replace("generation 0","generation $i")}; i=$((i+1)); sleep 0.3; done\nexit 0`);return repo;};
const input=(repo:{dir:string;path:string},directory:string,over:object={})=>({command:repo.path,cwd:tmpdir(),environment:{},reason:"request" as const,signal:undefined,language:"en" as const,directory,policy,...over});
const count=(file:string)=>existsSync(file)?readFileSync(file,"utf8").split("\n").filter(Boolean).length:0;
const recordOf=(directory:string)=>readReadRecordV1(JSON.parse(readFileSync(join(directory,GRAPH_READ_FILES_V1.record),"utf8")));
const here=import.meta.dir,modulePath=join(here,"..","..","..","🕸️graph","🔭️read","🟦️.ts");

test("the read survives the death of the process that started it (SIGKILL, as a launcher that is replaced does it); the next process attaches, starts no second read and gets the generation of the last wave",async()=>{
 const repo=slowRead("kill",12),directory=temp("kill-dir"),parent=join(temp("kill-parent"),"parent.ts");
 writeFileSync(parent,`import{inferGraphReadV1}from${JSON.stringify(modulePath)};\nawait inferGraphReadV1(${JSON.stringify(input(repo,directory))});\n`);
 const preload=process.env.R5_PRELOAD===undefined?[]:["--preload",process.env.R5_PRELOAD];
 const child=Bun.spawn([process.execPath,...preload,parent],{stdin:"ignore",stdout:"ignore",stderr:"ignore",env:process.env as Record<string,string>});
 await until("the read started",()=>existsSync(join(directory,GRAPH_READ_FILES_V1.record))&&count(repo.starts)===1&&readFileSync(join(directory,GRAPH_READ_FILES_V1.log),"utf8").includes("generation 2 "));
 const before=recordOf(directory)!;child.kill("SIGKILL");await child.exited;
 expect(alive(before.runner),"the runner outlives its parent").toBe(true);expect(alive(before.read!),"the read outlives its parent").toBe(true);
 const lines:string[]=[],started=Date.now(),result=await inferGraphReadV1(input(repo,directory,{say:(line:string)=>lines.push(line)}));
 expect(result).toMatchObject({ok:true,status:0,generation:12,detail:"request"});
 expect(count(repo.starts),"one read for the generation: it was started once and attached to once").toBe(1);
 expect(lines.some(line=>line.includes(`attached to the graph read (pid ${before.read})`))).toBe(true);
 expect(Date.now()-started,"it waited for the read, it did not start one of its own").toBeGreaterThan(1000);
 expect(existsSync(join(directory,GRAPH_READ_FILES_V1.record)),"the answer was given, the record is gone").toBe(false);
},60000);

test("the cancellation of the server lets go of the read and does not stop it; the next process adopts it and the answer is the same as if nothing happened",async()=>{
 const repo=slowRead("detach",8),directory=temp("detach-dir"),controller=new AbortController(),lines:string[]=[];
 const first=inferGraphReadV1(input(repo,directory,{signal:controller.signal,say:(line:string)=>lines.push(line)}));
 await until("the read runs",()=>count(repo.starts)===1&&existsSync(join(directory,GRAPH_READ_FILES_V1.log)));
 controller.abort();const detached=await first;
 expect(detached).toMatchObject({ok:false,detail:"detached"});expect(lines.some(line=>line.includes("running; the next process of the server attaches"))).toBe(true);
 const record=recordOf(directory)!;expect(record.status).toBeNull();expect(alive(record.runner)).toBe(true);
 const adopted=adoptGraphReadV1(input(repo,directory));
 expect(adopted?.reason).toBe("request");
 expect(await adopted!.done).toMatchObject({ok:true,status:0,generation:8});
 expect(count(repo.starts),"no double read").toBe(1);
 expect(adoptGraphReadV1(input(repo,directory)),"nothing left to attach to once the answer was given").toBeUndefined();
},60000);

test("a runner nobody watches stops its read after the orphan time and records the end; a later process is given that result, not a new read",async()=>{
 const repo=fake("orphan",'echo start >> "$(dirname "$0")/starts"\necho "graph: waiting" >&2\nexec sleep 120'),directory=temp("orphan-dir"),controller=new AbortController();
 const first=inferGraphReadV1(input(repo,directory,{signal:controller.signal,policy:{...policy,readOrphanMilliseconds:2500}}));
 await until("the read runs",()=>count(repo.starts)===1&&existsSync(join(directory,GRAPH_READ_FILES_V1.record)));
 controller.abort();await first;
 const record=recordOf(directory)!;
 await until("the orphan runner wrote the terminal status and exited",()=>recordOf(directory)?.status!=null&&!alive(record.runner),30000);
 expect(alive(record.read!),"the read is gone").toBe(false);expect(alive(record.runner),"the runner ended").toBe(false);
 const found=liveGraphReadV1(directory,policy);expect(found?.status,"the result is still wanted").not.toBeNull();
 const adopted=adoptGraphReadV1(input(repo,directory));
 const result=await adopted!.done;
 expect(result.ok,"a stopped read is not a good read").toBe(false);expect(count(repo.starts)).toBe(1);
},60000);

test("a read that ended while no server watched is answered from its record without starting another",async()=>{
 const repo=slowRead("ended",3),directory=temp("ended-dir"),controller=new AbortController();
 const first=inferGraphReadV1(input(repo,directory,{signal:controller.signal}));
 await until("the read runs",()=>count(repo.starts)===1);controller.abort();await first;
 await until("the read ended",()=>recordOf(directory)?.status===0);
 const lines:string[]=[],adopted=adoptGraphReadV1(input(repo,directory,{say:(line:string)=>lines.push(line)}));
 expect(await adopted!.done).toMatchObject({ok:true,status:0,generation:3});
 expect(lines.some(line=>line.includes("ended with status 0 while no server watched"))).toBe(true);
 expect(count(repo.starts)).toBe(1);
},60000);

test("a read that crashed is not attached to: the answer says how it failed and the next read starts anew",async()=>{
 const repo=fake("crash",`echo start >> "$(dirname "$0")/starts"\nif [ "$(wc -l < "$(dirname "$0")/starts")" -le 1 ]; then ${wave(1)}; echo "boom: out of memory" >&2; exit 3; fi\n${wave(2)}\nexit 0`),directory=temp("crash-dir");
 const failed=await inferGraphReadV1(input(repo,directory));
 expect(failed).toMatchObject({ok:false,status:3,generation:1,detail:"boom: out of memory"});
 expect(adoptGraphReadV1(input(repo,directory)),"nothing to attach to").toBeUndefined();
 const next=await inferGraphReadV1(input(repo,directory));
 expect(next).toMatchObject({ok:true,status:0,generation:2});expect(count(repo.starts),"the second read is a second start, once").toBe(2);
},60000);

test("a record of a runner that is gone is a leftover: nobody attaches to it and it is removed; damaged and foreign records are not read",async()=>{
 const directory=temp("stale-dir"),dead=Bun.spawn(["true"]);await dead.exited;
 mkdirSync(directory,{recursive:true});const file=join(directory,GRAPH_READ_FILES_V1.record);
 const row={version:1,reason:"request",runner:dead.pid,read:dead.pid,startedMs:1,beatMs:Date.now(),log:"x",status:null,endedMs:null};
 writeFileSync(file,JSON.stringify(row));expect(liveGraphReadV1(directory,policy)).toBeUndefined();expect(existsSync(file)).toBe(false);
 writeFileSync(file,JSON.stringify({...row,runner:process.pid,beatMs:Date.now()-60_000}));expect(liveGraphReadV1(directory,policy),"a runner that does not beat any more").toBeUndefined();
 writeFileSync(file,"{damaged");expect(liveGraphReadV1(directory,policy)).toBeUndefined();
 writeFileSync(file,JSON.stringify({...row,runner:process.pid}));expect(liveGraphReadV1(directory,policy)?.runner).toBe(process.pid);
 const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/ReadRecord",definitions:schema.definitions});
 expect(validate({...row,status:0,endedMs:5}),JSON.stringify(validate.errors)).toBe(true);expect(validate({...row,version:2})).toBe(false);expect(validate({...row,runner:0})).toBe(false);expect(validate({...row,extra:1})).toBe(false);
});

test("the record the runner writes has the shape of the schema while the read runs and after it ended, and a runner that is told to stop stops its read with the interrupt",async()=>{
 const repo=fake("shape",'echo start >> "$(dirname "$0")/starts"\ntrap \'echo interrupted >> "$(dirname "$0")/starts"; exit 130\' INT TERM\nwhile true; do sleep 0.1; done'),directory=temp("shape-dir"),controller=new AbortController();
 const first=inferGraphReadV1(input(repo,directory,{signal:controller.signal}));
 await until("the read runs",()=>existsSync(join(directory,GRAPH_READ_FILES_V1.record))&&count(repo.starts)===1);controller.abort();await first;
 const running=recordOf(directory)!;expect(running).toMatchObject({version:1,reason:"request",status:null,endedMs:null});expect(running.read).toBeGreaterThan(0);expect(running.log).toBe(join(directory,GRAPH_READ_FILES_V1.log));
 process.kill(running.runner,"SIGTERM");
 await until("the record shows the end",()=>recordOf(directory)?.status!==null&&recordOf(directory)?.status!==undefined);
 const ended=recordOf(directory)!;expect(ended.status).toBe(130);expect(ended.endedMs).toBeGreaterThan(0);expect(readFileSync(repo.starts,"utf8")).toContain("interrupted");
});

test("the read gets the environment of the server and the arguments `show projects`, and its output reaches the log in order",async()=>{
 const repo=fake("env",'echo "args=$* bg=$SEMIO_GRAPH_BACKGROUND fleet=$SEMIO_GRAPH_FLEET_READ steer=$SEMIO_GRAPH_STEER_DIRECTORY mark=$MARK" >&2\necho "second line" >&2\nexit 0'),directory=temp("env-dir"),steer=temp("steer");
 const result=await inferGraphReadV1(input(repo,directory,{environment:{MARK:"m1"},steerDirectory:steer,reason:"refresh"}));
 expect(result).toMatchObject({ok:true,detail:"refresh"});
 const log=readFileSync(join(directory,GRAPH_READ_FILES_V1.log),"utf8");
 expect(log).toBe(`args=show projects bg=1 fleet=off steer=${steer} mark=m1\nsecond line\n`);
});

test("R57 law: a new read never truncates the log of the previous one: the log is renamed by its time and only the newest N stay",()=>{
 const directory=temp("rotate");
 for(let index=1;index<=7;index++)writeFileSync(join(directory,`read.${1000+index}.log`),"old "+index);
 writeFileSync(join(directory,GRAPH_READ_FILES_V1.log),"the evidence of the dead read");
 const moved=rotateGraphReadLogV1(directory,wantedServerPolicyV1.readLogsKept)!;
 expect(existsSync(join(directory,GRAPH_READ_FILES_V1.log))).toBe(false);expect(readFileSync(moved,"utf8")).toBe("the evidence of the dead read");
 const left=require("node:fs").readdirSync(directory).filter((name:string)=>/^read\.\d+\.log$/u.test(name));
 expect(left.length).toBe(wantedServerPolicyV1.readLogsKept);expect(left).toContain(moved.split("/").at(-1)!);expect(left).not.toContain("read.1001.log");
 expect(rotateGraphReadLogV1(directory,wantedServerPolicyV1.readLogsKept)).toBeUndefined();
});

const progressFixture=(label:string)=>{
 const directory=temp(label),startedMs=Date.now(),child=Bun.spawn([process.execPath,"-e","setTimeout(()=>{},60_000)"],{stdin:"ignore",stdout:"ignore",stderr:"ignore",windowsHide:true});
 const log=join(directory,GRAPH_READ_FILES_V1.log),path=join(directory,GRAPH_READ_FILES_V1.record);
 writeFileSync(log,"");
 writeFileSync(path,JSON.stringify({version:1,reason:"request",runner:child.pid,read:child.pid,startedMs,beatMs:startedMs,log,status:null,endedMs:null}));
 return{directory,startedMs,child,log,repo:{dir:directory,path:process.execPath}};
};

test("normal detach carries quiet time and byte ownership across a partial UTF-8 row",async()=>{
 const row=handoff.resume,{directory,startedMs:base,child,log,repo}=progressFixture("resume-dir"),old=new AbortController(),next=new AbortController();
 let at=base;const now=()=>at;
 const graphPolicy={...policy,stallDumpMilliseconds:row.stallDumpAt},quiet:number[]=[];
 const first=inferGraphReadV1(input(repo,directory,{signal:old.signal,now,policy:graphPolicy}));
 let adopted:ReturnType<typeof adoptGraphReadV1>;
 try{
  at=base+row.progressAt;appendFileSync(log,"graph: generation "+row.generation+" published\n");
  await sleep(graphPolicy.readPollMilliseconds*3);
  at=base+row.before;
  const nextRow=Buffer.from("Graph: Generation "+row.nextGeneration+" veröffentlicht\n"),boundary=nextRow.indexOf(0xc3)+1;
  appendFileSync(log,nextRow.subarray(0,boundary));
  await sleep(graphPolicy.readPollMilliseconds*3);
  expect(existsSync(join(directory,GRAPH_READ_FILES_V1.checkpoint))).toBe(false);
  old.abort();
  expect(await first).toMatchObject({detail:"detached",generation:row.generation});
  const saved=JSON.parse(readFileSync(join(directory,GRAPH_READ_FILES_V1.checkpoint),"utf8"));
  const valid=new Ajv({strict:false}).compile({$ref:"#/definitions/ReadWatchCheckpoint",definitions:schema.definitions});
  expect(valid(saved)).toBe(true);
  expect(saved).toMatchObject({runner:child.pid,read:child.pid,startedMs:base,offset:Buffer.byteLength("graph: generation "+row.generation+" published\n"),observation:{progressAt:base+row.progressAt,generation:row.generation}});
  expect(valid({...saved,observation:{...saved.observation,extra:true}})).toBe(false);
  at=base+row.adoptedAt;
  adopted=adoptGraphReadV1(input(repo,directory,{signal:next.signal,now,policy:graphPolicy,diagnose:async({silentMilliseconds}: {silentMilliseconds:number})=>{quiet.push(silentMilliseconds);return [];}}));
  expect(adopted).toBeDefined();
  const sql=new Database(":memory:");
  try{expect(sql.query("select ?1 - ?2 as quiet").get(row.adoptedAt,row.progressAt)).toEqual({quiet:row.silent});}
  finally{sql.close();}
  expect(quiet).toEqual([row.silent]);
  appendFileSync(log,nextRow.subarray(boundary));
  await sleep(graphPolicy.readPollMilliseconds*3);
  next.abort();
  expect(await adopted!.done).toMatchObject({detail:"detached",generation:row.nextGeneration});
 }finally{
  old.abort();next.abort();await first.catch(()=>undefined);await adopted?.done;
  child.kill();await child.exited;
 }
},60000);

test("measurement resolved after detach cannot stop the detached runner",async()=>{
 const row=handoff.lateMeasure,{directory,startedMs:base,child,repo}=progressFixture("measure-handoff-dir"),controller=new AbortController();
 let at=base,began=false,answer!:(sample:{footprintMegabytes:number;grantMegabytes:number;pressure:string})=>void;
 const measure=new Promise<{footprintMegabytes:number;grantMegabytes:number;pressure:string}>(resolve=>{answer=resolve;});
 const first=inferGraphReadV1(input(repo,directory,{signal:controller.signal,now:()=>at,policy:{...policy,memoryOverrunMilliseconds:row.after},measure:()=>{began=true;return measure;}}));
 try{
  at=base+row.after*2;
  await until("measurement began",()=>began);
  controller.abort();expect(await first).toMatchObject({detail:"detached"});
  answer({footprintMegabytes:row.maxMegabytes,grantMegabytes:row.grantMegabytes,pressure:"critical"});
  await sleep(policy.readPollMilliseconds*3);
  expect(existsSync(join(directory,GRAPH_READ_FILES_V1.stop))).toBe(false);
  expect(child.exitCode).toBeNull();
 }finally{
  controller.abort();await first.catch(()=>undefined);child.kill();await child.exited;
 }
},60000);

test("an exact runner rejects a foreign or mismatched-log checkpoint and keeps its fresh monitoring window",async()=>{
 const row=handoff.resume,{directory,startedMs:base,child,log,repo}=progressFixture("foreign-handoff-dir");
 const old=new AbortController(),file=join(directory,GRAPH_READ_FILES_V1.checkpoint);
 const graphPolicy={...policy,stallDumpMilliseconds:row.stallDumpAt,inferStallMilliseconds:row.stallDumpAt};
 let at=base+row.progressAt;
 appendFileSync(log,"graph: generation "+row.generation+" published\n");
 const first=inferGraphReadV1(input(repo,directory,{signal:old.signal,now:()=>at,policy:graphPolicy}));
 try{
  old.abort();await first;
  const saved=JSON.parse(readFileSync(file,"utf8"));
  for(const replacement of [{...saved,startedMs:base+1},{...saved,prefix:"0".repeat(64)},{...saved,offset:saved.offset+1}]){
   writeFileSync(file,JSON.stringify(replacement));
   at=base+row.adoptedAt;const next=new AbortController();
   const adopted=adoptGraphReadV1(input(repo,directory,{signal:next.signal,now:()=>at,policy:graphPolicy}));
   next.abort();
   expect(await adopted?.done).toMatchObject({detail:"detached",generation:row.generation});
   expect(existsSync(join(directory,GRAPH_READ_FILES_V1.stop))).toBe(false);
   expect(child.exitCode).toBeNull();
  }
 }finally{
  old.abort();await first.catch(()=>undefined);child.kill();await child.exited;
 }
},60000);

test("an authentic capacity wait pauses quiet across handoff while its release resumes the read clock",async()=>{
 const repo=fake("funded-queue",`echo start >> "$(dirname "$0")/starts"
while [ ! -f "$(dirname "$0")/finish" ]; do sleep 0.05; done
exit 0`),directory=temp("funded-queue"),controller=new AbortController();
 const timeline=handoff.capacityWait;let start=0,time:number|undefined,samples=0,waiting=true;
 const capacityWait=async()=>{samples++;return {id:"fixture-grant",scope:"fixture-scope",arrivedAtMs:start+timeline.arrival,reservedMegabytes:waiting?4096:14336,...waiting?{}:{bookedAtMs:start+timeline.booking}};};
 const options=input(repo,directory,{now:()=>time??Date.now(),signal:controller.signal,capacityWait,policy:{...policy,inferStallMilliseconds:1000,inferMaximumMilliseconds:10000,stallDumpMilliseconds:120,readBeatMilliseconds:500}});
 try{
  const run=inferGraphReadV1(options);
  await until("the read started",()=>count(repo.starts)===1&&existsSync(join(directory,GRAPH_READ_FILES_V1.record)));
  start=recordOf(directory)!.startedMs;time=start+timeline.firstProof;
  await until("the watchdog sampled the capacity queue or stopped",()=>samples>0||existsSync(join(directory,GRAPH_READ_FILES_V1.stop)));
  expect(samples).toBeGreaterThan(0);
  await sleep(policy.readPollMilliseconds);
  expect(existsSync(join(directory,GRAPH_READ_FILES_V1.stop))).toBe(false);
  const record=recordOf(directory)!;controller.abort();expect(await run).toMatchObject({detail:"detached"});
  const saved=JSON.parse(readFileSync(join(directory,GRAPH_READ_FILES_V1.checkpoint),"utf8")),pause=saved.capacityPause,db=new Database(":memory:");
  try{expect(db.query("select max(?1-max(?2,?3),0) as credit").get(pause.throughMs,pause.progressAtMs,start+timeline.arrival)).toEqual({credit:pause.creditedMs});}finally{db.close();}
  const adopted=adoptGraphReadV1({...options,signal:undefined})!;
  await until("the next observer samples the same wait",()=>samples>=2,3000);
  time=start+timeline.lastWait;
  await sleep(100);
  waiting=false;
  time=start+timeline.bookedProof;
  await sleep(100);
  time=start+timeline.afterBooking;await sleep(100);
  expect(alive(record.read!)).toBe(true);
  expect(existsSync(join(directory,GRAPH_READ_FILES_V1.stop))).toBe(false);
  writeFileSync(join(repo.dir,"finish"),"");
  expect(await adopted.done).toMatchObject({ok:true,status:0});
 }finally{
  controller.abort();writeFileSync(join(repo.dir,"finish"),"");
 }
},10000);

for(const row of [{name:"lost",verdict:"stalled"},{name:"absolute",verdict:"too-long"},{name:"memory",verdict:"memory-overrun"}] as const)test("the graph watchdog stops a "+row.name+" read despite prior capacity evidence",async()=>{
 const {directory,startedMs:base,child,repo}=progressFixture("wait-hard-"+row.name),controller=new AbortController();
 let at=base,samples=0;
 const run=inferGraphReadV1(input(repo,directory,{signal:controller.signal,now:()=>at,
  policy:{...policy,inferStallMilliseconds:600,inferMaximumMilliseconds:row.name==="absolute"?1500:5000,stallDumpMilliseconds:50,readBeatMilliseconds:500,memoryOverrunMilliseconds:1000},
  capacityWait:async()=>{samples++;return row.name==="lost"&&samples>1?undefined:{id:"fixture-grant",scope:"fixture-scope",arrivedAtMs:base+10,reservedMegabytes:4096};},
  measure:async()=>row.name==="memory"?{footprintMegabytes:4096*wantedServerPolicyV1.memoryOverrunFactor+1,grantMegabytes:undefined,pressure:"critical"}:undefined
 }));
 const stopPath=join(directory,GRAPH_READ_FILES_V1.stop);
 try{
  at=base+500;
  await until("verified capacity elapsed",()=>samples>0);
  await sleep(80);
  expect(existsSync(stopPath)).toBe(false);
  at=base+1600;
  await until("the hard policy stopped the read",()=>existsSync(stopPath));
  expect(JSON.parse(readFileSync(stopPath,"utf8")).verdict).toBe(row.verdict);
  expect(await run).toMatchObject({ok:false});
 }finally{controller.abort();child.kill();await child.exited;await run;}
},10000);

test("a capacity wait survives reattachment when the wall clock advances between watchdog observations",async()=>{
 const {directory,startedMs:base,child,repo}=progressFixture("capacity-clock"),first=new AbortController(),second=new AbortController();
 const row=handoff.capacityClockRace;
 let time=base,split=false,samples=0;
 const now=()=>split?time++:time;
 const capacityWait=async()=>{samples++;return {id:"fixture-grant",scope:"fixture-scope",arrivedAtMs:base+row.arrival,reservedMegabytes:4096};};
 const options=input(repo,directory,{now,signal:first.signal,capacityWait,policy:{...policy,inferStallMilliseconds:row.stall,stallDumpMilliseconds:row.cadence,readBeatMilliseconds:500}});
 const run=inferGraphReadV1(options),stop=join(directory,GRAPH_READ_FILES_V1.stop);
 let adopted:ReturnType<typeof adoptGraphReadV1>;
 try{
  time=base+row.sampled;
  await until("the initial capacity proof",()=>samples===1);
  await sleep(30);
  first.abort();await run;
  adopted=adoptGraphReadV1({...options,signal:second.signal});
  expect(adopted).toBeDefined();
  await until("the replacement observer verified the same exact wait",()=>samples===2);
  await sleep(30);
  const sql=new Database(":memory:");
  try{
   const expected=sql.query("select ?1 >= ?2 as rawStall, ?1-(?3-?4) >= ?2 as creditedStall").get(row.secondTick,row.stall,row.sampled,row.arrival);
   expect(expected).toEqual({rawStall:1,creditedStall:0});
  }finally{sql.close();}
  time=base+row.secondTick;split=true;
  await sleep(100);
  expect(existsSync(stop)).toBe(false);
  expect(alive(child.pid)).toBe(true);
  second.abort();expect(await adopted!.done).toMatchObject({detail:"detached"});
 }finally{first.abort();second.abort();child.kill();await child.exited;await run;await adopted?.done;}
},10000);

test("only an exact recoverable host-capacity wait pauses a graph rebuild watchdog",()=>{
 const db=new Database(":memory:");
 try{
  const truth=db.query("select case when ?1=?2 and ?3='wait' and ?4 not in ('never','starved') then 1 else 0 end as paused");
  for(const row of handoff.capacityDecisions){
   const {label,id,verdict,reason,paused}=row;
   expect(truth.get("exact",id,verdict,reason),label).toEqual({paused:Number(paused)});
   const decisions=[{id,verdict,reason,jobs:verdict==="admit"?1:0,reservedMegabytes:verdict==="admit"?14336:0}] as Parameters<typeof queuedGraphRebuildDecisionV1>[1];
   expect(queuedGraphRebuildDecisionV1("exact",decisions),label).toBe(paused);
  }
  expect(queuedGraphRebuildDecisionV1("exact",[])).toBe(false);
 }finally{db.close();}
});
