import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {applySchedulerEventV1,decideSubmitV1,decideTickV1,emptySchedulerStateV1,formatSchedulerSnapshotV1,fleetFactOfSnapshotV1,replaySchedulerV1,schedulerLearnedPeaksV1,schedulerSnapshotV1,schedulerTextV1,schedulerTicketStateV1,SCHEDULER_POLICY_V1,type JobRequestV1,type SchedulerEventV1,type SchedulerPolicyV1} from "../🟦️.ts";
import {createFileSchedulerStoreV1} from "../💾️store/🟦️.ts";
import {runScheduledV1} from "../🎟️ticket/🟦️.ts";
import {runSchedulerCliV1} from "../⌨️command/🟦️.ts";
import {formatFleetReportV1,summarizeFleetV1,validateFleetFactsV1,validateFleetReportV1,type FleetFactsV1} from "../../🚢️fleet-status/🟦️.ts";
import {readFleetSchedulerV1} from "../../🚢️fleet-status/📥️readers/🟦️.ts";
import {requireTestToolV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";

const oracle=join(import.meta.dir,"🐍️.py"),ajv=new Ajv({strict:false,allowUnionTypes:true}),definitions=schema.definitions;
const validator=(name:string)=>ajv.compile({$schema:schema.$schema,definitions,$ref:`#/definitions/${name}`});
type Scenario={id:string;policy:Record<string,unknown>;budgetMegabytes:number;availableMegabytes?:number;jobs:{ticket:string;at:number;agent:string;durationMs:number;priority:"background"|"agent"|"interactive";depth:number;key:string;peakMegabytes:number;lane?:string;cancelAt?:number}[];starts:[string,number][]};
const scenarios=corpus.scenarios as unknown as Scenario[];
const policyOf=(overrides:Record<string,unknown>):SchedulerPolicyV1=>({...SCHEDULER_POLICY_V1,...overrides}) as SchedulerPolicyV1;
const requestOf=(job:Scenario["jobs"][number]):JobRequestV1=>({id:job.ticket,agent:job.agent,workspace:"w",class:"compile",priority:job.priority,depth:job.depth,key:job.key,...job.lane===undefined?{}:{lane:job.lane},estimate:{class:"phase:compile",floorMegabytes:0,learnedPeakMegabytes:job.peakMegabytes,limitMegabytes:job.peakMegabytes}});

function simulate(scenario:Scenario):[string,number][]{
 const policy=policyOf(scenario.policy),state=emptySchedulerStateV1(),starts:[string,number][]=[],ends=new Map<string,number>(),host={budgetMegabytes:scenario.budgetMegabytes,...scenario.availableMegabytes===undefined?{}:{availableMegabytes:scenario.availableMegabytes}};
 const specs=new Map(scenario.jobs.map(job=>[job.ticket,job])),apply=(events:readonly SchedulerEventV1[],at:number):void=>{
  for(const event of events){
   applySchedulerEventV1(policy,state,event);
   if(event.type==="started"){starts.push([event.jobId,at]);ends.set(event.jobId,at+specs.get(event.runner)!.durationMs);}
   if(event.type==="completed"||event.type==="expired")ends.delete(event.jobId);
  }
 };
 const times=new Set(scenario.jobs.flatMap(job=>[job.at,...job.cancelAt===undefined?[]:[job.cancelAt]]));
 let clock=-1;
 for(;;){
  const next=[...times,...ends.values()].filter(time=>time>clock);if(next.length===0)break;
  clock=Math.min(...next);
  for(const[id]of[...ends].filter(([,end])=>end<=clock).sort((a,b)=>a[1]-b[1]||state.jobs[a[0]]!.seq-state.jobs[b[0]]!.seq)){
   apply([{type:"completed",at:clock,jobId:id,exit:"ok"}],clock);
  }
  for(const job of scenario.jobs)if(job.cancelAt===clock){
   apply([{type:"cancelled",at:clock,ticket:job.ticket}],clock);
   const row=state.jobs[state.ticketJob[job.ticket]??""];
   if(row?.stopping&&row.runner===job.ticket)apply([{type:"completed",at:clock,jobId:row.id,exit:"cancelled"}],clock);
  }
  for(const job of scenario.jobs)if(job.at===clock)apply(decideSubmitV1(policy,state,requestOf(job),clock).events,clock);
  apply(decideTickV1(policy,state,clock,host),clock);
 }
 return starts;
}
const scenario=(id:string):Scenario=>scenarios.find(row=>row.id===id)!;
const job=(ticket:string,agent:string,over:Partial<Scenario["jobs"][number]>={}):Scenario["jobs"][number]=>({ticket,at:0,agent,durationMs:1000,priority:"agent",depth:0,key:ticket,peakMegabytes:512,...over});
const run=(policy:Record<string,unknown>,jobs:Scenario["jobs"],budgetMegabytes=8192):string[]=>simulate({id:"x",policy:{heartbeatTimeoutMs:1e9,...policy},budgetMegabytes,jobs,starts:[]}).map(([id])=>id);

test("the policy, the corpus, every event kind and a snapshot satisfy the closed schema (Ajv)",()=>{
 expect(validator("PolicySchema")(definitions.Policy.const)).toBe(true);
 expect(validator("Corpus")(corpus)).toBe(true);
 const request=requestOf(job("a","x")),event=validator("Event"),sample:SchedulerEventV1[]=[{type:"submitted",at:1,jobId:"a",request},{type:"attached",at:1,jobId:"a",subscriber:{ticket:"b",agent:"y",workspace:"w",priority:"agent",depth:1}},{type:"started",at:2,jobId:"a",runner:"a",reserveMegabytes:512},{type:"heartbeat",at:3,ticket:"a"},{type:"progressed",at:3,ticket:"a",progress:{phase:"p",done:1,total:2}},{type:"cancelled",at:4,ticket:"a"},{type:"lost",at:4,ticket:"a"},{type:"expired",at:4,jobId:"a"},{type:"completed",at:5,jobId:"a",exit:"ok",observedPeakMegabytes:400},{type:"pruned",at:6}];
 for(const row of sample)expect(event(row),JSON.stringify(row)).toBe(true);
 expect(validator("JobRequest")(request)).toBe(true);
 const state=emptySchedulerStateV1(),policy=SCHEDULER_POLICY_V1;
 replaySchedulerV1(policy,sample.slice(0,3),state);
 expect(validator("Snapshot")(schedulerSnapshotV1(policy,state,10,{budgetMegabytes:100}))).toBe(true);
});

test("every text exists in English and German with identical placeholders",()=>{
 for(const[key,row]of Object.entries(schema.definitions.Messages.const as Record<string,{en:string;de:string}>)){
  expect(row.en.length>0&&row.de.length>0,key).toBe(true);
  const names=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(names(row.de),key).toEqual(names(row.en));
 }
 expect(schedulerTextV1("de","idle")).not.toBe(schedulerTextV1("en","idle"));
});

test("the start order of every checked-in scenario equals the Python oracle recorded in the corpus",()=>{
 for(const row of scenarios)expect(simulate(row),row.id).toEqual(row.starts);
});

test("the start order of 80 seeded random scenarios equals a live run of the independent Python oracle",()=>{
 const result=spawnSync(requireTestToolV1("python"),[oracle,"generate","1000","80"],{encoding:"utf8",maxBuffer:1<<28});
 expect(result.status,result.stderr).toBe(0);
 const rows=result.stdout.split("\n").filter(Boolean).map(line=>JSON.parse(line) as Scenario);
 expect(rows.length).toBe(80);
 for(const row of rows)expect(simulate(row),row.id).toEqual(row.starts);
});

test("shared crates (deeper dependency chains) start before their dependents within one agent",()=>{
 expect(run({maxConcurrent:1},[job("leaf","a",{depth:0}),job("mid","a",{depth:2}),job("core","a",{depth:5})])).toEqual(["core","mid","leaf"]);
});

test("two agents alternate while one agent has a longer queue",()=>{
 const jobs=[...[1,2,3,4,5,6].map(n=>job(`a${n}`,"a")),...[1,2].map(n=>job(`b${n}`,"b"))];
 expect(run({maxConcurrent:1,maxPerAgent:1},jobs).slice(0,4).map(id=>id[0])).toEqual(["a","b","a","b"]);
});

test("identical requests of two agents share one run and one start",()=>{
 const policy=SCHEDULER_POLICY_V1,state=emptySchedulerStateV1(),first=decideSubmitV1(policy,state,requestOf(job("t1","a",{key:"core:build"})),0);
 replaySchedulerV1(policy,first.events,state);
 const second=decideSubmitV1(policy,state,requestOf(job("t2","b",{key:"core:build",priority:"interactive",depth:4})),5);
 expect(second.outcome).toEqual({kind:"attached",jobId:"t1"});replaySchedulerV1(policy,second.events,state);
 expect(Object.keys(state.jobs)).toEqual(["t1"]);expect(state.jobs.t1!.priority).toBe("interactive");expect(state.jobs.t1!.depth).toBe(4);
 replaySchedulerV1(policy,decideTickV1(policy,state,6,{budgetMegabytes:4096}),state);
 expect(schedulerTicketStateV1(policy,state,"t2",6)).toEqual({status:"running",role:"watcher"});
 expect(schedulerTicketStateV1(policy,state,"t1",6)).toEqual({status:"running",role:"runner"});
 replaySchedulerV1(policy,[{type:"completed",at:9,jobId:"t1",exit:"ok"}],state);
 for(const ticket of["t1","t2"])expect(schedulerTicketStateV1(policy,state,ticket,9)).toEqual({status:"finished",exit:"ok"});
 expect(decideSubmitV1(policy,state,requestOf(job("t1","a")),10).outcome.kind).toBe("duplicate");
});

test("a background job is not starved: aging lifts it above a stream of fresh interactive jobs",()=>{
 const stream=Array.from({length:12},(_,n)=>job(`i${n}`,`agent${n}`,{at:n*200,priority:"interactive",durationMs:1000})),jobs=[job("old","z",{priority:"background"}),...stream];
 const starts=simulate({id:"x",policy:{maxConcurrent:1,maxPerAgent:1,agingMs:1000,heartbeatTimeoutMs:1e9},budgetMegabytes:8192,jobs,starts:[]}),at=starts.find(([id])=>id==="old")![1];
 expect(at).toBeLessThanOrEqual(4000);
});

test("admission is preemption-free: nothing is stopped, a blocked aged head drains the host before smaller jobs backfill past it",()=>{
 const jobs=[job("run","a",{peakMegabytes:3000,durationMs:5000}),job("big","b",{peakMegabytes:3000,at:100,durationMs:1000}),job("small","c",{peakMegabytes:500,at:200,durationMs:1000})];
 expect(run({agingMs:100000},jobs,4000)).toEqual(["run","small","big"]);
 expect(run({agingMs:50},jobs,4000)).toEqual(["run","big","small"]);
});

test("a job larger than the whole budget runs alone instead of deadlocking, and a low MemAvailable only blocks while another job runs",()=>{
 expect(run({},[job("huge","a",{peakMegabytes:20000}),job("next","b",{at:10})],4096)).toEqual(["huge","next"]);
 const starts=simulate({id:"x",policy:{heartbeatTimeoutMs:1e9},budgetMegabytes:8192,availableMegabytes:1000,jobs:[job("p","a",{peakMegabytes:600}),job("q","b",{peakMegabytes:600})],starts:[]});
 expect(starts).toEqual([["p",0],["q",1000]]);
});

test("shared compiler lanes cap concurrent rustc and tsc jobs",()=>{
 const jobs=[1,2,3].map(n=>job(`r${n}`,`a${n}`,{lane:"rustc"})),starts=simulate({id:"x",policy:{sharedCompilerLanes:{rustc:1,tsc:2},heartbeatTimeoutMs:1e9},budgetMegabytes:99999,jobs,starts:[]});
 expect(starts.map(([,at])=>at)).toEqual([0,1000,2000]);
});

test("cancellation: a queued job vanishes, a running runner is told to stop, watchers outlive a cancelled watcher",()=>{
 const policy=SCHEDULER_POLICY_V1,state=emptySchedulerStateV1(),host={budgetMegabytes:100000};
 const submit=(id:string,key:string,at:number)=>replaySchedulerV1(policy,decideSubmitV1(policy,state,requestOf(job(id,id,{key})),at).events,state);
 submit("one","k1",0);submit("two","k2",0);submit("three","k1",1);
 replaySchedulerV1(policy,decideTickV1(policy,state,2,{...host,budgetMegabytes:600}),state);
 expect(schedulerTicketStateV1(policy,state,"two",2).status).toBe("queued");
 replaySchedulerV1(policy,[{type:"cancelled",at:3,ticket:"two"}],state);
 expect(state.jobs.two).toBeUndefined();expect(schedulerTicketStateV1(policy,state,"two",3)).toEqual({status:"finished",exit:"cancelled"});
 replaySchedulerV1(policy,[{type:"cancelled",at:4,ticket:"three"}],state);
 expect(schedulerTicketStateV1(policy,state,"one",4).stopping).toBeUndefined();
 replaySchedulerV1(policy,[{type:"cancelled",at:5,ticket:"one"}],state);
 expect(schedulerTicketStateV1(policy,state,"one",5)).toMatchObject({status:"running",stopping:true});
 replaySchedulerV1(policy,[{type:"completed",at:6,jobId:"one",exit:"cancelled"}],state);
 expect(state.finished.at(-1)!.exit).toBe("cancelled");
});

test("a silent ticket is declared lost after the heartbeat timeout; a lost runner ends the shared job",()=>{
 const policy=policyOf({heartbeatTimeoutMs:1000}),state=emptySchedulerStateV1(),host={budgetMegabytes:100000};
 replaySchedulerV1(policy,decideSubmitV1(policy,state,requestOf(job("r","a")),0).events,state);
 replaySchedulerV1(policy,decideTickV1(policy,state,0,host),state);
 replaySchedulerV1(policy,decideSubmitV1(policy,state,requestOf(job("w","b",{key:"r"})),500).events,state);
 replaySchedulerV1(policy,[{type:"heartbeat",at:1200,ticket:"w"}],state);
 const events=decideTickV1(policy,state,1500,host);
 expect(events.map(event=>event.type)).toEqual(["lost"]);replaySchedulerV1(policy,events,state);
 expect(state.finished.at(-1)).toMatchObject({jobId:"r",exit:"runner-lost"});
 expect(schedulerTicketStateV1(policy,state,"w",1500)).toEqual({status:"finished",exit:"runner-lost"});
});

test("a queued job past its deadline expires",()=>{
 const policy=policyOf({maxConcurrent:1}),state=emptySchedulerStateV1(),host={budgetMegabytes:100000};
 replaySchedulerV1(policy,decideSubmitV1(policy,state,requestOf(job("a","a")),0).events,state);
 replaySchedulerV1(policy,decideTickV1(policy,state,0,host),state);
 replaySchedulerV1(policy,decideSubmitV1(policy,state,{...requestOf(job("b","b")),deadlineMs:100},1).events,state);
 replaySchedulerV1(policy,decideTickV1(policy,state,200,host),state);
 expect(state.finished.at(-1)).toMatchObject({jobId:"b",exit:"expired"});
});

test("memory is bounded: queue refuses beyond maxQueued, finished and idle agents are capped",()=>{
 const policy=policyOf({maxQueued:3,maxFinished:4,maxAgents:2,maxConcurrent:1}),state=emptySchedulerStateV1();
 const outcomes=[0,1,2,3,4].map(n=>{const decision=decideSubmitV1(policy,state,requestOf(job(`j${n}`,`agent${n}`)),0);replaySchedulerV1(policy,decision.events,state);return decision.outcome.kind;});
 expect(outcomes).toEqual(["queued","queued","queued","busy","busy"]);
 for(let n=0;n<10;n++){
  replaySchedulerV1(policy,decideTickV1(policy,state,n*10,{budgetMegabytes:9999}),state);
  const running=Object.values(state.jobs).find(row=>row.status==="running");if(running)replaySchedulerV1(policy,[{type:"completed",at:n*10+1,jobId:running.id,exit:"ok"}],state);
  replaySchedulerV1(policy,decideSubmitV1(policy,state,requestOf(job(`k${n}`,`agent${n+9}`)),n*10).events,state);
 }
 expect(state.finished.length).toBeLessThanOrEqual(4);expect(Object.keys(state.agents).length).toBeLessThanOrEqual(policy.maxAgents+policy.maxQueued+1);
 const attached=policyOf({maxAttached:2}),other=emptySchedulerStateV1();
 for(const id of["x","y"])replaySchedulerV1(attached,decideSubmitV1(attached,other,requestOf(job(id,id,{key:"same"})),0).events,other);
 expect(decideSubmitV1(attached,other,requestOf(job("z","z",{key:"same"})),0).outcome.kind).toBe("busy");
});

test("fairness: 16 agents with identical load get a max/min throughput ratio of at most 1.5 and nobody waits longer than the aging bound",()=>{
 const jobs=Array.from({length:16*6},(_,n)=>job(`j${n}`,`agent${n%16}`,{at:Math.floor(n/16)*10,durationMs:1000,peakMegabytes:1024,key:`j${n}`}));
 const starts=simulate({id:"x",policy:{maxConcurrent:4,maxPerAgent:1,heartbeatTimeoutMs:1e9},budgetMegabytes:99999,jobs,starts:[]}),window=starts.filter(([,at])=>at<=12000),served=new Map<string,number>();
 for(const[id]of window){const agent=jobs.find(row=>row.ticket===id)!.agent;served.set(agent,(served.get(agent)??0)+1);}
 const counts=[...served.values()];expect(served.size).toBe(16);expect(Math.max(...counts)/Math.min(...counts)).toBeLessThanOrEqual(1.5);
});

test("the log is the truth: replaying the events yields the same state, and snapshots render in both languages",()=>{
 const row=scenario("mixed-agents"),policy=policyOf(row.policy),live=emptySchedulerStateV1(),log:SchedulerEventV1[]=[];
 for(const spec of row.jobs.slice(0,6)){const decision=decideSubmitV1(policy,live,requestOf(spec),spec.at);replaySchedulerV1(policy,decision.events,live);log.push(...decision.events);}
 const events=decideTickV1(policy,live,10,{budgetMegabytes:row.budgetMegabytes});replaySchedulerV1(policy,events,live);log.push(...events);
 expect(replaySchedulerV1(policy,log)).toEqual(live);
 const snapshot=schedulerSnapshotV1(policy,live,20,{budgetMegabytes:row.budgetMegabytes});
 expect(formatSchedulerSnapshotV1("en",snapshot)[0]).toContain("running");expect(formatSchedulerSnapshotV1("de",snapshot)[0]).toContain("laufen");
 expect(formatSchedulerSnapshotV1("en",schedulerSnapshotV1(policy,emptySchedulerStateV1(),0,{budgetMegabytes:1}))).toEqual(["Scheduler: idle"]);
});

const tempStore=(overrides:Record<string,unknown>={},host={budgetMegabytes:100000})=>{
 const directory=mkdtempSync(join(tmpdir(),"scheduler-")),policy=policyOf({pollMs:5,heartbeatTimeoutMs:60000,...overrides});
 return{directory,policy,store:createFileSchedulerStoreV1(directory,policy,{host:()=>host}),dispose:()=>rmSync(directory,{recursive:true,force:true})};
};

test("the file store replays after compaction, survives a torn last line and serialises concurrent writers",async()=>{
 const t=tempStore({compactEveryEvents:5});
 try{
  await Promise.all(Array.from({length:12},(_,n)=>t.store.transact((state,at)=>({events:decideSubmitV1(t.policy,state,requestOf(job(`c${n}`,`agent${n}`,{peakMegabytes:100})),at).events,result:n}))));
  const reopened=createFileSchedulerStoreV1(t.directory,t.policy,{host:()=>({budgetMegabytes:100000})}),snapshot=await reopened.snapshot();
  expect(snapshot.running.length+snapshot.queued.length).toBe(12);
  const logPath=join(t.directory,"log.ndjson");writeFileSync(logPath,readFileSync(logPath,"utf8")+'{"n":9999,"e":{"type":"ca');
  expect((await reopened.snapshot()).running.length+(await reopened.snapshot()).queued.length).toBe(12);
 }finally{t.dispose();}
});

test("two agents asking for the same target share one run, progress is visible and the work runs exactly once",async()=>{
 const t=tempStore();let runs=0;
 try{
  const work=async(context:{progress:(p:{phase:string;done:number;total:number})=>void})=>{runs++;context.progress({phase:"compile",done:1,total:2});await new Promise(resolve=>setTimeout(resolve,60));return{exit:"ok" as const,observedPeakMegabytes:300};};
  const first=runScheduledV1(t.store,requestOf(job("p1","a",{key:"core:build"})),work);
  await new Promise(resolve=>setTimeout(resolve,15));
  const second=runScheduledV1(t.store,requestOf(job("p2","b",{key:"core:build"})),work);
  await new Promise(resolve=>setTimeout(resolve,25));
  const mid=await t.store.snapshot();expect(mid.running.length).toBe(1);expect(mid.running[0]!.agents).toEqual(["a","b"]);
  expect(mid.running[0]!.progress).toEqual({phase:"compile",done:1,total:2});
  const[one,two]=await Promise.all([first,second]);
  expect(runs).toBe(1);expect([one.exit,two.exit]).toEqual(["ok","ok"]);expect([one.shared,two.shared]).toEqual([false,true]);
 }finally{t.dispose();}
});

test("queued jobs wait for capacity in priority order, report their position and a cancel signal aborts a running job",async()=>{
 const t=tempStore({maxConcurrent:1}),positions:number[]=[],order:string[]=[];
 try{
  const controller=new AbortController(),work=(name:string,ms:number)=>async(context:{signal:AbortSignal})=>{order.push(name);await new Promise<void>(resolve=>{const timer=setTimeout(resolve,ms);context.signal.addEventListener("abort",()=>{clearTimeout(timer);resolve();});});return{exit:context.signal.aborted?"cancelled" as const:"ok" as const};};
  const first=runScheduledV1(t.store,requestOf(job("q1","a",{depth:0})),work("q1",40),{});
  await new Promise(resolve=>setTimeout(resolve,20));
  const low=runScheduledV1(t.store,requestOf(job("q2","b",{depth:1})),work("q2",10),{onQueued:position=>positions.push(position)});
  const high=runScheduledV1(t.store,requestOf(job("q3","c",{depth:9})),work("q3",10));
  await Promise.all([first,low,high]);
  expect(order).toEqual(["q1","q3","q2"]);expect(positions).toContain(2);
  const victim=runScheduledV1(t.store,requestOf(job("v1","a",{key:"v"})),work("v1",5000),{signal:controller.signal});
  await new Promise(resolve=>setTimeout(resolve,40));controller.abort();
  const outcome=await victim;expect(outcome.exit).toBe("cancelled");
  expect((await t.store.snapshot()).running.length).toBe(0);
 }finally{t.dispose();}
});

test("`repo scheduler status` and `cancel` read and change the same store",async()=>{
 const t=tempStore({maxConcurrent:1});
 try{
  const lines:string[]=[],write=(line:string)=>lines.push(line);
  await t.store.transact((state,at)=>({events:decideSubmitV1(t.policy,state,requestOf(job("s1","a")),at).events,result:0}));
  await t.store.transact((state,at)=>({events:decideSubmitV1(t.policy,state,requestOf(job("s2","b")),at).events,result:0}));
  expect(await runSchedulerCliV1("/w",["status","--lang=de"],{},write,{store:t.store})).toBe(0);expect(lines[0]).toContain("laufen");
  lines.length=0;expect(await runSchedulerCliV1("/w",["status","--json","--lang=en"],{},write,{store:t.store})).toBe(0);expect(JSON.parse(lines.join("")).queued[0].jobId).toBe("s2");
  lines.length=0;expect(await runSchedulerCliV1("/w",["cancel","s2","--lang=en"],{},write,{store:t.store})).toBe(0);expect(lines[0]).toBe("Ticket s2 cancelled");
  lines.length=0;expect(await runSchedulerCliV1("/w",["cancel","nope","--lang=en"],{},write,{store:t.store})).toBe(1);
  await expect(runSchedulerCliV1("/w",["bogus"],{SEMIO_LANGUAGE:"en"},write,{store:t.store})).rejects.toThrow("usage");
 }finally{t.dispose();}
});

test("`repo scheduler run -- <command>` queues a child process and returns its exit code",async()=>{
 const t=tempStore({maxConcurrent:1}),home=mkdtempSync(join(tmpdir(),"scheduler-home-"));
 try{
  const environment={...process.env,SEMIO_LANGUAGE:"en",HOME:home,XDG_CACHE_HOME:join(home,"cache")},lines:string[]=[];
  expect(await runSchedulerCliV1("/w",["run","--key=ok-run","--peak-mb=1024","--","node","-e","process.exit(0)"],environment,line=>lines.push(line),{store:t.store})).toBe(0);
  expect(await runSchedulerCliV1("/w",["run","--key=bad-run","--","node","-e","process.exit(3)"],environment,line=>lines.push(line),{store:t.store})).toBe(1);
  expect((await t.store.snapshot()).running.length).toBe(0);
 }finally{t.dispose();rmSync(home,{recursive:true,force:true});}
});

test("`repo fleet status` shows the scheduler section in both languages, validated against the fleet schemas, read without creating or locking the store",async()=>{
 const t=tempStore({maxConcurrent:1});
 try{
  await t.store.transact((state,at)=>({events:decideSubmitV1(t.policy,state,requestOf(job("f1","a",{key:"core:build",peakMegabytes:2048})),at).events,result:0}));
  await t.store.transact((state,at)=>({events:decideSubmitV1(t.policy,state,requestOf(job("f2","b",{key:"core:build"})),at).events,result:0}));
  await t.store.transact((state,at)=>({events:decideSubmitV1(t.policy,state,requestOf(job("f3","b",{key:"app:build",depth:3})),at).events,result:0}));
  const empty=mkdtempSync(join(tmpdir(),"scheduler-empty-")),absent=join(empty,"none");
  expect(await readFleetSchedulerV1({root:"/w",environment:{SEMIO_SCHEDULER_DIRECTORY:absent} as NodeJS.ProcessEnv,language:"en",now:Date.now})).toMatchObject({scheduler:{running:[],queued:[]}});
  expect(existsSync(absent)).toBe(false);rmSync(empty,{recursive:true,force:true});
  const read=await readFleetSchedulerV1({root:"/w",environment:{SEMIO_SCHEDULER_DIRECTORY:t.directory} as NodeJS.ProcessEnv,language:"en",now:Date.now}),scheduler=read.scheduler!;
  expect({...scheduler,budgetMegabytes:0}).toEqual({...fleetFactOfSnapshotV1(await t.store.snapshot()),budgetMegabytes:0});
  const facts:FleetFactsV1={version:1,atMs:1_793_000_000_000,workspace:"/w",scheduler,sources:[{id:"scheduler",ok:true,milliseconds:1}]};
  expect(validateFleetFactsV1(facts)).toEqual([]);
  const report=summarizeFleetV1(facts,"en");
  expect(validateFleetReportV1(report)).toEqual([]);
  expect(report.scheduler).toMatchObject({running:1,queued:1,sharedTickets:1,reservedMegabytes:2048});
  const english=formatFleetReportV1("en",report).filter(line=>line.includes("core:build")||line.includes("app:build")||line.startsWith("Scheduler")),german=formatFleetReportV1("de",report).filter(line=>line.includes("core:build")||line.includes("app:build")||line.startsWith("Planer"));
  expect(english.length).toBe(3);expect(german.length).toBe(3);expect(english[0]).toContain("1 shared request(s)");expect(german[0]).toContain("geteilte Anfrage");
  expect(english[2]).toContain("#1 app:build");
  const without=summarizeFleetV1({...facts,scheduler:undefined,sources:[]} as FleetFactsV1,"en");
  expect("scheduler" in without).toBe(false);
 }finally{t.dispose();}
});

test("a limited run that exceeds its memory fails alone with a typed outcome and its observed peak is what the job-limits lane learns",async()=>{
 const learned:ReturnType<typeof schedulerLearnedPeaksV1>=[],t=tempStore({maxConcurrent:2}),lines:string[]=[];
 const store=createFileSchedulerStoreV1(t.directory,t.policy,{host:()=>({budgetMegabytes:100000}),onEvents:(events,state)=>{learned.push(...schedulerLearnedPeaksV1(events,state));}});
 try{
  const limited=async(job:{job:string;policy:{memoryMegabytes:number}})=>({kind:"memory-exceeded" as const,enforcement:"fake" as const,limitMegabytes:job.policy.memoryMegabytes,peakMegabytes:job.policy.memoryMegabytes+50,exitCode:null,signal:"SIGKILL",durationMilliseconds:5});
  const code=await runSchedulerCliV1("/w",["run","--key=hog","--peak-mb=256","--","never"],{...process.env,SEMIO_LANGUAGE:"de",HOME:t.directory},line=>lines.push(line),{store,limited:limited as never});
  expect(code).toBe(1);expect(lines.join(" ")).toContain("256");
  expect(learned).toEqual([{class:"phase:compile",peakMegabytes:320,exit:"memory-exceeded"}]);
  expect((await store.snapshot()).finished.at(-1)).toMatchObject({key:"hog",exit:"memory-exceeded",estimateClass:"phase:compile"});
 }finally{t.dispose();}
});
