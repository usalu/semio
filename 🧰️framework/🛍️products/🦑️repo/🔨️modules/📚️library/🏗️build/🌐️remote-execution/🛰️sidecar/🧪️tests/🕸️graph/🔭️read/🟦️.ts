import Ajv from"ajv";
import{afterAll,expect,test}from"bun:test";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readReadRecordV1}from"../../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import schema from"../../../../../🪪️graph-state/🙋️wanted/🧬️schema/🔣️.json"with{type:"json"};
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
 await until("the runner stopped the read of the absent server",()=>recordOf(directory)?.status!==null&&recordOf(directory)?.status!==undefined,30000);
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
